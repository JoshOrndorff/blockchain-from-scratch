//! PoW and PoA each have their own set of strengths and weaknesses. Many chains are happy to choose
//! one of them. But other chains would like consensus properties that fall in between. To achieve this
//! we could consider interleaving PoW blocks with PoA blocks. Some very early designs of Ethereum considered
//! this approach as a way to transition away from PoW.

use super::{Consensus, ConsensusAuthority, Header, p1_pow::Pow, p3_poa::SimplePoa};

/// A Consensus engine that alternates back and forth between PoW and PoA sealed blocks.
/// 
/// Odd blocks are PoW
/// Even blocks are PoA
struct AlternatingPowPoa {
    // One approach to the fields is to store the pow threshold and poa authorities directly.
    // I think it is more elegant to store whole PoW instances that can be re-used.

    /// An instance of a PoW consensus that will be used for the PoW blocks
    pow: Pow,
    /// An instance of a PoA consensus that will be used for the PoA blocks
    poa: SimplePoa,
}

impl AlternatingPowPoa {
    fn new(threshold: u64, authorities: Vec<ConsensusAuthority>) -> Self {
        Self {
            pow: Pow { threshold },
            poa: SimplePoa { authorities },
        }
    }
}

/// In order to implement a consensus that can be sealed with either work or a signature,
/// we will need an enum that wraps the two individual digest types.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum PowOrPoaDigest {
    Pow(u64),
    Poa(ConsensusAuthority),
}

impl From<u64> for PowOrPoaDigest {
    fn from(nonce: u64) -> Self {
        Self::Pow(nonce)
    }
}

impl TryFrom<PowOrPoaDigest> for u64 {
    type Error = ();

    fn try_from(d: PowOrPoaDigest) -> Result<Self, Self::Error> {
        match d {
            PowOrPoaDigest::Pow(nonce) => Ok(nonce),
            PowOrPoaDigest::Poa(_) => Err(()),
        }
    }
}

impl From<ConsensusAuthority> for PowOrPoaDigest {
    fn from(authority: ConsensusAuthority) -> Self {
        Self::Poa(authority)
    }
}

impl TryFrom<PowOrPoaDigest> for ConsensusAuthority {
    type Error = ();

    fn try_from(d: PowOrPoaDigest) -> Result<Self, Self::Error>  {
        match d {
            PowOrPoaDigest::Pow(_) => Err(()),
            PowOrPoaDigest::Poa(authority) => Ok(authority),
        }
    }
}

impl Consensus for AlternatingPowPoa {
    type Digest = ConsensusAuthority;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        // The requirement is based on evenness or oddness of the block height.
        if header.height % 2 == 0 {
            self.poa.va
        }
    }

    fn seal(
        &self,
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        todo!("Exercise 6")
    }
}
