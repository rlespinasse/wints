use crate::core::module::WintsModule;
use crate::core::storage::WintsStorage;
use crate::util::log::{CAUTION, DRY_RUN, WRITE};
use anyhow::Result;
use std::path::PathBuf;
use std::process;

pub struct InitOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub module_name: String,
    pub global_module: bool,
    pub template: String,
    pub dry_run: bool,
}

pub fn init(options: InitOptions) -> Result<()> {
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }

    println!(
        " {} Initialise '{}' with template '{}'...",
        WRITE, options.module_name, options.template
    );

    let module = match options.template.as_str() {
        "empty" => WintsModule::empty_module(),
        "default" => WintsModule::default_module(),
        _ => {
            println!(
                " {} Need a valid template : '{}' is unknown",
                CAUTION, options.template
            );
            process::exit(1);
        }
    };

    match options.dry_run {
        true => {
            println!(
                " {} Add '{}' module with {} contexts and {} URLs",
                DRY_RUN,
                options.module_name,
                &module.list_of_all_contexts().len(),
                &module.list_of_all_urls().len()
            );
        }
        false => {
            let mut storage = WintsStorage::load(
                options.local_basedir.clone(),
                options.global_basedir.clone(),
            )?;
            storage.add_module(
                options.module_name.clone(),
                module.clone(),
                options.global_module,
            );
            storage.store()?;
            println!(
                " {} A new '{}' module have been created with {} contexts and {} URLs",
                WRITE,
                options.module_name,
                &module.list_of_all_contexts().len(),
                &module.list_of_all_urls().len()
            );
        }
    }

    Ok(())
}
