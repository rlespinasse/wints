#[macro_use]
extern crate clap;
extern crate grep;
extern crate ignore;

use ansi_term::Colour::Green;
use ansi_term::Style;
use clap::{App, Arg, ArgMatches, SubCommand};

use cli_log::DRY_RUN;

mod cfg;
mod cli_config;
mod cli_log;
mod cli_scan;
mod cli_search;
mod scan;

static ARG_TERMS: &str = "terms";
static CMD_SCAN: &str = "scan";
static OPT_DRY_RUN: &str = "dry-run";

fn main() {
    let cli_name = Style::new().bold().paint("wints");
    let display_name = Style::new().bold().paint("What I Need To See");
    let about = Green.bold().blink().paint("a fuzzy term-based URLs opener");
    let version = Green.bold().paint(crate_version!());
    let dry_run_as_arg = Arg::with_name(OPT_DRY_RUN)
        .help("Do not actually change anything, just log what are going to do")
        .long(OPT_DRY_RUN);

    let matches = App::new(cli_name.to_string())
        .about(format!("{} - {}", display_name, about).as_str())
        .version(version.to_string().as_str())
        .arg(dry_run_as_arg.clone())
        .arg(
            Arg::with_name(ARG_TERMS)
                .help("Terms to search for")
                .conflicts_with(CMD_SCAN)
                .multiple(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name(CMD_SCAN)
                .about("Scan the current directory for new URLs")
                .arg(dry_run_as_arg),
        )
        .get_matches();

    match matches.subcommand() {
        ("scan", Some(scan_matches)) => {
            cli_scan::scan_for_new_urls(option_dry_run(&matches) || option_dry_run(&scan_matches))
        }
        _ => cli_search::search_terms(matches.values_of_lossy(ARG_TERMS), option_dry_run(&matches)),
    }
}

fn option_dry_run(matches: &ArgMatches) -> bool {
    let option = matches.is_present(OPT_DRY_RUN);
    if option {
        println!(
            " {} Mode {} activated.",
            DRY_RUN,
            Green.bold().paint(OPT_DRY_RUN)
        );
    }
    option
}
