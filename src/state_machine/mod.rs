//! This module is all about modeling phenomina and systems as state machines. We begin with a few simple
//! examples, and then proceed to build bigger and more complex state machines all implementing the same simple interface.


/// A state machine - Generic over the transition type
pub trait StateMachine<Transition> {

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(&self, t: &Transition) -> Self;
}