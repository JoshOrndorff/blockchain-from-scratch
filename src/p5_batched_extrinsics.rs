//! With our state now removed from the block headers, we are ready to tackle our final (for now) design
//! challenge which is that each block currently only contains a single extrinsic, and really we would prefer
//! to batch them. Finally, we stop relying solely on headers, and instead, now, create complete blocks.


/// A much more realistic header
#[derive(Debug, PartialEq, Eq, Hash)]
struct Header {
    parent: Hash,
    height: u64,
    // We know from the lecture that we will probably need these, but we don't need them yet.
    extrinsics_root: Hash,
    state_root: Hash,
    consensus_digest: u64,
}

/// The most basic blockchain header possible. We learned its basic structure from lecture.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Block {
    header: Header,
    body: Vec<u64>,
}