//! PoW and PoA each have their own set of strengths and weaknesses. Many chains are happy to choose
//! one of them. But other chains would like consensus properties that fall in between. To achieve this
//! we could consider interleaving PoW blocks with PoA blocks. Some very early designs of Ethereum considered
//! this approach as a way to transition away from PoW.

use super::{p1_pow::Pow, p3_poa::SimplePoa, Consensus, ConsensusAuthority, Header};

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

    fn try_from(d: PowOrPoaDigest) -> Result<Self, Self::Error> {
        match d {
            PowOrPoaDigest::Pow(_) => Err(()),
            PowOrPoaDigest::Poa(authority) => Ok(authority),
        }
    }
}

impl Consensus for AlternatingPowPoa {
    type Digest = PowOrPoaDigest;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        // The requirement is based on evenness or oddness of the block height.
        if header.height % 2 == 0 {
            // TODO it may be nicer to have the From impls be on the header type as well
            let poa_header = Header::<ConsensusAuthority> {
                parent: header.parent,
                height: header.height,
                state_root: header.state_root,
                extrinsics_root: header.extrinsics_root,
                consensus_digest: header
                    .consensus_digest
                    .try_into()
                    .expect("Even blocks should have PoA seal which can be downcast."),
            };
            let poa_parent_digest: ConsensusAuthority = parent_digest
                .clone()
                .try_into()
                .expect("Even blocks should have PoA seal which can be downcast.");

            self.poa.validate(&poa_parent_digest, &poa_header)
        } else {
            let pow_header = Header::<u64> {
                parent: header.parent,
                height: header.height,
                state_root: header.state_root,
                extrinsics_root: header.extrinsics_root,
                consensus_digest: header
                    .consensus_digest
                    .try_into()
                    .expect("Odd blocks should have PoW seal which can be downcast."),
            };
            let pow_parent_digest: u64 = parent_digest
                .clone()
                .try_into()
                .expect("Odd blocks should have PoW seal which can be downcast.");

            self.pow.validate(&pow_parent_digest, &pow_header)
        }

        // If we wanted to be extra thorough, we could also check that we have the opposite seal
        // type from the parent header. But really that is just checking that the parent was valid.
    }

    fn seal(
        &self,
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        if partial_header.height % 2 == 0 {
            let poa_parent_digest: ConsensusAuthority = parent_digest
                .clone()
                .try_into()
                .expect("Even blocks should have PoA seal which can be downcast.");

            let poa_sealed_header = self.poa.seal(&poa_parent_digest, partial_header);

            poa_sealed_header.map(|h| Header::<PowOrPoaDigest> {
                parent: h.parent,
                height: h.height,
                state_root: h.state_root,
                extrinsics_root: h.extrinsics_root,
                consensus_digest: h.consensus_digest.into(),
            })
        } else {
            let pow_parent_digest: u64 = parent_digest
                .clone()
                .try_into()
                .expect("Odd blocks should have PoW seal which can be downcast.");

            let pow_sealed_header = self.pow.seal(&pow_parent_digest, partial_header);

            pow_sealed_header.map(|h| Header::<PowOrPoaDigest> {
                parent: h.parent,
                height: h.height,
                state_root: h.state_root,
                extrinsics_root: h.extrinsics_root,
                consensus_digest: h.consensus_digest.into(),
            })
        }
    }
}
