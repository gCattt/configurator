use crate::{
    app::APPID,
    config::{gen_schema, SCHEMAS_PATH},
    node::NodeContainer,
};

use super::Node;
use schemars::schema::RootSchema;
use std::{fs::File, io::Read, path::Path, str::FromStr};

#[test]
fn t() {
    gen_schema();

    let path = Path::new(SCHEMAS_PATH).join(format!("{}.json", APPID));

    let schema = get_json_schema(&path).unwrap();

    let node = NodeContainer::from_json_schema(&schema);

    dbg!(&node);
}

pub fn get_json_schema(path: &Path) -> anyhow::Result<RootSchema> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let value = json::value::Value::from_str(&contents).unwrap();

    let json_schema: RootSchema = json::from_value(value)?;

    Ok(json_schema)
}
