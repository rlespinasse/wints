use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WintsModule {
    pub version: u8,
    elements: Vec<Element>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Element {
    context: String,
    urls: Vec<String>,
}

impl WintsModule {
    pub fn empty_module() -> WintsModule {
        WintsModule {
            version: 1,
            elements: vec![],
        }
    }

    pub fn default_module() -> WintsModule {
        WintsModule {
            version: 1,
            elements: vec![
                Element {
                    context: "repository code".into(),
                    urls: vec!["https://github.com/rlespinasse/wints".into()],
                },
                Element {
                    context: "repository issues".into(),
                    urls: vec!["https://github.com/rlespinasse/wints/issues".into()],
                },
                Element {
                    context: "repository crate".into(),
                    urls: vec!["https://crates.io/crates/wints".into()],
                },
                Element {
                    context: "repository usage".into(),
                    urls: vec!["https://github.com/rlespinasse/wints/blob/v0.x/USAGE.adoc".into()],
                },
            ],
        }
    }

    pub fn contextualise_urls(module: &mut WintsModule, context: String, urls: Vec<String>) {
        let mut updated_elements: Vec<Element> = module.elements.clone();
        let position = module
            .elements
            .iter()
            .position(|element| element.context == context);
        match position {
            Some(p) => {
                let mut current_urls = updated_elements[p].urls.clone();
                current_urls.extend(urls.iter().cloned());
                updated_elements[p].urls = current_urls;
            }
            None => updated_elements.push(Element { context, urls }),
        }

        module.elements = updated_elements
    }

    pub fn list_of_all_contexts(&self) -> Vec<String> {
        self.elements
            .iter()
            .map(|element| element.context.clone())
            .collect()
    }

    pub fn list_of_all_urls(&self) -> Vec<String> {
        self.elements
            .iter()
            .flat_map(|element| element.urls.clone())
            .collect()
    }

    pub fn list_of_urls_from_matching_context(&self, context: Vec<String>) -> Vec<String> {
        let matcher = SkimMatcherV2::default();
        self.elements
            .iter()
            .filter(|element| {
                let matching_terms_count =
                    WintsModule::accuracy_of_matching_context(&matcher, &context, element);
                matching_terms_count == context.capacity()
            })
            .flat_map(|element| element.urls.clone())
            .collect()
    }

    pub fn nearest_matching_context(&self, context: Vec<String>) -> Option<String> {
        self.sorted_list_of_contexts_by_matching_accuracy(context)
            .first()
            .cloned()
    }

    fn sorted_list_of_contexts_by_matching_accuracy(&self, context: Vec<String>) -> Vec<String> {
        let matcher = SkimMatcherV2::default();
        let mut partially_matching_elements: Vec<&Element> = self
            .elements
            .iter()
            .filter(|element| {
                let matching_terms_count =
                    WintsModule::accuracy_of_matching_context(&matcher, &context, element);
                matching_terms_count != context.capacity() && matching_terms_count != 0
            })
            .collect();

        partially_matching_elements.sort_by(|first, second| {
            let first_count = WintsModule::accuracy_of_matching_context(&matcher, &context, first);
            let second_count =
                WintsModule::accuracy_of_matching_context(&matcher, &context, second);
            first_count.cmp(&second_count)
        });

        partially_matching_elements
            .iter()
            .map(|element| element.context.clone())
            .collect()
    }

    fn accuracy_of_matching_context(
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

#[cfg(test)]
mod tests {
    use crate::core::module::WintsModule;

    macro_rules! vec_of_strings {
      ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    fn print_serde_error_if_any(result: &serde_yaml::Result<WintsModule>) {
        if let Err(err) = &result {
            println!("{}", err);
        }
    }

    fn some_testing_module() -> WintsModule {
        let yaml = r#"
            version: 0
            module: default
            elements:
            - context: some terms
              urls:
              - https://test1.tld
              - https://test2.tld
            - context: another terms
              urls:
              - https://test3.tld
              - https://test4.tld
        "#;

        let module: serde_yaml::Result<WintsModule> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&module);
        assert!(module.is_ok());
        module.unwrap()
    }

    fn empty_testing_module() -> WintsModule {
        let yaml = r#"
            version: 1
            module: default
            elements: []
        "#;

        let module: serde_yaml::Result<WintsModule> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&module);
        assert!(module.is_ok());
        module.unwrap()
    }

    #[test]
    fn deserialize() {
        let yaml = r#"
            version: 1
            module: default
            elements: []
        "#;

        let module: serde_yaml::Result<WintsModule> = serde_yaml::from_str(yaml);
        print_serde_error_if_any(&module);
        assert!(module.is_ok());
        assert_eq!(1, module.unwrap().version)
    }

    #[test]
    fn list_of_all_urls() {
        let module = some_testing_module();
        assert_eq!(
            vec![
                "https://test1.tld",
                "https://test2.tld",
                "https://test3.tld",
                "https://test4.tld",
            ],
            module.list_of_all_urls()
        );
    }

    #[test]
    fn list_of_all_urls_on_empty_module() {
        let module = empty_testing_module();
        assert!(module.list_of_all_urls().is_empty());
    }

    #[test]
    fn list_of_all_contexts() {
        let module = some_testing_module();
        assert_eq!(
            vec!["some terms", "another terms"],
            module.list_of_all_contexts()
        );
    }

    #[test]
    fn list_of_all_contexts_on_empty_module() {
        let module = empty_testing_module();
        assert!(module.list_of_all_contexts().is_empty());
    }

    #[test]
    fn nearest_matching_context() {
        let module = some_testing_module();
        let nearest_matching_context =
            module.nearest_matching_context(vec_of_strings!["some", "words"]);
        assert!(nearest_matching_context.is_some());
        assert_eq!("some terms", nearest_matching_context.unwrap())
    }

    #[test]
    fn nearest_matching_context_on_empty_module() {
        let module = some_testing_module();
        let no_nearest_matching_context =
            module.nearest_matching_context(vec_of_strings!["lot", "of", "words"]);
        assert!(no_nearest_matching_context.is_none());
    }

    #[test]
    fn list_of_urls_from_matching_context() {
        let module = some_testing_module();
        assert_eq!(
            vec![
                "https://test1.tld",
                "https://test2.tld",
                "https://test3.tld",
                "https://test4.tld",
            ],
            module.list_of_urls_from_matching_context(vec_of_strings!["terms"])
        );
    }

    #[test]
    fn list_of_urls_from_matching_context_partial() {
        let module = some_testing_module();
        assert_eq!(
            vec!["https://test1.tld", "https://test2.tld"],
            module.list_of_urls_from_matching_context(vec_of_strings!["some"])
        );
    }

    #[test]
    fn list_of_urls_from_matching_context_unordered() {
        let module = some_testing_module();
        assert_eq!(
            vec!["https://test1.tld", "https://test2.tld"],
            module.list_of_urls_from_matching_context(vec_of_strings!["terms", "some"])
        );
    }
}
