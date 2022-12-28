//! This module explores the blockchain. A distributed hash-linked tree-like data structure that is used
//! to track alternative histories of a shared resource. It also explores a simple work-based consensus
//! algorithm to help users decide which history is the canonical one.

// We make the complete Block and Header types publicly visible so that we can continue developing
// against them in future chapters. The prior iterations are not available outside this chapter.
pub use p6_rich_state::{Block, Header};

mod p1_header_chain;
mod p2_extrinsic_state;
mod p3_consensus;
pub mod p4_batched_extrinsics;
mod p5_fork_choice;
mod p6_rich_state;
