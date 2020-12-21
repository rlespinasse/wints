mod cfg;
mod scan;

#[macro_use]
extern crate clap;
extern crate grep;
extern crate ignore;

use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use cfg::Config;
use clap::{App, Arg, SubCommand};
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
            Arg::with_name("terms")
                .help("Terms to search for")
                .conflicts_with("scan")
                .multiple(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name("scan").about("Scan the current directory for new URLs"))
        .get_matches();

    let config_file = Path::new(CONFIG_FILENAME);

    if matches.is_present("scan") {
        scan_for_new_urls(config_file);
        exit(0)
    }

    search_terms(config_file, matches.values_of_lossy("terms"));
}

fn search_terms(config_file: &Path, matching_terms: Option<Vec<String>>) {
    let config = load_configuration(config_file);
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

fn scan_for_new_urls(config_file: &Path) {
    println!(" {} Scanning for new URLs...", SEARCH);
    let scanned_urls = scan_urls("./");
    add_urls_in_configuration(
        config_file,
        "need to be contextualised".to_string(),
        scanned_urls,
    );
    println!(" {} Scan completed", DONE);
}

fn add_urls_in_configuration(config_file: &Path, context: String, urls: Vec<String>) {
    let mut config = read_configuration(config_file).unwrap_or_else(Config::new);

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

fn load_configuration(config_file: &Path) -> Config {
    ensure_configuration_file_exists(config_file);
    read_configuration(config_file).unwrap()
}

fn read_configuration(config_file: &Path) -> Option<Config> {
    if !config_file.exists() {
        return None;
    }
    match Config::read_file(config_file) {
        Err(why) => {
            eprintln!(
                "can't read {}\n{}",
                Green.bold().paint(config_file.display().to_string()),
                Red.paint(why.to_string())
            );
            exit(1)
        }
        Ok(config) => Some(config),
    }
}

fn ensure_configuration_file_exists(config_file: &Path) {
    if !config_file.exists() {
        let filename = config_file.display().to_string();
        println!(
            " {} Can't find any {} file.",
            CAUTION,
            Green.bold().paint(&filename),
        );

        match Config::write_default_file(config_file) {
            Err(why) => {
                eprintln!(
                    "couldn't create {}\n{}",
                    Green.bold().paint(&filename),
                    Red.paint(why.to_string())
                );
                exit(1)
            }
            Ok(_) => println!(
                " {} So an empty {} file has been created.",
                WRITE,
                Green.bold().paint(&filename)
            ),
        };
    }
}
