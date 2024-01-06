use crate::commands::{general_args, global_arg, module_arg};
use anyhow::Result;
use clap::ArgMatches;
use directories_next::BaseDirs;
use std::path::PathBuf;
use wints::ops;
use wints::ops::wints_init::InitOptions;
use wints::util::command_prelude::*;

pub fn command() -> App {
    subcommand("init")
        .about("Initialise a new module")
        .args(general_args())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            arg("template")
                .help("Template name to use")
                .value_name("TEMPLATE")
                .possible_values(["empty", "default"])
                .default_value("empty")
                .index(1),
        )
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let module_name = args.value_of("module").unwrap().to_string();
    let template = args.value_of("template").unwrap().to_string();
    let global_module = args.is_present("global");
    let dry_run = args.is_present("dry-run");

    ops::wints_init::init(InitOptions {
        local_basedir,
        global_basedir,
        module_name,
        global_module,
        template,
        dry_run,
    })
}
