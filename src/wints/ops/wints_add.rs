use crate::core::storage::WintsStorage;
use crate::util::log::{DRY_RUN, WRITE};
use anyhow::Result;
use std::path::PathBuf;

pub struct AddOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub module_name: String,
    pub global_module: bool,
    pub url: String,
    pub context: String,
    pub dry_run: bool,
}

pub fn add(options: AddOptions) -> Result<()> {
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }

    println!(
        " {} Add '{}' to '{}' in '{}'...",
        WRITE, options.url, options.context, options.module_name,
    );

    match options.dry_run {
        true => {
            println!(
                " {} Add '{}' to '{}' in '{}'",
                DRY_RUN, options.url, options.context, options.module_name
            );
        }
        false => {
            let mut storage = WintsStorage::load(
                options.local_basedir.clone(),
                options.global_basedir.clone(),
            )?;
            storage.contextualise_urls(
                options.module_name.clone(),
                options.global_module,
                options.context.as_str(),
                vec![options.url.clone()],
            );
            storage.store()?;
            println!(
                " {} '{}' have been added to '{}' in '{}'",
                WRITE, options.url, options.context, options.module_name
            );
        }
    }

    Ok(())
}
