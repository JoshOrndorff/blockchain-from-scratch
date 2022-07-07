//! In this lesson we expand our simple notion of state, and show how the state is typically not stored in the header,
//! Or indeed anywhere in the block at all.
//! 
//! To facilitate this exercise, consider that we want our blockchain to store not only the sum of the extrinsics,
//! but also the product. You can also imagine many other calculations the chain may want to track (min, max, median mean, etc).
//! 
//! As the state data gets large, it is no longer reasonable to store it in the blocks. But if the state isn't in the blocks,
//! then how can we perform the state-related validation checks we previously performed? We use a state root to cryptographically
//! link our heder to a complete state


/// In this section we will use suma nd product together to be our state. While this is only a doubling of state size
/// remember that in real world blockchains, the state is often really really large.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    sum: u64,
    product: u64,
}

//TODO