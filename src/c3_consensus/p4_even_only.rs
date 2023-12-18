//! In the previous chapter, we considered a hypothetical scenario where blocks must contain an even state root
//! in order to be valid. Now we will express that logic here as a higher-order consensus engine. It is higher-
//! order because it will wrap an inner consensus engine, such as PoW or PoA and work in either case.

use std::marker::PhantomData;

use super::{Consensus, Header};

/// A Consensus engine that wraps another consensus engine. This engine enforces the requirement that
/// a block must have an even state root in order to be valid

/// A Consensus engine that requires the state root to be even for the header to be valid.
/// Wraps an inner consensus engine whose rules will also be enforced.
struct EvenOnly<Inner: Consensus>(PhantomData<Inner>);

impl<Inner: Consensus> Consensus for EvenOnly<Inner> {
    type Digest = Inner::Digest;

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

/// Using the moderate difficulty PoW algorithm you created in section 1 of this chapter as the inner engine,
/// create a PoW chain that is valid according to the inner consensus engine, but is not valid according to
/// this engine because the state roots are not all even.
fn almost_valid_but_not_all_even() -> Vec<Header<u64>> {
    todo!("Exercise 3")
}
