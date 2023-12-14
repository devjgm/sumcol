# `sumcol`

[![sumcol CI](https://github.com/devjgm/sumcol/actions/workflows/ci.yml/badge.svg)](https://github.com/devjgm/sumcol/actions/workflows/ci.yml)
[![sumcol crates.io](https://img.shields.io/crates/v/sumcol.svg)](https://crates.io/crates/sumcol)

`sumcol` is a simple unix-style command-line tool for summing numbers from a
column of text. It's a replacement for the tried and true Unix-isms, like `awk
'{s += $3} END {print s}'` (prints the sum of the numbers in the third
whitespace delimited column), without all the verbosity.

## Quick Install
```console
$ cargo install sumcol
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
  -x, --hex                    Treat all numbers as hex, not just those with a leading 0x
  -d, --delimiter <DELIMITER>  The regex on which to split fields [default: \s+]
  -h, --help                   Print help
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
17469
```
Which is equivalent to (but shorter than) the classic awk incantation:
```console
$ ls -l | awk '{s += $5} END {print s}'
17469
```

### Sum all input

Sometimes you use other tools to extact a column of numbers, in which case you
can still use sumcol with no arguments to simply sum all of the input. Using
the file listing from above, we could do the following:

```console
$ ls -l | awk '{print $5}' | sumcol 
17469
```

### Summing hex numbers

Programmers are often dealing with numbers written in hex. Typically in forms
like `0x123abc` or even simply `0000abcd`. When `sumcol` sees a number starting
with `0x` it always assumes it's written in hex and parses it accordingly.
However, a hex number written without that prefix requires that we tell sumcol
to use hex.

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

Yuck. That has numbers, and non-numbers. Luckily, `sumcol` will easily handle
this! It quietly ignores non-numbers treating them as if they're a `0`. So
let's see what answer we get:

```console
$ objdump -h target/release/sumcol | sumcol -f3
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "0014c350". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "000003b4". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "0004f458". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "0000cae8". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "000087c8". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "0002e5e0". Consider using -x
[2023-11-10T21:02:06Z WARN  sumcol] Failed to parse "0002c9c0". Consider using -x
732
```

Interesting. Sumcol quietly ignores non-numbers like `LOAD` in the above
example, but here it's warning us that it's seeing strings that _look like_ hex
numbers but we didn't tell it to parse the numbers as hex. Let's try again
following the recommendation to use `-x`.

```console
$ objdump -h target/release/sumcol | sumcol -f3 -x
0x20C3AC
```
NOTE: If the hex numbers started with a leading `'0x`, `sumcol` would have
silently parsed them correctly and omitted the warning.

## Debugging

If `sumcol` doesn't seem to be working right, feel free to look at the code on
github (it's pretty straight forward), or run it with the `RUST_LOG=debug`
environment variable set. For example:

```console
$ ls -l /etc/ | sumcol -f3
0
```
Zero? Hmm. That's weird. Let's debug.

```console
$ ls -l /etc/ | RUST_LOG=debug sumcol -f3
[2023-11-10T21:13:46Z DEBUG sumcol] args=Args { field: 3, hex: false, delimiter: Regex("\\s+"), files: [] }
[2023-11-10T21:13:46Z DEBUG sumcol] 0: line="total 840"
[2023-11-10T21:13:46Z INFO  sumcol] ParseIntError { kind: Empty }, col="". Using 0 instead.
[2023-11-10T21:13:46Z DEBUG sumcol] 0: col="", n=0, sum=0
[2023-11-10T21:13:46Z DEBUG sumcol] 1: line="-rw-r--r--   1 root  wheel     515 Sep 16 09:28 afpovertcp.cfg"
[2023-11-10T21:13:46Z INFO  sumcol] ParseIntError { kind: InvalidDigit }, col="root". Using 0 instead.
[2023-11-10T21:13:46Z DEBUG sumcol] 1: col="root", n=0, sum=0
[2023-11-10T21:13:46Z DEBUG sumcol] 2: line="lrwxr-xr-x   1 root  wheel      15 Sep 16 09:28 aliases -> postfix/aliases"
[2023-11-10T21:13:46Z INFO  sumcol] ParseIntError { kind: InvalidDigit }, col="root". Using 0 instead.
[2023-11-10T21:13:46Z DEBUG sumcol] 2: col="root", n=0, sum=0
[2023-11-10T21:13:46Z DEBUG sumcol] 3: line="-rw-r-----   1 root  wheel   16384 Sep 16 09:28 aliases.db"
[2023-11-10T21:13:46Z INFO  sumcol] ParseIntError { kind: InvalidDigit }, col="root". Using 0 instead.
...
```
And we can see here that it's trying to parse things like `col="root"` as a
number, which doesn't make sense. The problem is that we are trying to sum
column three (the file owner) rather than column 5 (the file size).