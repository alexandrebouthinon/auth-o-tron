<p align="center">
  <img src="https://user-images.githubusercontent.com/7868838/66727261-4be20f00-ee3e-11e9-8c33-a9d4b8b8428e.png"/>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/tested%20on-linux%20%7C%20osx%20%7C%20windows-blue.svg">
  <a href="https://github.com/alexandrebouthinon/auth-o-tron/blob/master/LICENSE">
    <img alt="undefined" src="https://img.shields.io/github/license/alexandrebouthinon/auth-o-tron.svg?style=flat">
  </a>
  <a href="https://travis-ci.com/alexandrebouthinon/auth-o-tron">
    <img src="https://travis-ci.com/alexandrebouthinon/auth-o-tron.svg?branch=master"/>
  </a>
  <a href="https://codecov.io/gh/alexandrebouthinon/auth-o-tron">
    <img src="https://codecov.io/gh/alexandrebouthinon/auth-o-tron/branch/master/graph/badge.svg" />
  </a>
</p>

## About

_Extract accounts from a FreeOTP URI backup and generate codes._

## Installation

```bash
$ cargo install auth-o-tron
```

> On Linux systems, you'll need to install `xorg-dev` package to make the save in clipboard feature working.

## Usage

```bash
$ auth-o-tron -h

auth-o-tron 0.1.0

USAGE:
    auth-o-tron --file <file>

FLAGS:
        --clipboard    Save code in clipboard
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -f, --file <file>    Input backup file
```



