use clap::{App, Arg};
use std::rc::Rc;
use auditor::cfg::CfgBuildError;
use auditor::wasm_wrapper::wasm;
use auditor::fmt::CodeWriter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("auditor")
        .version(VERSION)
        .arg(
            Arg::with_name("show-graph")
                .long("show-graph")
                .help("Print the constructed CFG in dot format before structuring"),
        )
        .arg(
            Arg::with_name("file")
                .help("The wasm binary to decompile")
                .required(true),
        )
        .arg(Arg::with_name("function_name").help("The index of the function to decompile"))
        .get_matches();

    let file_path = args.value_of("file").unwrap();

    let wasm = match wasm::Instance::from_file(file_path) {
        Ok(instance) => Rc::new(instance),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let _show_graph = args.is_present("show-graph");

    if let Some(func_index) = args.value_of("function_name") {
        let func_index = func_index.parse().unwrap();
        let mut printer = CodeWriter::printer(wasm, func_index);
        match printer.decompile_func(func_index) {
            Ok(()) => (),
            Err(CfgBuildError::NoSuchFunc) => eprintln!("No function with index {}", func_index),
            Err(CfgBuildError::FuncIsImported) => {
                eprintln!("Function {} is imported and can not be decompiled", func_index)
            }
        }
    } else {
        for (i, func) in wasm.module().functions().iter().enumerate() {
            if !func.is_imported() {
                let mut printer = CodeWriter::printer(wasm.clone(), i as u32);
                let name = func.name();
                eprintln!("//Decompiling {}({})", name, i);
                printer.decompile_func(i as u32).unwrap();
                println!();
            }
        }
    }
}

