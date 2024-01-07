use anyhow::Result;
use clap::builder::PossibleValuesParser;
use clap::{Arg, ArgMatches, Command};

use wints::ops;
use wints::ops::wints_init::InitOptions;

use crate::commands::{
    general_args, get_global_basedir, get_pathbuf_arg, get_string_arg, global_arg, module_arg,
};

pub fn command() -> Command {
    Command::new("init")
        .about("Initialise a new module")
        .args(general_args())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            Arg::new("template")
                .help("Template name to use")
                .value_name("TEMPLATE")
                .value_parser(PossibleValuesParser::new(["empty", "default"]))
                .default_value("empty")
                .index(1),
        )
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let module_name = get_string_arg(args, "module");
    let template = get_string_arg(args, "template");
    let global_module = args.get_flag("global");
    let dry_run = args.get_flag("dry-run");

    ops::wints_init::init(InitOptions {
        local_basedir,
        global_basedir,
        module_name,
        global_module,
        template,
        dry_run,
    })
}
