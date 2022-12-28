use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod c1_state_machine;
mod c2_blockchain;
mod c3_consensus;
mod c4_framework;

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
