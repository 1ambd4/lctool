# leetcode-cli

> **A Simple Cli Tool for Leetcode**

## Install

``` bash
git clone https://github.com/1ambd4/leetcode-cli.git
cd leetcode-cli
cargo install --path .
```

## Usage

```
A Simple Cli Tool for Leetcode

Usage: leetcode-cli [COMMAND]

Commands:
  data  manage cache
  edit  edit problem by id
  info  show problem detail
  list  list problems
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Config

The default config file is :`~/.config/leetcode/leetcode.toml`.

``` toml
[cookies]
csrf = ""
session = ""

[storage]
cache = "leetcode.sqlite3"
project = "~/.config/leetcode/code"
root = "~/.config/leetcode"
```
