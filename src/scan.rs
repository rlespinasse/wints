use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;

use ignore::Walk;
use linkify::{LinkFinder, LinkKind};
use std::error;
use std::path::{Path, PathBuf};

const URL_PATTERN: &str = r#"(http://|https://)"#;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) fn scan_urls(path: String) -> Vec<String> {
    let files: Vec<PathBuf> = Walk::new(path)
        .into_iter()
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
