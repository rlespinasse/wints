use crate::WintsCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;
use std::fs;

#[test]
fn ignore_glob_locally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("FILE");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE' to the ignore list..."))
        .stdout(contains(" 📝 'FILE' have been added to the ignore list"));

    let ignore_path = wints.local_config_dir.join("ignore");
    assert!(ignore_path.exists());
    let content = fs::read_to_string(ignore_path)?;
    assert_eq!(
        content,
        r#"FILE
"#
    );

    Ok(())
}

#[test]
fn ignore_glob_globally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("FILE");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE' to the global ignore list..."))
        .stdout(contains(
            " 📝 'FILE' have been added to the global ignore list",
        ));

    let ignore_path = wints.home_dir_config_dir.join("ignore");
    assert!(ignore_path.exists());
    let content = fs::read_to_string(ignore_path)?;
    assert_eq!(
        content,
        r#"FILE
"#
    );

    Ok(())
}

#[test]
fn multiple_ignore_glob_locally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("FILE");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE' to the ignore list..."))
        .stdout(contains(" 📝 'FILE' have been added to the ignore list"));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("FILE2");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE2' to the ignore list..."))
        .stdout(contains(" 📝 'FILE2' have been added to the ignore list"));

    let ignore_path = wints.local_config_dir.join("ignore");
    assert!(ignore_path.exists());
    let content = fs::read_to_string(ignore_path)?;
    assert_eq!(
        content,
        r#"FILE
FILE2
"#
    );

    Ok(())
}

#[test]
fn multiple_ignore_glob_globally() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("FILE");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE' to the global ignore list..."))
        .stdout(contains(
            " 📝 'FILE' have been added to the global ignore list",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--global")
        .arg("FILE2");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE2' to the global ignore list..."))
        .stdout(contains(
            " 📝 'FILE2' have been added to the global ignore list",
        ));

    let ignore_path = wints.home_dir_config_dir.join("ignore");
    assert!(ignore_path.exists());
    let content = fs::read_to_string(ignore_path)?;
    assert_eq!(
        content,
        r#"FILE
FILE2
"#
    );

    Ok(())
}

#[test]
fn ignore_glob_dry_run() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("url")
        .arg("ignore-glob")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("FILE");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Add 'FILE' to the ignore list..."))
        .stdout(contains(" 🌀 Add 'FILE' to the ignore list"));

    Ok(())
}
