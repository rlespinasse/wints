use std::path::PathBuf;

use anyhow::Result;

use crate::core::storage::WintsStorage;
use crate::util::log::{DRY_RUN, WRITE};

pub struct IgnoreGlobOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub global: bool,
    pub glob: String,
    pub dry_run: bool,
}

pub fn ignore_glob(options: IgnoreGlobOptions) -> Result<()> {
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }

    let scope = match options.global {
        true => "global ignore list",
        false => "ignore list",
    };
    println!(" {} Add '{}' to the {}...", WRITE, options.glob, scope);

    match options.dry_run {
        true => {
            println!(" {} Add '{}' to the {}", DRY_RUN, options.glob, scope);
        }
        false => {
            let storage = WintsStorage::load(
                options.local_basedir.clone(),
                options.global_basedir.clone(),
            )?;
            storage.ignore_glob(options.glob.clone(), options.global)?;
            println!(
                " {} '{}' have been added to the {}",
                WRITE, options.glob, scope
            );
        }
    }

    Ok(())
}
