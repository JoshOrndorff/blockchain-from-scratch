//! We now have a hash-linked header chain that accepts simple extrinsics and tracks simple state.
//! Now we will explore consensus. We are not looking at finality or fork choice here. Rather,
//! we are adding validity rules. There are two common types of validity rules and we will explore both.
//! 1. Rules to throttle authoring. In this case we will use a simple PoW.
//! 2. Arbitrary / Political rules. Here we will implement two alternate validity rules

use crate::hash;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so the code is slightly more readable.
type Hash = u64;

/// In this lesson we are introducing proof of work onto our blocks. We need a hash threshold.
/// You may change this as you see fit, and I encourage you to experiment. Probably best to start
/// high so we aren't wasting time mining. I'll start with 1 in 100 blocks being valid.
const THRESHOLD: u64 = u64::max_value() / 100;

/// In this lesson we introduce the concept of a contentuous hard fork. The fork will happen at
/// this block height.
const FORK_HEIGHT: u64 = 2;

/// The header is no expanded to contain an extrinsic and a state. Note that we are not
/// using roots yet, but rather directly embedding some minimal extrinsic and state info
/// into the header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Header {
    parent: Hash,
    height: u64,
    extrinsic: u64,
    state: u64,
    /// For Proof of Work, the consensus digest is basically just a nonce which gets the block
    /// hash below a certain threshold. Although we could call the field `nonce` we will leave
    /// the more general `digest` term. For PoA we would have a cryptographic signature in this field.
    consensus_digest: u64,
}

impl Header {
    fn parent(&self) -> Hash {
        self.parent
    }

    fn height(&self) -> u64 {
        self.height
    }

    fn extrinsic(&self) -> u64 {
        self.extrinsic
    }

    fn state(&self) -> u64 {
        self.state
    }

    fn consensus_digest(&self) -> u64 {
        self.consensus_digest
    }
}

// Here are the methods for creating new hedaer and verifying headers.
// It is your job to write them.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        Self {
            parent: 0,
            height: 0,
            extrinsic: 0,
            state: 0,
            consensus_digest: 0,
        }
    }

    /// Create and return a valid child header.
    fn child(&self, extrinsic: u64) -> Self {
        // When creating a new block, we need to perform some consensus tasks.
        // We start by creating a complete header with everything _except_
        // the consensus digest.
        let mut header = Self {
            parent: hash(self),
            height: self.height + 1,
            extrinsic,
            state: self.state + extrinsic,
            consensus_digest: 0,
        };

        // Now we perform the consensus task, which, in this case, means we mine.
        // We'll create a helper function for solving the PoW
        let pow_nonce = solve_pow(&header);

        // Finally we insert the consensus digest into the header and return it
        header.consensus_digest = pow_nonce;
        header

        // When you were implementing the PoW mining, you probably noticed
        // that we could have optimized a bit by returning the entire completed
        // header rather than just the consensus digest. Very astute! I chose to
        // follow this pre_header -> consensus_tasks -> post_header workflow
        // because it also works for PoA / PoS.
    }

    /// Helper function to verify an individual child block
    fn verify_child(&self, child: &Header) -> bool {
        // Check the parent pointer
        if hash(self) != child.parent {
            return false;
        }

        // Check the height
        if self.height + 1 != child.height {
            return false;
        }

        // Check that the state has transitioned correctly
        // Here we re-calculate the post state, and compare it
        // to the post state that the block author found
        let expected_state = self.state + child.extrinsic;
        if expected_state != child.state {
            return false;
        }

        // Check the Consensus tasks (PoW in this case)
        // Like before, we will make a helper function to abstract
        // the consensus tasks.
        if !verify_pow(child) {
            return false;
        }

        return true;
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    ///
    /// In addition to all the rules we had before, we now need to check that the block hash
    /// is below a specific threshold.
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        // Terminating case
        if chain.is_empty() {
            return true;
        }

        // Check the immediate child
        let next = &chain[0];
        if !self.verify_child(next){
            return false;
        }

        // Recurse to check the rest of the chain
        next.verify_sub_chain(&chain[1..])
    }

    // After the blockchain ran for a while, a political rift formed in the community.
    // (See the constant FORK_HEIGHT) which is set to 2 by default.
    // Most community members have become obsessed over the state of the blockchain.
    // On the one side, people believe that only blocks with even states should be valid.
    // On the other side, people bleieve in only blocks with odd states.

    /// Varify that the given headers form a valid chain.
    /// In this case "valid" means that the STATE MUST BE EVEN.
    fn verify_sub_chain_even(&self, chain: &[Header]) -> bool {
        // We use basically the same logic from `verify_sub_chain`
        // with an additional check for even-ness
        if chain.is_empty() {
            return true;
        }

        let next = &chain[0];
        if !self.verify_child(next) {
            return false;
        }

        if next.height > 2 && next.state %2 != 0 {
            return false;
        }
        
        next.verify_sub_chain_even(&chain[1..])
    }

    /// Varify that the given headers form a valid chain.
    /// In this case "valid" means that the STATE MUST BE ODD.
    fn verify_sub_chain_odd(&self, chain: &[Header]) -> bool {
        // We use basically the same logic from `verify_sub_chain`
        // with an additional check for even-ness
        if chain.is_empty() {
            return true;
        }

        let next = &chain[0];
        if !self.verify_child(next) {
            return false;
        }

        if next.height > 2 && next.state %2 != 1 {
            return false;
        }
        
        next.verify_sub_chain_odd(&chain[1..])
    }

}

