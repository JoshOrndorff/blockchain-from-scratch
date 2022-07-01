use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
mod p1_header_chain;

fn main() {
    println!("Hello, world!");
}


// Overall Structure
// Simple header chain
// add consensus
// add "payload" instead of extrinsic root
// add simple state -- adder
// contentious fork where one side only accepts even payloads and the other only accepts odd payloads
// state root - now we want more complex state. We want to track the sum and the product and maybe a third thing 
// batching and complete blocks

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}