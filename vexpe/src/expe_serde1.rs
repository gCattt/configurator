use figment::{providers, Figment, Profile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
#[serde(default)]
struct Config2 {
    a: A,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
enum A {
    B(B),
}

impl Default for A {
    fn default() -> Self {
        A::B(B { s: "inner".into() })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(default)]
struct B {
    s: String,
}

impl Default for B {
    fn default() -> Self {
        Self { s: "nested".into() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path,
    };

    use schemars::schema_for;

    #[test]
    fn deser() {
        let schema = schema_for!(Config2);

        let mut file = File::create("schema.json").unwrap();
        file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())
            .unwrap();

        // dbg!(&schema);

        let p = Path::new("test1.json");

        let mut file = File::open(p).unwrap();

        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let s = serde_json::from_str::<Config2>(&content).unwrap();

        dbg!(&s);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct Config {
    nested: NestedConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct NestedConfig {
    value: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nested: NestedConfig { value: 0 },
        }
    }
}

impl Default for NestedConfig {
    fn default() -> Self {
        Self { value: 1 }
    }
}

#[test]
fn a() {
    let e = Figment::new().merge(providers::Serialized::from(
        Config::default(),
        Profile::Default,
    ));

    let v = r#"
    {
    "nested": {
       
    }
    }
    "#;

    let e = serde_json::to_string_pretty(&Config::default()).unwrap();

    println!("{}", e);

    let v: Config = serde_json::from_str(v).unwrap();

    dbg!(&v);
}
