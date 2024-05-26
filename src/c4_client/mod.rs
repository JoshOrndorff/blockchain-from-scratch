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

use crate::{
    c1_state_machine::StateMachine,
    c3_consensus::{Consensus, Header},
};
use p1_data_structure::Block;
use p3_fork_choice::ForkChoice;

mod p1_data_structure;
mod p2_importing_blocks;
mod p3_fork_choice;
mod p4_transaction_pool;
mod p5_authoring_blocks;
mod p6_finality;

type Hash = u64;

/// A client represents one view of an evolving blockchain network. It knows of blocks,
/// forks, state, and it also pools transactions waiting to be included in upcoming blocks.
/// It can import new blocks, author its own blocks.
///
/// The client that we are writing is very reusable and is generic in several ways including:
/// * state machines - It can use any state machine that implements our trait.
/// * consensus system - It can use any consensus engine that implements our trait.
/// * Fork Choice - It can use any fork choice we discussed and more. This is explored shortly.
/// * Transaction Pool - It can use any logic for queueing and prioritizing incoming future transactions.
///
/// As you work through the sections in this chapter, you will add features to the client
/// by implementing more and more methods on it.
/// 
/// In practice the trait bounds here will always be the same:
/// C: Client
/// SM: StateMachine
/// FC: ForkChoice<C>
/// P: TransactionPool<SM>
/// 
/// But we leave them unconstrained here to avoid repeating many where clauses throughout the section.
/// Instead we bind them on impl blocks.
pub struct FullClient<C, SM, FC, P>
{
    /// The consensus engine used by this client.
    consensus_engine: C,
    /// The state machine used by this client.
    state_machine: SM,
    /// The fork choice strategy used by this client.
    fork_choice: FC,
    /// The transaction pool used by this client.
    transaction_pool: P,

    // TODO: You are free to add more fields here, and you will probably need to.
    // Please document them as you add them.
}

//TODO Consider exploring LightClient as well. It may import headers but not blocks for example.
