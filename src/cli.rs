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

use std::io::{BufRead, BufReader, Read};

#[derive(StructOpt)]
pub struct Cli {
    /// Input backup file
    #[structopt(short = "f", long = "file")]
    pub file: String,
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
