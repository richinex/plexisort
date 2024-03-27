use log;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub source_directories: Vec<String>, // List of source directories as strings
    pub destination: String, // Single destination directory as string
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        log::info!("Attempting to load config from: {}", file_path);
        let contents = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&contents)?;

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.source_directories.is_empty() {
            return Err("At least one source directory must be specified.".into());
        }

        for dir in &self.source_directories {
            if !Path::new(dir).exists() {
                return Err(format!("Source directory does not exist: {}", dir).into());
            }
        }

        if !Path::new(&self.destination).exists() {
            log::warn!("Destination directory does not exist and will be created: {}", self.destination);
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            source_directories: vec![], // No default source directories
            destination: String::new(), // An empty string as the default destination
        }
    }
}
