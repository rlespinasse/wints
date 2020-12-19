mod cfg;

#[macro_use]
extern crate clap;

use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use cfg::Config;
use clap::{App, Arg};
use std::path::Path;

// ⚠️ is composed of two chars, so need an extra space after it to have the correct "⚠️" output
static CAUTION: &str = "⚠️ ";
static SEARCH: &str = "🔎";
static WRITE: &str = "📝";
static SAD: &str = "😢";

fn main() {
    let cli_name = Style::new().bold().paint("wints");
    let display_name = Style::new().bold().paint("What I Need To See");
    let about = Green.bold().blink().paint("a fuzzy term-based url opener");
    let version = Green.bold().paint(crate_version!());

    let matches = App::new(cli_name.to_string())
        .about(format!("{} - {}", display_name, about).as_str())
        .version(version.to_string().as_str())
        .arg(
            Arg::with_name("terms")
                .help("Terms to search for")
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let config = load_configuration();

    match matches.values_of_lossy("terms") {
        Some(terms) => open_urls_based_on_terms(terms, config),
        None => terms_are_mandatory(config),
    }
}

fn terms_are_mandatory(config: Config) {
    println!(" {} No terms passed, can't search anything.", CAUTION);
    if let Some(possible_terms) = config.list_of_contexts().first() {
        println!(
            " {} Try with {}.",
            SEARCH,
            Green.bold().paint(possible_terms)
        )
    }
}

fn open_urls_based_on_terms(terms_search: Vec<String>, config: Config) {
    println!(
        " {} Searching for {}.",
        SEARCH,
        Green.bold().paint(terms_search.join(" "))
    );
    let urls = config.urls_from_context(terms_search.clone());
    if urls.is_empty() {
        match config.nearest_context(terms_search) {
            Some(nearest_context) => println!(
                " {} Missed, try with terms like in '{}'.",
                SAD,
                Green.bold().paint(nearest_context)
            ),
            None => println!(" {} Nothing found, try with another term.", SAD),
        }
    }
    for url in urls.iter() {
        match webbrowser::open(url) {
            Ok(_) => println!("open {}", url),
            Err(why) => eprintln!("can't open {}: {}", url, Red.paint(why.to_string())),
        }
    }
}

fn load_configuration() -> Config {
    let config_filename = ".wints.yaml";

    ensure_configuration_file_exists(config_filename);

    match Config::read_file(config_filename) {
        Err(why) => panic!(
            "can't load configuration file {}: {}",
            Green.bold().paint(config_filename),
            Red.paint(why.to_string())
        ),
        Ok(config) => config,
    }
}

fn ensure_configuration_file_exists(config_filename: &str) {
    let path = Path::new(config_filename);
    if !path.exists() {
        println!(" {} Can't find any '{}' file.", CAUTION, config_filename);

        match Config::write_default_file(config_filename) {
            Err(why) => panic!(
                "couldn't create {}: {}",
                Green.bold().paint(config_filename),
                Red.paint(why.to_string())
            ),
            Ok(_) => println!(
                " {} So an empty {} file has been created.",
                WRITE,
                Green.bold().paint(config_filename)
            ),
        };
    }
}
