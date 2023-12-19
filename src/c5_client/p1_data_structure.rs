//! Before we implement any serious  methods on our client, we must, create the `Block` and `Header`
//! data structures one last time like we did in Chapter 1. the logic you wrote there
//! will be useful here as well and can probably be reused to some extent.
//!
//! Throughout the Blockchain chapter, we created a blockchain data structure that had:
//! 1. a built-in addition accumulator state machine
//! 2. A built-in pow consensus mechanism
//!
//! In the State Machine and Consensus chapters, we designed abstractions over both
//! the state machine and the consensus. We also implemented several examples of each
//! trait.
//!
//! This will be the last time we have to write this blockchain data structure,
//! because this time it will be fully generic over both the state machine and consensus
//! logic, thanks to our traits.
//!
//! This abstraction is the key idea behind blockchain _frameworks_ like Substrate or the Cosmos SDK.

use super::{Consensus, ForkChoice, Header, StateMachine};

use super::FullClient;
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
pub struct Block<C: Consensus, SM: StateMachine> {
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

// To wrap this section up, we will implement the first two simple methods on our client.
// These methods simply create a new instance of the client initialized with a proper
// genesis block.
impl<C, SM, FC> FullClient<C, SM, FC>
where
    C: Consensus,
    SM: StateMachine,
    FC: ForkChoice<C>,
{
    fn new(genesis_state: SM::State) -> Self {
        todo!("Exercise 9")
    }
}

// The default client is initialized with the default genesis state.
// Depending on the state machine definition there may not _be_ a default
// genesis state. There is only a default client when there is also a
// default genesis state.
impl<C, SM, FC> Default for FullClient<C, SM, FC>
where
    C: Consensus,
    SM: StateMachine,
    FC: ForkChoice<C>,
{
    fn default() -> Self {
        todo!("Exerise 10")
    }
}

//TODO tests
