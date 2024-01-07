use anyhow::Result;
use clap::{Arg, ArgMatches, Command};

use wints::ops;
use wints::ops::wints_scan::ScanOptions;

use crate::commands::{
    general_args, get_global_basedir, get_pathbuf_arg, get_string_arg, global_arg, module_arg,
};

pub fn command() -> Command {
    Command::new("scan")
        .about("Scan a directory tree for new URLs")
        .args(general_args())
        .arg(module_arg())
        .arg(global_arg())
        .arg(
            Arg::new("path")
                .help("Path to scan (file or directories)")
                .value_name("PATH")
                .default_value(".")
                .index(1),
        )
}

pub fn exec(args: &ArgMatches) -> Result<()> {
    let local_basedir = get_pathbuf_arg(args, "config");
    let global_basedir = get_global_basedir(args);
    let module_name = get_string_arg(args, "module");
    let scan_path = get_pathbuf_arg(args, "path");
    let global_module = args.get_flag("global");
    let dry_run = args.get_flag("dry-run");

    ops::wints_scan::scan(ScanOptions {
        local_basedir,
        global_basedir,
        module_name,
        global_module,
        scan_path,
        dry_run,
    })
}
