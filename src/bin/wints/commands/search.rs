use crate::commands::module_arg;
use anyhow::Result;
use clap::ArgMatches;
use directories_next::BaseDirs;
use std::path::PathBuf;
use wints::ops;
use wints::ops::wints_search::SearchOptions;
use wints::util::command_prelude::*;

pub fn args() -> Vec<Arg> {
    vec![
        module_arg(),
        arg("terms")
            .help("Terms to search for")
            .value_name("TERM")
            .conflicts_with("scan")
            .multiple(true)
            .index(1),
    ]
}

pub fn exec(args: &ArgMatches<'_>) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let module_name = args.value_of("module").unwrap().to_string();
    let matching_terms = args.values_of_lossy("terms");
    let dry_run = args.is_present("dry-run");

    ops::wints_search::search(SearchOptions {
        local_basedir,
        global_basedir,
        module_name,
        matching_terms,
        dry_run,
    })
}
