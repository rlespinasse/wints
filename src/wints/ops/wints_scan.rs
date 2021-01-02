use crate::core::storage::WintsStorage;
use crate::util::log::{DONE, DRY_RUN, INFO, SAD, SEARCH, WRITE};
use crate::util::url_scanner;
use anyhow::Result;
use std::path::PathBuf;

pub struct ScanOptions {
    pub local_basedir: PathBuf,
    pub global_basedir: PathBuf,
    pub module_name: String,
    pub global_module: bool,
    pub scan_path: PathBuf,
    pub dry_run: bool,
}

pub fn scan(options: ScanOptions) -> Result<()> {
    println!(" {} Using module '{}'", INFO, options.module_name);
    if options.dry_run {
        println!(" {} Dry-run mode activated.", DRY_RUN);
    }
    let mut storage = WintsStorage::load(
        options.local_basedir.clone(),
        options.global_basedir.clone(),
    )?;

    println!(" {} Scanning for new URLs...", SEARCH);
    let url_scanner_options = url_scanner::UrlScannerOptions {
        ignore_files: storage.list_of_ignore_files(),
        scan_path: options.scan_path.clone(),
    };
    let scanned_urls = url_scanner::scan_urls(url_scanner_options);
    let new_urls = find_new_urls(&options, &storage, scanned_urls);
    match new_urls.len() {
        0 => println!(" {} Scan found no new urls.", SAD),
        _ => {
            append_new_urls(options, &mut storage, new_urls)?;
            println!(" {} Scan completed.", DONE);
        }
    }
    Ok(())
}

fn append_new_urls(
    options: ScanOptions,
    storage: &mut WintsStorage,
    new_urls: Vec<String>,
) -> Result<()> {
    let url_word = match new_urls.len() {
        1 => "URL",
        _ => "URLs",
    };
    let context = "need to be contextualised";
    match options.dry_run {
        true => println!(
            " {} Append {} {} on the context '{}'.",
            DRY_RUN,
            new_urls.len(),
            url_word,
            context
        ),
        false => {
            storage.contextualise_urls(
                options.module_name,
                options.global_module,
                context,
                new_urls.clone(),
            );
            storage.store()?;
            println!(
                " {} {} {} have been added in context '{}', ready to be sorted.",
                WRITE,
                new_urls.len(),
                url_word,
                context
            );
        }
    }
    Ok(())
}

fn find_new_urls(
    options: &ScanOptions,
    storage: &WintsStorage,
    scanned_urls: Vec<String>,
) -> Vec<String> {
    let known_urls = storage.list_of_all_urls(options.module_name.clone());
    let ignored_urls = storage.ignored_urls();

    scanned_urls
        .into_iter()
        .filter(|url| !known_urls.contains(url) && !ignored_urls.contains(url))
        .collect()
}
