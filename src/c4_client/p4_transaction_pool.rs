//! In this section our client will begin maintaining a transaction pool.
//!
//! The transaction pool is where transactions that are not yet included in the blockchain
//! are queued before they are inserted into blocks.
//! 
//! Maintaining a transaction pool includes:
//! * Accepting transactions from users
//! * Removing transactions that are included in blocks as they are imported
//! * Making the current transactions available for a block authoring process
//! * Re-queueing transactions from orphaned blocks when re-orgs happen (This one happens IRL; might not cover it in BFS; TBD)

use super::{ForkChoice, FullClient, StateMachine, Consensus};

impl<C, SM, FC> FullClient<C, SM, FC>           
    where
    C: Consensus,
    SM: StateMachine,
    FC: ForkChoice<C>,
{
    /// Submit a new transaction to the pool;
    fn submit_transaction(&mut self, t: SM::Transition) {
        todo!("Exercise 1")
    }

    /// Check whether the specified transaction exists in the pool
    fn pool_contains(&self, t: SM::Transition) {
        todo!("Exercise 2")
    }
}
