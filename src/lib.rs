use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod c1_state_machine;
mod c2_blockchain;
mod c3_consensus;
mod c4_framework;

//TODO This is just me scribbling a quick little idea to disambiguate the hash type throughout this project.
/// A Simple 64-bit hash value that will be used whenever a cryptographic hash is needed.
pub struct HashValue(u64);

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
