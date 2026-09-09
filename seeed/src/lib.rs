//! Seeed's Sticky.

#![cfg_attr(target_os = "none", no_std)]

use xpui::Button;
use xpui::host::KeyRow;
use xpui_boards_core::{Bezel, Board, Key, Orientation, PhysicalButton, Plan, Run};

/// Seeed Sticky — ESP32-S3, an 800x480 panel held portrait, so 480x800.
///
/// A touchscreen with three keys beside it. Back and the directions come
/// from touch; what is wired is a shared OK/Power key and a page pair.
///
/// The X4's framebuffer in a smaller body — 3.97" against 4.26", so 234
/// ppi against 218, and every pixel here is 7% smaller than the same pixel
/// on an X4. A touch device, so it takes the scale its firmware profile
/// gives it, which puts a row at 5.2mm.
pub const STICKY: Board = Board {
    name: "Seeed Sticky",
    slug: "sticky",
    width: 480,
    height: 800,
    framebuffer: (800, 480),
    orientation: Orientation::Portrait,
    diagonal_hundredths_inch: Some(397),
    ui_scale_percent: 120,
    keys: KeyRow::READER,
    touch: true,
    refresh_ms: 1200,
    bezel: Some(STICKY_BEZEL),
};

/// The Sticky's body: three keys in a column on the right.
///
/// **Estimated**, like the readers'. The count is not — the firmware wires
/// exactly three pins — and neither is what they do.
///
/// The top one is the shared OK and Power key, shorter than the two below
/// it: Seeed call it the AI Voice key, and there is no separate power key.
/// The firmware puts confirm and power on that pin — a click confirms, four
/// hundred milliseconds sleeps — so the label is what the firmware does with
/// it rather than what is printed beside it. Their side and order come from
/// the device; the firmware wires the pins but says nothing about where they
/// sit.
const STICKY_PLAN: Plan = Plan::new((560, 1010), (450, 750), 95).right(Run::new(
    (44, 140),
    &[
        Key::new("OK", Button::Confirm).spanning(120),
        Key::new("Prev", Button::PageBack),
        Key::new("Next", Button::PageForward),
    ],
));

const STICKY_KEYS: [PhysicalButton; STICKY_PLAN.count()] = STICKY_PLAN.keys();
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

/// The crate's prose, compiled: a README that does not build is worse than
/// none.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
}
