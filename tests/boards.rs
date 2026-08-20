//! What a board promises about itself.

use xpui::Button;
use xpui_boards::Board;
use xpui_boards::KeyAction;
use xpui_chrome::RowKey;

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

// -- how big a pixel is ----------------------------------------------------

use xpui_boards::Tokens;

/// A panel with no diagonal cannot be asked how big its chrome is, and every
/// preset here has to be able to answer.
#[test]
fn every_preset_panel_has_been_measured() {
    for board in Board::ALL {
        assert!(
            board.ppi().is_some(),
            "{} has no diagonal, so nothing can say what its chrome measures",
            board.name
        );
    }

    assert_eq!(
        Board::custom("Mine", 800, 480, false).ppi(),
        None,
        "a panel nobody has measured says so rather than guessing"
    );
}

/// The derivation, against the densities these panels are sold with. A digit
/// wrong in a diagonal is invisible until something is measured in
/// millimetres, and then it is wrong everywhere at once.
#[test]
fn the_derived_density_matches_the_panel() {
    for (board, published) in [
        (Board::X3, 257),
        (Board::X4, 218),
        (Board::X4_PRO, 218),
        (Board::STICKY, 234),
        (Board::BADGER_2040, 111),
        (Board::TUFTY_2040, 166),
    ] {
        let ppi = board.ppi().expect("a measured panel");
        assert!(
            (ppi - published).abs() <= 2,
            "{}: {}x{} over {}\" derives {ppi} ppi, not the {published} it is sold as",
            board.name,
            board.framebuffer.0,
            board.framebuffer.1,
            board.diagonal_hundredths_inch.unwrap_or(0) as f32 / 100.0
        );
    }
}

/// The scale is the firmware's, board for board: a finger bumps it, a button
/// does not.
///
/// If a board ever needs one without the other, this is the place to say why —
/// the rule is `BoardConfig`'s, not an accident of which boards exist.
#[test]
fn the_touch_boards_are_the_scaled_ones() {
    for board in Board::ALL {
        let expected = if board.touch { 120 } else { 100 };
        assert_eq!(
            board.ui_scale_percent,
            expected,
            "{} is a {} board",
            board.name,
            if board.touch { "touch" } else { "button" }
        );
    }
}

/// A board carries both the factor and the chrome it produced, so the two can
/// disagree. This is what stops them, and it spells out the whole derivation:
/// the preset its panel calls for, at its own scale, without a hint band if it
/// has no keys to name.
#[test]
fn a_boards_chrome_is_its_own_preset_scaled() {
    for board in Board::ALL {
        let expected = Tokens::for_panel(board.width, board.height).scaled(board.ui_scale_percent);
        let expected = if board.touch {
            expected.without_button_hints()
        } else {
            expected.with_row(board.tokens.row)
        };

        assert_eq!(
            board.tokens, expected,
            "{}: its tokens are not the preset for its panel at its own scale",
            board.name
        );
    }
}

/// A device that takes Back and Confirm from its touchscreen has no row of keys
/// along the bottom, so a hint bar there names keys that do not exist.
///
/// The firmware's themes return before drawing one on exactly these boards.
#[test]
fn only_a_board_with_keys_reserves_a_hint_band() {
    for board in Board::ALL {
        if board.touch {
            assert_eq!(
                board.tokens.button_hints_height, 0,
                "{}: a touch board must not reserve a band for keys it lacks",
                board.name
            );
        } else {
            assert!(
                board.tokens.button_hints_height > 0,
                "{}: its keys need labelling",
                board.name
            );
        }
    }
}

/// The point of the scale, in the only units that matter. A touch board's row
/// has to be findable by a fingertip; the firmware's own note is that 3mm is
/// not.
#[test]
fn a_touch_boards_row_is_wider_than_a_fingertip() {
    for board in Board::ALL.into_iter().filter(|board| board.touch) {
        let tenths = board
            .tenths_of_a_mm(board.tokens.list_row_height)
            .expect("a measured panel");
        assert!(
            tenths >= 50,
            "{}: a row is {}.{}mm",
            board.name,
            tenths / 10,
            tenths % 10
        );
    }
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
                .button_labelled(label)
                .unwrap_or_else(|| panic!("{} has no {label} key", board.name))
        };

        assert!(
            key("Prev").centre.0 < left_edge,
            "{}: the previous-page key belongs on the left edge",
            board.name
        );
        assert!(
            key("Next").centre.0 > right_edge,
            "{}: the next-page key belongs on the right edge",
            board.name
        );
    }

    // The X4 is the other arrangement: both on one side.
    let bezel = Board::X4.bezel.expect("the X4 has a body");
    let up = bezel
        .button_labelled("Prev")
        .expect("the X4 has a previous key");
    let down = bezel
        .button_labelled("Next")
        .expect("the X4 has a next key");
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
        .button_labelled("Home")
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

