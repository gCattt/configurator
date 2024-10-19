use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(default)]
struct Config3 {
    hash: HashMap<String, String>,
}

impl Default for Config3 {
    fn default() -> Self {
        let mut h = HashMap::new();

        h.insert("hello".into(), "hella".into());

        Self { hash: h }
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read, path::Path};

    use super::*;

    #[test]
    fn deser() {
        // let schema = schema_for!(Config3);

        // let mut file = File::create("schema.json").unwrap();
        // file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())
        //     .unwrap();

        // dbg!(&schema);

        let p = Path::new("test2.json");

        let mut file = File::open(p).unwrap();

        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let s = serde_json::from_str::<Config3>(&content).unwrap();

        dbg!(&s);
    }
}
