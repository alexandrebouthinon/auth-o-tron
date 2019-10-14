# OATH Keeper

_Extract accounts from a FreeOTP URI backup and generate codes._

## Installation

```bash
$ cargo install oath-keeper
```

> On Linux systems, you'll need to install `xorg-dev` package to make the save in clipboard feature working.

## Usage

```bash
$ oath-keeper -h

oath-keeper 0.1.0

USAGE:
    oath-keeper [FLAGS] --file <file>

FLAGS:
        --clipboard    Save code in clipboard
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -f, --file <file>    Input backup file
```



