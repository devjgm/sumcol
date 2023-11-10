use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn help_works() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: sumcol"));
    Ok(())
}

#[test]
fn no_args_sum() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    1
    2
    3
    ";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("6"));
    Ok(())
}

#[test]
fn simple_column_sum() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 foo
    hello 2 foo
    hello 2 foo
    ";
    cmd.write_stdin(input)
        .args(["-f2"])
        // .env("RUST_LOG", "debug")
        .assert()
        .success()
        .stdout(predicate::str::contains("6"));
    Ok(())
}

#[test]
fn sum_implicit_hex() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 foo
    hello 0xa foo
    hello 0xB foo
    ";
    cmd.write_stdin(input)
        .args(["-f2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("23"));
    Ok(())
}

#[test]
fn sum_explicit_hex() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 foo
    hello a foo
    hello 0xa foo
    hello 0xB foo
    ";
    cmd.write_stdin(input)
        .args(["-f2", "-x"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0x21")); // 33 in decimal
    Ok(())
}

#[test]
fn sum_delimiter() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello:2:foo
    hello:2:foo
    hello:2:foo
    ";
    cmd.write_stdin(input)
        .args(["-f=2", "-d:"])
        .assert()
        .success()
        .stdout(predicate::str::contains("6")); // 33 in decimal
    Ok(())
}

#[test]
fn sum_first_delimiter() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    :2:foo
    :2:foo
    :2:foo
    ";
    cmd.write_stdin(input)
        .args(["-f=2", "-d:"])
        .assert()
        .success()
        .stdout(predicate::str::contains("6")); // 33 in decimal
    Ok(())
}

#[test]
fn sum_mixed_column() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 foo
    hello OOPS foo
    hello 2 foo
    ";
    cmd.write_stdin(input)
        .args(["-f=2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4"));
    Ok(())
}

#[test]
fn sum_mixed_column_looks_like_number() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 foo
    hello a foo
    hello 2 foo
    ";
    cmd.write_stdin(input)
        .args(["-f=2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4"))
        .stderr(predicate::str::contains("Consider using"));
    Ok(())
}