// The next two functions are consensus related. Here we are using PoW
// so the functions do the mining, and check the threshold. Substrate achieves
// pluggable consensus by abstracting this behavior behind traits

/// Try sequential nonces until one that meets the threshold is found
fn solve_pow(h: &Header) -> u64 {
    let mut trial_header = h.clone();
    while !verify_pow(&trial_header) {
        trial_header.consensus_digest += 1;
    }

    trial_header.consensus_digest
}

/// Verify that the threshold is met on the given block
fn verify_pow(h: &Header) -> bool {
    hash(h) < THRESHOLD
}

//

/// Build and return two different chains with a common prefix.
/// They should have the same genesis header.
/// 
/// Both chains should be valid according to the original validity rules.
/// The first chain should be valid only according tothe even rules.
/// The second chain should be valid only according tothe odd rules.
/// 
/// Return your solutions as three vectors:
/// 1. The common prefix including genesis
/// 2. The even suffix (non-overlapping with the common prefix)
/// 3. The oddn suffix (non-overlapping with the common prefix)
/// 
/// Here is an example of two such chains:
///            /-- 3 -- 4
/// G -- 1 -- 2
///            \-- 3'-- 4'
fn build_contentious_forked_chain() -> (Vec<Header>, Vec<Header>, Vec<Header>) {
    // The common prefix. Ending state is 3 (odd)
    let g = Header::genesis();
    let b1 = g.child(1);
    let b2 = b1.child(2);

    // The even side
    let b3 = b2.child(3);
    let b4 = b3.child(4);

    // The odd side
    let b3_prime = b2.child(6);
    let b4_prime = b3_prime.child(8);

    (
        vec![g, b1, b2],
        vec![b3, b4],
        vec![b3_prime, b4_prime]
    )
}

// To run these tests: `cargo test part_1`
#[test]
fn part_3_genesis_block_height() {
    let g = Header::genesis();
    assert!(g.height() == 0);
}

#[test]
fn part_3_genesis_block_parent() {
    let g = Header::genesis();
    assert!(g.parent == 0);
}

#[test]
fn part_3_genesis_block_extrinsic() {
    // Typically genesis blocks do not have any extrinsics.
    // In Substrate they never do. So our convention is to have the extrinsic be 0.
    let g = Header::genesis();
    assert!(g.extrinsic == 0);
}

#[test]
fn part_3_genesis_block_state() {
    let g = Header::genesis();
    assert!(g.state == 0);
}

