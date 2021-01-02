use crate::WintsCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;

#[test]
fn add_to_main_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("add")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("https://site.tld")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to 'context' in 'main'..",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to 'context' in 'main'",
        ));

    wints.new_cmd()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'context'..."))
        .stdout(contains(" 🌀 Open https://site.tld"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn add_using_context_with_spaces() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("add")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("https://site.tld")
        .arg("some context about this url");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to 'some context about this url' in 'main'..",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to 'some context about this url' in 'main'",
        ));

    wints.new_cmd()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("context url");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'context url'..."))
        .stdout(contains(" 🌀 Open https://site.tld"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn add_to_some_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("add")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("some")
        .arg("https://site.tld")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to 'context' in 'some'..",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to 'context' in 'some'",
        ));

    wints.new_cmd()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("--module")
        .arg("some")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'context'..."))
        .stdout(contains(" 🌀 Open https://site.tld"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn add_to_some_global_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("add")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("some")
        .arg("--global")
        .arg("https://site.tld")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Add 'https://site.tld' to 'context' in 'some'..",
        ))
        .stdout(contains(
            " 📝 'https://site.tld' have been added to 'context' in 'some'",
        ));

    wints.new_cmd()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("--module")
        .arg("some")
        .arg("context");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'context'..."))
        .stdout(contains(" 🌀 Open https://site.tld"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}
