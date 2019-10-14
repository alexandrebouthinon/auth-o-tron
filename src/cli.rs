// The MIT License
// 
// Copyright (c) 2019 Alexandre BOUTHINON
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use super::accounts;

use structopt::StructOpt;
use url::Url;
use failure::format_err;
use clipboard::{ClipboardContext, ClipboardProvider};

use std::io::{BufRead, BufReader, Read};

#[derive(StructOpt)]
pub struct Cli {
    /// Input backup file
    #[structopt(short = "f", long = "file")]
    pub file: String,
    /// Save code in clipboard
    #[structopt(long)]
    pub clipboard: bool,
}

pub fn parse_backup<B: Read>(
    backup: B,
) -> Result<Vec<accounts::Account>, failure::Error> {
    let reader = BufReader::new(backup);
    let mut accounts: Vec<accounts::Account> = vec![];
    for line in reader.lines() {
        let url = Url::parse(&line.unwrap())?;
        accounts.push(accounts::Account::new(&url)?)
    }
    Ok(accounts)
}

pub fn send_to_clipboard(value: &str) -> Result<(), failure::Error> {
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(c) => c,
        Err(err) => return Err(format_err!("{}", err))
    };

    match ctx.set_contents(value.to_owned()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format_err!("{}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clipboard::{ClipboardContext, ClipboardProvider};

    #[test]
    fn parse_backup_ok() {
        let backup = "otpauth://totp/one.website.net%3Amyuser%40mail.com?\
                      secret=MYSUPERSECRET&\
                      issuer=one.website.net&\
                      algorithm=SHA1&\
                      digits=6&\
                      period=30\n"
            .as_bytes();

        let accounts = parse_backup(backup);
        assert!(accounts.is_ok());
    }

    #[test]
    fn parse_backup_err() {
        let backup = "NOT_REALLY_AN_URI"
            .as_bytes();

        let accounts = parse_backup(backup);
        assert!(accounts.is_err());
    }

    #[test]
    fn send_to_clipboard_test() {
        let code = "012345";

        let res = send_to_clipboard(&code);
        assert!(res.is_ok());
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        assert_eq!(&ctx.get_contents().unwrap(), &code.to_string());
    }
}
