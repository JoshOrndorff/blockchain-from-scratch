use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
mod p1_header_chain;
mod p2_extrinsic_state;
mod p3_consensus;
mod p4_batched_extrinsics;

fn main() {
    println!("Hello, world!");
}

// Ideas
// Do state machine stuff FIRST! in lesson 1.
//  * machine for single light bulb on single switch
//  * for single bulb on two switches
//  * same as last but also a bulb on each switch
//  * Really could do it for any digital circuit... but also other stuff
//  * Can also do it for other stuff, game, token
//  * Implement three of your own state machines. Also design them carefully


// Write a fork choice rule somewhere?
// Genesis state
// Proof of authority
// Abstract Consensus
// Abstract interface for writing a state machines
// Write a full client

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}