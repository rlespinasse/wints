use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use wints::ops;
use wints::ops::wints_add::AddOptions;

use crate::commands::{
    general_args, get_global_basedir, get_pathbuf_arg, get_string_arg, global_arg, module_arg,
};

pub fn command() -> Command {
    Command::new("add")
        .about("Add a url to a context")
        .args(general_args())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            Arg::new("url")
                .help("URL to set")
                .value_name("URL")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("context")
                .help("Context of the URL")
                .value_name("CONTEXT")
                .required(true)
                .index(2),
        )
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let module_name = get_string_arg(args, "module");
    let url = get_string_arg(args, "url");
    let context = get_string_arg(args, "context");
    let global_module = args.get_flag("global");
    let dry_run = args.get_flag("dry-run");

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
