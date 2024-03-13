use soroban_sdk::xdr::ScSpecEntry;
use std::io::{self, Read};
use std::rc::Rc;

use clap::{App, Arg};

use quote::{format_ident, quote};

use std::fs::File;
use auditor::analysis;
use auditor::cfg::{Cfg, CfgBuildError};
use auditor::fmt;
use auditor::ssa;
use auditor::structuring;
use auditor::wasm;

const VERSION: &str = env!("CARGO_PKG_VERSION");
use soroban_spec::read::{ from_wasm };
use soroban_spec_rust::types::{generate_enum, generate_error_enum, generate_struct, generate_union, generate_type_ident};

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
        .arg(Arg::with_name("function").help("The index of the function to decompile"))
        .get_matches();

    let file_path = args.value_of("file").unwrap();
    let specs = read_contract_specs(file_path);

    let wasm = match wasm::Instance::from_file(file_path) {
        Ok(instance) => Rc::new(instance),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let show_graph = args.is_present("show-graph");

    if let Some(func_index) = args.value_of("function_name") {
        let func_index = func_index.parse().unwrap();
        match decompile_func(wasm, func_index, show_graph) {
            Ok(()) => (),
            Err(CfgBuildError::NoSuchFunc) => eprintln!("No function with index {}", func_index),
            Err(CfgBuildError::FuncIsImported) => {
                eprintln!("Function {} is imported and can not be decompiled", func_index)
            }
        }
    } else {
        for (i, func) in wasm.module().functions().iter().enumerate() {
            if !func.is_imported() {
                eprintln!("Decompiling f{}", i);
                decompile_func(Rc::clone(&wasm), i as u32, show_graph).unwrap();
                println!();
            }
        }
    }
}

fn read_contract_specs(file_path: &str) -> io::Result<Vec<ScSpecEntry>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let entries = from_wasm(&buffer).unwrap();

    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    let mut spec_enums = Vec::new();
    let mut spec_error_enums = Vec::new();
    for s in entries.iter() {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
            ScSpecEntry::UdtEnumV0(e) => spec_enums.push(e),
            ScSpecEntry::UdtErrorEnumV0(e) => spec_error_enums.push(e),
        }
    }

    let trait_name = "Contract";
    let trait_ident = format_ident!("{}", trait_name);

    let fns: Vec<_> = spec_fns
        .iter()
        .map(|s| {
            let name = s.name.to_utf8_string().unwrap();
            let fn_ident = format_ident!("{}", name);
            let fn_inputs = s.inputs.iter().map(|input| {
                let name = format_ident!("{}", input.name.to_utf8_string().unwrap());
                let type_ident = generate_type_ident(&input.type_);
                quote! { #name: #type_ident }
            });
            let fn_output = s
                .outputs
                .to_option()
                .map(|t| generate_type_ident(&t))
                .map(|t| quote! { -> #t });
            quote! {
                fn #fn_ident(env: soroban_sdk::Env, #(#fn_inputs),*) #fn_output 
            }
        })
        .collect();

    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    let enums = spec_enums.iter().map(|s| generate_enum(s));
    let error_enums = spec_error_enums.iter().map(|s| generate_error_enum(s)); 

    Ok(entries)
}

fn decompile_func(wasm: Rc<wasm::Instance>, func_index: u32, print_graph: bool, specs) -> Result<(), CfgBuildError> {
    let mut cfg = Cfg::build(Rc::clone(&wasm), func_index)?;
    let mut def_use_map = ssa::transform_to_ssa(&mut cfg);

    ssa::transform_out_of_ssa(&mut cfg);

    if print_graph {
        println!("{}", cfg.dot_string());
    }

    let (decls, code) = structuring::structure(cfg);
    fmt::CodeWriter::printer(wasm, func_index).write_func(func_index, &decls, &code);
    Ok(())
}
