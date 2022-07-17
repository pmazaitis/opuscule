use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Network {
    address: IpAddr,
    port: u32,
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
            
        let s = Config::builder()
        .add_source(File::with_name("config/default.toml"))
        .build()?;  
        
        // Now that we're done, let's access our configuration
        println!("settings: {:?}", s);
        
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize() 
    }   
}
