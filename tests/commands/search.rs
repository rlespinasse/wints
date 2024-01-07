use anyhow::Result;
use assert_cmd::prelude::*;
use predicate::str::contains;
use predicates::prelude::*;

use crate::WintsCommand;

#[test]
fn using_unknown_module() -> Result<()> {
    let mut wints = WintsCommand::new()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--module")
        .arg("another");

    wints
        .cmd
        .assert()
        .failure()
        .stdout(contains(" ⚠️ Unknown module 'another'"));

    Ok(())
}

#[test]
fn ask_for_terms() -> Result<()> {
    let mut wints = WintsCommand::new()?;
    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string());

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" ⚠️ No terms passed, can't search anything."))
        .stdout(contains(" 🧭 Try with 'repository code'."));

    Ok(())
}

#[test]
fn using_dry_run() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" ⚠️ No terms passed, can't search anything."))
        .stdout(contains(" 🧭 Try with 'repository code'."));

    Ok(())
}

#[test]
fn open_urls_with_one_word_from_one_context() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("code");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'code'..."))
        .stdout(contains(" 🌀 Open https://github.com/rlespinasse/wints"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn open_urls_with_multiple_words_from_one_context() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("--dry-run")
        .arg("repo")
        .arg("code");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🌀 Dry-run mode activated."))
        .stdout(contains(" 🔎 Searching for 'repo code'..."))
        .stdout(contains(" 🌀 Open https://github.com/rlespinasse/wints"))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn open_urls_with_one_word_from_multiple_contexts() -> Result<()> {
    let mut wints = WintsCommand::new()?;

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
fn propose_matching_terms_when_some_words_are_unmatched() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("repo")
        .arg("source");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Searching for 'repo source'..."))
        .stdout(contains(
            " 😢 Missed, try with terms like in 'repository code'.",
        ))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}

#[test]
fn all_words_are_unmatched() -> Result<()> {
    let mut wints = WintsCommand::new()?;

    wints
        .cmd
        .arg("--config")
        .arg(wints.local_config_dir.display().to_string())
        .arg("--global-config")
        .arg(wints.home_dir_config_dir.display().to_string())
        .arg("git")
        .arg("source");

    wints
        .cmd
        .assert()
        .success()
        .stdout(contains(" 🔎 Searching for 'git source'..."))
        .stdout(contains(" 😢 Nothing found, try with another term."))
        .stdout(contains(" ✅ Search completed."));

    Ok(())
}
