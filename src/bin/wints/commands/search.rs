use anyhow::Result;
use clap::{Arg, ArgMatches};

use wints::ops;
use wints::ops::wints_search::SearchOptions;

use crate::commands::{get_global_basedir, get_pathbuf_arg, get_string_arg, module_arg};

pub fn args() -> Vec<Arg> {
    vec![
        module_arg(),
        Arg::new("terms")
            .help("Terms to search for")
            .value_name("TERM")
            .num_args(1..)
            .index(1),
    ]
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let module_name = get_string_arg(args, "module");
    let dry_run = args.get_flag("dry-run");

    let matching_terms = args.get_many::<String>("terms").map(|values_ref| {
        values_ref
            .collect::<Vec<&String>>()
            .iter()
            .map(|&a| a.to_string())
            .collect()
    });

    ops::wints_search::search(SearchOptions {
        local_basedir,
        global_basedir,
        module_name,
        matching_terms,
        dry_run,
    })
}
