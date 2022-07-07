//! This module is all about modeling phenomina and systems as state machines. We begin with a few simple
//! examples, and then proceed to build bigger and more complex state machines all implementing the same simple interface.

mod p1_switches;
mod p2_laundry_machine;
mod p3_atm;

/// A state machine - Generic over the transition type
pub trait StateMachine<Transition> {

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(&self, t: &Transition) -> Self;
}

//TODO Some kind of main program that allows users to interact with their state machine in a repl-like way.
// Might require From<String> implementation for the transition type.