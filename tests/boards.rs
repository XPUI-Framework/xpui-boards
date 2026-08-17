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

/// The X3 and the X4 Pro put one page key on each side edge; the X4 stacks
/// both on the same side.
///
/// That difference is not cosmetic — the firmware branches on it, in
/// `HalGPIO::hasEdgeSideButtons` and in every theme that draws a hint beside a
/// key. A simulator that got it wrong would teach the wrong muscle memory.
#[test]
fn the_page_keys_sit_where_the_firmware_says() {
    for board in [Board::X3, Board::X4_PRO] {
        let bezel = board.bezel.expect("this board has a body");
        let (left_edge, _, panel_width, _) = bezel.panel_rect();
        let right_edge = left_edge + panel_width;

        let key = |label: &str| {
            bezel
                .buttons
                .iter()
                .find(|button| button.label == label)
                .unwrap_or_else(|| panic!("{} has no {label} key", board.name))
        };

        assert!(
            key("Up").centre.0 < left_edge,
            "{}: Up belongs on the left edge",
            board.name
        );
        assert!(
            key("Dn").centre.0 > right_edge,
            "{}: Down belongs on the right edge",
            board.name
        );
    }

    // The X4 is the other arrangement: both on one side.
    let bezel = Board::X4.bezel.expect("the X4 has a body");
    let up = bezel.buttons.iter().find(|b| b.label == "Up").unwrap();
    let down = bezel.buttons.iter().find(|b| b.label == "Dn").unwrap();
    assert_eq!(
        up.centre.0, down.centre.0,
        "the X4 stacks its page keys on one side rather than splitting them"
    );
}

/// The X4 Pro's face carries one key, because the touchscreen does the rest.
#[test]
fn the_x4_pro_has_a_home_key_below_the_panel() {
    let bezel = Board::X4_PRO.bezel.expect("the X4 Pro has a body");
    let (_, top, _, panel_height) = bezel.panel_rect();

    let home = bezel
        .buttons
        .iter()
        .find(|button| button.label == "Home")
        .expect("the X4 Pro has a Home key");

    assert!(
        home.centre.1 > top + panel_height,
        "the Home key sits below the panel, not beside it"
    );
    assert_eq!(
        home.size.0, home.size.1,
        "it is round, so its box is square"
    );
}

/// The Sticky's three keys are a column, not a row.
#[test]
fn the_sticky_stacks_its_three_keys() {
    let bezel = Board::STICKY.bezel.expect("the Sticky has a body");
    assert_eq!(
        bezel.buttons.len(),
        3,
        "the firmware wires exactly three pins"
    );

    let column = bezel.buttons[0].centre.0;
    for button in bezel.buttons {
        assert_eq!(
            button.centre.0, column,
            "{:?} is out of the column",
            button.label
        );
    }

    let mut heights: Vec<i32> = bezel.buttons.iter().map(|b| b.centre.1).collect();
    let sorted = {
        let mut copy = heights.clone();
        copy.sort_unstable();
        copy
    };
    heights.dedup();
    assert_eq!(heights.len(), 3, "three keys at three heights");
    assert_eq!(
        bezel.buttons.iter().map(|b| b.centre.1).collect::<Vec<_>>(),
        sorted,
        "declared top to bottom, so the order reads as the device does"
    );
}

// -- orientation -----------------------------------------------------------

/// The readers scan landscape and are held portrait, so their canvas is the
/// framebuffer turned a quarter. Getting this backwards lays every screen out
/// against the wrong shape.
#[test]
fn a_portrait_board_presents_its_framebuffer_turned() {
    for board in Board::ALL {
        let (width, height) = board.orientation.canvas(board.framebuffer);
        assert_eq!(
            (width, height),
            (board.width, board.height),
            "{}: a {:?} board with a {}x{} framebuffer presents {}x{}, not {}x{}",
            board.name,
            board.orientation,
            board.framebuffer.0,
            board.framebuffer.1,
            width,
            height,
            board.width,
            board.height
        );
    }
}

/// Every Xteink reader is used upright, whatever way its controller scans.
#[test]
fn the_readers_are_portrait() {
    use xpui_boards::Orientation;

    for board in [Board::X3, Board::X4, Board::X4_PRO, Board::STICKY] {
        assert_eq!(board.orientation, Orientation::Portrait, "{}", board.name);
        assert!(
            board.height > board.width,
            "{}: a portrait canvas is taller than it is wide",
            board.name
        );
        assert!(
            board.framebuffer.0 > board.framebuffer.1,
            "{}: these panels scan landscape",
            board.name
        );
    }
}

/// The touch boards are the ones with a touchscreen, and only those.
#[test]
fn touch_is_recorded_where_the_hardware_has_it() {
    // Const, because these are compile-time facts: changing one fails the
    // build rather than a test run.
    const _: () = assert!(Board::X4_PRO.touch, "the X4 Pro has a touchscreen");
    const _: () = assert!(Board::STICKY.touch, "the Sticky has one");

    for board in [Board::X3, Board::X4, Board::BADGER_2040, Board::TUFTY_2040] {
        assert!(!board.touch, "{} has buttons only", board.name);
    }
}

/// The X4 and the X4 Pro share a panel and differ in what is on top of it.
#[test]
fn the_x4_pro_is_the_x4_with_a_touchscreen() {
    assert_eq!(
        (Board::X4.width, Board::X4.height),
        (Board::X4_PRO.width, Board::X4_PRO.height)
    );
    assert_eq!(Board::X4.framebuffer, Board::X4_PRO.framebuffer);
    const _: () = assert!(!Board::X4.touch && Board::X4_PRO.touch);
    assert_ne!(Board::X4.slug, Board::X4_PRO.slug);
}
