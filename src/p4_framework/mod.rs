use crate::p1_state_machine::StateMachine;
/// Braindump (revise later):
///
/// We have a blockchain data structure featuring:
/// 1. A built in addition accumulator state machine
/// 2. A built-in pow consensus mechanism
///
/// We also have abstractions over:
/// 1. State Machines
/// 2. Consensus Engines
///
/// Let's refactor our blockchain to take advantage of these two abstractions
/// In doing so, we create a blockchain framework
use crate::p3_consensus::{Consensus, Header};
type Hash = u64;

impl<Digest> Header<Digest> {
    /// Returns a new valid genesis header.
    fn genesis(genesis_state_root: Hash) -> Self {
        todo!("Exercise 1")
    }

    /// Create and return a valid child header.
    fn child(&self, state_root: Hash, extrinsics_root: Hash) -> Self {
        todo!("Exercise 2")
    }

    /// Verify a single child header.
    fn verify_child(&self, child: &Self) -> bool {
        todo!("Exercise 3")
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    fn verify_sub_chain(&self, chain: &[Self]) -> bool {
        todo!("Exercise 4")
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Block<C: Consensus, SM: StateMachine> {
    header: Header<C::Digest>,
    body: Vec<SM::Transition>,
}

impl<C: Consensus, SM: StateMachine> Block<C, SM> {
    /// Returns a new valid genesis block. By convention this block has no extrinsics.
    pub fn genesis(genesis_state: &SM::State) -> Self {
        todo!("Exercise 5")
    }

    /// Create and return a valid child block.
    pub fn child(&self, pre_state: &SM::State, extrinsics: Vec<u8>) -> Self {
        todo!("Exercise 6")
    }

    /// Verify that all the given blocks form a valid chain from this block to the tip.
    pub fn verify_sub_chain(&self, pre_state: &SM::State, chain: &[Self]) -> bool {
        todo!("Exercise 7")
    }
}

/// Create and return a block chain that is n blocks long starting from the given genesis state.
/// The blocks should not contain any transactions.
fn create_empty_chain<C: Consensus, SM: StateMachine>(
    n: u64,
    genesis_state: &SM::State,
) -> Vec<Block<C, SM>> {
    todo!("Exercise 8")
}

//TODO tests

//TODO maybe this shouldn't be a whole chapter. Maybe it is the first
// section in the chapter on building a client
