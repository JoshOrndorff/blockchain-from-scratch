//! We are now ready to give out client the ability to author blocks.
//! Clients that perform this task are usually known as "miners", "authors", or "authorities".

use super::{ForkChoice, FullClient, StateMachine, Consensus};

impl<C, SM, FC> FullClient<C, SM, FC>           
    where
    C: Consensus,
    SM: StateMachine,
    FC: ForkChoice<C>,
{
    /// Author a new block with the given transactions on top of the given parent
    /// and import the new block into the local database.
    fn author_and_import_manual_block(&mut self, transactions: Vec<SM::Transition>, parent_hash: u64) {
        todo!("Exercise 1")
    }

    /// Author a new block with the transactions from the pool on top of the "best" block
    /// and import the new block into the local database.
    fn author_and_import_automatic_block(&self) {
        todo!("Exercise 2")
    }
}

//TODO tests