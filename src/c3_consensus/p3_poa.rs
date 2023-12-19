//! Proof of Work is very energy intensive but is decentralized. Dictator is energy cheap, but
//! is completely centralized. Let's achieve a middle ground by choosing a set of authorities
//! who can sign blocks as opposed to a single dictator. This arrangement is typically known as
//! Proof of Authority.
//!
//! In public blockchains, Proof of Authority is often moved even further toward the decentralized
//! and permissionless end of the spectrum by electing the authorities on-chain through an economic
//! game in which users stake tokens. In such a configuration it is often known as "Proof of Stake".
//! Even when using the Proof of Stake configuration, the underlying consensus logic is identical to
//! the proof of authority we are writing here.

use super::{Consensus, ConsensusAuthority, Header};

/// A Proof of Authority consensus engine. If any of the authorities have signed the block, it is valid.
pub struct SimplePoa {
    pub authorities: Vec<ConsensusAuthority>,
}

impl Consensus for SimplePoa {
    type Digest = ConsensusAuthority;

    fn validate(&self, _parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        self.authorities.contains(&header.consensus_digest)
    }

    fn seal(
        &self,
        _parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        // We can sign by any authority, so for simplicity, we just choose the first one in the list.
        // For the first time, we an engine that may not be able to produce a valid block.
        // If there are no authorities in the list, we have to return None.
        let Some(&signer) = self.authorities.get(0) else {
            return None;
        };

        Some(Header::<Self::Digest> {
            parent: partial_header.parent,
            height: partial_header.height,
            state_root: partial_header.state_root,
            extrinsics_root: partial_header.extrinsics_root,
            consensus_digest: signer,
        })
    }
}

/// A Proof of Authority consensus engine. Only one authority is valid at each block height.
/// As ever, the genesis block does not require a seal. After that the authorities take turns
/// in order.
struct PoaRoundRobinByHeight {
    authorities: Vec<ConsensusAuthority>,
}

impl Consensus for PoaRoundRobinByHeight {
    type Digest = ConsensusAuthority;

    fn validate(&self, _parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        // First make sure there are actually some authorities
        if self.authorities.is_empty() {
            return false;
        }

        // Calculate the proper authority
        let authority_index = header.height as usize % self.authorities.len();
        let proper_authority = self.authorities[authority_index];

        // Make sure the proper authority signed
        header.consensus_digest == proper_authority
    }

    fn seal(
        &self,
        _parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        // First make sure there are actually some authorities
        if self.authorities.is_empty() {
            return None;
        }

        // Calculate the proper authority
        let authority_index = partial_header.height as usize % self.authorities.len();
        let proper_authority = self.authorities[authority_index];

        // Sign as the proper authority
        Some(Header::<Self::Digest> {
            parent: partial_header.parent,
            height: partial_header.height,
            state_root: partial_header.state_root,
            extrinsics_root: partial_header.extrinsics_root,
            consensus_digest: proper_authority,
        })
    }
}

/// Both of the previous PoA schemes have the weakness that a single dishonest authority can corrupt the chain.
/// * When allowing any authority to sign, the single corrupt authority can sign blocks with invalid transitions
///   with no way to throttle them.
/// * When using the round robin by height, their is throttling, but the dishonest authority can stop block production
///   entirely by refusing to ever sign a block at their height.
///
/// A common PoA scheme that works around these weaknesses is to divide time into slots, and then do a round robin
/// by slot instead of by height
struct PoaRoundRobinBySlot {
    authorities: Vec<ConsensusAuthority>,
}

/// A digest used for PoaRoundRobinBySlot. The digest contains the slot number as well as the signature.
/// In addition to checking that the right signer has signed for the slot, you must check that the slot is
/// always strictly increasing. But remember that slots may be skipped.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct SlotDigest {
    slot: u64,
    signature: ConsensusAuthority,
}

impl Consensus for PoaRoundRobinBySlot {
    type Digest = SlotDigest;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        todo!("Exercise 5")
    }

    fn seal(
        &self,
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        todo!("Exercise 6")
    }
}