/// A panel sits in the middle of what is left for it.
///
/// Not simply in the middle of the body: a badge carries both its side keys on
/// one edge, so its screen is pushed off-centre by exactly the column those
/// keys need. Asserting plain symmetry there would be asserting the device is
/// something other than it is.
#[test]
fn every_panel_is_centred_in_the_room_it_has() {
    for (board, bezel) in bezels() {
        let (x, _, width, _) = bezel.panel_rect();
        let (left, right) = (x, bezel.body.0 - (x + width));

        let (_, top, _, height) = bezel.panel_rect();
        // Only keys level with the screen take room from its margins. One below
        // it is in the footer and costs the sides nothing.
        let flanking = |on_the_right: bool| {
            bezel
                .buttons
                .iter()
                .filter(|key| key.centre.1 < top + height)
                .filter(|key| {
                    let beside = key.centre.0 < x || key.centre.0 > x + width;
                    beside && (key.centre.0 > x) == on_the_right
                })
                .map(|key| key.size.0)
                .max()
                .unwrap_or(0)
        };

        // What each side needs for the keys on it, and what is left over.
        let spare = |margin: i32, keys: i32| margin - keys;

        assert!(
            (spare(left, flanking(false)) - spare(right, flanking(true))).abs() <= 2,
            "{}: {left} on the left and {right} on the right, with {} and {} of \
             that taken by keys — the screen is not centred in what remains",
            board.name,
            flanking(false),
            flanking(true)
        );
    }
}

/// How many keys sit below the panel, which is how many hints there is room to
/// label.
fn bottom_keys(board: Board) -> u8 {
    let Some(bezel) = board.bezel else { return 4 };
    let (_, y, _, height) = bezel.panel_rect();

    bezel
        .buttons
        .iter()
        .filter(|key| key.centre.1 > y + height)
        .count() as u8
}

/// A board must not label more keys than it has.
///
/// Four hints over three keys is worse than none: every label after the first
/// sits over the wrong key, and the last names one that is not there.
///
/// The row is written by hand and the count is derived from where the bezel
/// actually puts the keys, so this is the two disagreeing rather than a
/// restatement of one of them.
#[test]
fn a_board_labels_only_the_keys_it_has() {
    for board in Board::ALL {
        if board.touch {
            continue;
        }
        assert_eq!(
            board.tokens.row.len(),
            bottom_keys(board) as usize,
            "{}: its row describes {} keys and its body has {}",
            board.name,
            board.tokens.row.len(),
            bottom_keys(board)
        );
    }
}

/// The hint painted over a key names the job that key actually does.
///
/// Two descriptions of the same thing sit in `crates/boards`: the row, which
/// is what the hint bar paints, and the footer keys, which are what pressing
/// one sends. Nothing makes them agree, and they did not — a change to the row
/// left the keys alone, so on both Pimoroni boards every label sat one key to
/// the left of what it named and no key anywhere sent `Back`. The suite was
/// green and a person found it by clicking.
///
/// This is the check that was missing. It walks every board rather than the
/// one someone happened to open.
#[test]
fn a_boards_keys_match_the_row_it_paints() {
    // Three `continue`s below, and a board added before its body is described
    // would slip past all of them. A test that skips everything passes.
    let mut checked = 0;

    for board in Board::ALL {
        // A board that takes Back and Confirm from a touchscreen paints no
        // hint band, so there is nothing to agree with.
        if board.touch || board.tokens.button_hints_height == 0 {
            continue;
        }
        let Some(bezel) = board.bezel else { continue };
        let keys = footer_of(bezel);

        assert_eq!(
            board.tokens.row.len(),
            keys.len(),
            "{}: its row describes {} keys and its footer has {}",
            board.name,
            board.tokens.row.len(),
            keys.len()
        );

        checked += 1;
        for (index, (row_key, key)) in board.tokens.row.iter().zip(&keys).enumerate() {
            assert_eq!(
                *row_key,
                names(key.action),
                "{}: slot {} paints {:?} over a key that does {:?}",
                board.name,
                index,
                row_key,
                key.action
            );
        }
    }

    // Five boards paint a hint bar: three readers, and the two badges. The X4
    // Pro and the Sticky take Back from a touchscreen and have no bar.
    assert_eq!(
        checked, 5,
        "the loop skipped a board it should have checked"
    );
}

