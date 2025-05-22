use serde::Deserialize;
use std::fs;
use std::path::Path;

impl Configurator {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}