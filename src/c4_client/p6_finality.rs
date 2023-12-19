//! Sometimes you want a block to never be reverted.
//! In practice this is usually implemented by some kind of BFT based consensus game.
//! We will model a very simple alternative where node operators manually request finality.
//! 
//! Although we elide the details of the game itself, this model still allows us to explore
//! the consequences of having some blocks that are never reverted.
