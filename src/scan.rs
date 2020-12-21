use std::error;
use std::path::{Path, PathBuf};

use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use ignore::{Walk, WalkBuilder};
use linkify::{LinkFinder, LinkKind};

const URL_PATTERN: &str = r#"(http://|https://)"#;
const IGNORE_FILENAME: &str = ".wintsignore";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) fn scan_urls(path: &str) -> Vec<String> {
    let files: Vec<PathBuf> = build_walk(path)
        .filter_map(|r| r.ok())
        .filter(|d| d.path().is_file())
        .map(|d| d.into_path())
        .collect();

    let mut raw_urls: Vec<String> = files
        .into_iter()
        .flat_map(|file| extract_lines_with_url(file.as_path()).unwrap_or_default())
        .flat_map(extract_urls)
        .collect();

    raw_urls.sort();
    raw_urls.dedup();
    raw_urls
}

fn build_walk(path: &str) -> Walk {
    let mut walker = WalkBuilder::new(path);
    walker.add_custom_ignore_filename(IGNORE_FILENAME);
    if let Some(home) = dirs::home_dir() {
        walker.add_custom_ignore_filename(home.join(IGNORE_FILENAME));
    }
    walker.build()
}

fn extract_lines_with_url(path: &Path) -> Result<Vec<String>> {
    let matcher = RegexMatcher::new(URL_PATTERN).unwrap();

    let mut matches = vec![];
    Searcher::new().search_path(
        &matcher,
        &path,
        UTF8(|_lnum, line| {
            matches.push(line.trim().to_string());
            Ok(true)
        }),
    )?;

    Ok(matches)
}

fn extract_urls(line: String) -> Vec<String> {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);

    finder
        .links(line.as_str())
        .map(|url| url.as_str().to_string())
        .map(url_cleaner)
        .collect()
}

fn url_cleaner(url: String) -> String {
    match url.find('[') {
        Some(position) => url[..position].to_string(),
        None => url,
    }
}

#[cfg(test)]
mod tests {
    use crate::scan::url_cleaner;

    #[test]
    fn should_clean_url_in_asciidoc() {
        // linkify leave some trailing text due to URL rfc and brackets in asciidoc syntax
        assert_eq!(
            "https://site.tld",
            url_cleaner("https://site.tld[Text]".to_string())
        );
        assert_eq!(
            "https://site.tld",
            url_cleaner("https://site.tld[Some".to_string())
        );
    }
}
