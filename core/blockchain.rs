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
use std::path::PathBuf;
use errors::*;
use types::*;
use util;
use std::fs;

const HEAD_FILE_NAME: &str = "HEAD";

/// A representation of the blockchain
pub struct Blockchain {
    block_dir: PathBuf,
}

impl Blockchain {
    /// Adds a block to the chain
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        fs::create_dir(self.index_path_from_pointer(&block.hash))
            .chain_err(|| "unable to create directory structure")?;

        block.to_file(
            &self.path_buf_from_block_pointer(&block.hash),
        )?;

        self.set_head(&block.hash)
    }

    pub fn is_block_valid_next(&self, block: &Block) -> Result<bool> {
        let head = self.head_block()?;

        if !block.is_valid() {
            Ok(false)
        } else if block.index != head.index + 1 {
            Ok(false)
        } else if block.previous_hash != head.hash {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn set_head(&self, head: &Hash) -> Result<()> {
        util::serialize(&self.block_dir.join(HEAD_FILE_NAME), head)
    }

    /// Returns the head block
    fn head_block(&self) -> Result<Block> {
        let ptr: String = util::deserialize(&self.block_dir.join(HEAD_FILE_NAME))?;
        Block::from_file(&self.path_buf_from_block_pointer(&ptr))
    }

    fn path_buf_from_block_pointer(&self, pointer: &Hash) -> PathBuf {
        self.index_path_from_pointer(pointer).join(pointer)
    }

    fn index_path_from_pointer(&self, pointer: &Hash) -> PathBuf {
        let chars: Vec<_> = pointer.clone().chars().collect();
        let first = format!("{}{}", chars[0], chars[1]);
        let second = format!("{}{}", chars[2], chars[3]);
        let third = format!("{}{}", chars[4], chars[5]);

        self.block_dir.join(first).join(second).join(third)
    }

    pub fn new(block_dir: PathBuf) -> Blockchain {
        Blockchain { block_dir: block_dir }
    }
}
