//! Sometimes you want a block to never be reverted.
//! In practice this is usually implemented by some kind of BFT based consensus game.
//! We will model a very simple alternative where node operators manually request finality.
//! 
//! Although we elide the details of the game itself, this model still allows us to explore
//! the consequences of having some blocks that are never reverted.

use super::FullClient;

impl<C, SM, FC, P> FullClient<C, SM, FC, P> {
    /// Mark the given block as final so that it will never be reverted.
    /// Returns whether or not the block was known and marked successfully.
    pub fn manually_finalize_block(&mut self, block_hash: u64) -> bool {
        todo!("Exercise 1")
    }
}

//TODO tests