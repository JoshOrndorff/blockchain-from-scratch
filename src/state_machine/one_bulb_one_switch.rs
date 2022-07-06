
//! TODO

use super::StateMachine;

/// A state machine that represents a single light bulb that is controlled by a single switch.
/// The internal state, a single bool, represents whether the lgiht is on or not.
pub struct OneBulbOneSwitch(bool);

/// We model this simple system as a state machine with a single transition - toggling the switch
/// Because there is only a single kind of transition, we can use a unit struct.
impl StateMachine<()> for OneBulbOneSwitch {
    fn next_state(&self, t: &()) -> Self {
        todo!("Exercise 1")
    }
}


#[test]
fn toggles_off() {
    let on = OneBulbOneSwitch(true);

    // Toggle once and assert it goes off
    let s1 = on.next_state(&());
    assert!(!s1.0);
}

#[test]
fn toggles_on() {
    let off = OneBulbOneSwitch(false);

    // Toggle once and assert it comes on
    let s1 = off.next_state(&());
    assert!(s1.0);
}