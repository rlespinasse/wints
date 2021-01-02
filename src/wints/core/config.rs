use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WintsConfig {
    pub version: u8,
    pub ignored_urls: Option<Vec<String>>,
}

impl WintsConfig {
    pub fn empty_config() -> WintsConfig {
        WintsConfig {
            version: 1,
            ignored_urls: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::config::WintsConfig;

    fn print_serde_error_if_any(result: &serde_yaml::Result<WintsConfig>) {
        if let Err(err) = &result {
            println!("{}", err);
        }
    }

    #[test]
    fn deserialize_without_ignored_urls() {
        let yaml = r#"
            version: 1
        "#;

        let result: serde_yaml::Result<WintsConfig> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&result);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(1, config.version);
        assert!(config.ignored_urls.is_none());
    }

    #[test]
    fn deserialize_wit_empty_ignored_urls() {
        let yaml = r#"
            version: 1
            ignored_urls: []
        "#;

        let result: serde_yaml::Result<WintsConfig> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&result);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(1, config.version);
        assert!(config.ignored_urls.is_some());
        assert!(config.ignored_urls.unwrap().is_empty());
    }

    #[test]
    fn deserialize() {
        let yaml = r#"
            version: 1
            ignored_urls:
            - https://site.tld
        "#;

        let result: serde_yaml::Result<WintsConfig> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&result);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(1, config.version);
        assert!(config.ignored_urls.is_some());
        assert_eq!(vec!["https://site.tld"], config.ignored_urls.unwrap());
    }

    #[test]
    fn no_ignored_urls() {
        let config = WintsConfig::empty_config();
        assert!(config.ignored_urls.is_none());
    }
}
