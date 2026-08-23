//! What Seeed's own boards promise.
//!
//! Only this vendor's. Anything comparing one vendor's board to another's is
//! in the conformance suite, `xpui-gallery`'s `gallery/tests/boards.rs`, because there
//! is no crate below the caller that knows more than one vendor.

use xpui_boards_seeed as seeed;

/// The Sticky's three keys are a column, not a row.
#[test]
fn the_sticky_stacks_its_three_keys() {
    let bezel = seeed::STICKY.bezel.expect("the Sticky has a body");
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
