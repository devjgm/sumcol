use clap::Parser;
use env_logger::Env;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead, BufReader};
use sumcol::Sum;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The field to sum. If not specified, uses the full line.
    #[arg(long, short, default_value("0"))]
    field: usize,

    /// Treat all numbers as hex, not just those with a leading 0x.
    #[arg(long, short = 'x')]
    hex: bool,

    /// The regex on which to split fields.
    #[arg(long, short, default_value(r"\s+"))]
    delimiter: Regex,

    /// Files to read input from, otherwise uses stdin.
    #[arg(trailing_var_arg = true)]
    pub files: Vec<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    // Gets a list of input sources. Either the file(s) specified on the command line or stdin.
    type Reader = Box<dyn BufRead>;
    let readers: Vec<Reader> = if args.files.is_empty() {
        vec![Box::new(BufReader::new(io::stdin()))]
    } else {
        let mut v: Vec<Reader> = vec![];
        for f in args.files {
            v.push(Box::new(BufReader::new(fs::File::open(f)?)));
        }
        v
    };

    let mut sum = Sum::Integer(0);
    for reader in readers {
        for (i, line) in reader.lines().enumerate() {
            let line = line?.trim().to_string();
            log::debug!("{i}: line={line:?}");
            if line.is_empty() {
                continue;
            }
            let col = match args.field {
                0 => line.as_str(),
                f => args.delimiter.split(&line).nth(f - 1).unwrap_or_default(),
            };
            let default_radix = if args.hex { 16 } else { 10 };
            let (col, radix) = match col.strip_prefix("0x") {
                Some(s) => (s, 16),
                None => (col, default_radix),
            };
            let n = match i128::from_str_radix(col, radix) {
                Ok(n) => Sum::Integer(n),
                Err(e) => {
                    log::info!("Not integer. {e:?}, col={col:?}, radix={radix:?}.");
                    // Try parsing as a float
                    match col.parse::<f64>() {
                        Ok(n) => Sum::Float(n),
                        Err(e) => {
                            log::info!("Not float. {e:?}, col={col:?}.");
                            // If it parses as hex, warn the user that they may want to use -x.
                            if i128::from_str_radix(col, 16).is_ok() {
                                log::warn!(
                                    "Failed to parse {col:?}, but it may be hex. Consider using -x"
                                );
                            }
                            Sum::Integer(0)
                        }
                    }
                }
            };
            sum += n;
            log::debug!("{i}: col={col:?}, n={n:?}, sum={sum:?}");
        }
    }

    if args.hex {
        println!("{sum:#X}");
    } else {
        println!("{sum}");
    }
    Ok(())
}
