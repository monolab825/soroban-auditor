use std::io::{self, Read};
use soroban_sdk::xdr::ScSpecEntry;
use std::fs::File;
use std::fmt;
use quote::format_ident;
use soroban_spec::read::from_wasm;
use soroban_spec_rust::types::{generate_enum, generate_error_enum, generate_struct, generate_union, generate_type_ident};
use soroban_sdk::xdr::ScSpecTypeDef;

// Custom data structure to represent a function
#[derive(Debug)]
pub struct FunctionInfo {
    name: String,
    inputs: Vec<(String, String)>,
    output: Option<String>,
}

impl FunctionInfo {
    // Public method to access the `name` field
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs_str: Vec<String> = self
            .inputs
            .iter()
            .map(|(name, type_ident)| format!("{}: {}", name, type_ident))
            .collect();

        let output_str = self.output.as_deref().unwrap_or("");

        write!(f, "fn {}({}) {}", self.name, inputs_str.join(", "), output_str)
    }
}

pub fn find_function_in_specs(spec_fns_result: &io::Result<Vec<FunctionInfo>>, function_name_to_find: &str) {
    // Use if let to handle the result
    if let Ok(spec_fns) = spec_fns_result {
        if let Some(found_function) = spec_fns.iter().find(|&s| s.name() == function_name_to_find) {
            println!("Found function specs : {}", found_function.name());
        } else {
            println!("Function specs not found: {}", function_name_to_find);
        }
    } else {
        // Handle the error outside the match block
        eprintln!("Error reading contract specs: {}", spec_fns_result.as_ref().err().unwrap());
    }
}

pub fn read_contract_specs(file_path: &str) -> io::Result<Vec<FunctionInfo>> {
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

    let fns: Vec<_> = spec_fns
        .iter()
        .map(|s| {
            let name = s.name.to_utf8_string().unwrap();
            let inputs: Vec<_> = s.inputs.iter().map(|input| {
                let name = input.name.to_utf8_string().unwrap();
                let type_ident = generate_type_ident_string(&input.type_);
                (name, type_ident)
            }).collect();

            let output = s
                .outputs
                .to_option()
                .map(|t| generate_type_ident_string(&t));

            FunctionInfo {
                name,
                inputs,
                output,
            }
        })
        .collect();

    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    let enums = spec_enums.iter().map(|s| generate_enum(s));
    let error_enums = spec_error_enums.iter().map(|s| generate_error_enum(s)); 

    Ok(fns)
}

pub fn generate_type_ident_string(spec: &ScSpecTypeDef) -> String {
    match spec {
        ScSpecTypeDef::Val => "soroban_sdk::Val".to_string(),
        ScSpecTypeDef::U64 => "u64".to_string(),
        ScSpecTypeDef::I64 => "i64".to_string(),
        ScSpecTypeDef::U32 => "u32".to_string(),
        ScSpecTypeDef::I32 => "i32".to_string(),
        ScSpecTypeDef::U128 => "u128".to_string(),
        ScSpecTypeDef::I128 => "i128".to_string(),
        ScSpecTypeDef::Bool => "bool".to_string(),
        ScSpecTypeDef::Symbol => "soroban_sdk::Symbol".to_string(),
        ScSpecTypeDef::Error => "soroban_sdk::Error".to_string(),
        ScSpecTypeDef::Bytes => "soroban_sdk::Bytes".to_string(),
        ScSpecTypeDef::Address => "soroban_sdk::Address".to_string(),
        ScSpecTypeDef::String => "soroban_sdk::String".to_string(),
        ScSpecTypeDef::Option(o) => {
            let value_ident = generate_type_ident(&o.value_type);
            format!("Option<{}>", value_ident)
        }
        ScSpecTypeDef::Result(r) => {
            let ok_ident = generate_type_ident(&r.ok_type);
            let error_ident = generate_type_ident(&r.error_type);
            format!("Result<{}, {}>", ok_ident, error_ident)
        }
        ScSpecTypeDef::Vec(v) => {
            let element_ident = generate_type_ident(&v.element_type);
            format!("soroban_sdk::Vec<{}>", element_ident)
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident(&m.key_type);
            let value_ident = generate_type_ident(&m.value_type);
            format!("soroban_sdk::Map<{}, {}>", key_ident, value_ident)
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents: Vec<_> = t.value_types.iter().map(|ty| generate_type_ident(ty)).collect();
            format!("({})", type_idents.iter().map(ToString::to_string).collect::<Vec<_>>().join(", "))
        }
        ScSpecTypeDef::BytesN(b) => format!("soroban_sdk::BytesN<{}>", b.n),
        ScSpecTypeDef::Udt(u) => format_ident!("{}", u.name.to_utf8_string().unwrap()).to_string(),
        ScSpecTypeDef::Void => "()".to_string(),
        ScSpecTypeDef::Timepoint => "soroban_sdk::Timepoint".to_string(),
        ScSpecTypeDef::Duration => "soroban_sdk::Duration".to_string(),
        ScSpecTypeDef::U256 => "soroban_sdk::U256".to_string(),
        ScSpecTypeDef::I256 => "soroban_sdk::I256".to_string(),
    }
}
