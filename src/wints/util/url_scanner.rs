use std::path::PathBuf;

use anyhow::Result;
use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;
use ignore::{Walk, WalkBuilder};
use linkify::{LinkFinder, LinkKind};

const URL_PATTERN: &str = r#"(http://|https://)"#;

pub struct UrlScannerOptions {
    pub ignore_files: Vec<PathBuf>,
    pub scan_path: PathBuf,
}

pub fn scan_urls(options: UrlScannerOptions) -> Vec<String> {
    let files: Vec<PathBuf> = build_walk(options)
        .filter_map(|r| r.ok())
        .filter(|d| d.path().is_file())
        .map(|d| d.into_path())
        .collect();

    let mut raw_urls: Vec<String> = files
        .into_iter()
        .flat_map(|file| extract_lines_with_url(file).unwrap_or_default())
        .flat_map(extract_urls)
        .collect();

    raw_urls.sort();
    raw_urls.dedup();
    raw_urls
}

fn build_walk(options: UrlScannerOptions) -> Walk {
    let mut walker = WalkBuilder::new(options.scan_path);
    options.ignore_files.into_iter().for_each(|file| {
        walker.add_custom_ignore_filename(file);
    });
    walker.build()
}

fn extract_lines_with_url(path: PathBuf) -> Result<Vec<String>> {
    let matcher = RegexMatcher::new(URL_PATTERN).unwrap();

    let mut matches = vec![];
    Searcher::new().search_path(
        &matcher,
        &path,
        UTF8(|_line_num, line| {
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

// linkify leave some trailing text due to URL rfc and brackets in asciidoc syntax
fn url_cleaner(url: String) -> String {
    match url.find('[') {
        Some(position) => url[..position].to_string(),
        None => url,
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use crate::util::url_scanner;
    use crate::util::url_scanner::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    fn assert_extracted_urls(line: &str, expected: Vec<String>) {
        assert_eq!(expected, extract_urls(line.to_string()))
    }

    #[test]
    fn scan_urls_with_one_ignore_file() -> Result<()> {
        let dir = tempdir()?;
        let ignore1_path = dir.path().join("ignore1");
        let mut ignore1_file = File::create(&ignore1_path)?;
        writeln!(ignore1_file, "file2.txt")?;

        let options = url_scanner::UrlScannerOptions {
            ignore_files: vec![ignore1_path.clone()],
            scan_path: dir.path().clone().to_path_buf(),
        };

        let mut file1 = File::create(dir.path().join("file1.txt"))?;
        let content1 = r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#;
        writeln!(file1, "{}", content1)?;

        let mut file2 = File::create(dir.path().join("file2.txt"))?;
        let content2 = r#"
            Got to https://site4.tld
            Got to https://site5.tld
            Got to https://site6.tld
        "#;
        writeln!(file2, "{}", content2)?;

        let actual = scan_urls(options);
        assert_eq!(
            vec_of_strings![
                "https://site1.tld",
                "https://site2.tld",
                "https://site3.tld"
            ],
            actual
        );

        Ok(())
    }

    #[test]
    fn scan_urls_with_two_ignore_files() -> Result<()> {
        let dir = tempdir()?;
        let ignore1_path = dir.path().join("ignore1");
        let mut ignore1_file = File::create(&ignore1_path)?;
        writeln!(ignore1_file, "file1.txt")?;
        let ignore2_path = dir.path().join("ignore2");
        let mut ignore2_file = File::create(&ignore2_path)?;
        writeln!(ignore2_file, "file2.txt")?;

        let options = url_scanner::UrlScannerOptions {
            ignore_files: vec![ignore1_path.clone(), ignore2_path.clone()],
            scan_path: dir.path().clone().to_path_buf(),
        };

        let mut file1 = File::create(dir.path().join("file1.txt"))?;
        let content1 = r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#;
        writeln!(file1, "{}", content1)?;

        let mut file2 = File::create(dir.path().join("file2.txt"))?;
        let content2 = r#"
            Got to https://site4.tld
            Got to https://site5.tld
            Got to https://site6.tld
        "#;
        writeln!(file2, "{}", content2)?;

        let actual = scan_urls(options);
        assert!(actual.is_empty());

        Ok(())
    }

    #[test]
    fn extract_no_url() {
        assert_extracted_urls("Got to www.site.tld", vec_of_strings![]);
    }

    #[test]
    fn extract_one_url() {
        assert_extracted_urls(
            "Got to https://site.tld",
            vec_of_strings!["https://site.tld"],
        );
    }

    #[test]
    fn extract_two_urls() {
        assert_extracted_urls(
            "Got to https://site.tld and https://site2.tld",
            vec_of_strings!["https://site.tld", "https://site2.tld"],
        );
    }

    #[test]
    fn extract_urls_from_markdown() {
        assert_extracted_urls(
            "Got to this [Site](https://site.tld) and [this other site](https://site2.tld)",
            vec_of_strings!["https://site.tld", "https://site2.tld"],
        );
    }

    #[test]
    fn extract_urls_from_asciidoctor() {
        assert_extracted_urls(
            "Got to this https://site.tld[Site] and https://site2.tld[this other site]",
            vec_of_strings!["https://site.tld", "https://site2.tld"],
        );
        assert_extracted_urls(
            "Got to this link::https://site.tld[Site] and link:https://site2.tld[this other site]",
            vec_of_strings!["https://site.tld", "https://site2.tld"],
        );
    }
}
