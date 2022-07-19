//! Now that we have a functioning hash-linked data structure, we can use it to actually
//! track some state. Here we will start to explore the idea of extrinsics and state by
//! slightly abusing the header's extrinsics_root and state_root fields. As the names imply,
//! these are typically used for Merkle roots of large data sets. But in our case we will use
//! these fields to directly contain a single extrinsic per block, and a single piece of state.
//! 
//! In the coming parts of this tutorial, we will expand this to be more real-world like and
//! use some real batching.

use crate::hash;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so the code is slightly more readable.
type Hash = u64;

/// The header is no expanded to contain an extrinsic and a state. Note that we are not
/// using roots yet, but rather directly embedding some minimal extrinsic and state info
/// into the header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsic: u64,
    state: u64,
    // Still no consensus. That's the next part.
    consensus_digest: (),
}

// Here are the methods for creating new hedaer and verifying headers.
// It is your job to write them.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        todo!("Exercise 1")
    }

    /// Create and return a valid child header.
    fn child(&self, extrinsic: u64) -> Self {
        todo!("Exercise 2")
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    ///
    /// In addition to the consecutive heights and linked hashes, we now need to consider our state.
    /// This blockchain will work as an adder. That means that the state starts at zero,
    /// and at each block we add the extrinsic to the state.
    /// 
    /// So in order for a block to verify, we must have that relationship between the extrinsic,
    /// the previous state, and the current state.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        todo!("Exercise 3")
    }
}

// And finally a few functions to use the code we just

/// Build and return a valid chain with the given number of blocks.
fn build_valid_chain(n: u64) -> Vec<Header> {
    todo!("Exercise 4")
}


/// Build and return a chain with at least three headers.
/// The chain should start with a proper genesis header,
/// but the entire chain should NOT be valid.
/// 
/// As we saw in the last unit, this is rivial when we construct arbitrary blocks.
/// However, from outside this crate, it is not so trivial. Our interface for creating
/// new blocks, `genesis()` and `child()`, makes it impossible to create arbitrary blocks.
///
/// For this function, ONLY USE the the `genesis()` and `child()` methods to create blocks.
/// The exercise is still possible.
fn build_an_invalid_chain() -> Vec<Header> {
    todo!("Exercise 5")
}

/// Build and return two header chains.
/// Both chains should individually be valid.
/// They should have the same genesis header.
/// They should not be the exact same chain.
/// 
/// Here is an example of two such chains:
///            /-- 3 -- 4
/// G -- 1 -- 2
///            \-- 3'-- 4'
/// 
/// Side question: What is the fewest number of headers you could create to achieve this goal.
fn build_forked_chain() -> (Vec<Header>, Vec<Header>) {
    todo!("Exercise 6")

    // Exercise 7: After you have completed this task, look at how its test is written below.
    // There is a critical thinking question for you there.
}


// To run these tests: `cargo test part_1`
#[test]
fn part_2_genesis_block_height() {
    let g = Header::genesis();
    assert!(g.height == 0);
}

#[test]
fn part_2_genesis_block_parent() {
    let g = Header::genesis();
    assert!(g.parent == 0);
}

#[test]
fn part_2_genesis_block_extrinsic() {
    // Typically genesis blocks do not have any extrinsics.
    // In Substrate they never do. So our convention is to have the extrinsic be 0.
    let g = Header::genesis();
    assert!(g.extrinsic == 0);
}

#[test]
fn part_2_genesis_block_state() {
    let g = Header::genesis();
    assert!(g.state == 0);
}

#[test]
fn part_2_child_block_height() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert!(b1.height == 1);
}

#[test]
fn part_2_child_block_parent() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert!(b1.parent == hash(&g));
}

#[test]
fn part_2_child_block_extrinsic() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.extrinsic,  7);
}

#[test]
fn part_2_child_block_state() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.state, 7);
}

#[test]
fn part_2_verify_genesis_only() {
    let g = Header::genesis();

    assert!(g.verify_sub_chain(&vec![]));
}

#[test]
fn part_2_verify_three_blocks() {
    let g = Header::genesis();
    let b1 = g.child(5);
    let b2 = b1.child(6);

    assert_eq!(b2.state, 11);
    assert!(g.verify_sub_chain(&vec![b1, b2]));
}

#[test]
fn part_2_cant_verify_invalid_parent() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.parent = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_2_cant_verify_invalid_number() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.height = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_2_cant_verify_invalid_state() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.state = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_2_verify_forked_chain() {
    let g = Header::genesis();
    let(c1, c2) = build_forked_chain();

    // Both chains have the same valid genesis block
    assert_eq!(g, c1[0]);
    assert_eq!(g, c2[0]);

    // Both chains are individually valid
    assert!(g.verify_sub_chain(&c1[1..]));
    assert!(g.verify_sub_chain(&c2[1..]));

    // The two chains are not identical
    // Question for students: I've only compared the last blocks here.
    // Is that enough? Is it possible that the two chains have the same final block,
    // but differ somewhere else?
    assert_ne!(c1.last(), c2.last());


}