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

    let mut contents = String::new();
    file.read_to_string(&mut contents).chain_err(|| "")?;

    let mut de = Deserializer::new(contents.as_bytes());

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

    file.flush().chain_err(|| "failed to flush data")?;

    Ok(())
}
