use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use tempfile::tempdir;

mod commands;

pub struct WintsCommand {
    pub cmd: Command,
    pub current_dir: PathBuf,
    pub local_config_dir: PathBuf,
    pub home_dir_config_dir: PathBuf,
}

impl WintsCommand {
    pub fn new_file(&self, name: &str, content: &str) -> Result<()> {
        let mut file = File::create(self.current_dir.join(name))?;
        writeln!(file, "{}", content)?;
        Ok(())
    }

    pub fn new_file_in_folder(&self, folder: &str, name: &str, content: &str) -> Result<PathBuf> {
        let folder = self.current_dir.join(folder);
        fs::create_dir_all(folder.clone())?;
        let mut file = File::create(folder.join(name))?;
        writeln!(file, "{}", content)?;
        Ok(folder)
    }

    pub fn new_cmd(&mut self) -> Result<()> {
        self.cmd = Command::cargo_bin("wints")?;
        Ok(())
    }

    pub fn new() -> Result<WintsCommand> {
        let cmd = Command::cargo_bin("wints")?;

        let tempdir = tempdir()?;
        let current_dir = tempdir.into_path();
        let local_config_dir = current_dir.join("local_config_dir");
        let home_dir_config_dir = current_dir.join("home_dir_config_dir");

        let wints_command = WintsCommand {
            cmd,
            current_dir,
            local_config_dir,
            home_dir_config_dir,
        };

        Ok(wints_command)
    }
}
