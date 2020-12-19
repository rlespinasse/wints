mod cfg;

#[macro_use]
extern crate clap;

use cfg::Config;
use clap::{App, Arg};
use std::path::Path;

// ⚠️ is two chars, so need an extra space after it to have the correct "⚠️" output
static CAUTION: &str = "⚠️ ";
static SEARCH: &str = "🔎";
static WRITE: &str = "📝";

fn main() {
    let matches = App::new("wints")
        .about("What I Need To See - a fuzzy term-based url opener")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("terms")
                .help("Terms to search for")
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let config = load_configuration();

    match matches.values_of_lossy("terms") {
        Some(terms) => open_urls_based_on_terms(terms.join(" "), config),
        None => terms_are_mandatory(config),
    }
}

fn terms_are_mandatory(config: Config) {
    println!(" {} No terms passed, can't search anything.", CAUTION);
    if let Some(possible_terms) = config.list_of_contexts().first() {
        println!(" {} Try with '{}'", SEARCH, possible_terms)
    }
}

fn open_urls_based_on_terms(terms_search: String, config: Config) {
    println!(" {} Search for '{}'", SEARCH, terms_search);
    for url in config.urls_from_context(terms_search).iter() {
        match webbrowser::open(url) {
            Ok(_) => println!("open {}", url),
            Err(err) => eprintln!("can't open {}: {}", url, err),
        }
    }
}

fn load_configuration() -> Config {
    let config_filename = ".wints.yaml";

    ensure_configuration_file_exists(config_filename);

    match Config::read_file(config_filename) {
        Err(why) => panic!(
            "can't load configuration file '{}': {}",
            config_filename, why
        ),
        Ok(config) => config,
    }
}

fn ensure_configuration_file_exists(config_filename: &str) {
    let path = Path::new(config_filename);
    if !path.exists() {
        println!(" {} Can't find any '{}' file.", CAUTION, config_filename);

        match Config::write_default_file(config_filename) {
            Err(why) => panic!("couldn't create '{}': {}", config_filename, why),
            Ok(_) => println!(
                " {} So an empty '{}' file has been created.",
                WRITE, config_filename
            ),
        };
    }
}
