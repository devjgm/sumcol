use clap::Parser;
use colored::Colorize;
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

    /// Print each number that's being summed, along with some metadata
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Files to read input from, otherwise uses stdin.
    #[arg(trailing_var_arg = true)]
    pub files: Vec<String>,
}

fn fmt_sum(sum: Sum, is_hex: bool) -> String {
    if is_hex {
        format!("{sum:#X}")
    } else {
        format!("{sum}")
    }
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
    let mut cnt = 0; // Count of numbers we parse successfully.
    for reader in readers {
        for (i, line) in reader.lines().enumerate() {
            let line = line?.trim().to_string();
            log::debug!("{i}: line={line:?}");
            if line.is_empty() {
                continue;
            }
            let raw_str = match args.field {
                0 => line.as_str(),
                f => args.delimiter.split(&line).nth(f - 1).unwrap_or_default(),
            };
            // Trim and remove commas. This may break localized numbers.
            let clean_str = raw_str.trim().replace(',', "");
            let (clean_str, radix) = match clean_str.strip_prefix("0x") {
                Some(s) => (s, 16),
                None => (&clean_str as &str, if args.hex { 16 } else { 10 }),
            };
            // Holds an optional error string from parsing that we may display in verbose output.
            let mut err = None;
            let n = match i128::from_str_radix(clean_str, radix) {
                Ok(n) => {
                    cnt += 1;
                    Sum::Integer(n)
                }
                Err(e) => {
                    log::info!("Not integer. {e:?}, clean={clean_str:?}, radix={radix:?}.");
                    // Try parsing as a float
                    match clean_str.parse::<f64>() {
                        Ok(n) => {
                            cnt += 1;
                            Sum::Float(n)
                        }
                        Err(e) => {
                            log::info!("Not float. {e:?}, clean={clean_str:?}.");
                            // If it parses as hex, warn the user that they may want to use -x.
                            if i128::from_str_radix(clean_str, 16).is_ok() {
                                log::warn!(
                                    "Failed to parse {clean_str:?}, but it may be hex. Consider using -x"
                                );
                            }
                            err = Some(format!("{e:?}"));
                            Sum::Integer(0)
                        }
                    }
                }
            };
            sum += n;
            if args.verbose {
                // Print each number that we're summing, along with some metadata.
                let mut metadata = vec![];
                metadata.push(format!("n={}", format!("{:?}", n).bold()).cyan());
                metadata.push(format!("sum={}", format!("{:?}", sum).bold()).cyan());
                metadata.push(format!("cnt={}", format!("{cnt}").bold()).cyan());
                metadata.push(format!("radix={}", format!("{radix}").bold()).cyan());
                metadata.push(format!("raw_str={}", format!("{raw_str:?}").bold()).cyan());
                if let Some(err) = err {
                    metadata.push(format!("err={}", format!("{err:?}").bold()).red());
                }
                print!("{}\t", fmt_sum(n, args.hex));
                ["#".cyan()]
                    .into_iter()
                    .chain(metadata.into_iter())
                    .for_each(|x| print!(" {}", x));
                println!();
            }
        }
    }

    if args.verbose {
        println!("{}", "==".cyan());
    }
    println!("{}", fmt_sum(sum, args.hex));

    Ok(())
}
