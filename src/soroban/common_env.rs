use std::{
    fs::File,
    io::BufReader,
    error::Error
};

use serde_json::Value;

pub fn read_env_common_from_file() -> Result<Value, Box<dyn Error>> {
    let file = File::open("env.json")?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

pub fn env_common_modules_result() -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    // Load common environment imports from file
    let common_env_imports = read_env_common_from_file()?;
    if let Some(modules) = common_env_imports.get("modules").and_then(Value::as_array) {
        let cloned_modules: Vec<Value> = modules.clone();
        return Ok(cloned_modules);
    } else {
        return Err("No modules found in the JSON data.".into());
    }
}

pub fn take_common_module(modules: Vec<Value>, module_name: &str, field_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over each module
    for module in modules {
        // Check if the module name matches
        if let Some(module_value) = module.get("name").and_then(Value::as_str) {
            if module_value == module_name {
                println!("{:?}", module_name);
            }
        }
    }

    Ok(())
}
