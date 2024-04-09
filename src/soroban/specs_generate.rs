use soroban_spec_rust::types::generate_error_enum;
use soroban_spec_rust::types::generate_enum;
use soroban_spec_rust::types::generate_union;
use soroban_spec_rust::types::generate_struct;
use parity_wasm::elements::ValueType;
use crate::wasm_wrapper::wasm_adapter::ExtendedValueType;
use std::io::Read;
use soroban_sdk::xdr::ScSpecEntry;
use std::fs::File;
use std::fmt;
use soroban_spec::read::from_wasm;
// use soroban_spec_rust::types::{generate_enum, generate_error_enum, generate_struct, generate_union};
use soroban_sdk::xdr::ScSpecTypeDef;

// Updated struct to represent function parameters with extended types
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ExtendedFunctionParam {
    name: String,
    type_ident: ExtendedValueType,
}

impl ExtendedFunctionParam {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_ident(&self) -> &ExtendedValueType {
        &self.type_ident
    }
}

impl fmt::Display for ExtendedFunctionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_ident)
    }
}

// Updated struct to represent function return type with extended types
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ExtendedFunctionReturnType {
    type_ident: ExtendedValueType,
}

impl ExtendedFunctionReturnType {
    pub fn type_ident(&self) -> &ExtendedValueType {
        &self.type_ident
    }
}

impl fmt::Display for ExtendedFunctionReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement formatting logic here
         write!(f, "Formatted representation of ExtendedFunctionReturnType")
    }
}
// Updated struct to represent function information with extended types
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FunctionInfo {
    name: String,
    inputs: Vec<ExtendedFunctionParam>,
    output: Option<ExtendedFunctionReturnType>,
}

// Implementation of FunctionInfo methods
impl FunctionInfo {
    pub fn default() -> Self {
        FunctionInfo {
            name: String::new(),
            inputs: Vec::new(),
            output: None,
        }
    }

    pub fn default_inputs(inputs: Vec<ExtendedFunctionParam>) -> Self {
        FunctionInfo {
            name: String::new(),
            inputs,
            output: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inputs(&self) -> &[ExtendedFunctionParam] {
        &self.inputs
    }

    pub fn output(&self) -> Option<&ExtendedFunctionReturnType> {
        self.output.as_ref()
    }
}

// Implementation of Display trait for FunctionInfo
impl fmt::Display for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inputs_str: Vec<String> = self
            .inputs
            .iter()
            .map(|param| format!("{}: {}", param.name, param.type_ident))
            .collect();

        let output_str = match &self.output {
            Some(return_type) => format!("{}", return_type.type_ident),
            None => "".to_string(),
        };

        write!(f, "fn {}({}) {}", self.name, inputs_str.join(", "), output_str)
    }
}

pub fn find_function_specs(spec_fns_result: &Vec<FunctionInfo>, function_name_to_find: &str) -> Option<FunctionInfo> {
     for function_info in spec_fns_result {
        if function_info.name == function_name_to_find {
            return Some(function_info.clone());
        }
    }
    None
}

pub fn read_contract_specs<P: AsRef<::std::path::Path>>(file_path: P) -> std::io::Result<Vec<FunctionInfo>> {
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
            ScSpecEntry::FunctionV0(f) => {
                let name = f.name.to_utf8_string().unwrap();
                let inputs: Vec<_> = f.inputs.iter().map(|input| {
                    let name = input.name.to_utf8_string().unwrap();
                    let type_ident = generate_type_ident_string(&input.type_);
                    ExtendedFunctionParam { name, type_ident }
                }).collect();

                let output = f.outputs
                    .to_option()
                    .map(|t| ExtendedFunctionReturnType { type_ident: generate_type_ident_string(&t) });

                spec_fns.push(FunctionInfo {
                    name,
                    inputs,
                    output,
                });
            },
            ScSpecEntry::UdtStructV0(s) => {
                let struct_info = generate_struct(s);
                spec_structs.push(struct_info);
            },
            ScSpecEntry::UdtUnionV0(u) => {
                let union_info = generate_union(u);
                spec_unions.push(union_info);
            },
            ScSpecEntry::UdtEnumV0(e) => {
                let enum_info = generate_enum(e);
                spec_enums.push(enum_info);
            },
            ScSpecEntry::UdtErrorEnumV0(e) => {
                let error_enum_info = generate_error_enum(e);
                spec_error_enums.push(error_enum_info);
            },
        }
    }

    Ok(spec_fns)
}

