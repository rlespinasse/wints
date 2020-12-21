use std::path::Path;
use std::process::exit;

use ansi_term::Colour::{Green, Red};

use crate::cfg::Config;
use crate::{cli_config, scan, DONE, SEARCH, WRITE};

pub(crate) fn scan_for_new_urls(config_file: &Path) {
    println!(" {} Scanning for new URLs...", SEARCH);
    let scanned_urls = scan::scan_urls("./");
    add_urls_in_configuration(
        config_file,
        "need to be contextualised".to_string(),
        scanned_urls,
    );
    println!(" {} Scan completed", DONE);
}

fn add_urls_in_configuration(config_file: &Path, context: String, urls: Vec<String>) {
    let mut config = cli_config::read_configuration(config_file).unwrap_or_else(Config::new);

    let already_known_urls = config.urls();
    let ignored_urls = config.option_scan_ignored_urls();

    let new_urls: Vec<String> = urls
        .into_iter()
        .filter(|url| !already_known_urls.contains(url) && !ignored_urls.contains(url))
        .collect();
    let new_urls_count = new_urls.capacity();

    if new_urls_count > 0 {
        config.append_to_context(context.clone(), new_urls);

        let url_word = match new_urls_count {
            1 => "URL",
            _ => "URLs",
        };

        match config.write_file(config_file) {
            Err(why) => {
                eprintln!(
                    "couldn't write {}\n{}",
                    Green.bold().paint(config_file.display().to_string()),
                    Red.paint(why.to_string())
                );
                exit(1)
            }
            Ok(_) => println!(
                " {} {} {} have been added in context '{}', ready to be sorted.",
                WRITE,
                Green.bold().paint(new_urls_count.to_string()),
                url_word,
                Green.bold().paint(context)
            ),
        };
    }
}
