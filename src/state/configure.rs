use justconfig::item::ValueExtractor;
use justconfig::processors::{Explode, Trim};
use justconfig::sources::defaults::Defaults;
use justconfig::sources::env::Env;
use justconfig::sources::text::{ConfigText, Error};
use justconfig::validators::Range;
use justconfig::ConfPath;
use justconfig::Config;
use std::fs::File;

pub struct OpSettings {
    config: Config,
}





impl OpSettings {
    pub fn new() -> OpSettings {
        let mut settings = Config::default();

        let file = File::open("opuscule_server.conf").expect("Could not open config file.");
        let settings_file = ConfigText::new(file, "opuscule_server.conf").unwrap();
        settings.add_source(settings_file);

        OpSettings { config: settings }
    }
    pub fn get_server_address(&self) -> String {
        self.config
            .get(self.config.root().push("server").push("address"))
            .trim()
            .value()
            .expect("Can't find server addr")
    }
}
