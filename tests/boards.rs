//! What a board promises about itself.

use xpui_boards::Board;

/// A slug has to survive the round trip, or `--board x3` opens something else.
#[test]
fn every_board_is_found_by_its_own_slug() {
    for board in Board::ALL {
        assert_eq!(
            Board::from_slug(board.slug),
            Some(board),
            "{} calls itself {:?} and that does not find it",
            board.name,
            board.slug
        );
    }
}

/// The X4 and the Sticky are both 800x480 and differ in whether a finger works.
///
/// Identity is a stored slug rather than a pair of dimensions for exactly this
/// reason: a size-based match cannot tell these two apart, and an earlier
/// version returned the same answer for both.
#[test]
fn two_boards_with_one_panel_size_stay_distinct() {
    assert_eq!(
        (Board::X4.width, Board::X4.height),
        (Board::STICKY.width, Board::STICKY.height),
        "this test is pointless if they stop sharing a size"
    );

    assert_ne!(Board::X4.slug, Board::STICKY.slug);
    assert_ne!(Board::X4, Board::STICKY);

    // Const rather than runtime: these are compile-time facts, so a change to
    // either one fails the build rather than a test run.
    const _: () = assert!(!Board::X4.touch, "the X4 has no touchscreen");
    const _: () = assert!(Board::STICKY.touch, "the Sticky has one");
}

/// A board of some other size must not claim to be one of these.
#[test]
fn a_custom_board_does_not_borrow_another_slug() {
    let mine = Board::custom("Mine", 800, 480, false);

    for board in Board::ALL {
        assert_ne!(
            mine.slug, board.slug,
            "a custom board reported itself as {}",
            board.name
        );
    }
    assert_eq!(Board::from_slug(mine.slug), None);
}

/// Two rows to compare and one to show there is more. Below that a list is not
/// a list, and the screen is unusable rather than merely cramped.
#[test]
fn every_board_holds_at_least_three_list_rows() {
    for board in Board::ALL {
        assert!(
            board.list_rows() >= 3,
            "{} ({}x{}) fits {} list rows",
            board.name,
            board.width,
            board.height,
            board.list_rows()
        );
    }
}

/// Slugs are what a command line accepts, so two boards cannot share one.
#[test]
fn no_two_boards_share_a_slug() {
    for (index, board) in Board::ALL.iter().enumerate() {
        for other in &Board::ALL[index + 1..] {
            assert_ne!(
                board.slug, other.slug,
                "{} and {} both answer to {:?}",
                board.name, other.name, board.slug
            );
        }
    }
}

/// An unknown name is not silently one of the presets.
#[test]
fn an_unknown_slug_finds_nothing() {
    for name in ["", "reader", "reader-landscape", "x5", "badger2041"] {
        assert_eq!(Board::from_slug(name), None, "{name:?} matched something");
    }
}

/// The short names people actually say.
#[test]
fn the_aliases_work() {
    assert_eq!(Board::from_slug("badger"), Some(Board::BADGER_2040));
    assert_eq!(Board::from_slug("tufty"), Some(Board::TUFTY_2040));
}

// -- bezels ----------------------------------------------------------------

use xpui_boards::Bezel;

fn bezels() -> impl Iterator<Item = (Board, Bezel)> {
    Board::ALL
        .into_iter()
        .filter_map(|board| board.bezel.map(|bezel| (board, bezel)))
}

/// A button has to be somewhere a thumb can reach: on the body, and not on top
/// of the screen.
#[test]
fn every_button_is_on_the_body_and_off_the_panel() {
    for (board, bezel) in bezels() {
        let (px, py, pw, ph) = bezel.panel_rect();

        for button in bezel.buttons {
            let (cx, cy) = button.centre;
            let (w, h) = button.size;
            let (left, top) = (cx - w / 2, cy - h / 2);
            let (right, bottom) = (cx + w / 2, cy + h / 2);

            assert!(
                left >= 0 && top >= 0 && right <= bezel.body.0 && bottom <= bezel.body.1,
                "{}: {:?} is off the body",
                board.name,
                button.label
            );

            let overlaps = right > px && left < px + pw && bottom > py && top < py + ph;
            assert!(
                !overlaps,
                "{}: {:?} sits on the panel. A physical button belongs beside \
                 the screen — one drawn over it would take taps the firmware \
                 should have had",
                board.name, button.label
            );
        }
    }
}

/// The panel has to fit inside the body it is set into.
#[test]
fn the_panel_fits_in_the_body() {
    for (board, bezel) in bezels() {
        let (x, y, w, h) = bezel.panel_rect();
        assert!(
            x >= 0 && y >= 0 && x + w <= bezel.body.0 && y + h <= bezel.body.1,
            "{}: the panel hangs off the body",
            board.name
        );
    }
}

/// Two buttons in the same place means one of them can never be pressed.
#[test]
fn no_two_buttons_overlap() {
    for (board, bezel) in bezels() {
        for (index, first) in bezel.buttons.iter().enumerate() {
            for second in &bezel.buttons[index + 1..] {
                let apart = (first.centre.0 - second.centre.0).abs()
                    >= (first.size.0 + second.size.0) / 2
                    || (first.centre.1 - second.centre.1).abs()
                        >= (first.size.1 + second.size.1) / 2;
                assert!(
                    apart,
                    "{}: {:?} and {:?} overlap",
                    board.name, first.label, second.label
                );
            }
        }
    }
}

/// Hit-testing has to find the button you aimed at, and nothing where there is
/// none.
#[test]
fn a_press_finds_the_button_under_it() {
    for (board, bezel) in bezels() {
        for button in bezel.buttons {
            assert_eq!(
                bezel.button_at(button.centre).map(|found| found.label),
                Some(button.label),
                "{}: pressing the middle of {:?} found something else",
                board.name,
                button.label
            );
        }

        // The middle of the panel is not a button on any of these.
        let (x, y, w, h) = bezel.panel_rect();
        assert!(
            bezel.button_at((x + w / 2, y + h / 2)).is_none(),
            "{}: the middle of the screen reported a button",
            board.name
        );
    }
}

/// The X3 carries its Up and Down on the side, which is the thing that makes it
/// feel like that device rather than a generic slab.
#[test]
fn the_x3_has_side_buttons() {
    let bezel = Board::X3.bezel.expect("the X3 has a bezel");
    let (px, _, pw, _) = bezel.panel_rect();

    for label in ["Up", "Dn"] {
        let button = bezel
            .buttons
            .iter()
            .find(|button| button.label == label)
            .unwrap_or_else(|| panic!("the X3 has a {label} button"));

        assert!(
            button.centre.0 > px + pw,
            "{label} should be beside the panel, not below it"
        );
    }
}
