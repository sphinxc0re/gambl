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

use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use errors::*;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

pub fn deserialize<'a, T>(file_path: &PathBuf) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut file = File::open(file_path).chain_err(|| "unable to open file")?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).chain_err(
        || "failed to read contents from file",
    )?;

    let mut de = Deserializer::new(&contents[..]);

    Deserialize::deserialize(&mut de).chain_err(|| "failed to deserialize block")
}

pub fn serialize<T: Serialize>(file_path: &PathBuf, obj: &T) -> Result<()> {
    let mut file = File::create(file_path).chain_err(
        || "unable to create file",
    )?;

    let mut buf = Vec::new();

    obj.serialize(&mut Serializer::new(&mut buf)).chain_err(
        || "failed to serialize block",
    )?;

    file.write_all(&buf[..]).chain_err(
        || "failed to write data",
    )?;

    file.flush().chain_err(|| "failed to flush data")?;

    Ok(())
}
