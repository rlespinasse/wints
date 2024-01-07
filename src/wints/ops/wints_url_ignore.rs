use std::path::PathBuf;

use anyhow::Result;

use crate::core::storage::WintsStorage;
use crate::util::log::{DRY_RUN, WRITE};

pub struct IgnoreUrlOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub global: bool,
    pub url: String,
    pub dry_run: bool,
}

pub fn ignore_url(options: IgnoreUrlOptions) -> Result<()> {
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }

    let scope = match options.global {
        true => "global ignore list",
        false => "ignore list",
    };
    println!(" {} Add '{}' to the {}...", WRITE, options.url, scope);

    match options.dry_run {
        true => {
            println!(" {} Add '{}' to the {}", DRY_RUN, options.url, scope);
        }
        false => {
            let mut storage = WintsStorage::load(
                options.local_basedir.clone(),
                options.global_basedir.clone(),
            )?;
            storage.ignore_url(options.url.clone(), options.global);
            storage.store()?;
            println!(
                " {} '{}' have been added to the {}",
                WRITE, options.url, scope
            );
        }
    }

    Ok(())
}
