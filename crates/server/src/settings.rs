use config::{Config, ConfigError, Environment, File};
use serde::{Serialize,Deserialize};
use std::env;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Network {
    host_ip: IpAddr,
    host_port: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    // debug: bool,
    network: Network,

}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
            
        let s = Config::builder()
        .add_source(File::with_name("config/default.toml"))
        .build()?;  
        
        // Now that we're done, let's access our configuration
        // println!("settings: {:?}", s);
        
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize() 
    }
    pub fn server_addr(&self) -> String {
        format!("{}:{}",&self.network.host_ip.to_string(),&self.network.host_port.to_string()) 
    }
    pub fn enabled_components(&self) -> String {
        "all".to_string()
    }  
}
