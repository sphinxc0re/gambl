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
#[macro_use]
extern crate error_chain;

use gcore::errors::*;
use gcore::blockchain::Blockchain;
use clap::{App, AppSettings, SubCommand};
use std::{env, fs};

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

const START_SUBCOMMAND: &str = "start";

fn run() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Julian Laubstein <contact@julianlaubstein.de>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColorAlways)
        .about("The gambl blockchain toolkit")
        .subcommand(SubCommand::with_name(START_SUBCOMMAND).about(
            "Start the gambl blockchain client",
        ))
        .get_matches();

    match matches.subcommand() {
        (START_SUBCOMMAND, Some(_)) => {
            let path = match env::home_dir() {
                Some(path) => path,
                None => bail!("failed to get home directory"),
            }.join(".gambl");

            fs::create_dir_all(&path).chain_err(
                || "unable to create directory structure",
            )?;

            let mut chain = Blockchain::init(path)?;

            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;


            chain.new_block(vec![
                0x43,
                0x43,
                0xe3,
                0x43,
                0xb3,
                0x43,
                0x43,
                0x13,
                0x43,
                0x43,
                0x43,
            ])?;
        }
        _ => {}
    }

    Ok(())
}
