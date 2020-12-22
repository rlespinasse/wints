use std::path::Path;

use ansi_term::Colour::{Green, Red};

use crate::cfg::Config;
use crate::cli_config;
use crate::cli_log::{CAUTION, DONE, DRY_RUN, GOTO, SAD, SEARCH, TRY};

pub(crate) fn search_terms(matching_terms: Option<Vec<String>>, dry_run: bool) {
    let config_file = Path::new(cli_config::CONFIG_FILENAME);
    let config = cli_config::load_configuration(config_file, dry_run);
    match matching_terms {
        Some(terms) => search_urls_based_on_terms(config, terms, dry_run),
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

fn search_urls_based_on_terms(config: Config, terms_search: Vec<String>, dry_run: bool) {
    println!(
        " {} Searching for {}...",
        SEARCH,
        Green.bold().paint(terms_search.join(" "))
    );
    let urls = config.urls_from_context(terms_search.clone());
    match urls.is_empty() {
        true => urls_not_found(config, terms_search),
        false => open_urls(urls, dry_run),
    }
}

fn open_urls(urls: Vec<String>, dry_run: bool) {
    for url in urls.iter() {
        match dry_run {
            true => println!(" {} Open {}", DRY_RUN, url),
            false => match webbrowser::open(url) {
                Ok(_) => println!(" {} Open {}", GOTO, url),
                Err(why) => eprintln!("can't open {} -> {}", url, Red.paint(why.to_string())),
            },
        }
    }
}

fn urls_not_found(config: Config, terms_search: Vec<String>) {
    match config.nearest_context(terms_search) {
        Some(nearest_context) => println!(
            " {} Missed, try with terms like in '{}'.",
            SAD,
            Green.bold().paint(nearest_context)
        ),
        None => println!(" {} Nothing found, try with another term.", SAD),
    }
}
