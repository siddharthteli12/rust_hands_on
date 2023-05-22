use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli_app")?;

    cmd.arg("--pattern").arg("sid").arg("--path").arg("temp.rs");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn find_pattern_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("find_pattern")?;
    file.write_str("My name is\ndadas\nname")?;
    // Calling binary with pattern & file path.
    let mut cmd = Command::cargo_bin("cli_app")?;
    cmd.arg("--pattern")
        .arg("name")
        .arg("--path")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0,My name is\n2,name"));

    Ok(())
}

#[test]
fn find_with_empty_pattern_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("find_pattern")?;
    file.write_str("My name is\ndadas\nname")?;
    // Calling binary with pattern & file path.
    let mut cmd = Command::cargo_bin("cli_app")?;
    cmd.arg("--pattern").arg("").arg("--path").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0,My name is\n1,dadas\n2,name"));

    Ok(())
}
