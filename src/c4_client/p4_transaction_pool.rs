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

use std::{collections::VecDeque, marker::PhantomData};

use super::{FullClient, StateMachine};

/// An abstraction over the notion of transaction pool.
pub trait TransactionPool<SM: StateMachine> {
    /// Try to add a new transaction to the pool. Return whether the operation succeeded.
    fn try_insert(&mut self, t: SM::Transition) -> bool;

    /// Remove a given transaction from the pool if it sexists there.
    fn remove(&mut self, t: SM::Transition);

    /// Get the total number of transactions in the pool
    fn size(&self) -> usize;

    /// Check whether the specified transaction exists in the pool
    fn contains(&self, t: SM::Transition) -> bool;

    //TODO actually, the pool itself could be an iterator.
    // Maybe make a blanket impl for all pools based on this method.
    /// Take the next transaction out of the transaction pool
    /// 
    /// The notion of next is opaque and implementation dependent.
    /// Different chains prioritize transactions differently, usually by economic means.
    fn next_from_pool(&mut self) -> Option<SM::Transition>;
}


// First we add some new user-facing methods to the client.
// These are basically wrappers around methods that the pool itself provides.
impl<C, SM, FC, P> FullClient<C, SM, FC, P>           
    where
    SM: StateMachine,
{
    /// Submit a transaction to the client's transaction pool to hopefully
    /// be included in a future block.
    pub fn submit_transaction(&mut self, t: SM::Transition) {
        todo!("Exercise 1")
    }

    /// Get the total number of transactions in the node's
    /// transaction pool.
    pub fn pool_size(&self) -> usize {
        todo!("Exercise 2")
    }

    /// Check whether a a given transaction is in the client's transaction pool.
    pub fn pool_contains(&self, t: SM::Transition) -> bool {
        todo!("Exercise 3")
    }
}

/// A simple state machine that is just a first-in-first-out queue.
pub struct SimplePool<SM: StateMachine>(VecDeque<SM::Transition>);

impl<SM: StateMachine> TransactionPool<SM> for SimplePool<SM> {
    fn try_insert(&mut self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn remove(&mut self, t: <SM as StateMachine>::Transition) {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn contains(&self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn next_from_pool(&mut self) -> Option<<SM as StateMachine>::Transition> {
        todo!()
    }
}

/// A transaction pool that assigns a priority to each transaction and then provides
/// them (to the authoring process, presumably) highest priority first.
/// 
/// It also refuses to queue transactions whose priority is below a certain threshold.
/// 
/// This is where the blockspace market takes place. A lot of interesting game theory
/// happens here.
pub struct PriorityPool<T, P: Fn(T) -> u64> {
    /// A means of determining a transaction's priority
    prioritizer: P,
    /// The minimum priority that will be accepted. Any transaction with a
    /// priority below this value will be rejected.
    minimum_priority: u64,
    ph_data: PhantomData<T>
}

impl<SM, P> TransactionPool<SM> for PriorityPool<SM::Transition, P>
where
    SM: StateMachine,
    P: Fn(SM::Transition) -> u64
{
    fn try_insert(&mut self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn remove(&mut self, t: <SM as StateMachine>::Transition) {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn contains(&self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn next_from_pool(&mut self) -> Option<<SM as StateMachine>::Transition> {
        todo!()
    }
}

/// A transaction pool that censors some transactions.
/// 
/// It refuses to queue any transactions that are might be associated with terrorists.
pub struct CensoringPool<T, P: Fn(T) -> bool> {
    /// A means of determining whether a transaction may be from a terrorist
    might_be_terrorist: P,
    ph_data: PhantomData<T>
}

impl<SM, P> TransactionPool<SM> for CensoringPool<SM::Transition, P>
where
    SM: StateMachine,
    P: Fn(SM::Transition) -> bool
{
    fn try_insert(&mut self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn remove(&mut self, t: <SM as StateMachine>::Transition) {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn contains(&self, t: <SM as StateMachine>::Transition) -> bool {
        todo!()
    }

    fn next_from_pool(&mut self) -> Option<<SM as StateMachine>::Transition> {
        todo!()
    }
}

//TODO tests

// #[test]
// fn simple_pool_starts_empty() {
//     let client = FullClient::<(), (), ()>::default();

//     assert!(client.pool_contains(t));
//     assert_eq!(client.next_from_pool(), None);

// }

// #[test]
// fn simple_pool_submitting_transaction_works() {
//     let mut client = FullClient::<(), (), ()>::default();

//     let t = Transition::default();

//     client.submit_transaction(t.clone());

//     assert!(client.pool_contains(t));
//     assert_eq!(client.next_from_pool(), t);

// }



// More tests for block importing to make sure that transactions that are imported
// to the chain are correctly removed from the pool.