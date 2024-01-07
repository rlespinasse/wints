use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use wints::ops;
use wints::ops::wints_url_ignore::IgnoreUrlOptions;
use wints::ops::wints_url_ignore_glob::IgnoreGlobOptions;

use crate::commands::{
    general_args, get_global_basedir, get_pathbuf_arg, get_string_arg, global_arg,
};

pub fn command() -> Command {
    Command::new("url")
        .about("Actions about url configuration")
        .subcommand(
            Command::new("ignore")
                .about("Add an URL to the ignore list during scan")
                .args(general_args())
                .arg(global_arg())
                .arg(
                    Arg::new("url")
                        .help("URL to ignore")
                        .value_name("URL")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("ignore-glob")
                .about("Add an glob/file to the ignore list during scan")
                .args(general_args())
                .arg(global_arg())
                .arg(
                    Arg::new("glob")
                        .help("Glob/file to ignore")
                        .value_name("GLOB")
                        .required(true)
                        .index(1),
                ),
        )
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("ignore", ignore_url_matches)) => exec_ignore(ignore_url_matches),
        Some(("ignore-glob", ignore_glob_matches)) => exec_ignore_glob(ignore_glob_matches),
        _ => unreachable!(),
    }
}

pub fn exec_ignore(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let url = get_string_arg(args, "url");
    let global = args.get_flag("global");
    let dry_run = args.get_flag("dry-run");

    ops::wints_url_ignore::ignore_url(IgnoreUrlOptions {
        local_basedir,
        global_basedir,
        global,
        url,
        dry_run,
    })
}

pub fn exec_ignore_glob(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let glob = get_string_arg(args, "glob");
    let global = args.get_flag("global");
    let dry_run = args.get_flag("dry-run");

    ops::wints_url_ignore_glob::ignore_glob(IgnoreGlobOptions {
        local_basedir,
        global_basedir,
        global,
        glob,
        dry_run,
    })
}
