//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use super::StateMachine;

/// The keys on the ATM keypad
pub enum Key{
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the pin that should
    /// be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The ATM itself. When you swipe your card, the ATM learns your correct pin.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
pub struct Atm {
    cash_inside: u64,
    expected_pin: Option<u64>,
    pin_register: Vec<Key>,
}

impl StateMachine for Atm {

    type State = ();
    type Transition = ();

    fn next_state(starting_state: &(), t: &()) -> () {
        todo!()
    }

}