use crate::commands::{general_args, global_arg};
use anyhow::Result;
use clap::ArgMatches;
use directories_next::BaseDirs;
use std::path::PathBuf;
use wints::ops;
use wints::ops::wints_url_ignore::IgnoreUrlOptions;
use wints::ops::wints_url_ignore_glob::IgnoreGlobOptions;
use wints::util::command_prelude::*;

pub fn command() -> App {
    subcommand("url")
        .about("Actions about url configuration")
        .subcommand(
            subcommand("ignore")
                .about("Add an URL to the ignore list during scan")
                .args(general_args().as_ref())
                .arg(global_arg())
                .arg(
                    arg("url")
                        .help("URL to ignore")
                        .value_name("URL")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            subcommand("ignore-glob")
                .about("Add an glob/file to the ignore list during scan")
                .args(general_args().as_ref())
                .arg(global_arg())
                .arg(
                    arg("glob")
                        .help("Glob/file to ignore")
                        .value_name("GLOB")
                        .required(true)
                        .index(1),
                ),
        )
}

pub fn exec(args: &ArgMatches<'_>) -> Result<()> {
    match args.subcommand() {
        ("ignore", Some(ignore_url_matches)) => exec_ignore(ignore_url_matches),
        ("ignore-glob", Some(ignore_glob_matches)) => exec_ignore_glob(ignore_glob_matches),
        _ => unreachable!(),
    }
}

pub fn exec_ignore(args: &ArgMatches<'_>) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let url = args.value_of("url").unwrap().to_string();
    let global = args.is_present("global");
    let dry_run = args.is_present("dry-run");

    ops::wints_url_ignore::ignore_url(IgnoreUrlOptions {
        local_basedir,
        global_basedir,
        global,
        url,
        dry_run,
    })
}

pub fn exec_ignore_glob(args: &ArgMatches<'_>) -> Result<()> {
    let local_basedir = PathBuf::from(args.value_of("config").unwrap().to_string());
    let global_basedir = match args.value_of("global-config") {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    };
    let glob = args.value_of("glob").unwrap().to_string();
    let global = args.is_present("global");
    let dry_run = args.is_present("dry-run");

    ops::wints_url_ignore_glob::ignore_glob(IgnoreGlobOptions {
        local_basedir,
        global_basedir,
        global,
        glob,
        dry_run,
    })
}
