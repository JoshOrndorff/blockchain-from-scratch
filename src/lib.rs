use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod p1_state_machine;
mod p2_blockchain;
// mod p3_client;

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}