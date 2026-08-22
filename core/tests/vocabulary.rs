//! What the vocabulary promises, with no device in sight.
//!
//! This crate describes no board — the devices live one crate per vendor — so
//! what is left to check here is the shape a description is written in: that a
//! key with no job maps to no word on the hint bar, and that a pair of keys is
//! a pair only when both are there.
//!
//! The census over real hardware is in the conformance suite,
//! `examples/gallery/tests/boards.rs`, because comparing one vendor's board to
//! another's has no home below the caller. Each vendor's own boards are checked
//! in that vendor's crate.

use xpui::Button;
use xpui_boards_core::{Bezel, Board, KeyAction, PhysicalButton};

/// Only a key that presses something sends a button.
///
/// A firmware reading pins asks this of every key it wires, so the two arms
/// that answer `None` decide whether a switch is silent. `Home` is one of them
/// and no board's footer carries it, so nothing else in the suite reaches that
/// arm — split it and every other test still passes.
#[test]
fn a_key_that_is_not_a_button_sends_nothing() {
    assert_eq!(
        KeyAction::Press(Button::Confirm).button(),
        Some(Button::Confirm),
        "a key that presses something sends it"
    );
    assert_eq!(
        KeyAction::Home.button(),
        None,
        "Home arrives as a gesture from the touch controller, not through a pin"
    );
    assert_eq!(
        KeyAction::Unassigned.button(),
        None,
        "an unassigned key is under a thumb and does nothing"
    );
}

/// Half a pair is not a pair.
///
/// No board here carries one of Left and Right without the other, so the seven
/// real answers cannot tell `&&` from `||` — this is the case that can. It is
/// not hypothetical bookkeeping: a value that can be raised and never lowered
/// is the fault [`Key::unassigned`] exists to avoid on the badges' third key.
#[test]
fn half_a_pair_is_not_a_pair() {
    const ONLY_RIGHT: [PhysicalButton; 1] = [PhysicalButton {
        label: "Right",
        action: KeyAction::Press(Button::Right),
        centre: (100, 900),
        size: (60, 60),
    }];
    const LOPSIDED: Bezel = Bezel {
        body: (600, 1000),
        panel_origin: (50, 50),
        panel_size: (500, 800),
        buttons: &ONLY_RIGHT,
        artwork: None,
    };

    let board = Board {
        bezel: Some(LOPSIDED),
        ..Board::custom("Lopsided", 400, 300, false)
    };

    assert!(
        !board.has_left_right_keys(),
        "one key of the two is not a pair — the value could be raised and never lowered"
    );

    // And the same board with no body at all, which is what `Board::custom`
    // gives every panel nobody has described.
    assert!(
        !Board::custom("Bare", 400, 300, false).has_left_right_keys(),
        "a board with no bezel cannot promise keys it has not described"
    );
}
