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

// Methods for creating and verifying headers.
//
// With the extrinsics no longer stored in the header, we can no longer do
// "on-chain" execution with just headers. That means that this code actually
// gets simpler in many ways. All the old execution logic, plus some new logic
// for batching moves to the block level now.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        todo!("Exercise 1")
    }

    /// Create and return a valid child header.
    /// Without the extrinsics themselves, we cannot calculate the final state
    /// so that information is passed in.
    fn child(&self, extrinsic_root: Hash, state: u64) -> Self {
        todo!("Exercise 2")
    }

    /// Verify a single child header.
    /// 
    /// This is a slightly different interface fro mthe previous units. Rather
    /// than verify an entire subchain, this function checks a single header.
    /// This is useful because checking the header can now be thought of as a 
    /// subtask of checking an entire block. So it doesn't make sense to check
    /// the entire header chain at once if the chain may be invalid at the second block.
    fn verify_child(&self, child: &Header) -> bool {
        todo!("Exercise 3")
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    ///
    /// We can now trivially write the old verification function in terms of the new one.
    /// Extra street cred if you can write it
    ///  * with a loop
    ///  * with head recursion
    ///  * with tail recursion
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        todo!("Exercise 4")
    }
}

/// A complete Block is a header and the extrinsics.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Block {
    header: Header,
    body: Vec<u64>,
}

// Methods for creating and verifying blocks.
//
// These methods are analogous to the methods on the headers. All of the
// transaction execution logic is now handled at the block level because
// the transactions are no longer available at the Header level.
impl Block {
    /// Returns a new valid genesis block. By convention this block has no extrinsics.
    fn genesis() -> Self {
        todo!("Exercise 5")
    }

    /// Create and return a valid child block.
    /// The extrinsics are batched now, so we need to execute each of them.
    fn child(&self, extrinsics: Vec<u8>) -> Self {
        todo!("Exercise 6")
    }

    /// Verify that all the given blocks form a valid chain from this block to the tip.
    ///
    /// We need to verify the headers as well as execute all transactions and check the final state.
    fn verify_sub_chain(&self, chain: &[Block]) -> bool {
        todo!("Exercise 7")
    }
}

/// Create a child block of the given block. The child block should be invalid, but
/// the header should be valid.
/// 
/// Now that extrinsics are separate from headers, the logic for checking headers does
/// not include actual transaction execution. That means its possible for a header to be
/// valid, but the block containing that header to be invalid.
/// 
/// Notice that you do not need the entire parent block to do this. You only need the header.
fn build_invald_child_block_with_valid_header(parent: &Header) {
    todo!("Exercise 8")
}