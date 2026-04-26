use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

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
fn sum_empty_input() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
    Ok(())
}

#[test]
fn sum_field_zero() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = "1\n2\n3\n";
    cmd.write_stdin(input)
        .args(["-f=0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("6"));
    Ok(())
}

#[test]
fn sum_field_out_of_range() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = "1 2 3\n4 5 6\n";
    cmd.write_stdin(input)
        .args(["-f=99"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"))
        .stderr(predicate::str::contains(
            "Field index out of range, skipping",
        ));
    Ok(())
}

#[test]
fn sum_negative_integers() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("3\n-1\n-2\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
    Ok(())
}

#[test]
fn sum_negative_floats() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("1.5\n-0.5\n-0.5\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.5"));
    Ok(())
}

#[test]
fn sum_negative_mixed() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("3\n-1.5\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("1.5"));
    Ok(())
}

#[test]
fn sum_comma_numbers() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("1,000\n2,000\n3,000\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("6000"))
        .stderr(predicate::str::contains("Stripped commas from value"));
    Ok(())
}

#[test]
fn sum_invalid_0x_prefix() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("0xGG\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"))
        .stderr(predicate::str::contains(
            "Failed to parse as hex, treating as 0",
        ));
    Ok(())
}

#[test]
fn sum_field_out_of_range_no_double_warn() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("1 2 3\n")
        .args(["-f=99"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Field index out of range, skipping",
        ))
        .stderr(
            predicate::str::contains("Failed to parse (use --radix=hex if hex), treating as 0")
                .not(),
        );
    Ok(())
}

#[test]
fn sum_large_integer_warns() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    // i128::MAX is 170141183460469231731687303715884105727, so one digit more overflows
    cmd.write_stdin("999999999999999999999999999999999999999999\n")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Value too large for integer, using float (may lose precision)",
        ));
    Ok(())
}

#[test]
fn sum_radix_decimal_rejects_0x_prefix() -> TestResult {
    // With --radix=decimal, a 0x-prefixed value must not be auto-detected as hex.
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("5\n0xFF\n")
        .args(["--radix=decimal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"))
        .stderr(predicate::str::contains("Failed to parse"));
    Ok(())
}

#[test]
fn sum_hex_flag_no_float_fallback() -> TestResult {
    // With -x, values that look like floats must not silently fall back to float parsing.
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin("1.5\n2.5\n")
        .args(["--radix=hex"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"))
        .stderr(predicate::str::contains(
            "Failed to parse as hex, treating as 0",
        ));
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
        .args(["-f2", "--radix=hex"])
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
        .stdout(predicate::str::contains("4"))
        .stderr(predicate::str::contains(
            "Failed to parse (use --radix=hex if hex), treating as 0",
        ));
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
        .stderr(predicate::str::contains(
            "Failed to parse (use --radix=hex if hex), treating as 0",
        ));
    Ok(())
}

#[test]
fn sum_float() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 blah
    hello 1.0 foo
    hello 2.2 oo
    blah 3e0 mumble
    ";
    cmd.write_stdin(input)
        .args(["-f=2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8.2"));

    // With -x, 3e0 will be parsed as 0x3e0 (decimal 992)
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin(input)
        .args(["-f=2", "--radix=hex"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0x3E2")) // 2 + 0x3e0=992 = 994; 1.0 and 2.2 fail hex
        .stderr(predicate::str::contains(
            "Failed to parse as hex, treating as 0",
        ));
    Ok(())
}

#[test]
fn sum_float_0xhex() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 blah
    hello 0xA blah
    hello 1.0 foo
    hello 2.2 oo
    ";
    cmd.write_stdin(input)
        .args(["-f=2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("15.2"));
    Ok(())
}

#[test]
fn sum_float_hex_flag() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 blah
    hello A blah
    hello 1.0 foo
    hello 2.2 oo
    ";
    // Without the -x flag, the A in the second line will be ignored.
    cmd.write_stdin(input)
        .args(["-f=2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5.2"));

    // With -x, A=10 and 2 are parsed as hex; 1.0 and 2.2 fail hex parsing and warn.
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.write_stdin(input)
        .args(["-f=2", "--radix=hex"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0xC"))
        .stderr(predicate::str::contains(
            "Failed to parse as hex, treating as 0",
        ));
    Ok(())
}

#[test]
fn sum_nonexistent_file() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.arg("/nonexistent/path/to/file.txt").assert().failure();
    Ok(())
}

#[test]
fn sum_one_file() -> TestResult {
    let mut file = tempfile::NamedTempFile::new()?;
    writeln!(file, "1\n2\n3")?;
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("6"));
    Ok(())
}

#[test]
fn sum_two_files() -> TestResult {
    let mut file1 = tempfile::NamedTempFile::new()?;
    writeln!(file1, "1\n2\n3")?;
    let mut file2 = tempfile::NamedTempFile::new()?;
    writeln!(file2, "4\n5\n6")?;
    let mut cmd = Command::cargo_bin("sumcol")?;
    cmd.args([file1.path(), file2.path()])
        .assert()
        .success()
        .stdout(predicate::str::contains("21"));
    Ok(())
}

#[test]
fn sum_verbose_flag() -> TestResult {
    let mut cmd = Command::cargo_bin("sumcol")?;
    let input = r"
    hello 2 blah
    hello OOPS blah
    hello 1.0 foo
    hello 2.2 oo
    ";

    // Without the -x flag, the A in the second line will be ignored.
    cmd.write_stdin(input)
        .args(["-f=2", "-v"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"n=Integer(2) sum=Integer(2) radix=Decimal raw_str="2""#,
        ))
        .stdout(predicate::str::contains(
            r#"n=Integer(0) sum=Integer(2) radix=Decimal raw_str="OOPS" err="Failed to parse (use --radix=hex if hex), treating as 0""#,
        ))
        .stdout(predicate::str::contains(
            r#"n=Float(2.2) sum=Float(5.2) radix=Decimal raw_str="2.2""#
        ))
        .stdout(predicate::str::contains(
            r#"=="#))
        .stdout(predicate::str::contains(
            r#"5.2"#
        ));
    Ok(())
}
