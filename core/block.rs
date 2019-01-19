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

use crate::errors::*;
use crate::types::*;
use crate::util;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::TimeZone;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block<T> {
    pub index: u64,
    pub previous_hash: Hash,
    pub timestamp: DateTime<Utc>,
    pub data: T,
    pub hash: Hash,
}

impl<'a, T: Debug + Default + Serialize + Deserialize<'a>> Block<T> {
    /// Generates the genesis block of the chain
    pub fn genesis() -> Block<T> {
        let mut gnew = Self::new(
            0,
            Hash::new(),
            Utc.timestamp(0, 0),
            Default::default(),
            Hash::new(),
        );

        gnew.hash = gnew.hash();

        gnew
    }

    /// Constructs a new `Block`
    pub fn new(
        index: u64,
        previous_hash: Hash,
        timestamp: DateTime<Utc>,
        data: T,
        hash: Hash,
    ) -> Block<T> {
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            hash,
        }
    }

    /// Creates a Block with the current timestamp
    pub fn create_now(index: u64, previous_hash: Hash, data: T) -> Block<T> {
        let timestamp = Utc::now();

        let mut block = Block::new(index, previous_hash, timestamp, data, Hash::new());

        block.hash = block.hash();

        block
    }

    /// Generates the hash for the block
    fn hash(&self) -> Hash {
        let mut hasher = Sha3::sha3_512();

        let input_string = format!(
            "{}{}{}{:?}",
            &self.index, &self.previous_hash, &self.timestamp, &self.data
        );

        hasher.input_str(input_string.as_str());

        hasher.result_str().to_owned()
    }

    /// Checks whether the internal integrity of the block is given
    pub fn is_valid(&self) -> bool {
        let calced_hash = self.hash();

        calced_hash == self.hash
    }

    /// Loads a block from a file
    pub fn from_file(file_name: &PathBuf) -> Result<Block<T>> {
        util::deserialize(file_name)
    }

    /// Saves the block to a file
    pub fn to_file(&self, file_name: &PathBuf) -> Result<()> {
        util::serialize(file_name, self)
    }
}
