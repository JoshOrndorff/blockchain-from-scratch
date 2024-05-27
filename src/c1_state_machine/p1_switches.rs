//! We begin our hands on exploration of state machines with two very simple examples.
//! In these examples, we use actually switch boards as the state machine. The state is,
//! well, just the state of the switches.

use super::StateMachine;

/// This state machine models a single light switch.
/// The internal state, a bool, represents whether the switch is on or not.
pub struct LightSwitch;

/// We model this simple system as a state machine with a single transition - toggling the switch
/// Because there is only a single kind of transition, we can use a unit struct.
impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    fn next_state(starting_state: &bool, t: &()) -> bool {
        !starting_state
    }
}

/// This second  state machine models two light switches with one weird property.
/// Whenever switch one is turned off, switch two also goes off.
pub struct WeirdSwitchMachine;

/// The state is now two switches instead of one so we use a struct.
#[derive(PartialEq, Eq, Debug)]
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
impl StateMachine for WeirdSwitchMachine {
    type State = TwoSwitches;
    type Transition = Toggle;

    fn next_state(starting_state: &TwoSwitches, t: &Toggle) -> TwoSwitches {
        match t {
            Toggle::FirstSwitch => {
                if starting_state.first_switch {
                    TwoSwitches {
                        first_switch: false,
                        second_switch: false,
                    }
                } else {
                    TwoSwitches {
                        first_switch: true,
                        second_switch: starting_state.second_switch,
                    }
                }
            }
            Toggle::SecondSwitch => TwoSwitches {
                first_switch: starting_state.first_switch,
                second_switch: !starting_state.second_switch,
            },
        }
    }
}

#[test]
fn sm_1_light_switch_toggles_off() {
    assert!(!LightSwitch::next_state(&true, &()));
}

#[test]
fn sm_1_light_switch_toggles_on() {
    assert!(LightSwitch::next_state(&false, &()));
}

#[test]
fn sm_1_two_switches_first_goes_on() {
    let state = TwoSwitches {
        first_switch: false,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: true,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_on() {
    // This is the special case. We have to make sure the second one goes off with it.
    let state = TwoSwitches {
        first_switch: true,
        second_switch: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_first_goes_off_second_was_off() {
    // This is adjacent to the special case. We have to make sure the second one stays off.
    let state = TwoSwitches {
        first_switch: true,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::FirstSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: false,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_on() {
    let state = TwoSwitches {
        first_switch: false,
        second_switch: false,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoSwitches {
            first_switch: false,
            second_switch: true,
        }
    );
}

#[test]
fn sm_1_two_switches_second_goes_off() {
    let state = TwoSwitches {
        first_switch: true,
        second_switch: true,
    };

    assert_eq!(
        WeirdSwitchMachine::next_state(&state, &Toggle::SecondSwitch),
        TwoSwitches {
            first_switch: true,
            second_switch: false,
        }
    );
}
