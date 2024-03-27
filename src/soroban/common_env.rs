use std::{
    fs::File,
    io::BufReader
};

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ModuleFunction {
    pub module_name: String,
    pub function_name: String,
    pub function: Value,
}

pub fn read_env_common_from_file() -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open("env.json")?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

pub fn env_common_modules_result() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let common_env_imports = read_env_common_from_file()?;
    if let Some(modules) = common_env_imports.get("modules").and_then(Value::as_array) {
        let cloned_modules: Vec<Value> = modules.clone();
        return Ok(cloned_modules);
    } else {
        return Err("No modules found in the JSON data.".into());
    }
}

pub fn take_common_module(modules: &Vec<Value>, module_name: &str, field_name: &str) -> Result<ModuleFunction, Box<dyn std::error::Error>> {
    for module in modules {
        if let Some(module_export) = module.get("export").and_then(Value::as_str) {
            if module_export == module_name {
                if let Some(functions) = module.get("functions").and_then(Value::as_array) {
                    for function in functions {
                        if let Some(function_obj) = function.as_object() {
                            if let Some(export_value) = function_obj.get("export").and_then(Value::as_str) {
                                if export_value == field_name {
                                    return Ok(ModuleFunction {
                                        module_name: module.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
                                        function_name: function_obj.get("name").and_then(Value::as_str).unwrap_or_default().to_string(),
                                        function: function.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Err("Function not found in module".into())
}
