use std::path::PathBuf;

use anyhow::Result;
use clap::ArgAction::SetTrue;
use clap::{Arg, ArgMatches, Command};
use directories_next::BaseDirs;

mod add;
mod init;
mod scan;
mod search;
mod url;

pub fn builtin() -> Vec<Command> {
    vec![
        init::command(),
        add::command(),
        scan::command(),
        url::command(),
    ]
}

pub fn global_args() -> Vec<Arg> {
    vec![general_args(), search::args()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
}

pub fn general_args() -> Vec<Arg> {
    vec![
        Arg::new("config")
            .help("Folder of local configuration storage")
            .value_name("PATH")
            .default_value(".wints")
            .short('C')
            .long("config"),
        Arg::new("global-config")
            .help("Folder of global configuration storage [default: HOME_DIR/.wints]")
            .value_name("PATH")
            .short('G')
            .long("global-config"),
        Arg::new("dry-run")
            .help("Do not actually change anything, just log what are going to do")
            .short('n')
            .long("dry-run")
            .action(SetTrue),
    ]
}

pub fn module_arg() -> Arg {
    Arg::new("module")
        .help("Module name to use")
        .value_name("MODULE NAME")
        .default_value("main")
        .short('m')
        .long("module")
}

pub fn global_arg() -> Arg {
    Arg::new("global")
        .help("Work with global configuration")
        .short('g')
        .long("global")
        .action(SetTrue)
}

pub fn builtin_exec(cmd: &str) -> fn(&ArgMatches) -> Result<()> {
    match cmd {
        "init" => init::exec,
        "add" => add::exec,
        "scan" => scan::exec,
        "url" => url::exec,
        _ => global_exec(),
    }
}

pub fn global_exec() -> fn(&ArgMatches) -> Result<()> {
    search::exec
}

fn get_string_arg(args: &ArgMatches, arg_name: &str) -> String {
    args.get_one::<String>(arg_name)
        .map(|s| s.as_str())
        .unwrap()
        .to_string()
}

fn get_pathbuf_arg(args: &ArgMatches, arg_name: &str) -> PathBuf {
    PathBuf::from(get_string_arg(args, arg_name))
}

fn get_global_basedir(args: &ArgMatches) -> PathBuf {
    match args.get_one::<String>("global-config").map(|s| s.as_str()) {
        None => BaseDirs::new().unwrap().home_dir().join(".wints"),
        Some(value) => PathBuf::from(value),
    }
}
