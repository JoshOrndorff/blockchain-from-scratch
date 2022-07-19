//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use super::StateMachine;

/// The keys on the ATM keypad
#[derive(Hash)]
pub enum Key{
    One,
    Two,
    Three,
    Four,
    Enter,
}

/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The hash of a user's expected pin. The ATM will not expect any particular pin
    /// before the user swipes their card.
    expected_pin_hash: Option<u64>,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}

impl StateMachine for Atm {

    // Exercise: Fill in these associated types.
    type State = ();
    type Transition = ();

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> () {
        todo!()
    }

}