pub fn generate_type_ident_string(spec: &ScSpecTypeDef) -> ExtendedValueType {
    match spec {
        ScSpecTypeDef::Val => ExtendedValueType::new(ValueType::I32, "soroban_sdk::Val"),
        ScSpecTypeDef::U64 => ExtendedValueType::new(ValueType::I64, "u64"),
        ScSpecTypeDef::I64 => ExtendedValueType::new(ValueType::I64, "i64"),
        ScSpecTypeDef::U32 => ExtendedValueType::new(ValueType::I32, "u32"),
        ScSpecTypeDef::I32 => ExtendedValueType::new(ValueType::I32, "i32"),
        ScSpecTypeDef::U128 => ExtendedValueType::new(ValueType::I64, "u128"),
        ScSpecTypeDef::I128 => ExtendedValueType::new(ValueType::I64, "i128"),
        ScSpecTypeDef::Bool => ExtendedValueType::new(ValueType::I32, "bool"),
        ScSpecTypeDef::Symbol => ExtendedValueType::new(ValueType::I64, "soroban_sdk::Symbol"),
        ScSpecTypeDef::Error => ExtendedValueType::new(ValueType::I32, "soroban_sdk::Error"),
        ScSpecTypeDef::Bytes => ExtendedValueType::new(ValueType::I32, "soroban_sdk::Bytes"),
        ScSpecTypeDef::Address => ExtendedValueType::new(ValueType::I32, "soroban_sdk::Address"),
        ScSpecTypeDef::String => ExtendedValueType::new(ValueType::I32, "soroban_sdk::String"),
        ScSpecTypeDef::Option(o) => {
            let value_ident = generate_type_ident_string(&o.value_type);
            ExtendedValueType::new(ValueType::I32, &format!("Option<{}>", value_ident))
        }
        ScSpecTypeDef::Result(r) => {
            let ok_ident = generate_type_ident_string(&r.ok_type);
            let error_ident = generate_type_ident_string(&r.error_type);
            ExtendedValueType::new(
                ValueType::I32,
                &format!("Result<{}, {}>", ok_ident, error_ident),
            )
        }
        ScSpecTypeDef::Vec(v) => {
            let element_ident = generate_type_ident_string(&v.element_type);
            ExtendedValueType::new(
                ValueType::I32,
                &format!("soroban_sdk::Vec<{}>", element_ident),
            )
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident_string(&m.key_type);
            let value_ident = generate_type_ident_string(&m.value_type);
            ExtendedValueType::new(
                ValueType::I32,
                &format!("soroban_sdk::Map<{}, {}>", key_ident, value_ident),
            )
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents: Vec<_> = t
                .value_types
                .iter()
                .map(|ty| generate_type_ident_string(ty))
                .collect();
            ExtendedValueType::new(
                ValueType::I32,
                &format!("({})", type_idents.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),
            )
        }
        ScSpecTypeDef::BytesN(b) => {
            ExtendedValueType::new(ValueType::I32, &format!("soroban_sdk::BytesN<{}>", b.n))
        }
        ScSpecTypeDef::Udt(u) => {
            ExtendedValueType::new(
                ValueType::I32,
                &format!("{}", u.name.to_utf8_string().unwrap()),
            )
        }
        ScSpecTypeDef::Void => ExtendedValueType::new(ValueType::I32, "()"),
        ScSpecTypeDef::Timepoint => {
            ExtendedValueType::new(ValueType::I32, "soroban_sdk::Timepoint")
        }
        ScSpecTypeDef::Duration => {
            ExtendedValueType::new(ValueType::I32, "soroban_sdk::Duration")
        }
        ScSpecTypeDef::U256 => ExtendedValueType::new(ValueType::I32, "soroban_sdk::U256"),
        ScSpecTypeDef::I256 => ExtendedValueType::new(ValueType::I32, "soroban_sdk::I256"),
    }
}
