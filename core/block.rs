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

use chrono::offset::Utc;
use std::path::PathBuf;
use chrono::DateTime;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use chrono::TimeZone;
use errors::*;
use types::*;
use util;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub previous_hash: Hash,
    pub timestamp: DateTime<Utc>,
    pub data: Vec<u8>,
    pub hash: Hash,
}

impl Block {
    pub fn genesis() -> Block {
        let mut gnew = Self::new(0, Hash::new(), Utc.timestamp(0, 0), Vec::new(), Hash::new());

        gnew.hash = gnew.hash();

        gnew
    }

    /// Constructs a new `Block`
    pub fn new(
        index: u64,
        previous_hash: Hash,
        timestamp: DateTime<Utc>,
        data: Vec<u8>,
        hash: Hash,
    ) -> Block {
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            hash,
        }
    }

    /// Creates a Block with the current timestamp
    pub fn create_now(index: u64, previous_hash: Hash, data: Vec<u8>) -> Block {
        let timestamp = Utc::now();

        let hash = calculate_hash(&index, &previous_hash, &timestamp, &data);

        Block::new(index, previous_hash, timestamp, data, hash)
    }

    /// Generates the hash for the block
    fn hash(&self) -> Hash {
        calculate_hash(
            &self.index,
            &self.previous_hash,
            &self.timestamp,
            &self.data,
        )
    }

    /// Checks whether the internal integrity of the block is given
    pub fn is_valid(&self) -> bool {
        let calced_hash = self.hash();

        calced_hash == self.hash
    }

    /// Loads a block from a file
    pub fn from_file(file_name: &PathBuf) -> Result<Block> {
        util::deserialize(file_name)
    }


    /// Saves the block to a file
    pub fn to_file(&self, file_name: &PathBuf) -> Result<()> {
        util::serialize(file_name, self)
    }
}

fn calculate_hash(
    index: &u64,
    previous_hash: &Hash,
    timestamp: &DateTime<Utc>,
    data: &Vec<u8>,
) -> Hash {
    let mut hasher = Sha3::sha3_512();

    let input_string = format!("{}{}{}{:?}", index, previous_hash, timestamp, data);

    hasher.input_str(input_string.as_str());

    hasher.result_str().to_owned()
}
