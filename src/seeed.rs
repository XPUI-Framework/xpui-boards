//! Seeed's Sticky.

use crate::{Bezel, Board, Orientation, PhysicalButton};
use xpui::Button;
use xpui_chrome::Tokens;

impl Board {
    /// Seeed Sticky — ESP32-S3, an 800x480 panel held portrait, so 480x800.
    ///
    /// A touchscreen with three keys beside it. Back and the directions come
    /// from touch; what is wired is a shared OK/Power key and a page pair.
    pub const STICKY: Board = Board {
        name: "Seeed Sticky",
        slug: "sticky",
        width: 480,
        height: 800,
        framebuffer: (800, 480),
        orientation: Orientation::Portrait,
        tokens: Tokens::DEFAULT,
        touch: true,
        refresh_ms: 1200,
        bezel: Some(STICKY_BEZEL),
    };
}

/// The Sticky's body: three keys in a column on the right.
///
/// **Estimated**, like the readers'. The count is not — the firmware wires
/// exactly three pins — and neither is what they do.
///
/// The top one is the shared OK and Power key: a click confirms, a hold of
/// four hundred milliseconds sleeps the device. It is one key doing two jobs
/// rather than a separate sleep button, which is why it is labelled for the
/// press rather than the hold.
///
/// Their side and order are the one thing here taken from the device rather
/// than from the code: the firmware wires the pins but says nothing about where
/// they sit, and its own themes model this pair as a stacked rocker instead.
pub const STICKY_BEZEL: Bezel = Bezel {
    body: (580, 1010),
    panel_origin: (55, 90),
    panel_size: (420, 700),
    buttons: &[
        PhysicalButton {
            label: "OK",
            button: Button::Confirm,
            centre: (545, 260),
            size: (50, 130),
        },
        PhysicalButton {
            label: "Up",
            button: Button::Up,
            centre: (545, 430),
            size: (50, 130),
        },
        PhysicalButton {
            label: "Dn",
            button: Button::Down,
            centre: (545, 590),
            size: (50, 130),
        },
    ],
    artwork: None,
};
