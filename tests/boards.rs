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
