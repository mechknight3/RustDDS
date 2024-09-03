use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::net::Ipv4Addr; // Import Ipv4Addr

#[derive(Debug, Deserialize)]
pub struct Config {
    pub multicast_address: Ipv4Addr,
    pub multicast_port: u16,
    pub publish_rate: u64,
    pub max_message_size : u64
}

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn new(config_file: &str) -> Self {
        let config = ConfigManager::parse_config(config_file)
            .expect("Failed to parse the configuration file");

        ConfigManager { config }
    }

    fn parse_config(config_file: &str) -> io::Result<Config> {
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config =
            serde_xml_rs::from_str(&contents).expect("Unable to parse the configuration file");
        
        Ok(config)
    }

    pub fn get_multicast_address(&self) -> Ipv4Addr {
        self.config.multicast_address
    }

    pub fn get_multicast_port(&self) -> u16 {
        self.config.multicast_port
    }

    pub fn get_publish_rate(&self) -> u64 {
        self.config.publish_rate
    }

    pub fn get_max_message_size(&self) -> u64{
        self.config.max_message_size
    }
}
