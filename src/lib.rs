use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod state_machine;
mod blockchain;

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

// Abstract interface so blockchain can run any state machine
// Don't assume all transactions are valid
// free execution

// full client
// light client. tracks headers doesn't store state. We don't need real merkle proofs, jsut send the full state.

// Simple helper to do some hashing.
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}