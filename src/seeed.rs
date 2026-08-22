//! Seeed's Sticky.

use crate::{Bezel, Board, Key, Orientation, PhysicalButton, Plan, Run};
use xpui::Button;
use xpui::host::KeyRow;
use xpui_chrome::{Labels, Metrics};

impl Board {
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
        metrics: Metrics::DEFAULT.scaled(120).without_button_hints(),
        labels: Labels::ENGLISH,
        keys: KeyRow::READER,
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
/// The top one is the shared OK and Power key, and it is shorter than the two
/// below it. Seeed call it the AI Voice key: a short press goes home or starts
/// voice input, and three seconds powers the device on. There is no separate
/// power key. CrossPoint puts confirm and power on that same pin — a click
/// confirms, four hundred milliseconds sleeps — so the label is what this
/// firmware does with it rather than what is printed beside it on the device.
///
/// Their side and order are the one thing here taken from the device rather
/// than from the code: the firmware wires the pins but says nothing about where
/// they sit, and its own themes model this pair as a stacked rocker instead.
const STICKY: Plan = Plan::new((560, 1010), (450, 750), 95).right(Run::new(
    (44, 140),
    &[
        Key::new("OK", Button::Confirm).spanning(120),
        Key::new("Prev", Button::PageBack),
        Key::new("Next", Button::PageForward),
    ],
));

const STICKY_KEYS: [PhysicalButton; STICKY.count()] = STICKY.keys();
pub const STICKY_BEZEL: Bezel = STICKY.bezel(&STICKY_KEYS);
