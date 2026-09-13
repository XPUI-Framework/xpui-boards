//! What Pimoroni's own boards promise.
//!
//! Only this vendor's. Anything comparing one vendor's board to another's is
//! in the conformance suite, `xpui-gallery`'s `gallery/tests/boards.rs`, because there
//! is no crate below the caller that knows more than one vendor.

use xpui::Button;
use xpui_boards_core::{Bezel, PhysicalButton};
use xpui_boards_pimoroni as pimoroni;

/// The keys beside the panel on one side, top to bottom: a board's edge column.
fn column_of(bezel: Bezel, on_the_right: bool) -> Vec<PhysicalButton> {
    let (x, y, width, height) = bezel.panel_rect();
    let mut keys: Vec<PhysicalButton> = bezel
        .buttons
        .iter()
        .copied()
        .filter(|key| key.centre.1 < y + height)
        .filter(|key| {
            let beside = key.centre.0 < x || key.centre.0 > x + width;
            beside && (key.centre.0 > x) == on_the_right
        })
        .collect();
    keys.sort_by_key(|key| key.centre.1);
    keys
}

/// The keys below the panel, left to right: a board's footer row.
fn footer_of(bezel: Bezel) -> Vec<PhysicalButton> {
    let (_, y, _, height) = bezel.panel_rect();
    let mut keys: Vec<PhysicalButton> = bezel
        .buttons
        .iter()
        .copied()
        .filter(|key| key.centre.1 > y + height)
        .collect();
    keys.sort_by_key(|key| key.centre.0);
    keys
}

/// The short names people actually say.
#[test]
fn the_aliases_work() {
    assert_eq!(pimoroni::from_slug("badger"), Some(pimoroni::BADGER_2040));
    assert_eq!(pimoroni::from_slug("tufty"), Some(pimoroni::TUFTY_2040));
}

/// A label no key carries finds nothing rather than the nearest key.
///
/// This is the firmware's silent-failure path: `Buttons::new` resolves each pin
/// through a label, and a name the board does not have leaves that switch
/// sampled and quiet. Nothing on a device would say so except the boot line, so
/// the branch is pinned here where it can be.
#[test]
fn a_label_no_key_carries_finds_nothing() {
    let bezel = pimoroni::BADGER_2040.bezel.expect("the Badger has a body");

    assert!(
        bezel.button_labelled("a").is_some(),
        "the Badger does carry an `a` key, or the negative below proves nothing"
    );
    assert_eq!(
        bezel.button_labelled("A"),
        None,
        "labels are matched exactly — the Inky Frame's keys are `A` to `E` and \
         the badges' are lower case, so a firmware written for one board finds \
         nothing on the other rather than the wrong key"
    );
    assert!(
        bezel.button_labelled("").is_none(),
        "and a blank label matches no key either"
    );
}

/// The badges' keys send what the firmware wires them to, by name.
///
/// `Buttons::new` in `xpui-rp2040` looks each pin's key up here by name, so
/// **what a key sends has one copy**: `BADGE_FOOTER` for the first three,
/// `BADGE_EDGE` for the pair. Which switch carries which *name* is written in
/// two places the firmware reads — those tables, and its own five labels — and
/// `wired` below is a third, existing only to hold the first against what the
/// second is expected to say. The firmware's own literals are held by nothing,
/// because that crate has no host build to run a test in; they are checked on
/// the first boot instead.
///
/// Which GPIO sits under a switch is beyond any test here: its one source of
/// truth is a schematic this repository does not hold.
#[test]
fn the_badges_keys_send_what_the_firmware_wires() {
    // The labels `Buttons::new` looks its five pins up by, in field order.
    let wired = ["a", "b", "c", "Up", "Dn"];

    for board in [pimoroni::BADGER_2040, pimoroni::TUFTY_2040] {
        let bezel = board.bezel.expect("both badges have a body");

        // A key the board describes and the firmware does not wire is a switch
        // that cannot be pressed; a label renamed here is one that goes quietly
        // dead. Both are caught before the button values are looked at, because
        // a missing label would otherwise read as a mapping fault.
        assert_eq!(
            bezel.buttons.len(),
            wired.len(),
            "{}: the firmware wires {} switches and the board describes {}",
            board.name,
            wired.len(),
            bezel.buttons.len()
        );
        for label in wired {
            assert!(
                bezel.button_labelled(label).is_some(),
                "{}: nothing is labelled {label}, so that switch would read as dead",
                board.name
            );
        }

        let sends = |label: &str| bezel.button_labelled(label).map(|key| key.action.button());

        assert_eq!(
            sends("a"),
            Some(Some(Button::Back)),
            "{}: the leftmost key goes back",
            board.name
        );
        assert_eq!(
            sends("b"),
            Some(Some(Button::Confirm)),
            "{}: the second key confirms",
            board.name
        );
        assert_eq!(
            sends("c"),
            Some(None),
            "{}: the third key has nothing on it — giving it a job is an edit to \
             `BADGE_FOOTER`, to `BADGE_ROW` and to this line, never to the firmware",
            board.name
        );
        assert_eq!(
            sends("Up"),
            Some(Some(Button::Up)),
            "{}: the edge pair walks the list",
            board.name
        );
        assert_eq!(
            sends("Dn"),
            Some(Some(Button::Down)),
            "{}: the edge pair walks the list",
            board.name
        );
    }
}

/// The board the builder was written for: five keys along the footer and
/// nothing down either edge, which no board here had before.
#[test]
fn the_inky_frame_is_a_footer_and_nothing_else() {
    let bezel = pimoroni::INKY_FRAME
        .bezel
        .expect("the Inky Frame has a body");

    assert_eq!(
        footer_of(bezel).len(),
        5,
        "Pimoroni's own module exposes button_a through button_e"
    );
    assert!(
        column_of(bezel, false).is_empty() && column_of(bezel, true).is_empty(),
        "nothing sits beside this panel"
    );
    assert_eq!(
        bezel.buttons.len(),
        5,
        "and nothing is anywhere else either"
    );
}

/// Its diagonal, against the figure Pimoroni publish that is not a diagonal.
///
/// They quote a 0.1915mm dot pitch, which is 132 ppi; the panel is sold as
/// 5.7". A digit wrong in either is invisible until something is measured in
/// millimetres, and then it is wrong everywhere at once.
#[test]
fn the_inky_frames_diagonal_agrees_with_its_dot_pitch() {
    // Both in ten-thousandths of a millimetre: an inch over one dot.
    let from_pitch = 254_000 / 1915;
    let derived = pimoroni::INKY_FRAME.ppi().expect("a measured panel");

    assert!(
        (derived - from_pitch).abs() <= 2,
        "a 5.7\" diagonal derives {derived} ppi; a 0.1915mm pitch is {from_pitch}"
    );
}
