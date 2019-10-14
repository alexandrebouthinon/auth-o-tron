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

mod accounts;
mod cli;

use std::fs::File;
use std::io::{self, Write};

use dialoguer::Input;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let cli = cli::Cli::from_args();
    let file = File::open(&cli.file)
        .with_context(|_| format!("Could not open file `{}`", &cli.file))?;

    let accounts: Vec<accounts::Account> = cli::parse_backup(&file)?;
    print!("{}", &accounts::pretty_display(&accounts));
    io::stdout().flush()?;

    let mut choice: usize;
    loop {
        choice = Input::<usize>::new()
            .with_prompt("[?] Select an account")
            .interact()?;
        if choice < accounts.len() {
            break;
        }
        println!("[!] Account {} not found!", choice);
    }

    let code = &accounts.get(choice).unwrap().generate_code()?;

    if cli.clipboard {
        cli::send_to_clipboard(&code)?;
        println!(
            "[+] Code saved in clipboard. \
             Use it and press any key to exit...",
        );
        io::stdin().read_line(&mut String::new())?;
    } else {
        println!("[+] Code generated: {}", code);
    }

    Ok(())
}
