#[macro_use]
extern crate clap;
extern crate grep;
extern crate ignore;

use std::path::Path;
use std::process::exit;

use ansi_term::Colour::Green;
use ansi_term::Style;
use clap::{App, Arg, ArgMatches, SubCommand};

mod cfg;
mod cli_config;
mod cli_scan;
mod cli_search;
mod scan;

// ⚠️ is composed of two chars, so need an extra space after it to have the correct "⚠️" output
static CAUTION: &str = "⚠️ ";
static SEARCH: &str = "🔎";
static WRITE: &str = "📝";
static SAD: &str = "😢";
static TRY: &str = "🧭";
static DONE: &str = "✅";

static CONFIG_FILENAME: &str = ".wints.yaml";
static ARG_TERMS: &str = "terms";
static ARG_SCAN: &str = "scan";

fn main() {
    let cli_name = Style::new().bold().paint("wints");
    let display_name = Style::new().bold().paint("What I Need To See");
    let about = Green.bold().blink().paint("a fuzzy term-based URLs opener");
    let version = Green.bold().paint(crate_version!());

    let matches = App::new(cli_name.to_string())
        .about(format!("{} - {}", display_name, about).as_str())
        .version(version.to_string().as_str())
        .arg(
            Arg::with_name(ARG_TERMS)
                .help("Terms to search for")
                .conflicts_with(ARG_SCAN)
                .multiple(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name(ARG_SCAN).about("Scan the current directory for new URLs"),
        )
        .get_matches();

    execute(matches);
}

fn execute(matches: ArgMatches) {
    let config_file = Path::new(CONFIG_FILENAME);

    if matches.is_present(ARG_SCAN) {
        cli_scan::scan_for_new_urls(config_file);
        exit(0) // stop the flow
    }

    cli_search::search_terms(config_file, matches.values_of_lossy(ARG_TERMS));
}
