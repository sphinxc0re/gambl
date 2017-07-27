// Gambl - a Blockchain written in Rust
// Copyright (C) 2017  Julian Laubstein
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate gambl_core as gcore;
extern crate clap;

use gcore::errors::*;
use gcore::blockchain::Blockchain;
use clap::{App, AppSettings, SubCommand};
use std::{env, fs};

const GAMBLE_HOME_DIR: &str = ".gambl";

const START_SUBCOMMAND: &str = "start";
const HEAD_SUBCOMMAND: &str = "head";
const SEED_SUBCOMMAND: &str = "seed";

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColorAlways)
        .about("The gambl blockchain toolkit")
        .subcommand(SubCommand::with_name(START_SUBCOMMAND).about(
            "Start the gambl client",
        ))
        .subcommand(SubCommand::with_name(HEAD_SUBCOMMAND).about(
            "Show the head block of the blockchain",
        ))
        .subcommand(SubCommand::with_name(SEED_SUBCOMMAND).about(
            "Populate the blockchain with random blocks",
        ))
        .get_matches();

    let home_dir = env::home_dir().ok_or("failed to get home directory")?;

    let gamble_base_dir = home_dir.join(GAMBLE_HOME_DIR);

    if !gamble_base_dir.exists() {
        fs::create_dir_all(&gamble_base_dir).chain_err(
            || "unable to create directory structure",
        )?;
    }

    let mut chain = Blockchain::init(gamble_base_dir)?;

    match matches.subcommand() {
        (START_SUBCOMMAND, Some(..)) => {
            // TODO: Start the network client
        }
        (HEAD_SUBCOMMAND, Some(..)) => {
            let head = chain.head_block()?;
            println!("{:#?}", head);
        }
        (SEED_SUBCOMMAND, Some(..)) => {
            for i in 0..64 {
                chain.new_block(i)?;
            }
        }
        _ => {}
    }

    Ok(())
}