/// Which hint names a key, given what pressing it sends.
///
/// `Previous` and `Next` are `Left` and `Right`: a reader's footer calls them
/// Up and Down because that is what they do to a list, and the pins are named
/// for the direction. Anything the hint vocabulary has no word for — a Power
/// key, an unassigned one — is [`RowKey::Unassigned`], which draws nothing.
fn names(action: KeyAction) -> RowKey {
    match action {
        KeyAction::Press(Button::Back) => RowKey::Back,
        KeyAction::Press(Button::Confirm) => RowKey::Confirm,
        KeyAction::Press(Button::Left) => RowKey::Previous,
        KeyAction::Press(Button::Right) => RowKey::Next,
        // Every remaining action, spelled out. A catch-all here reads the same
        // and lets half the fault through: a key that gains a job the bar has
        // no word for still maps to `Unassigned`, so the row keeps painting
        // nothing over a key that now does something, and this test — whose
        // whole purpose is to notice that — stays green.
        KeyAction::Press(Button::Up)
        | KeyAction::Press(Button::Down)
        | KeyAction::Press(Button::Power)
        | KeyAction::Press(Button::PageBack)
        | KeyAction::Press(Button::PageForward)
        | KeyAction::Press(Button::NavNext)
        | KeyAction::Press(Button::NavPrevious)
        | KeyAction::Press(Button::ScreenLeft)
        | KeyAction::Press(Button::ScreenRight)
        | KeyAction::Press(Button::ScreenUp)
        | KeyAction::Press(Button::ScreenDown)
        | KeyAction::Home
        | KeyAction::Unassigned => RowKey::Unassigned,
    }
}

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

