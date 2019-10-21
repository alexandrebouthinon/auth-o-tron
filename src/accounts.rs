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

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use failure::format_err;
use otpauth::TOTP;
use percent_encoding::percent_decode_str;
use prettytable::{Cell, Row, Table};
use url::Url;

/// Used to store information extract from a FreeOTP+ URI
#[derive(Clone, Debug)]
pub struct Account {
    username: Option<String>,
    issuer: String,
    secret: String,
    period: u64,
}

impl Account {
    /// Create an Account from a FreeOTP+ URI
    pub fn new(url: &Url) -> Result<Account, failure::Error> {
        // Remove unused URL part
        let cleaned_url: String = url
            .to_string()
            .drain(&url.to_string().find('/').unwrap()..)
            .collect();
        let mut decoded_url: String =
            percent_decode_str(&cleaned_url).decode_utf8()?.into();

        let mut username: Option<String> = None;
        if decoded_url.contains(":") {
            // Extract username from URI
            let username_beg = &decoded_url.find(':').unwrap() + 1;
            let username_end = &decoded_url.find('?').unwrap();
            username = Some(
                decoded_url
                    .drain(..username_end)
                    .collect::<String>()
                    .drain(username_beg..)
                    .collect(),
            );
        }

        let query: HashMap<_, _> = url.query_pairs().collect();
        let issuer: String = query.get("issuer").unwrap().to_string();
        let secret: String = query.get("secret").unwrap().to_string();
        let period: u64 = query.get("period").unwrap().parse()?;

        Ok(Account {
            username,
            issuer,
            secret,
            period,
        })
    }

    /// Generate TOTP code for given Account
    pub fn generate_code(&self) -> Result<String, failure::Error> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let code = TOTP::from_base32(&self.secret)
            .ok_or(format_err!(
                "Unable to initiate TOTP verification using secret `{}`",
                &self.secret
            ))?
            .generate(self.period, now);

        // Fill missing digits with zero
        Ok(format!("{:06}", &code))
    }
}

/// Returns a nicely formatted table containing
/// information about given accounts
pub fn pretty_display(accounts: &Vec<Account>) -> String {
    let mut table = Table::new();

    // Table headers
    table.add_row(Row::new(vec![
        Cell::new("#"),
        Cell::new("Issuer"),
        Cell::new("Username"),
    ]));

    // Accounts rows
    for (index, account) in accounts.iter().enumerate() {
        let username: &str = match &account.username {
            Some(u) => u,
            None => "-",
        };
        table.add_row(Row::new(vec![
            Cell::new(&(index).to_string()),
            Cell::new(&account.issuer),
            Cell::new(username),
        ]));
    }

    table.to_string()
}

mod tests {
    use super::*;
    use failure;

    #[allow(dead_code)]
    fn create_fake_account(fake_url: &str) -> Result<Account, failure::Error> {
        Account::new(&url::Url::parse(fake_url)?)
    }

    #[test]
    fn account_from_url_ok() {
        let account_result = create_fake_account(
            "
            otpauth://totp/one.website.net%3Amyuser%40mail.com?\
            secret=MYSUPERSECRET&\
            issuer=one.website.net&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        );
        assert!(account_result.is_ok());
        let account = account_result.unwrap();
        assert_eq!("myuser@mail.com", account.username.unwrap());
        assert_eq!("one.website.net", account.issuer);
        assert_eq!("MYSUPERSECRET", account.secret);
        assert_eq!(30, account.period);
    }

    #[test]
    fn account_from_url_without_username_ok() {
        let account_result = create_fake_account(
            "
            otpauth://totp/mukeymos?\
            secret=AAAAAAAAA&\
            issuer=Real-Debrid&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        );
        assert!(account_result.is_ok());
        let account = account_result.unwrap();
        assert_eq!(None, account.username);
        assert_eq!("Real-Debrid", account.issuer);
        assert_eq!("AAAAAAAAA", account.secret);
        assert_eq!(30, account.period);
    }

    #[test]
    fn account_from_url_err() {
        let account_result = create_fake_account(
            "
            otpauth://totp/one.website.net%3Amyuser%40mail.com?\
            secret=MYSUPERSECRET&\
            issuer=one.website.net&\
            algorithm=SHA1&\
            period=toto
            ",
        );
        assert!(account_result.is_err());
    }

    #[test]
    fn generate_code_ok() {
        let account_result = create_fake_account(
            "
            otpauth://totp/one.website.net%3Amyuser%40mail.com?\
            secret=MYSUPERSECRET&\
            issuer=one.website.net&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        );

        let account = account_result.unwrap();
        let code = account.generate_code();
        assert!(code.is_ok());
        assert_eq!(code.unwrap().len(), 6);
    }

    #[test]
    fn generate_code_without_username_ok() {
        let account_result = create_fake_account(
            "
            otpauth://totp/mukeymos?\
            secret=AAAAAAAAA&\
            issuer=Real-Debrid&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        );
        let account = account_result.unwrap();
        let code = account.generate_code();
        assert!(code.is_ok());
        assert_eq!(code.unwrap().len(), 6);
    }

    #[test]
    fn generate_code_err() {
        let account_result = create_fake_account(
            "
            otpauth://totp/one.website.net%3Amyuser%40mail.com?\
            secret=NOTABASE64SUPERSECRET???&\
            issuer=one.website.net&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        );

        let account = account_result.unwrap();
        let code = account.generate_code();
        assert!(code.is_err());
    }

    #[test]
    fn pretty_display_test() {
        let account = create_fake_account(
            "
            otpauth://totp/one.website.net%3Amyuser%40mail.com?\
            secret=MYSUPERSECRET&\
            issuer=one.website.net&\
            algorithm=SHA1&\
            digits=6&\
            period=30",
        )
        .unwrap();
        let accounts: Vec<Account> = vec![account];

        let table: Vec<String> = pretty_display(&accounts)
            .split('\n')
            .map(|x| x.to_string())
            .collect();

        // Check headers
        assert!(&table[1].contains("#"));
        assert!(&table[1].contains("Issuer"));
        assert!(&table[1].contains("Username"));
        // Check cells
        assert!(&table[3].contains("0"));
        assert!(&table[3].contains("one.website.net"));
        assert!(&table[3].contains("myuser@mail.com"));
    }
}
