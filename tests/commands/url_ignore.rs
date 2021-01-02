use crate::WintsCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;
use std::fs;

#[test]
fn ignore_locally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("https://site.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'https://site.tld' to the ignore list..."))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to the ignore list",
        ));

    let config_path = wints.local_config_dir.join("options.yaml");
    assert!(config_path.exists());
    let content = fs::read_to_string(config_path)?;
    assert_eq!(
        content,
        r#"---
version: 1
ignored_urls:
  - "https://site.tld""#
    );

    Ok(())
}

#[test]
fn ignore_globally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("https://site.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to the global ignore list...",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to the global ignore list",
        ));

    let config_path = wints.home_dir_config_dir.join("options.yaml");
    assert!(config_path.exists());
    let content = fs::read_to_string(config_path)?;
    assert_eq!(
        content,
        r#"---
version: 1
ignored_urls:
  - "https://site.tld""#
    );

    Ok(())
}

#[test]
fn multiple_ignore_locally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("https://site.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'https://site.tld' to the ignore list..."))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to the ignore list",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("https://site2.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site2.tld' to the ignore list...",
        ))
        .stdout(contains(
            " 📝 'https://site2.tld' have been added to the ignore list",
        ));

    let config_path = wints.local_config_dir.join("options.yaml");
    assert!(config_path.exists());
    let content = fs::read_to_string(config_path)?;
    assert_eq!(
        content,
        r#"---
version: 1
ignored_urls:
  - "https://site.tld"
  - "https://site2.tld""#
    );

    Ok(())
}

#[test]
fn multiple_ignore_globally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("https://site.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to the global ignore list...",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to the global ignore list",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("https://site2.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site2.tld' to the global ignore list...",
        ))
        .stdout(contains(
            " 📝 'https://site2.tld' have been added to the global ignore list",
        ));

    let config_path = wints.home_dir_config_dir.join("options.yaml");
    assert!(config_path.exists());
    let content = fs::read_to_string(config_path)?;
    assert_eq!(
        content,
        r#"---
version: 1
ignored_urls:
  - "https://site.tld"
  - "https://site2.tld""#
    );

    Ok(())
}

#[test]
fn ignore_dry_run() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("https://site.tld");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'https://site.tld' to the ignore list..."))
        .stdout(contains(" 🌀 Add 'https://site.tld' to the ignore list"));

    Ok(())
}
