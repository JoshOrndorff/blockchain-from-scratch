//! Now is your chance to get creative. Choose a state machine that interests you and model it here.
//! Get as fancy as you like. The only constraint is that it should be simple enough that you can
//! realistically model it in an hour or two.
//!
//! Here are some ideas:
//! * Board games:
//!   * Chess
//!   * Checkers
//!   * Tic tac toe
//! * Beaurocracies:
//!   * Beauro of Motor Vehicles - maintains driving licenses and vehicle registrations.
//!   * Public Utitiy Provider - Customers open accounts, consume the utility, pay their bill periodically, maybe utility prices fluctuate
//!   * Land ownership registry
//! * Tokenomics:
//!   * Token Curated Registry
//!   * Prediction Market
//!   * There's a game where there's a prize to be split among players and the prize grows over time. Any player can stop it at any point and take most of the prize for themselves.
//! * Social Systems:
//!   * Social Graph
//!   * Web of Trust
//!   * Reputation System

use super::StateMachine;

pub struct State {

}

pub enum Transition {

}

impl StateMachine for State {

    type State = State;
    type Transition = Transition;
    
    fn next_state(_starting: &Self::State, _t: &Self::Transition) -> Self::State {
        todo!()
    }
}