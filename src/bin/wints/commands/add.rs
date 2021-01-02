use crate::commands::{general_args, global_arg, module_arg};
use anyhow::Result;
use clap::ArgMatches;
use directories_next::BaseDirs;
use std::path::PathBuf;
use wints::ops;
use wints::ops::wints_add::AddOptions;
use wints::util::command_prelude::*;

pub fn command() -> App {
    subcommand("add")
        .about("Add a url to a context")
        .args(general_args().as_ref())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            arg("url")
                .help("URL to set")
                .value_name("URL")
                .required(true)
                .index(1),
        )
        .arg(
            arg("context")
                .help("Context of the URL")
                .value_name("CONTEXT")
                .required(true)
                .index(2),
        )
}

pub fn exec(args: &ArgMatches<'_>) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let module_name = args.value_of("module").unwrap().to_string();
    let url = args.value_of("url").unwrap().to_string();
    let context = args.value_of("context").unwrap().to_string();
    let global_module = args.is_present("global");
    let dry_run = args.is_present("dry-run");

    ops::wints_add::add(AddOptions {
        local_basedir,
        global_basedir,
        module_name,
        global_module,
        url,
        context,
        dry_run,
    })
}
