//! As a blockchain node watches the chain evolve, it must constantly be assessing which chain
//! is currently the best chain. We explored the concept of fork-choice briefly in the Blockchain chapter.
//!
//! The concepts are identical here, but now that we have a client tracking a proper block database,
//! we can explore more advanced fork choice algorithms. In particular, we can now explore GHOST.

use super::{Header, FullClient, Consensus};
use crate::c3_consensus::{Pow, SimplePoa, ConsensusAuthority};

/// A means for a blockchain client to decide which chain is best among the many
/// that it potentially knows about.
///
/// Our client is generic over this bit of logic just like it is generic over the state machine and
/// consensus
///
/// Some implementations are light and just make a quick comparison, like the longest chain rule.
/// Others are more complex and associate additional logic with block import, like GHOST.
pub trait ForkChoice<C: Consensus> {
    /// Return the hash of the best block currently known according to this fork choice rule.
    fn best_block(&self, header: Header<C::Digest>) -> Option<u64>;

    /// Perform some bookkeeping activities when importing a new block.
    fn import_hook(&mut self, header: Header<C::Digest>);
}

/// The chain with the highest block height is the best
pub struct LongestChain {
    // You may add fields here if you need to.
}

impl<C: Consensus> ForkChoice<C> for LongestChain {
    fn best_block(&self, header: Header<C::Digest>) -> Option<u64> {
        todo!("Exercise 1")
    }

    fn import_hook(&mut self, header: Header<C::Digest>) {
        todo!("Exercise 2")
    }
}

/// The chain with the most accumulated proof of work is the best.
/// This fork choice rule only makes sense with the PoW consensus engine
/// and the generics reflect that.
pub struct HeaviestChain {
    // You may add fields here if you need to.
}

impl ForkChoice<Pow> for HeaviestChain {
    fn best_block(&self, header: Header<u64>) -> Option<u64> {
        todo!("Exercise 3")
    }

    fn import_hook(&mut self, header: Header<u64>) {
        todo!("Exercise 4")
    }
}

/// The chain with the most signatures from the Alice authority is the best.
/// This fork choice rule only makes sense with the PoA consensus engine
/// and the generics reflect that.
pub struct MostAliceSigs {
    // You may add fields here if you need to.
}

impl ForkChoice<SimplePoa> for MostAliceSigs {
    fn best_block(&self, header: Header<ConsensusAuthority>) -> Option<u64> {
        todo!("Exercise 5")
    }

    fn import_hook(&mut self, header: Header<ConsensusAuthority>) {
        todo!("Exercise 6")
    }
}

/// In the Greedy Heaviest Observed Subtree rule, the fork choice is iterative.
/// You start from the genesis block, and at each fork, you choose the side of the fork
/// that has the most accumulated proof of work on _all_ of its descendants.
pub struct Ghost {
    // You may add fields here if you need to.
}

impl ForkChoice<Pow> for Ghost {
    fn best_block(&self, header: Header<u64>) -> Option<u64> {
        todo!("Exercise 7")
    }

    fn import_hook(&mut self, header: Header<u64>) {
        todo!("Exercise 8")
    }
}

// Finally, we will provide a convenience method directly on our client that simply calls
// into the corresponding method on the ForkChoice rule. You may need to add some trait
// bounds to make this work.
impl<C, SM, FC, P> FullClient<C, SM, FC, P> {
    /// Return the hash of the best block currently known to the client
    fn best_block(&self) -> u64 {
        todo!("Exercise 9")
    }
}

//TODO lots of tests for all the algos.
// Especially a subtle one in Ghost, where importing a new
// header causes a re-org to a different header than the one that was imported.
