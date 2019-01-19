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

use crate::block::Block;
use crate::errors::*;
use crate::types::*;
use crate::util;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

const HEAD_FILE_NAME: &str = "HEAD";
const BLOCK_DIR_NAME: &str = "blocks";

/// A representation of the blockchain
pub struct Blockchain<T> {
    block_dir: PathBuf,
    data_type: PhantomData<T>,
}

impl<'a, T: Debug + Default + Serialize + Deserialize<'a>> Blockchain<T> {
    /// Adds a block to the chain
    pub fn add_block(&mut self, block: Block<T>) -> Result<()> {
        fs::create_dir_all(self.index_path_from_pointer(&block.hash))
            .chain_err(|| "unable to create directory structure")?;

        block.to_file(&self.path_buf_from_block_hash(&block.hash))?;

        self.set_head(&block.hash)
    }

    /// Creates a new block from the given data and adds it to the chain
    pub fn new_block(&mut self, data: T) -> Result<()> {
        let head: Block<T> = self.head_block()?;

        self.add_block(Block::create_now(head.index + 1, head.hash, data))?;

        Ok(())
    }

    /// Checks whether the given block can be added to the  chain
    pub fn is_block_valid_next(&self, block: &Block<T>) -> Result<bool> {
        let head: Block<T> = self.head_block()?;

        Ok(block.is_valid() && block.index == head.index + 1 && block.previous_hash != head.hash)
    }

    /// Set the current head block given its hash
    fn set_head(&self, head: &Hash) -> Result<()> {
        util::serialize(&self.block_dir.join(HEAD_FILE_NAME), head)
    }

    /// Returns the head block
    pub fn head_block(&self) -> Result<Block<T>> {
        let ptr: String = util::deserialize(&self.block_dir.join(HEAD_FILE_NAME))?;
        Block::from_file(&self.path_buf_from_block_hash(&ptr))
    }

    /// Returns the path of a block given its hash
    fn path_buf_from_block_hash(&self, pointer: &Hash) -> PathBuf {
        self.index_path_from_pointer(pointer).join(pointer)
    }

    /// Returns the index path of a block given its hash
    fn index_path_from_pointer(&self, pointer: &Hash) -> PathBuf {
        let chars: Vec<_> = pointer.clone().chars().collect();

        let mut path_buf = PathBuf::from(BLOCK_DIR_NAME);

        for i in 0..8 {
            path_buf = path_buf.join(format!("{}{}", chars[i * 2], chars[(i * 2) + 1]));
        }

        self.block_dir.join(path_buf)
    }

    /// Initializes a blockchain at the given directory
    pub fn init(block_dir: PathBuf) -> Result<Blockchain<T>> {
        let mut chain = Blockchain {
            block_dir: block_dir,
            data_type: PhantomData::<T>,
        };

        if !chain.block_dir.join(HEAD_FILE_NAME).exists() {
            chain.add_block(Block::genesis())?;
        }

        Ok(chain)
    }
}
