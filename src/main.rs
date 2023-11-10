use clap::Parser;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The field to sum. If not specified, uses the full line.
    #[arg(long, short, default_value("0"))]
    field: usize,

    #[arg(long, short = 'x')]
    hex: bool,

    /// The regex on which to split fields.
    #[arg(long, short, default_value(r"\s+"))]
    delimiter: Regex,

    /// Extra (unparsed) cargo args
    #[arg(trailing_var_arg = true)]
    pub files: Vec<String>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let mut sum = 0i128;
    for (i, line) in io::stdin().lock().lines().enumerate() {
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
            Ok(n) => n,
            Err(e) => {
                log::warn!("{e:?}, col={col:?}. Using 0 instead.");
                0
            }
        };
        sum += n;
        log::debug!("{i}: col={col:?}, n={n:?}, sum={sum:?}");
    }

    if args.hex {
        println!("{sum:#X}");
    } else {
        println!("{sum}");
    }
    Ok(())
}
