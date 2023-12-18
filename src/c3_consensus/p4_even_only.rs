//! In the previous chapter, we considered a hypothetical scenario where blocks must contain an even state root
//! in order to be valid. Now we will express that logic here as a higher-order consensus engine. It is higher-
//! order because it will wrap an inner consensus engine, such as PoW or PoA and work in either case.

use std::marker::PhantomData;

use super::{Consensus, Header};

/// A Consensus engine that requires the state root to be even for the header to be valid.
/// Wraps an inner consensus engine whose rules will also be enforced.
struct EvenOnly<Inner: Consensus> {
    /// The inner consensus engine that will be used in addition to the even-only requirement.
    inner: Inner,
}

impl<Inner: Consensus> Consensus for EvenOnly<Inner> {
    type Digest = Inner::Digest;

    fn validate(&self, parent_digest: &Self::Digest, header: &Header<Self::Digest>) -> bool {
        // First we ensure that the state root is even, that is the requirement of this engine.
        if header.state_root % 2 != 0 {
            return false;
        }

        // Then we pass along to the inner consensus engine to perform its own checks.
        self.inner.validate(parent_digest, header)
    }

    fn seal(
        &self,
        parent_digest: &Self::Digest,
        partial_header: Header<()>,
    ) -> Option<Header<Self::Digest>> {
        // It is our duty to only sign even blocks. Therefore, we refuse to sign any others.
        if partial_header.state_root % 2 != 0 {
            return None;
        }

        // If the state root is even, as it should be, we alow the inner engine to seal.
        self.inner.seal(parent_digest, partial_header)
    }
}

/// Using the moderate difficulty PoW algorithm you created in section 1 of this chapter as the inner engine,
/// create a PoW chain that is valid according to the inner consensus engine, but is not valid according to
/// this engine because the state roots are not all even.
fn almost_valid_but_not_all_even() -> Vec<Header<u64>> {
    // The minimal solution is a single header with an odd state root and valid PoW.

    // We create a partial header with the desired odd state root.
    let partial_header = Header {
        parent: 0,
        height: 0,
        state_root: 1, // ODD!
        extrinsics_root: 0,
        consensus_digest: (),
    };

    // Now use the requested engine to seal the block
    let sealed_header = super::p1_pow::moderate_difficulty_pow()
        .seal(&0, partial_header)
        .expect("PoW can always seal a header.");

    // Return a chain with a single header
    vec![sealed_header]
}
