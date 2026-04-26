# `sumcol`

[![sumcol CI](https://github.com/devjgm/sumcol/actions/workflows/ci.yml/badge.svg)](https://github.com/devjgm/sumcol/actions/workflows/ci.yml)
[![sumcol crates.io](https://img.shields.io/crates/v/sumcol.svg)](https://crates.io/crates/sumcol)

`sumcol` is a simple unix-style command-line tool for summing numbers from a
column of text. It's a replacement for the tried and true Unix-isms, like `awk
'{s += $3} END {print s}'` (prints the sum of the numbers in the third
whitespace delimited column), without all the verbosity. `sumcol` tries to be
smart and interpret hex, float, and decimal values automatically, though you
can force the radix with the `--radix` flag.

## Quick Install
```console
$ cargo install --locked sumcol
```

## Examples

NOTE: If you don't have `sumcol` installed in your path, you can run the
following commands directly out of this repo by replacing `sumcol` with `cargo
run -q --`.

### Help

```console
$ sumcol -h
A command-line tool to sum a column of numbers.

Usage: sumcol [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Files to read input from, otherwise uses stdin

Options:
  -f, --field <FIELD>          The field to sum. If not specified, uses the full line [default: 0]
      --radix <RADIX>          How to interpret numeric input [default: auto] [possible values: auto, hex, decimal]
  -d, --delimiter <DELIMITER>  The regex on which to split fields [default: \s+]
  -v, --verbose                Print each number that's being summed, along with some metadata
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

### Sum file sizes

Here we'll sum the sizes of all the files in my current directory:
```console
$ ls -l
total 48
-rw-r--r--  1 greg  staff  14938 Nov 10 13:56 Cargo.lock
-rw-r--r--  1 greg  staff    399 Nov 10 15:06 Cargo.toml
-rw-r--r--  1 greg  staff   1871 Nov 10 15:16 README.md
drwxr-xr-x  3 greg  staff     96 Nov 10 11:55 src
drwxr-xr-x@ 6 greg  staff    192 Nov 10 11:59 target
drwxr-xr-x  3 greg  staff     96 Nov 10 11:59 tests
```
The size is shown in column -- or field -- number 5 (starting from 1), so we can use `sumcol` as follows:

```console
$ ls -l | sumcol -f5
 WARN sumcol: Field index out of range, skipping field=5 line="total 48"
17469
```
The warning is from the `total 48` summary line which doesn't have a fifth
field; it's safely skipped and the sum is still correct. Equivalent to (but
shorter than) the classic awk incantation:
```console
$ ls -l | awk '{s += $5} END {print s}'
17469
```

### Sum all input

Sometimes you use other tools to extract a column of numbers, in which case you
can still use sumcol with no arguments to simply sum all of the input. Using
the file listing from above, we could do the following:

```console
$ ls -l | awk '{print $5}' | sumcol 
17469
```

### Summing hex numbers

Programmers are often dealing with numbers written in hex. Typically in forms
like `0x123abc` or even simply `0000abcd`. By default, when `sumcol` sees a
number starting with `0x` it assumes it's written in hex and parses it
accordingly. However, a hex number written without that prefix requires that we
tell sumcol to use hex via `--radix=hex`.

For this example we'll sum the sizes of each section in the compiled `sumcol`
binary. We can see this information with the `objdump` command.

```console
$ objdump -h target/release/sumcol

target/release/sumcol:     file format mach-o-arm64

Sections:
Idx Name          Size      VMA               LMA               File off  Algn
  0 .text         0014c350  0000000100000c0c  0000000100000c0c  00000c0c  2**2
                  CONTENTS, ALLOC, LOAD, CODE
  1 __TEXT.__stubs 000003b4  000000010014cf5c  000000010014cf5c  0014cf5c  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  2 .const        0004f458  000000010014d310  000000010014d310  0014d310  2**4
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  3 __TEXT.__gcc_except_tab 0000cae8  000000010019c768  000000010019c768  0019c768  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  4 __TEXT.__unwind_info 000087c8  00000001001a9250  00000001001a9250  001a9250  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  5 .eh_frame     0002e5e0  00000001001b1a18  00000001001b1a18  001b1a18  2**3
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  6 __DATA_CONST.__got 00000280  00000001001e0000  00000001001e0000  001e0000  2**3
                  CONTENTS, ALLOC, LOAD, DATA
  7 __DATA_CONST.__const 0002c9c0  00000001001e0280  00000001001e0280  001e0280  2**3
                  CONTENTS, ALLOC, LOAD, DATA
  8 .data         00000028  0000000100210000  0000000100210000  00210000  2**3
                  CONTENTS, ALLOC, LOAD, DATA
  9 __DATA.__thread_vars 00000108  0000000100210028  0000000100210028  00210028  2**3
                  CONTENTS, ALLOC, LOAD, DATA
 10 __DATA.__thread_data 00000040  0000000100210130  0000000100210130  00210130  2**3
                  CONTENTS, ALLOC, LOAD, DATA
 11 __DATA.__thread_bss 00000090  0000000100210170  0000000100210170  00210170  2**3
                  CONTENTS, ALLOC, LOAD, DATA
 12 __DATA.__common 00000038  0000000100210200  0000000100210200  00000000  2**3
                  ALLOC
 13 .bss          00000148  0000000100210238  0000000100210238  00000000  2**3
                  ALLOC
```

We see here that the size is in field three and it's written in hex without a leading `0x`. Let's look at field three:

```console
$ objdump -h target/release/sumcol | awk '{print $3}'

format


Size
0014c350
LOAD,
000003b4
LOAD,
0004f458
LOAD,
0000cae8
LOAD,
000087c8
LOAD,
0002e5e0
LOAD,
00000280
LOAD,
0002c9c0
LOAD,
00000028
LOAD,
00000108
LOAD,
00000040
LOAD,
00000090
LOAD,
00000038

00000148
```

Yuck. That has numbers, and non-numbers. The numeric values are hex without a
`0x` prefix, so we need to pass `--radix=hex` to tell `sumcol` to parse them as
hex. Non-numeric tokens (table headers, comma-separated description tags) will
emit warnings and be treated as `0`:

```console
$ objdump -h target/release/sumcol | sumcol -f3 --radix=hex
 WARN sumcol: Failed to parse as hex, treating as 0 clean_str="format"
 WARN sumcol: Field index out of range, skipping field=3 line="Sections:"
 WARN sumcol: Failed to parse as hex, treating as 0 clean_str="Size"
 WARN sumcol: Stripped commas from value original="LOAD," clean="LOAD"
 WARN sumcol: Failed to parse as hex, treating as 0 clean_str="LOAD"
 ... (similar warnings for each header and description line) ...
0x20C3AC
```

The warnings here are expected and benign -- `format`, `Size`, `LOAD,`, etc. are
not hex values and contribute `0` to the sum, so the final answer is correct.

If the values had been written with a `0x` prefix, `sumcol` would have
auto-detected them as hex with no flag needed.

## Debugging

If `sumcol` doesn't seem to be working right, feel free to look at the code on
github (it's pretty straight forward), or run it with the `-v` or `--verbose`
flag, or run with the `RUST_LOG=debug` environment variable set. For example:

```console
$ printf "1\n2.5\nOOPS\n3" | sumcol -v
1       # n=Integer(1) sum=Integer(1) radix=Decimal raw_str="1"
2.5     # n=Float(2.5) sum=Float(3.5) radix=Decimal raw_str="2.5"
0       # n=Integer(0) sum=Float(3.5) radix=Decimal raw_str="OOPS" err="Failed to parse (use --radix=hex if hex), treating as 0"
3       # n=Integer(3) sum=Float(6.5) radix=Decimal raw_str="3"
==
6.5
```

The metadata that's displayed on each line is

| Name | Description |
|------|-------------|
| `n` | The parsed numeric value |
| `sum` | The running sum up to and including the current `n` |
| `radix` | The effective radix used when parsing the value (`Hex` or `Decimal`) |
| `raw_str` | The raw string data that was parsed |
| `err` | If present, the warning message from a failed parse |

This should be enough to help you debug the problem you're seeing. However, if
that's not enough, give it a try with `RUST_LOG=debug`.