//! When you wear clothes they get dirty. When you wash them they get wet. When you dry them, they're
//! ready to be worn again. Or course washing and wearing clothes takes its toll on the clothes, and
//! eventually they get tattered.

/// Models a piece of clothing throughout its lifecycle.
pub enum Clothes {
    /// Clean clothes ready to be worn. With some given life left.
    Clean(u64),
    /// Dirty clothes. With some given life left.
    Dirty(u64),
    /// Wet clothes. With some given life left.
    Wet(u64),
    /// Tattered clothes beyond their useful life. Cannot be used anymore and will always be tattered.
    Tattered,
}

/// Something you can do with clothes
pub enum ClothesAction {
    Wear,
    Wash,
    Dry,
}

impl StateMachine<ClothesAction> for Clothes {
    fn next_state(&self, t: &ClothesAction) -> Self {
        todo!("Implement this state machine.")
    }
}

#[test]
fn sm_2_wear_clean_clothes() {

}

#[test]
fn sm_2_wear_dirty_clothes() {

}

#[test]
fn sm_2_wear_wet_clothes() {

}

#[test]
fn sm_2_wear_tattered_clothes() {

}

#[test]
fn sm_2_wash_clean_clothes() {

}

#[test]
fn sm_2_wash_dirty_clothes() {

}

#[test]
fn sm_2_wash_wet_clothes() {

}

#[test]
fn sm_2_wash_tattered_clothes() {

}

#[test]
fn sm_2_dry_clean_clothes() {

}

#[test]
fn sm_2_dry_dirty_clothes() {

}

#[test]
fn sm_2_dry_wet_clothes() {

}

#[test]
fn sm_2_dry_tattered_clothes() {

}