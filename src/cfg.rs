use std::error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
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
            elements: vec![
                Element {
                    context: "repository code".into(),
                    urls: vec!["https://github.com/rlespinasse/wints".into()],
                },
                Element {
                    context: "repository issues".into(),
                    urls: vec!["https://github.com/rlespinasse/wints/issues".into()],
                },
            ],
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

    pub(crate) fn urls_from_context(&self, context: Vec<String>) -> Vec<String> {
        let matcher = SkimMatcherV2::default();
        self.elements
            .iter()
            .filter(|element| {
                let matching_terms_count =
                    Config::matching_term_count(&matcher, &context, &element);
                matching_terms_count == context.capacity()
            })
            .flat_map(|element| element.urls.clone())
            .collect()
    }

    pub(crate) fn nearest_context(&self, context: Vec<String>) -> Option<String> {
        self.contexts_sorted_by_matching_accuracy(context)
            .first()
            .cloned()
    }

    fn contexts_sorted_by_matching_accuracy(&self, context: Vec<String>) -> Vec<String> {
        let matcher = SkimMatcherV2::default();
        let mut partially_matching_elements: Vec<&Element> = self
            .elements
            .iter()
            .filter(|element| {
                let matching_terms_count =
                    Config::matching_term_count(&matcher, &context, &element);
                matching_terms_count != context.capacity() && matching_terms_count != 0
            })
            .collect();

        partially_matching_elements.sort_by(|first, second| {
            let first_count = Config::matching_term_count(&matcher, &context, &first);
            let second_count = Config::matching_term_count(&matcher, &context, &second);
            first_count.cmp(&second_count)
        });

        partially_matching_elements
            .iter()
            .map(|element| element.context.clone())
            .collect()
    }

    fn matching_term_count(
        matcher: &SkimMatcherV2,
        context: &[String],
        element: &Element,
    ) -> usize {
        context
            .iter()
            .filter(|term| {
                matcher
                    .fuzzy_match(element.context.as_str(), term.as_str())
                    .is_some()
            })
            .count()
    }
}
