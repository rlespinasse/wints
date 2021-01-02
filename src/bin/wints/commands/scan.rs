use crate::commands::{general_args, global_arg, module_arg};
use anyhow::Result;
use clap::ArgMatches;
use directories_next::BaseDirs;
use std::path::PathBuf;
use wints::ops;
use wints::ops::wints_scan::ScanOptions;
use wints::util::command_prelude::*;

pub fn command() -> App {
    subcommand("scan")
        .about("Scan a directory tree for new URLs")
        .args(general_args().as_ref())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            arg("path")
                .help("Path to scan (file or directories)")
                .value_name("PATH")
                .default_value(".")
                .index(1),
        )
}

pub fn exec(args: &ArgMatches<'_>) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let module_name = args.value_of("module").unwrap().to_string();
    let scan_path = PathBuf::from(args.value_of("path").unwrap());
    let global_module = args.is_present("global");
    let dry_run = args.is_present("dry-run");

    ops::wints_scan::scan(ScanOptions {
        local_basedir,
        global_basedir,
        module_name,
        global_module,
        scan_path,
        dry_run,
    })
}
