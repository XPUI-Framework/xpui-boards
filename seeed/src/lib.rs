//! Seeed's Sticky, described for `xpui`.
//!
//! One board: a 480x800 e-ink panel at 234 ppi with a touchscreen and three
//! keys beside it. It is the X4's framebuffer in a smaller body — 3.97"
//! against 4.26" — so every pixel here is about 7% smaller than the same
//! pixel on an X4, which is why it asks for a larger UI scale despite the
//! identical resolution. Back and the directions come from touch, so the
//! three wired keys are a shared OK/Power and a page pair.
//!
//! The panel is driven by the firmware that ships on it, and a screen reaches
//! it by that firmware hosting `xpui` over the C ABI — `xpui-cpp` is that
//! boundary. `xpui-esp32` carries a bare-metal image for the Sticky that
//! builds and links, with `Panel::present` marked where a driver would go;
//! nothing has been run on the board. One vendor, one crate.

#![cfg_attr(target_os = "none", no_std)]
#![deny(missing_docs)]

use xpui::Button;
use xpui::host::KeyRow;
use xpui_boards_core::{Bezel, Board, Key, Orientation, PhysicalButton, Plan, Run};

/// Seeed Sticky — ESP32-S3, an 800x480 panel held portrait, so 480x800.
///
/// A touchscreen with three keys beside it. Back and the directions come
/// from touch; what is wired is a shared OK/Power key and a page pair.
///
/// 234 ppi against the X4's 218 for the same framebuffer; the touch scale its
/// firmware profile gives it puts a row at 5.2mm.
pub const STICKY: Board = Board {
    name: "Seeed Sticky",
    slug: "sticky",
    width: 480,
    height: 800,
    framebuffer: (800, 480),
    orientation: Orientation::Portrait,
    diagonal_hundredths_inch: Some(397),
    ui_scale_percent: 120,
    // No row along the bottom: its three keys are a column on the right.
    keys: KeyRow::new(&[]),
    touch: true,
    refresh_ms: 1200,
    bezel: Some(STICKY_BEZEL),
};

/// The Sticky's body: three keys in a column on the right.
///
/// **The surround is estimated**, like the readers': the glass is what the
/// diagonal makes it, 51.9 x 86.5 mm. The count is not — the firmware wires
/// exactly three pins — and neither is what they do.
///
/// The top one is the shared OK and Power key, shorter than the two below
/// it: Seeed call it the AI Voice key, and there is no separate power key.
/// The firmware puts confirm and power on that pin — a click confirms, four
/// hundred milliseconds sleeps — so the label is what the firmware does with
/// it rather than what is printed beside it. Their side and order come from
/// the device; the firmware wires the pins but says nothing about where they
/// sit.
const STICKY_PLAN: Plan = Plan::new((629, 1125), (519, 865), 95).right(Run::new(
    (44, 140),
    &[
        Key::new("OK", Button::Confirm).spanning(120),
        Key::new("Prev", Button::PageBack),
        Key::new("Next", Button::PageForward),
    ],
));

const STICKY_KEYS: [PhysicalButton; STICKY_PLAN.count()] = STICKY_PLAN.keys();
/// The Sticky's body with its keys placed, for a simulator to draw.
pub const STICKY_BEZEL: Bezel = STICKY_PLAN.bezel(&STICKY_KEYS);

/// This vendor's boards, so a caller can offer them without a table of its
/// own that would fall behind this one.
///
/// One vendor's list, not the framework's — there is no such thing. An
/// application that ships against more than one concatenates them; see
/// `xpui-gallery`'s `gallery/src/boards.rs`.
pub const ALL: [Board; 1] = [STICKY];

/// Looks one of this vendor's boards up by its short name, for a command line.
///
/// `None` for a slug this vendor does not own, including one another vendor
/// does — a caller offering several asks each in turn. No aliases: nobody
/// shortens these names.
pub fn from_slug(slug: &str) -> Option<Board> {
    ALL.into_iter().find(|board| board.slug == slug)
}

/// The crate's prose, compiled: a page that does not build is worse than
/// none.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
}
