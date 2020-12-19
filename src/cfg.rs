use std::error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Config {
    elements: Vec<Element>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Element {
    context: String,
    urls: Vec<String>,
}

impl Config {
    fn default_config() -> Config {
        Config {
            elements: vec![Element {
                context: "A context".into(),
                urls: vec![
                    "https://first.website.com".into(),
                    "https://second.website.com".into(),
                ],
            }],
        }
    }

    pub(crate) fn read_file(filename: &str) -> Result<Config> {
        let file = File::open(filename)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    pub(crate) fn write_file(&self, config_filename: &str) -> Result<()> {
        let path = Path::new(config_filename);
        let mut file = File::create(&path)?;
        let config_content_string = serde_yaml::to_string(&self)?;
        file.write_all(config_content_string.as_bytes())?;
        Ok(())
    }

    pub(crate) fn write_default_file(config_filename: &str) -> Result<()> {
        Config::default_config().write_file(config_filename)
    }

    pub(crate) fn list_of_contexts(&self) -> Vec<String> {
        self.elements
            .iter()
            .map(|element| element.context.clone())
            .collect()
    }

    pub(crate) fn urls_from_context(&self, context: String) -> Vec<String> {
        self.elements
            .iter()
            .filter(|element| element.context == context)
            .flat_map(|element| element.urls.clone())
            .collect()
    }
}
