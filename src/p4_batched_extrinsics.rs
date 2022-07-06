//! Until now, each block has contained just a single extrinsic. Rreally we would prefer to batch them.
//! Now, we stop relying solely on headers, and instead, create complete blocks.


/// The s
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Header {
    parent: Hash,
    height: u64,
    // We now switch from storing an extrinsic directly, to storing an extrinsic root.
    // This is basically a concise cryptographic commitment to the complete list of extrinsics.
    // For example, a hash or a Merkle root.
    extrinsics_root: Hash,
    state: u64,
    consensus_digest: u64,
}

impl Header {
    fn parent(&self) -> Hash {
        self.parent
    }

    fn height(&self) -> u64 {
        self.height
    }

    fn extrinsic(&self) -> u64 {
        self.extrinsic
    }

    fn state(&self) -> u64 {
        self.state
    }

    fn consensus_digest(&self) -> u64 {
        self.consensus_digest
    }
}

/// A complete Block is a header and the extrinsics.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Block {
    header: Header,
    body: Vec<u64>,
}

