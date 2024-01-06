use crate::commands;
use anyhow::Result;
use clap::AppSettings;
use wints::util::command_prelude::*;

pub fn main() -> Result<()> {
    let matches = cli().get_matches();

    let (command_exec, command_args) = match matches.subcommand_name() {
        None => (commands::global_exec(), &matches),
        Some(cmd) => (
            commands::builtin_exec(cmd),
            matches.subcommand_matches(cmd).unwrap(),
        ),
    };

    command_exec(command_args)
}

fn cli() -> clap::App<'static> {
    let args = commands::global_args();
    let subcommands = commands::builtin();

    App::new("wints")
        .about("What I Need To See - a fuzzy term-based URLs opener")
        .version(crate_version!())
        .settings(&[
            AppSettings::DeriveDisplayOrder,
            AppSettings::AllowExternalSubcommands,
        ])
        .args(args)
        .subcommands(subcommands)
}
