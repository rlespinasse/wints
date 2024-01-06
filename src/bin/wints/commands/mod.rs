mod add;
mod init;
mod scan;
mod search;
mod url;

use anyhow::Result;
use clap::ArgMatches;
use wints::util::command_prelude::*;

pub fn builtin() -> Vec<App> {
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
        arg("config")
            .help("Folder of local configuration storage")
            .value_name("PATH")
            .default_value(".wints")
            .short('C')
            .long("config"),
        arg("global-config")
            .help("Folder of global configuration storage [default: HOME_DIR/.wints]")
            .value_name("PATH")
            .short('G')
            .long("global-config"),
        arg("dry-run")
            .help("Do not actually change anything, just log what are going to do")
            .short('n')
            .long("dry-run"),
    ]
}

pub fn module_arg() -> Arg {
    arg("module")
        .help("Module name to use")
        .value_name("MODULE NAME")
        .default_value("main")
        .short('m')
        .long("module")
}

pub fn global_arg() -> Arg {
    arg("global")
        .help("Work with global configuration")
        .short('g')
        .long("global")
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
