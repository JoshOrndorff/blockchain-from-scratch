use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod p1_state_machine;
mod p2_blockchain;
mod p3_consensus;
mod p4_framework;
// mod p6_client;

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
