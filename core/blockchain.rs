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

use block::Block;
use std::path::{Path, PathBuf};
use errors::*;
use types::*;
use std::fs::File;
use std::io::Read;
use rmp_serde::Deserializer;
use serde::Deserialize;
use util;

const HEAD_FILE_NAME: &str = "HEAD";

/// A representation of the blockchain
pub struct Blockchain {
    block_dir: PathBuf,
    current_block_hash: Hash,
}

impl Blockchain {
    /// Adds a block to the chain
    pub fn add_block(&mut self, block: Block) {}

    fn is_block_valid_next(&self, block: &Block) -> Result<bool> {
        let head = self.head_block()?;



        Ok(true)
    }

    /// Returns the head block
    fn head_block(&self) -> Result<Block> {
        Block::from_file(&self.block_dir.join(HEAD_FILE_NAME))
    }

    pub fn new(block_dir: PathBuf) -> Result<Blockchain> {
        let blck_dir = block_dir.join(HEAD_FILE_NAME);

        let de = util::deserialize(&blck_dir)?;

        Ok(Blockchain {
            block_dir: blck_dir,
            current_block_hash: de,
        })
    }

    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }
}
