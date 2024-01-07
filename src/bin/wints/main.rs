#[macro_use]
extern crate clap;

use anyhow::Result;

pub mod cli;
pub mod commands;

fn main() -> Result<()> {
    cli::main()
}
