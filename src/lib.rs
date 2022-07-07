use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod state_machine;
mod blockchain;

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}