#[test]
fn part_3_genesis_consensus_digest() {
    // We could require that the genesis block have a valid proof of work as well.
    // But instead I've chosen the simpler path of defining the nonce = 0 in genesis.
    let g = Header::genesis();
    assert!(g.consensus_digest == 0);
}

#[test]
fn part_3_child_block_height() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert!(b1.height == 1);
}

#[test]
fn part_3_child_block_parent() {
    let g = Header::genesis();
    let b1 = g.child(0);
    assert!(b1.parent == hash(&g));
}

#[test]
fn part_3_child_block_extrinsic() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.extrinsic, 7);
}

#[test]
fn part_3_child_block_state() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert_eq!(b1.state, 7);
}

#[test]
fn part_3_child_block_consensus_digest() {
    let g = Header::genesis();
    let b1 = g.child(7);
    assert!(hash(&b1) < THRESHOLD);
}

#[test]
fn part_3_verify_genesis_only() {
    let g = Header::genesis();

    assert!(g.verify_sub_chain(&vec![]));
}

#[test]
fn part_3_verify_three_blocks() {
    let g = Header::genesis();
    let b1 = g.child(5);
    let b2 = b1.child(6);

    assert_eq!(b2.state(), 11);
    assert!(g.verify_sub_chain(&vec![b1, b2]));
}

#[test]
fn part_3_cant_verify_invalid_parent() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.parent = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_number() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.height = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_state() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    b1.state = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_cant_verify_invalid_pow() {
    let g = Header::genesis();
    let mut b1 = g.child(5);
    // It is possible that this test will pass with a false positive becaue
    // the PoW ifficulty is relatively low.
    b1.consensus_digest = 10;

    assert!(!g.verify_sub_chain(&vec![b1]));
}

#[test]
fn part_3_even_chain_valid() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    // It' all about the states, not the extrinsics. So once the state is even
    // we need to keep it that way. So add evens
    let b3 = b2.child(1); // 4
    let b4 = b3.child(2); // 6

    assert!(g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_even_chain_invalid_first_block_after_fork() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(2); // 5 - invalid
    let b4 = b3.child(1); // 6

    assert!(!g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_even_chain_invalid_second_block_after_fork() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(1); // 4
    let b4 = b3.child(1); // 5 - invalid

    assert!(!g.verify_sub_chain_even(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_valid() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    // It' all about the states, not the extrinsics. So once the state is odd
    // we need to keep it that way. So add evens
    let b3 = b2.child(2); // 5
    let b4 = b3.child(2); // 7

    assert!(g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_invalid_first_block_after_fork() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(1); // 4 - invalid
    let b4 = b3.child(1); // 5

    assert!(!g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_odd_chain_invalid_second_block_after_fork() {
    let g = Header::genesis();     // 0
    let b1 = g.child(2);  // 2
    let b2 = b1.child(1); // 3
    let b3 = b2.child(2); // 5
    let b4 = b3.child(1); // 6 - invalid

    assert!(!g.verify_sub_chain_odd(&vec![b1, b2, b3, b4]));
}

#[test]
fn part_3_verify_forked_chain() {
    let(prefix, even, odd) = build_contentious_forked_chain();

    let g = &prefix[0];
    let full_even_chain = [&prefix[1..], &even].concat();
    let full_odd_chain  = [&prefix[1..], &odd].concat();
    for h in full_even_chain.iter().cloned() {
        println!("{:?}: {:?}", h, hash(&h));
    }
    

    // Both chains are individually valid according to the original rules.
    assert!(g.verify_sub_chain(&full_even_chain[..]));
    assert!(g.verify_sub_chain(&full_odd_chain[..]));

    // Only the even chain is valid according to the even rules
    assert!(g.verify_sub_chain_even(&full_even_chain[..]));
    assert!(!g.verify_sub_chain_even(&full_odd_chain[..]));

    // Only the odd chain is valid according to the odd rules
    assert!(!g.verify_sub_chain_odd(&full_even_chain[..]));
    assert!(g.verify_sub_chain_odd(&full_odd_chain[..]));
}