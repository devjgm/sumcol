use clap::{Parser, ValueEnum};
use colored::Colorize;
use regex::Regex;
use std::io::{self, BufRead, BufReader};
use sumcol::Sum;

/// How to interpret numeric input.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Radix {
    /// Decimal unless the value has a leading 0x prefix (default).
    Auto,
    /// Always hex; values with a 0x prefix have it stripped first.
    Hex,
    /// Always decimal; 0x-prefixed values fail to parse.
    Decimal,
}

/// Sum a column of numbers from text input.
///
/// Examples:
///   ls -l | sumcol -f5
///
#[derive(Parser, Debug)]
#[command(version, verbatim_doc_comment)]
struct Args {
    /// The field to sum. If not specified, uses the full line.
    #[arg(long, short, default_value("0"))]
    field: usize,

    /// How to interpret numeric input.
    #[arg(long, value_enum, default_value_t = Radix::Auto)]
    radix: Radix,

    /// The regex on which to split fields.
    #[arg(long, short, default_value(r"\s+"))]
    delimiter: Regex,

    /// Print each number that's being summed, along with some metadata
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Files to read input from, otherwise uses stdin.
    #[arg(trailing_var_arg = true)]
    pub files: Vec<String>,
}

fn fmt_sum(sum: Sum, radix: Radix) -> String {
    match radix {
        Radix::Hex => format!("{sum:#X}"),
        _ => format!("{sum}"),
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .without_time()
        .with_writer(std::io::stderr)
        .init();
    let args = Args::parse();
    tracing::debug!(?args, "Starting sumcol");

    let readers: Vec<Box<dyn BufRead>> = if args.files.is_empty() {
        vec![Box::new(BufReader::new(io::stdin()))]
    } else {
        args.files
            .iter()
            .map(|f| fs_err::File::open(f).map(|f| Box::new(BufReader::new(f)) as Box<dyn BufRead>))
            .collect::<Result<_, _>>()?
    };

    let mut sum = Sum::Integer(0);
    for line in readers.into_iter().flat_map(|r| r.lines()) {
        let line = line?.trim().to_string();
        tracing::debug!(?line, "Read line");
        if line.is_empty() {
            continue;
        }
        let raw_str = match args.field {
            0 => Some(line.as_str()),
            f => args.delimiter.split(&line).nth(f - 1),
        };
        let Some(raw_str) = raw_str else {
            tracing::warn!(
                field = args.field,
                line,
                "Field index out of range, skipping"
            );
            continue;
        };
        let trimmed = raw_str.trim();
        let clean_str = trimmed.replace(',', "");
        if clean_str != trimmed {
            tracing::warn!(
                original = trimmed,
                clean = clean_str.as_str(),
                "Stripped commas from value"
            );
        }
        let (clean_str, radix) = match (args.radix, clean_str.strip_prefix("0x")) {
            (Radix::Decimal, _) => (clean_str.as_str(), Radix::Decimal),
            (_, Some(s)) => (s, Radix::Hex),
            (Radix::Hex, None) => (clean_str.as_str(), Radix::Hex),
            (Radix::Auto, None) => (clean_str.as_str(), Radix::Decimal),
        };
        let (n, err) = match parse_value(clean_str, radix) {
            Ok(n) => (n, None),
            Err(msg) => {
                tracing::warn!(?clean_str, "{msg}");
                (Sum::Integer(0), Some(msg))
            }
        };
        sum += n;
        if args.verbose {
            let meta = format!("# n={n:?} sum={sum:?} radix={radix:?} raw_str={raw_str:?}").cyan();
            let err_str = err
                .map(|e| format!(" err={e:?}").red().to_string())
                .unwrap_or_default();
            println!("{}\t {meta}{err_str}", fmt_sum(n, radix));
        }
    }

    if args.verbose {
        println!("{}", "==".cyan());
    }
    println!("{}", fmt_sum(sum, args.radix));

    Ok(())
}

/// Parses `s` according to the given `radix`. With `Radix::Hex`, only integers
/// are accepted (no float fallback) -- this keeps hex mode strict so users can
/// trust that a successful parse means the value was treated as hex. `Radix::Auto`
/// is treated the same as `Radix::Decimal` here; the caller is responsible for
/// resolving 0x-prefix detection before calling.
fn parse_value(s: &str, radix: Radix) -> Result<Sum, &'static str> {
    let hex = radix == Radix::Hex;
    if let Ok(n) = i128::from_str_radix(s, if hex { 16 } else { 10 }) {
        return Ok(Sum::Integer(n));
    }
    if hex {
        return Err("Failed to parse as hex, treating as 0");
    }
    if let Ok(n) = s.parse::<f64>() {
        if !s.contains(['.', 'e', 'E']) {
            tracing::warn!(
                clean_str = s,
                "Value too large for integer, using float (may lose precision)"
            );
        }
        return Ok(Sum::Float(n));
    }
    Err("Failed to parse (use --radix=hex if hex), treating as 0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_decimal() {
        assert_eq!(parse_value("42", Radix::Decimal), Ok(Sum::Integer(42)));
    }

    #[test]
    fn parse_integer_hex() {
        assert_eq!(parse_value("FF", Radix::Hex), Ok(Sum::Integer(255)));
    }

    #[test]
    fn parse_negative_integer() {
        assert_eq!(parse_value("-5", Radix::Decimal), Ok(Sum::Integer(-5)));
    }

    #[test]
    fn parse_float() {
        assert_eq!(parse_value("1.5", Radix::Decimal), Ok(Sum::Float(1.5)));
    }

    #[test]
    fn parse_negative_float() {
        assert_eq!(parse_value("-1.5", Radix::Decimal), Ok(Sum::Float(-1.5)));
    }

    #[test]
    fn parse_scientific_notation() {
        assert_eq!(parse_value("3e0", Radix::Decimal), Ok(Sum::Float(3.0)));
    }

    #[test]
    fn parse_float_in_hex_mode_fails() {
        assert_eq!(
            parse_value("1.5", Radix::Hex),
            Err("Failed to parse as hex, treating as 0")
        );
    }

    #[test]
    fn parse_invalid_decimal() {
        assert_eq!(
            parse_value("OOPS", Radix::Decimal),
            Err("Failed to parse (use --radix=hex if hex), treating as 0")
        );
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(
            parse_value("", Radix::Decimal),
            Err("Failed to parse (use --radix=hex if hex), treating as 0")
        );
    }

    #[test]
    fn parse_overflow_falls_back_to_float() {
        let result = parse_value("999999999999999999999999999999999999999999", Radix::Decimal);
        assert!(matches!(result, Ok(Sum::Float(_))));
    }

    #[test]
    fn parse_invalid_hex() {
        assert_eq!(
            parse_value("GG", Radix::Hex),
            Err("Failed to parse as hex, treating as 0")
        );
    }
}
