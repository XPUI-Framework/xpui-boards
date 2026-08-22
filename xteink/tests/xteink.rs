//! What Xteink's own boards promise.
//!
//! Only this vendor's. Anything comparing one vendor's board to another's is
//! in the conformance suite, `examples/gallery/tests/boards.rs`, because there
//! is no crate below the caller that knows more than one vendor.

use xpui_boards_xteink as xteink;

/// The X3 and the X4 Pro put one page key on each side edge; the X4 stacks
/// both on the same side.
///
/// That difference is not cosmetic — the firmware branches on it, in
/// `HalGPIO::hasEdgeSideButtons` and in every theme that draws a hint beside a
/// key. A simulator that got it wrong would teach the wrong muscle memory.
#[test]
fn the_page_keys_sit_where_the_firmware_says() {
    for board in [xteink::X3, xteink::X4_PRO] {
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
    let bezel = xteink::X4.bezel.expect("the X4 has a body");
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
    let bezel = xteink::X4_PRO.bezel.expect("the X4 Pro has a body");
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

/// The X4 and the X4 Pro share a panel and differ in what is on top of it.
#[test]
fn the_x4_pro_is_the_x4_with_a_touchscreen() {
    assert_eq!(
        (xteink::X4.width, xteink::X4.height),
        (xteink::X4_PRO.width, xteink::X4_PRO.height)
    );
    assert_eq!(xteink::X4.framebuffer, xteink::X4_PRO.framebuffer);
    const _: () = assert!(!xteink::X4.touch && xteink::X4_PRO.touch);
    assert_ne!(xteink::X4.slug, xteink::X4_PRO.slug);
}
