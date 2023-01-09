//! Until now we have focused primarily on the blockchain as a data structure. We've created instances of the
//! data structure, practiced validating it, and deciding on a canonical branch when forks occur and the
//! data structure becomes more like a tree than a list.
//! 
//! Now we turn 
//!
//! At this point much of the logic from before will be reusable, but it will be attached to the client
//! as methods on the client instead of on the block or the header. The block and header will be treated
//! mainly as data
//
// Exercise for later: Client does a hard fork at a particular block height. The fork logic is to change runtimes.

use std::collections::HashMap;
use super::p2_blockchain::p4_batched_extrinsics::{Block, Header};
//TODO use the latest one once that lesson is written
// use super::p5_rich_state::{Block, Header};

type Transaction = u64;
type State = u64;
type Hash = u64;

/// A client basically represents one view of an evolving blockchain network. It knows of blocks,
/// forks, state, and it also pools transactions waiting to be included in upcoming blocks.
/// It can import new blocks, author its own blocks
pub struct FullClient {
    transaction_pool: Vec<Transaction>,
    block_database: HashMap<Hash, Block>,
    state_database: HashMap<Hash, State>,
    leaves: HashSet<Hash>,
}

//TODO maybe make a trait `Client` and implement it for light client too.
// Let's see how many of the same methods make sense.
impl FullClient {
    fn import_block(&mut self, b: Block) -> Result<Hash, String> {
        todo!()
    }

    // Could provide an explicit parent, or could have a fork choice rule, or both
    // maybe we start with explicit parent and add the fork choice rule later.
    // should be able to do ghost now that we have a block database.
    fn create_block(){todo!()}

    fn get_block_by_hash(&self, h: Hash) -> Result<Block, String> {
        todo!()
    }

    fn get_block_by_number(){todo!()}

    fn best_block()-> Hash {todo!()}

    fn submit_transaction(t: Transaction) -> Result<Hash, String> {todo!()}

    //TODO maybe this method gets introduced later on and we see how it allows pruning
    // the leaves and limits how far back we have to iterate for things like seeing which block is best
    fn note_finality(b: Hash) { todo!()}
}