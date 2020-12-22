use std::path::Path;
use std::process::exit;

use ansi_term::Colour::{Green, Red};

use crate::cfg::Config;
use crate::cli_log::{CAUTION, DRY_RUN, WRITE};

pub(crate) static CONFIG_FILENAME: &str = ".wints.yaml";

pub(crate) fn load_configuration(config_file: &Path, dry_run: bool) -> Config {
    ensure_configuration_file_exists(config_file, dry_run);
    read_configuration(config_file).unwrap()
}

pub(crate) fn read_configuration(config_file: &Path) -> Option<Config> {
    if !config_file.exists() {
        return None;
    }
    match Config::read_file(config_file) {
        Err(why) => {
            eprintln!(
                "can't read {}\n{}",
                Green.bold().paint(config_file.display().to_string()),
                Red.paint(why.to_string())
            );
            exit(1)
        }
        Ok(config) => Some(config),
    }
}

fn ensure_configuration_file_exists(config_file: &Path, dry_run: bool) {
    if !config_file.exists() {
        let filename = config_file.display().to_string();
        println!(
            " {} Can't find any {} file.",
            CAUTION,
            Green.bold().paint(&filename),
        );

        match dry_run {
            true => {
                println!(
                    " {} Create a default file named {}.",
                    DRY_RUN,
                    Green.bold().paint(&filename)
                );
                exit(0)
            }
            false => create_default_configuration_file(config_file),
        }
    }
}

fn create_default_configuration_file(config_file: &Path) {
    let filename = config_file.display().to_string();
    match Config::write_default_file(config_file) {
        Err(why) => {
            eprintln!(
                "couldn't create {}\n{}",
                Green.bold().paint(&filename),
                Red.paint(why.to_string())
            );
            exit(1)
        }
        Ok(_) => println!(
            " {} So an empty {} file has been created.",
            WRITE,
            Green.bold().paint(&filename)
        ),
    }
}
