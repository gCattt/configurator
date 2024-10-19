use serde::Serialize;
use zconf::{ConfigHandler, ConfigHandlerExt, CosmicConfig};

#[derive(Debug, Clone, Default, Serialize, CosmicConfig)]
pub struct Config {
    #[serde(skip)]
    config_handler: Option<ConfigHandler>,
    pub private_mode: bool,
    pub maximum_entries_lifetime: Option<u64>,
    pub maximum_entries_number: Option<u32>,
    pub horizontal: bool,
    pub unique_session: bool,
}

impl ConfigHandlerExt for Config {
    fn config_handler(&mut self) -> &mut Option<ConfigHandler> {
        &mut self.config_handler
    }
}

fn main() {
    let mut config = Config::default().init("hello");

    config.set_private_mode(true);
}
