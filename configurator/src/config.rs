use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::APPID;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub last_used_page: Option<String>,
    pub cosmic_compat: bool,
    /// masked appid
    pub masked: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_used_page: Default::default(),
            cosmic_compat: true,
            masked: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::app::APPID;

    use super::Config;

    #[test]
    #[ignore = "schema generation. Should be an executable"]
    pub fn gen_schema() {
        let path = Path::new("../res/config_schema.json");

        let schema = configurator_schema::gen_schema::<Config>()
            .source_home_path(".config/configurator/configurator.json")
            .call()
            .unwrap();

        fs::write(path, &schema).unwrap();
    }
}
