use crate::WintsCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;

#[test]
fn specific_path() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints.new_file(
        "file1.txt",
        r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#,
    )?;

    let folder = wints.new_file_in_folder(
        "folder",
        "file2.txt",
        r#"
            Got to https://site4.tld
            Got to https://site5.tld
            Got to https://site6.tld
        "#,
    )?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg(folder.display().to_string());

    wints
        .cmd.assert()
        .success()
        .stdout(contains(" 🔎 Scanning for new URLs..."))
        .stdout(contains(" 📝 3 URLs have been added in context 'need to be contextualised', ready to be sorted."))
        .stdout(contains(" ✅ Scan completed."));

    Ok(())
}

#[test]
fn no_new_urls() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg(wints.current_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Scanning for new URLs..."))
        .stdout(contains(" 😢 Scan found no new urls."));

    Ok(())
}

#[test]
fn using_dry_run() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints.new_file(
        "file1.txt",
        r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#,
    )?;

    wints.new_file(
        "file2.txt",
        r#"
            Got to https://site4.tld
            Got to https://site5.tld
            Got to https://site6.tld
        "#,
    )?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg(wints.current_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Scanning for new URLs..."))
        .stdout(contains(
            " 🌀 Append 6 URLs on the context 'need to be contextualised'.",
        ))
        .stdout(contains(" ✅ Scan completed."));

    Ok(())
}

#[test]
fn using_wrong_command() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--path")
        .arg(wints.current_dir.display().to_string());

    wints.cmd.assert().failure();

    Ok(())
}

#[test]
fn using_another_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints.new_file(
        "file1.txt",
        r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#,
    )?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another")
        .arg(wints.current_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" ℹ️ Using module 'another'"))
        .stdout(contains(" 🔎 Scanning for new URLs..."))
        .stdout(contains(" 📝 3 URLs have been added in context 'need to be contextualised', ready to be sorted."))
        .stdout(contains(" ✅ Scan completed."));

    Ok(())
}

#[test]
fn using_another_global_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints.new_file(
        "file1.txt",
        r#"
            Got to https://site1.tld
            Got to https://site2.tld
            Got to https://site3.tld
        "#,
    )?;

    wints
        .cmd
        .arg("scan")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another")
        .arg("--global")
        .arg(wints.current_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" ℹ️ Using module 'another'"))
        .stdout(contains(" 🔎 Scanning for new URLs..."))
        .stdout(contains(" 📝 3 URLs have been added in context 'need to be contextualised', ready to be sorted."))
        .stdout(contains(" ✅ Scan completed."));

    Ok(())
}
