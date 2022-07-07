
//! We begin our hands on exploration of state machines with two very simple examples.
//! In these examples, we use actualy switch boards as the state machine. The state is,
//! well, just the state of the switches.

use super::StateMachine;

/// This state machine models a single light switch.
/// The internal state, a bool, represents whether the switch is on or not.
pub struct LightSwitch(bool);

/// We model this simple system as a state machine with a single transition - toggling the switch
/// Because there is only a single kind of transition, we can use a unit struct.
impl StateMachine<()> for LightSwitch {
    fn next_state(&self, t: &()) -> Self {
        todo!("Exercise 1")
    }
}

/// This second  state machine models two light switches (with one weird property).
/// Either switch can be manually toggled via a transition.
/// WEIRD PROPERTY: Whenever switch one is turned off, switch two also goes off.
pub struct TwoSwitches {
    first_switch: bool,
    second_switch: bool,
}

/// Now there are two switches so we need a proper type for the transition.
pub enum Toggle {
    FirstSwitch,
    SecondSwitch,
}

/// We model this system as a state machine with two possible transitions
impl StateMachine<Toggle> for TwoSwitches {
    fn next_state(&self, t:&Toggle) -> Self {
        todo!("Exercise 2")
    }
}


#[test]
fn sm_1_light_switch_toggles_off() {
    let on = LightSwitch(true);

    // Toggle once and assert it goes off
    let s1 = on.next_state(&());
    assert!(!s1.0);
}

#[test]
fn sm_1_light_switch_toggles_on() {
    let off = LightSwitch(false);

    // Toggle once and assert it comes on
    let s1 = off.next_state(&());
    assert!(s1.0);
}

#[test]
fn sm_1_two_switches_first_goes_on() {

}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_on() {
    // This is the special case. We have to make sure the second one goes off with it.
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_off() {
    // This is adjascent to the special case. We have to make sure the second one stays off.
}

#[test]
fn sm_1_two_switches_second_goes_on() {

}

#[test]
fn sm_1_two_switches_second_goes_off() {

}