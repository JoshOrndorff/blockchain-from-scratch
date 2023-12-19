//! PoW and PoA each have their own set of strengths and weaknesses. Many chains are happy to choose
//! one of them. But other chains would like consensus properties that fall in between. To achieve this
//! we could consider interleaving PoW blocks with PoA blocks. Some very early designs of Ethereum considered
//! this approach as a way to transition away from PoW.

/// A Consensus engine that alternates back and forth between PoW and PoA sealed blocks.
struct AlternatingPowPoa;
use super::{Consensus, Header, ConsensusAuthority};

/// In order to implement a consensus that can be sealed with either work or a signature,
/// we will need an enum that wraps the two individual digest types.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum PowOrPoaDigest {
    Pow(u64),
    Poa(ConsensusAuthority),
}

impl From<u64> for PowOrPoaDigest {
    fn from(_: u64) -> Self {
        todo!("Exercise 1")
    }
}

impl From<PowOrPoaDigest> for u64 {
    fn from(_: PowOrPoaDigest) -> Self {
        todo!("Exercise 2")
    }
}

impl From<ConsensusAuthority> for PowOrPoaDigest {
    fn from(_: ConsensusAuthority) -> Self {
        todo!("Exercise 3")
    }
}

impl From<PowOrPoaDigest> for ConsensusAuthority {
    fn from(_: PowOrPoaDigest) -> Self {
        todo!("Exercise 4")
    }
}

impl Consensus for AlternatingPowPoa {
    type Digest = ConsensusAuthority;

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