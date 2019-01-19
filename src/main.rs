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

use gambl_core::blockchain::Blockchain;
use gambl_core::errors::*;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    #[structopt(name = "start", about = "Start the gambl client")]
    Start,
    #[structopt(name = "head", about = "Show the head block of the blockchain")]
    Head,
    #[structopt(name = "seed", about = "Populate the blockchain with random blocks")]
    Seed,
}

const GAMBL_HOME_DIR: &str = ".gambl";

fn run() -> Result<()> {
    let args = Opt::from_args();

    let home_dir = dirs::home_dir().ok_or("failed to get home directory")?;

    let gamble_base_dir = home_dir.join(GAMBL_HOME_DIR);

    if !gamble_base_dir.exists() {
        fs::create_dir_all(&gamble_base_dir)
            .chain_err(|| "unable to create directory structure")?;
    }

    let mut chain = Blockchain::init(gamble_base_dir)?;

    match args {
        Opt::Start => {
            // TODO: Start the network client
        }
        Opt::Head => {
            let head = chain.head_block()?;
            println!("{:#?}", head);
        }
        Opt::Seed => {
            for i in 0..64 {
                chain.new_block(i)?;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}
