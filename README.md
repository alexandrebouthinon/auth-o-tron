<p align="center">
  <img src="https://user-images.githubusercontent.com/7868838/66727525-1a6a4300-ee40-11e9-81ff-d90719475c2b.png"/>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/tested%20on-linux%20%7C%20osx%20%7C%20windows-blue.svg">
  <a href="https://deps.rs/crate/auth-o-tron/0.2.2">
    <img src="https://deps.rs/crate/auth-o-tron/0.2.2/status.svg"/>
  </a>
  <a href="https://travis-ci.com/alexandrebouthinon/auth-o-tron">
    <img src="https://travis-ci.com/alexandrebouthinon/auth-o-tron.svg?branch=master"/>
  </a>
  <a href="https://codecov.io/gh/alexandrebouthinon/auth-o-tron">
    <img src="https://codecov.io/gh/alexandrebouthinon/auth-o-tron/branch/master/graph/badge.svg" />
  </a>
  <a href="https://github.com/alexandrebouthinon/auth-o-tron/blob/master/LICENSE">
    <img alt="undefined" src="https://img.shields.io/github/license/alexandrebouthinon/auth-o-tron.svg?style=flat">
  </a>
</p>

## About

Extract accounts from a FreeOTP URI backup and generate codes.

## Installation

> :warning: On Linux, you'll need to install X11 server dependencies required for the clipboard feature. For example on Ubuntu:
> ```
> # apt install -y libxcb-shape0-dev libxcb-xfixes0-dev
> ```

You can use cargo to install `auth-o-tron` easily.

```bash
$ cargo install auth-o-tron
```

## Usage

```bash
$ auth-o-tron -h

auth-o-tron 0.2.2

USAGE:
    auth-o-tron --file <file>

FLAGS:
        --clipboard    Save code in clipboard
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -f, --file <file>    Input backup file
```
