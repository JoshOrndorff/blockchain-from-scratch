//! This module is all about modeling phenomina and systems as state machines. We begin with a few simple
//! examples, and then proceed to build bigger and more complex state machines all implementing the same simple interface.

mod p1_switches;
mod p2_laundry_machine;
mod p3_atm;

/// A state machine - Generic over the transition type
pub trait StateMachine {

    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;

    /// A human-readable name for this state machine. This may be used in user-facing
    /// programs such as the repl described below. This is not in any way related to
    /// the correctness of the state machine.
    fn human_name() -> String {
        "Unnamed state machine".into()
    }
}

//TODO Some kind of main program that allows users to interact with their state machine in a repl-like way.
// Might require From<String> implementation for the transition type.