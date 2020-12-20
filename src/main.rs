mod cfg;
mod scan;

#[macro_use]
extern crate clap;
extern crate grep;
extern crate ignore;

use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use cfg::Config;
use clap::{App, Arg};
use scan::scan_urls;
use std::path::Path;
use std::process::exit;

// ⚠️ is composed of two chars, so need an extra space after it to have the correct "⚠️" output
static CAUTION: &str = "⚠️ ";
static SEARCH: &str = "🔎";
static WRITE: &str = "📝";
static SAD: &str = "😢";
static TRY: &str = "🧭";
static DONE: &str = "✅";

const CONFIG_FILENAME: &str = ".wints.yaml";

fn main() {
    let cli_name = Style::new().bold().paint("wints");
    let display_name = Style::new().bold().paint("What I Need To See");
    let about = Green.bold().blink().paint("a fuzzy term-based URLs opener");
    let version = Green.bold().paint(crate_version!());

    let matches = App::new(cli_name.to_string())
        .about(format!("{} - {}", display_name, about).as_str())
        .version(version.to_string().as_str())
        .arg(
            Arg::with_name("scan")
                .long("scan")
                .help("Scan the current directory for new URLs"),
        )
        .arg(
            Arg::with_name("terms")
                .help("Terms to search for")
                .conflicts_with("scan")
                .multiple(true)
                .index(1),
        )
        .get_matches();

    if matches.is_present("scan") {
        scan_for_new_urls();
        exit(0)
    }

    search_terms(matches.values_of_lossy("terms"));
}

fn search_terms(matching_terms: Option<Vec<String>>) {
    let config = load_configuration();
    match matching_terms {
        Some(terms) => open_urls_based_on_terms(terms, config),
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

fn open_urls_based_on_terms(terms_search: Vec<String>, config: Config) {
    println!(
        " {} Searching for {}...",
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
            Err(why) => eprintln!("can't open {} -> {}", url, Red.paint(why.to_string())),
        }
    }
}

fn scan_for_new_urls() {
    println!(" {} Scanning for new URLs...", SEARCH);
    let scanned_urls = scan_urls("./".to_string());
    add_urls_in_configuration("need to be contextualised".to_string(), scanned_urls);
    println!(" {} Scan completed", DONE);
}

fn add_urls_in_configuration(context: String, urls: Vec<String>) {
    let path = Path::new(CONFIG_FILENAME);
    let mut config: Config;
    if path.exists() {
        config = read_configuration();
    } else {
        config = Config::new();
    }

    let already_known_urls = config.urls();
    let ignored_urls = config.ignored_urls();
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

        match config.write_file(CONFIG_FILENAME) {
            Err(why) => {
                eprintln!(
                    "couldn't write {}\n{}",
                    Green.bold().paint(CONFIG_FILENAME),
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

fn load_configuration() -> Config {
    ensure_configuration_file_exists();
    read_configuration()
}

fn read_configuration() -> Config {
    match Config::read_file(CONFIG_FILENAME) {
        Err(why) => {
            eprintln!(
                "can't read {}\n{}",
                Green.bold().paint(CONFIG_FILENAME),
                Red.paint(why.to_string())
            );
            exit(1)
        }
        Ok(config) => config,
    }
}

fn ensure_configuration_file_exists() {
    let path = Path::new(CONFIG_FILENAME);
    if !path.exists() {
        println!(
            " {} Can't find any {} file.",
            CAUTION,
            Green.bold().paint(CONFIG_FILENAME),
        );

        match Config::write_default_file(CONFIG_FILENAME) {
            Err(why) => {
                eprintln!(
                    "couldn't create {}\n{}",
                    Green.bold().paint(CONFIG_FILENAME),
                    Red.paint(why.to_string())
                );
                exit(1)
            }
            Ok(_) => println!(
                " {} So an empty {} file has been created.",
                WRITE,
                Green.bold().paint(CONFIG_FILENAME)
            ),
        };
    }
}
