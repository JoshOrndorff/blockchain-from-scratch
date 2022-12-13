//! This module is all about consensus engines that allow nodes to reach agreement over what blockchain is the real one.
//! We begin by re-implementing the proof of work consensus from the previous module, the look at PoA, and other consensus
//! engines all implementing the same simple interface.

mod pow;
mod dictator;
mod poa;
mod interleave;

/// A Consensus Engine. Responsible for authoring and importing blocks
pub trait Consensus {

    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;

    /// A human-readable name for this engine. This may be used in user-facing
    /// programs error reporting. This is not in any way related to
    /// the correctness of the state machine.
    fn human_name() -> String {
        "Unnamed Consensus Engine".into()
    }
}
