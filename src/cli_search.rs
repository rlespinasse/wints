use std::path::Path;

use ansi_term::Colour::{Green, Red};

use crate::cfg::Config;
use crate::{cli_config, CAUTION, DONE, SAD, SEARCH, TRY};

pub(crate) fn search_terms(config_file: &Path, matching_terms: Option<Vec<String>>) {
    let config = cli_config::load_configuration(config_file);
    match matching_terms {
        Some(terms) => search_urls_based_on_terms(terms, config),
        None => terms_are_mandatory(config),
    }
    println!(" {} Search completed", DONE);
}

fn terms_are_mandatory(config: Config) {
    println!(" {} No terms passed, can't search anything.", CAUTION);
    if let Some(possible_terms) = config.list_of_contexts().first() {
        println!(" {} Try with {}.", TRY, Green.bold().paint(possible_terms))
    }
}

fn search_urls_based_on_terms(terms_search: Vec<String>, config: Config) {
    println!(
        " {} Searching for {}...",
        SEARCH,
        Green.bold().paint(terms_search.join(" "))
    );
    let urls = config.urls_from_context(terms_search.clone());
    match urls.is_empty() {
        true => urls_not_found(terms_search, config),
        false => open_urls(urls),
    }
}

fn open_urls(urls: Vec<String>) {
    for url in urls.iter() {
        match webbrowser::open(url) {
            Ok(_) => println!("open {}", url),
            Err(why) => eprintln!("can't open {} -> {}", url, Red.paint(why.to_string())),
        }
    }
}

fn urls_not_found(terms_search: Vec<String>, config: Config) {
    match config.nearest_context(terms_search) {
        Some(nearest_context) => println!(
            " {} Missed, try with terms like in '{}'.",
            SAD,
            Green.bold().paint(nearest_context)
        ),
        None => println!(" {} Nothing found, try with another term.", SAD),
    }
}
