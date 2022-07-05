use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
mod p1_header_chain;
mod p2_extrinsic_state;
mod p3_consensus;

fn main() {
    println!("Hello, world!");
}

// TODO write a fork choice rule somewhere?

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}