/// A label no key carries finds nothing rather than the nearest key.
///
/// This is the firmware's silent-failure path: `Buttons::new` resolves each pin
/// through a label, and a name the board does not have leaves that switch
/// sampled and quiet. Nothing on a device would say so except the boot line, so
/// the branch is pinned here where it can be.
#[test]
fn a_label_no_key_carries_finds_nothing() {
    let bezel = Board::BADGER_2040.bezel.expect("the Badger has a body");

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

/// No board names the same job on two keys, and every board can be left.
///
/// A row is written by hand, and a duplicated entry is the kind of typo that
/// paints plausibly — two keys both labelled `Back`, one of which does
/// nothing. A row with no `Back` at all is legitimate (a three-key badge
/// spends its keys elsewhere and reaches Back by a double press), so that is
/// asserted separately where it is true.
#[test]
fn a_row_gives_each_job_to_at_most_one_key() {
    for board in Board::ALL {
        let mut seen: Vec<RowKey> = Vec::new();
        for key in board.tokens.row {
            if *key == RowKey::Unassigned {
                continue;
            }
            assert!(
                !seen.contains(key),
                "{}: {:?} is on two keys of the same row",
                board.name,
                key
            );
            seen.push(*key);
        }
    }
}

/// Every board with a bottom row keeps Back and Confirm reachable from it.
///
/// The five-key boards here all have a key to spare, so spending one on Back
/// costs nothing — and a board that can be entered but not left is the fault
/// this pins. A genuine three-key badge would be the exception, and there is
/// not one in `Board::ALL`; if one is added, this test is the conversation.
#[test]
fn every_row_can_be_entered_and_left() {
    for board in Board::ALL {
        if board.touch || board.tokens.button_hints_height == 0 {
            continue;
        }
        assert!(
            board.tokens.row.contains(&RowKey::Back),
            "{}: its row has no Back key",
            board.name
        );
        assert!(
            board.tokens.row.contains(&RowKey::Confirm),
            "{}: its row has no Confirm key",
            board.name
        );
    }
}

// -- the shapes the keys are in --------------------------------------------
//
// Every body here is described as a row along the footer and a column down an
// edge, with the centres derived rather than written down. These say what
// "derived" has to come out as, so a plan that spaced a run by hand — or by
// arithmetic that is subtly wrong — is caught the way the misplaced panels
// were.

use xpui_boards::PhysicalButton;

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

/// The shell between one key and the next, along a run.
///
/// Between the *edges* rather than between the centres: a sleep key is shorter
/// than the page keys beside it, so even centres would be uneven shell — and
/// the shell is what a thumb feels.
fn gaps(along: impl Iterator<Item = (i32, i32)>) -> Vec<i32> {
    let mut gaps = Vec::new();
    let mut previous: Option<i32> = None;
    for (centre, size) in along {
        if let Some(end) = previous {
            gaps.push(centre - size / 2 - end);
        }
        previous = Some(centre + size / 2);
    }
    gaps
}

/// A gap of one tenth of a millimetre either way, which is integer division
/// dividing a body by five rather than anything anybody would see.
fn evenly_spaced(gaps: &[i32]) -> bool {
    match (gaps.iter().min(), gaps.iter().max()) {
        (Some(least), Some(most)) => most - least <= 1,
        _ => true,
    }
}

/// A footer of three and a footer of five differ by a number, not by a table of
/// positions — so the shell between one key and the next is the same all the
/// way along, and the same at both ends of the row.
#[test]
fn a_row_of_keys_is_evenly_spaced() {
    let mut rows = 0;
    for (board, bezel) in bezels() {
        let keys = footer_of(bezel);
        if keys.len() < 2 {
            continue;
        }
        rows += 1;

        let between = gaps(keys.iter().map(|key| (key.centre.0, key.size.0)));
        assert!(
            evenly_spaced(&between),
            "{}: the keys along the footer are spaced {between:?}",
            board.name
        );

        let first = &keys[0];
        let last = &keys[keys.len() - 1];
        let (before, after) = (
            first.centre.0 - first.size.0 / 2,
            bezel.body.0 - (last.centre.0 + last.size.0 / 2),
        );
        assert!(
            (before - after).abs() <= 1,
            "{}: {before} of shell before the row and {after} after it — the \
             row is not centred on the body",
            board.name
        );

        for key in &keys {
            assert_eq!(
                key.size, first.size,
                "{}: {:?} is not the size of the rest of the row",
                board.name, key.label
            );
        }
    }
    assert!(rows >= 4, "only {rows} boards have a row to check");
}

/// The same, down an edge — and a column's keys share a width even when a sleep
/// key makes them differ in height.
#[test]
fn a_column_of_keys_is_evenly_spaced() {
    let mut columns = 0;
    for (board, bezel) in bezels() {
        for on_the_right in [false, true] {
            let keys = column_of(bezel, on_the_right);
            if keys.len() < 2 {
                continue;
            }
            columns += 1;

            let between = gaps(keys.iter().map(|key| (key.centre.1, key.size.1)));
            assert!(
                evenly_spaced(&between),
                "{}: the keys down its {} edge are spaced {between:?}",
                board.name,
                if on_the_right { "right" } else { "left" }
            );

            for key in &keys {
                assert_eq!(
                    key.size.0, keys[0].size.0,
                    "{}: {:?} is not the width of the rest of the column",
                    board.name, key.label
                );
                assert_eq!(
                    key.centre.0, keys[0].centre.0,
                    "{}: {:?} is out of the column",
                    board.name, key.label
                );
            }
        }
    }
    assert!(columns >= 4, "only {columns} boards have a column to check");
}

/// The board the builder was written for: five keys along the footer and
/// nothing down either edge, which no board here had before.
#[test]
fn the_inky_frame_is_a_footer_and_nothing_else() {
    let bezel = Board::INKY_FRAME.bezel.expect("the Inky Frame has a body");

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
    let derived = Board::INKY_FRAME.ppi().expect("a measured panel");

    assert!(
        (derived - from_pitch).abs() <= 2,
        "a 5.7\" diagonal derives {derived} ppi; a 0.1915mm pitch is {from_pitch}"
    );
}
