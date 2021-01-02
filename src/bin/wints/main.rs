#[macro_use]
extern crate clap;

pub mod cli;
pub mod commands;

use anyhow::Result;

fn main() -> Result<()> {
    cli::main()
}
