//! This module is all about consensus engines that allow nodes to reach agreement over what blockchain is the real one.
//! We begin by re-implementing the proof of work consensus from the previous module, then look at PoA, and other consensus
//! engines all implementing the same simple interface.

mod p1_pow;
mod dictator;
mod even_only;
mod poa; // exercise: dictator is a special case of poa. Create dictator in terms of PoA.
mod interleave;

/// A Consensus Engine. Responsible for authoring and importing blocks
/// 
/// Consensus exists independently of execution logic, and therefore operates
/// only on the block headers.
pub trait Consensus<Header> {
    
    /// Validates that a header is valid according to consensus rules. This
    /// function checks ONLY consensus-related aspects such as the signature
    /// or the attached work proof. It does not check ancestry, execution, or
    /// anything else.
    fn validate(h: &Header) -> bool;

    /// Takes an existing header and mutates it to be valid according to
    /// consensus rules. This will typically mean attaching signatures or
    /// proving work.
    fn seal(h: &mut Header);

    /// A human-readable name for this engine. This may be used in user-facing
    /// programs error reporting. This is not in any way related to
    /// the correctness of the state machine.
    fn human_name() -> String {
        "Unnamed Consensus Engine".into()
    }
}

/// A set of consensus authority accounts that can be used in
/// identity-based consensus algorithms
pub enum ConsensusAuthority {
    Alice,
    Bob,
    Charlie,
}