use anyhow::Result;
use clap::Command;

use crate::commands;

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

fn cli() -> Command {
    let args = commands::global_args();
    let subcommands = commands::builtin();

    Command::new("wints")
        .about("What I Need To See - a fuzzy term-based URLs opener")
        .version(crate_version!())
        .allow_external_subcommands(true)
        .args(args)
        .subcommands(subcommands)
}
