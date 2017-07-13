use chrono::offset::Utc;
use chrono::DateTime;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

pub type Hash = [u8; 512];

pub struct Block {
    index: u64,
    previous_hash: Hash,
    timestamp: DateTime<Utc>,
    data: Vec<u8>,
    hash: Hash,
}

impl Block {
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
    fn is_valid(&self) -> bool {
        let mut count = 0;

        let mut res = 0;

        let calced_hash = self.hash();

        while count < 512 {
            res |= calced_hash[count] ^ self.hash[count];

            count += 1;
        }

        res == 0
    }
}

pub fn calculate_hash(
    index: &u64,
    previous_hash: &Hash,
    timestamp: &DateTime<Utc>,
    data: &Vec<u8>,
) -> Hash {
    let mut hasher = Sha3::sha3_512();

    let input_string = format!(
        "{}{:?}{}{:?}",
        index,
        previous_hash.to_vec(),
        timestamp,
        data
    );

    hasher.input(input_string.as_bytes());
    let mut hash: Hash = [0; 512];

    hasher.result(&mut hash);

    hash
}
