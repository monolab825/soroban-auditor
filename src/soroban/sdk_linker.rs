use tlsh::{BucketKind, ChecksumKind, TlshBuilder, Version, Tlsh, TlshError};
use std::{
    fs::File,
    io::BufReader
};
use serde_json::Value;

pub fn get_function_body_hash(code: String) -> Result<Tlsh, TlshError> {
    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::ThreeByte, Version::Version4);
    builder.update(code.as_bytes());
    builder.build()
}

pub fn replace_function_body(hash: &str) -> Option<String> {
    if let Ok(json_data_list) = load_hash_map() {
        for json_data in json_data_list.as_array()? {
            if let (Some(json_hash), Some(body)) = (json_data["hash"].as_str(), json_data["body"].as_str()) {
                if json_hash == hash {
                    return Some(body.to_string());
                }
            }
        }
    }
    None
}

pub fn load_hash_map() -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open("sdk_hashes.json")?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(Into::into)
}
