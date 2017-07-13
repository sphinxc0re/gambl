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

/// A representation of the blockchain
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn initial() -> Blockchain {
        let initial_chain = vec![Block::genesis()];

        Blockchain { chain: initial_chain }
    }

    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }

    pub fn is_block_valid(&self, block: &Block) -> bool {
        unimplemented!()
    }

    pub fn add_block(&mut self, block: Block) {
        unimplemented!()
    }
}
