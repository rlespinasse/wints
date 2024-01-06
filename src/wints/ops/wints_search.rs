use crate::core::module::WintsModule;
use crate::core::storage::WintsStorage;
use crate::util::log::{CAUTION, DONE, DRY_RUN, GOTO, INFO, SAD, SEARCH, TRY};
use anyhow::Result;
use std::path::PathBuf;
use std::process;

pub struct SearchOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub module_name: String,
    pub matching_terms: Option<Vec<String>>,
    pub dry_run: bool,
}

pub fn search(options: SearchOptions) -> Result<()> {
    println!(" {} Using module '{}'", INFO, options.module_name);
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }

    let storage = WintsStorage::load(
        options.local_basedir.clone(),
        options.global_basedir.clone(),
    )?;

    let module = match storage.find_module(options.module_name.clone()) {
        None => maybe_unknown_module(&options),
        Some(module) => module.clone(),
    };

    match options.matching_terms {
        Some(_) => search_urls_based_on_terms(&options, module),
        None => terms_are_mandatory(module),
    }
    Ok(())
}

fn maybe_unknown_module(options: &SearchOptions) -> WintsModule {
    match options.module_name.as_str() {
        "main" => WintsModule::default_module(),
        _ => {
            println!(
                " {} Unknown module '{}'",
                CAUTION,
                options.module_name.clone()
            );
            process::exit(1);
        }
    }
}

fn terms_are_mandatory(module: WintsModule) {
    println!(" {} No terms passed, can't search anything.", CAUTION);

    if let Some(possible_terms) = module.list_of_all_contexts().first() {
        println!(" {} Try with '{}'.", TRY, possible_terms);
    }
}

fn search_urls_based_on_terms(options: &SearchOptions, module: WintsModule) {
    let matching_terms = options.matching_terms.clone().unwrap();

    println!(
        " {} Searching for '{}'...",
        SEARCH,
        matching_terms.join(" ")
    );

    let urls = module.list_of_urls_from_matching_context(matching_terms);
    match urls.is_empty() {
        true => urls_not_found(options, module),
        false => open_urls(options, urls),
    };

    println!(" {} Search completed.", DONE)
}

fn open_urls(options: &SearchOptions, urls: Vec<String>) {
    for url in urls.iter() {
        match options.dry_run {
            true => println!(" {} Open {}", DRY_RUN, url),
            false => match webbrowser::open(url) {
                Ok(_) => println!(" {} Open {}", GOTO, url),
                Err(why) => println!("can't open {} -> {}", url, why),
            },
        };
    }
}

fn urls_not_found(options: &SearchOptions, module: WintsModule) {
    match module.nearest_matching_context(options.matching_terms.clone().unwrap()) {
        Some(nearest_context) => println!(
            " {} Missed, try with terms like in '{}'.",
            SAD, nearest_context
        ),
        None => println!(" {} Nothing found, try with another term.", SAD),
    }
}
