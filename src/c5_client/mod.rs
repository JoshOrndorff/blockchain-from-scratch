//! Until now we have focused primarily on the blockchain as a data structure. We've created instances of the
//! data structure, practiced validating it, and deciding on a canonical branch when forks occur and the
//! data structure becomes more like a tree than a list.
//! 
//! Even as we learned how to abstract out the common elements of the blockchain such as the consensus rules,
//! and state machine logic, we still remained focused on the data structure itself.
//! 
//! In this final chapter, we will shift our focus toward a blockchain client. A client is a piece of software
//! that follows a blockchain in real-time. It imports blocks, follows forks, queues transactions, and even authors blocks.
//! Throughout this chapter, we will use the state machine and consensus abstractions that we developed in the
//! previous two chapters.


// TODO Exercise for later: Client does a hard fork at a particular block height. The fork logic is to change runtimes.

use std::{collections::{HashMap, HashSet}, marker::PhantomData};
use crate::{
    c1_state_machine::StateMachine,
    c3_consensus::Consensus,
};

mod p1_data_structure;
mod p2_importing_blocks;
mod p3_fork_choice;
mod p4_transaction_pool;
mod p5_authoring_blocks;
// TODO maybe add a section about providing RPC services like a state_at method.
// mod p6_serving_data;

type Hash = u64;

/// A client represents one view of an evolving blockchain network. It knows of blocks,
/// forks, state, and it also pools transactions waiting to be included in upcoming blocks.
/// It can import new blocks, author its own blocks.
/// 
/// The client that we are writing is very reusable and is generic over multiple state machines
/// and consensus systems. These are represented as generic parameters.
/// 
/// As you work through the sections in this chapter, you will add features to the client
/// by implementing more and more traits on it. You may also need to revisit your struct definition
/// to add more fields.
pub struct FullClient<C: Consensus, SM: StateMachine> {
    // You are free to add fields here. Please document them as you add them.
    _ph_data: PhantomData<(C, SM)>,

    //TODO Probably don't want to include all of these right off the bat.
    // For now it is more of a brain dump.
    transaction_pool: Vec<SM::Transition>,
    // block_database: HashMap<Hash, Block>,
    // state_database: HashMap<Hash, State>,
    leaves: HashSet<Hash>,
}

//TODO Consider exploring LightClient as well. It may import headers but not blocks for example.


// Braindump of methodds that we may try to implement in the next few sections.
// TODO this should be removed after the next few sections are fleshed out.

impl<C: Consensus, SM: StateMachine> FullClient<C, SM> {
    fn import_block(&mut self, block: ()) -> Result<Hash, String> {
        todo!()
    }

    // Could provide an explicit parent, or could have a fork choice rule, or both
    // maybe we start with explicit parent and add the fork choice rule later.
    // should be able to do ghost now that we have a block database.
    fn create_block(){todo!()}

    fn get_block_by_hash(&self, h: Hash) -> Result<(), String> {
        todo!()
    }

    fn get_block_by_number(){todo!()}

    fn best_block()-> Hash {todo!()}

    fn submit_transaction(t: SM::Transition) -> Result<Hash, String> {todo!()}

    //TODO maybe this method gets introduced later on and we see how it allows pruning
    // the leaves and limits how far back we have to iterate for things like seeing which block is best
    fn note_finality(b: Hash) { todo!()}
}