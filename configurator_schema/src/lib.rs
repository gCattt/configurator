use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use json::Value;
use schemars::{schema_for, JsonSchema};

pub fn gen_schema<T: JsonSchema>(
    path: &Path,
    config_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let schema = schema_for!(T);
    let mut file = File::create(path)?;

    let mut value = json::value::to_value(&schema)?;

    if let Some(obj) = value.as_object_mut() {
        obj.insert("X_CONFIG_PATH".into(), Value::String(config_path));
    }

    file.write_all(json::to_string_pretty(&value)?.as_bytes())?;

    Ok(())
}
