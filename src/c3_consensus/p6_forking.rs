//! We saw in the previous chapter that blockchain communities sometimes opt to modify the
//! consensus rules from time to time. This process is knows as a fork. Here we implement
//! a higher-order consensus engine that allows such forks to be made.
//!
//! The consensus engine we implement here does not contain the specific consensus rules to
//! be enforced before or after the fork, but rather delegates to existing consensus engines
//! for that. Here we simply write the logic for detecting whether we are before or after the fork.

use std::marker::PhantomData;

use super::{Consensus, ConsensusAuthority, Header};

/// A Higher-order consensus engine that represents a change from one set of consensus rules (Before) to
/// another set (After) at a specific block height
struct Forked<D, Before, After> {
    /// The first block height at which the new consensus rules apply
    fork_height: u64,
    phdata: PhantomData<(D, Before, After)>,
}

impl<D, B, A> Consensus for Forked<D, B, A>
where
    D: Clone + core::fmt::Debug + Eq + PartialEq + std::hash::Hash,
    B: Consensus,
    A: Consensus,
    B::Digest: Into<D>,
    A::Digest: Into<D>,
{
    type Digest = D;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        todo!("Exercise 1")
    }

    fn seal(
        &self,
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        todo!("Exercise 2")
    }
}

/// Create a PoA consensus engine that changes authorities part way through the chain's history.
/// Given the initial authorities, the authorities after the fork, and the height at which the fork occurs.
fn change_authorities(
    fork_height: u64,
    initial_authorities: Vec<ConsensusAuthority>,
    final_authorities: Vec<ConsensusAuthority>,
) -> impl Consensus {
    todo!("Exercise 3")
}

/// Create a PoW consensus engine that changes the difficulty part way through the chain's history.
fn change_difficulty(
    fork_height: u64,
    initial_difficulty: u64,
    final_difficulty: u64,
) -> impl Consensus {
    todo!("Exercise 4")
}

/// Earlier in this chapter we implemented a consensus rule in which blocks are only considered valid if
/// they contain an even state root. Sometimes a chain will be launched with a more traditional consensus like
/// PoW or PoA and only introduce an additional requirement like even state root after a particular height.
///
/// Create a consensus engine that introduces the even-only logic only after the given fork height.
/// Other than the evenness requirement, the consensus rules should not change at the fork. This function
/// should work with either PoW, PoA, or anything else as the underlying consensus engine.
fn even_after_given_height<Original: Consensus>(fork_height: u64) -> impl Consensus {
    todo!("Exercise 5")
}

/// In the spirit of Ethereum's recent switch from PoW to PoA, let us model a similar
/// switch in our consensus framework. It should go without saying that the real-world ethereum
/// handoff was considerably more complex than it may appear in our simplified example, although
/// the fundamentals are the same.
/// 
/// For this task, you may use the PowOrPoaDigest type from the previous module if you like.
fn pow_to_poa(
    fork_height: u64,
    difficulty: u64,
    authorities: Vec<ConsensusAuthority>,
) -> impl Consensus {
    todo!("Exercise 6")
}
