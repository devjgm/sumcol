# `sumcol`

[![sumcol CI](https://github.com/devjgm/sumcol/actions/workflows/ci.yml/badge.svg)](https://github.com/devjgm/sumcol/actions/workflows/ci.yml)

`sumcol` is a simple unix-style command-line tool for summing numbers from a
column of text. It's a replacement for the tried and true Unix-isms, like `awk
'{s += $3} END {print s}'` (prints the sum of the numbers in the third
whitespace delimited column), without all the verbosity.