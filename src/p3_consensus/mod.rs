//! This module is all about consensus engines that allow nodes to reach agreement over what blockchain is the real one.
//! We begin by re-implementing the proof of work consensus from the previous module, then look at PoA, and other consensus
//! engines all implementing the same simple interface.

mod p1_pow;
mod p2_dictator;
mod p3_poa; // exercise: dictator is a special case of poa. Create dictator in terms of PoA.
mod p4_even_only;
mod p5_interleave;
mod p6_forking;

type Hash = u64;

/// A Block Header similar to prior chapters of this tutorial.
///
/// Different consensus engines, require different information in the consensus digest.
/// Therefore, the header is now generic over the digest type.
///
/// Consensus engines do not know or care about the blockchain's state machine,
/// which means they can operate entirely at the header level. They never need to touch
/// the complete blocks.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header<Digest> {
    parent: Hash,
    height: u64,
    state_root: Hash,
    extrinsics_root: Hash,
    consensus_digest: Digest,
}
/// A Consensus Engine. Responsible for Sealing blocks and verifying their seals
///
/// Consensus exists independently of execution logic, and therefore operates
/// only on the block headers.
pub trait Consensus {
    type Digest: Clone + core::fmt::Debug + Eq + PartialEq + std::hash::Hash;

    /// Validates that a header is valid according to consensus rules. This
    /// function checks ONLY consensus-related aspects such as the signature
    /// or the attached work proof. It does not check ancestry, execution, or
    /// anything else.
    ///
    /// Some consensus engines need to check a relationship between the current
    /// digest and the parent digest. For example, they may need to check that the
    /// slot number is increasing. Therefore the parent digest is also passed
    /// here. Other consensus engines will not need to use the parent digest at all.
    fn validate(parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool;

    /// Takes a partial header that does not yet have a consensus digest attached. Returns
    /// a new header including the consensus digest that is valid according to the consensus rules.
    ///
    /// Some consensus engines need to enforce a relationship between the current digest and
    /// the parent digest. For example, they may need to make sure that the slot number is always
    /// increasing. Therefore the parent digest is also passed here. Other consensus engines
    /// will not need to use the parent digest at all.
    ///
    /// This function returns an Option because in some consensus engines, it may not be
    /// possible to construct a valid sealed block from the information given.
    fn seal(
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>>;
    // NOTE TO SELF. For slot-based PoA etc, just look at the system time. It's what real-world aura does

    /// Verify that all the given headers are valid according to the consensus rules.
    ///
    /// This method assumes that the parent_digest is valid, and verifies all the
    /// following headers relative to the given parent digest. This is a provided method
    /// on the trait, so it must be general enough to work for any specific consensus engine.
    fn verify_sub_chain(parent_digest: &Self::Digest, chain: &[Header<Self::Digest>]) -> bool {
        todo!("Exercise 1")
    }

    /// A human-readable name for this engine. This may be used in user-facing
    /// programs error reporting. This is not in any way related to
    /// the correctness of the consensus logic.
    fn human_name() -> String {
        "Unnamed Consensus Engine".into()
    }
}

/// A trivial consensus engine that considers all blocks valid, and does not have
/// a meaningful consensus digest.
impl Consensus for () {
    type Digest = ();

    /// All blocks are considered valid
    fn validate(_: &Self::Digest, _: &Header<Self::Digest>) -> bool {
        todo!("Exercise 2")
    }

    /// No real sealing is required. The partial header has all the necessary information
    fn seal(_: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
        todo!("Exercise 3")
    }
}

/// A set of consensus authority accounts that can be used in
/// identity-based consensus algorithms.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConsensusAuthority {
    Alice,
    Bob,
    Charlie,
}
