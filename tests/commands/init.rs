use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;

use crate::WintsCommand;

#[test]
fn no_template() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("init")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Initialise 'main' with template 'empty'..."))
        .stdout(contains(
            " 📝 A new 'main' module have been created with 0 contexts and 0 URLs",
        ));

    wints.new_cmd()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("repo");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Searching for 'repo'..."))
        .stdout(contains(" 😢 Nothing found, try with another term."))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn default_template() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("init")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("default");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 📝 Initialise 'main' with template 'default'..."))
        .stdout(contains(
            " 📝 A new 'main' module have been created with 4 contexts and 4 URLs",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("repo");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'repo'..."))
        .stdout(contains(" 🌀 Open https://github.com/rlespinasse/wints"))
        .stdout(contains(
            " 🌀 Open https://github.com/rlespinasse/wints/issues",
        ))
        .stdout(contains(" 🌀 Open https://crates.io/crates/wints"))
        .stdout(contains(
            " 🌀 Open https://github.com/rlespinasse/wints/blob/v0.x/USAGE.adoc",
        ))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn using_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("init")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Initialise 'another' with template 'empty'...",
        ))
        .stdout(contains(
            " 📝 A new 'another' module have been created with 0 contexts and 0 URLs",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another")
        .arg("--dry-run")
        .arg("repo");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Searching for 'repo'..."))
        .stdout(contains(" 😢 Nothing found, try with another term."))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn using_global_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("init")
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another")
        .arg("--global");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(
            " 📝 Initialise 'another' with template 'empty'...",
        ))
        .stdout(contains(
            " 📝 A new 'another' module have been created with 0 contexts and 0 URLs",
        ));

    wints.new_cmd()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another")
        .arg("--dry-run")
        .arg("repo");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Searching for 'repo'..."))
        .stdout(contains(" 😢 Nothing found, try with another term."))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}
