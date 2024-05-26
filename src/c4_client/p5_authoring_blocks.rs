//! We are now ready to give out client the ability to author blocks.
//! Clients that perform this task are usually known as "miners", "authors", or "authorities".

use super::{FullClient, StateMachine};

// You may need to add trait bounds to make this work.
impl<C, SM, FC, P> FullClient<C, SM, FC, P>
    where
    SM: StateMachine,
{
    /// Author a new block with the given transactions on top of the given parent
    /// and import the new block into the local database.
    pub fn author_and_import_manual_block(&mut self, transactions: Vec<SM::Transition>, parent_hash: u64) {
        todo!("Exercise 1")
    }

    /// Author a new block with the transactions from the pool on top of the "best" block
    /// and import the new block into the local database.
    pub fn author_and_import_automatic_block(&self) {
        todo!("Exercise 2")
    }
}

//TODO tests