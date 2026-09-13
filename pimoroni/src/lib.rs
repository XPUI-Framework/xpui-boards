//! Pimoroni's RP2040 boards, described for `xpui`: the Badger 2040, the
//! Tufty 2040 and the Inky Frame.
//!
//! Each is a `const` [`Board`]: its panel, key row, refresh time and body in
//! tenths of a millimetre. The two badges share a key row of three — Back,
//! Confirm, and one with no job — because the pair that walks a list sits on
//! the edge rather than in the row a hint bar labels; the Inky Frame carries
//! five along the bottom, and its fifth takes the job that needs no label,
//! Power.
//!
//! The keys are not decoration. `xpui-rp2040` resolves a real GPIO pin by
//! looking a key up **by its label** in the bezel here, so a label changed
//! here changes which switch does what on the hardware. The Badger and the
//! Tufty have been run over a debug probe; the Inky Frame is described so a
//! screen can be laid out for it in the simulator, and has not been built.
//!
//! One vendor, one crate: taking these three costs no Xteink and no Seeed.

#![cfg_attr(target_os = "none", no_std)]
#![deny(missing_docs)]

use xpui::Button;
use xpui::host::{KeyRow, RowKey};
use xpui_boards_core::{Bezel, Board, Key, Orientation, PhysicalButton, Plan, Run};

/// A, B and C along the bottom edge, with an up/down pair elsewhere on the
/// board — so there *is* a key for Back, and it is the first. C has nothing
/// on it; a job for it is this entry, its twin in [`BADGE_FOOTER`], and the
/// test that pins both.
const BADGE_ROW: KeyRow = KeyRow::new(&[RowKey::Back, RowKey::Confirm, RowKey::Unassigned]);

/// Five keys along the bottom and nothing down either edge, so the pair that
/// walks the list lives in the row too. The fifth has nothing to label.
const INKY_ROW: KeyRow = KeyRow::new(&[
    RowKey::Back,
    RowKey::Confirm,
    RowKey::Previous,
    RowKey::Next,
    RowKey::Unassigned,
]);

/// Pimoroni Badger 2040 — RP2040, 296x128 monochrome e-ink (UC8151).
///
/// Buttons only, so the baseline scale; at 111 ppi a 24px row on this strip
/// is 5.5mm, wider than a 40px row on a reader.
pub const BADGER_2040: Board = Board {
    name: "Badger 2040",
    slug: "badger2040",
    width: 296,
    height: 128,
    framebuffer: (296, 128),
    orientation: Orientation::Landscape,
    // Pimoroni's 2.9" UC8151 strip.
    diagonal_hundredths_inch: Some(290),
    ui_scale_percent: 100,
    keys: BADGE_ROW,
    touch: false,
    // A full UC8151 update is close to a second; the partial modes are
    // faster but still nothing you would drive an animation with.
    refresh_ms: 900,
    bezel: Some(BADGER_BEZEL),
};

/// Pimoroni Tufty 2040 — RP2040, 320x240 colour IPS LCD (ST7789v).
///
/// Colour hardware running a monochrome framework: the backend maps ink and
/// background onto any two `Rgb565` values, so the same screens render
/// black-on-white, or amber-on-black, without a screen knowing.
///
/// Buttons only, so the baseline scale. 166 ppi puts its 30px row at
/// 4.6mm.
pub const TUFTY_2040: Board = Board {
    name: "Tufty 2040",
    slug: "tufty2040",
    width: 320,
    height: 240,
    framebuffer: (320, 240),
    orientation: Orientation::Landscape,
    // Pimoroni's 2.4" ST7789v.
    diagonal_hundredths_inch: Some(240),
    ui_scale_percent: 100,
    keys: BADGE_ROW,
    touch: false,
    refresh_ms: 0,
    bezel: Some(TUFTY_BEZEL),
};

/// Pimoroni Inky Frame 5.7" — RP2040 with a Pico W aboard, 600x448
/// seven-colour e-ink (E Ink Gallery Palette 4000).
///
/// Keys only along the footer: five, A to E, and nothing down either edge.
/// Buttons only, so the baseline scale, and at 131 ppi a 40px row is 7.7mm.
/// The standard row names four, so the fifth is [`RowKey::Unassigned`] rather
/// than shifting every label one key left.
pub const INKY_FRAME: Board = Board {
    name: "Inky Frame 5.7\"",
    slug: "inkyframe",
    width: 600,
    height: 448,
    framebuffer: (600, 448),
    orientation: Orientation::Landscape,
    // The size Pimoroni sell it as. Their published 0.1915mm dot pitch puts
    // the active area's own diagonal a shade under that, at 5.65".
    diagonal_hundredths_inch: Some(570),
    ui_scale_percent: 100,
    keys: INKY_ROW,
    touch: false,
    // **A documented estimate.** Pimoroni quote "about 30 seconds"; a
    // seven-colour panel cycles through each colour, and the figure people
    // report runs from twenty to forty. Two orders of magnitude slower than
    // the readers, which is the fact a screen would need to know.
    refresh_ms: 30_000,
    bezel: Some(INKY_FRAME_BEZEL),
};

/// The Badger's body and its five front buttons.
///
/// Body from Pimoroni's published dimensions: 85.6 x 48.7 mm, the size of a
/// credit card. The panel is a 2.9 inch strip roughly 66.9 x 29.1 mm of active
/// area, sitting above the button row in what the edge column leaves.
///
/// Button placement is **estimated** from product photographs: A, B and C run
/// along the bottom edge, with up and down stacked on the right-hand side.
/// Pimoroni publish the GPIO map but not the millimetre positions.
///
/// `a` goes back and `b` confirms, matching the row painted above them and the
/// firmware that reads the pins. `c` has nothing on it.
const BADGER_PLAN: Plan = Plan::new((856, 487), (669, 291), 60)
    .footer(Run::new((110, 60), &BADGE_FOOTER))
    .right(Run::new((60, 60), &BADGE_EDGE));

const BADGER_KEYS: [PhysicalButton; BADGER_PLAN.count()] = BADGER_PLAN.keys();
/// The Badger 2040's body with its keys placed, for a simulator to draw and a
/// firmware to resolve pins from.
pub const BADGER_BEZEL: Bezel = BADGER_PLAN.bezel(&BADGER_KEYS);

/// a, b and c along the footer, as the silkscreen has them, on both badges.
/// a goes back, b confirms and c is left bare.
///
/// **The names are what the hardware sends.** The rp2040 firmware looks each
/// pin up by these labels and the two in [`BADGE_EDGE`]: rename one and that
/// switch goes quiet on a real board. What each sends must match
/// [`BADGE_ROW`], which the hint bar paints over them; a test holds the two
/// together.
const BADGE_FOOTER: [Key; 3] = [
    Key::new("a", Button::Back),
    Key::new("b", Button::Confirm),
    Key::unassigned("c"),
];

/// Up and down beside the screen, stacked on the right edge. `Dn` because the
/// board prints an arrow and the firmware looks the key up by name — see
/// [`BADGE_FOOTER`]: spelling it `Down` compiles and leaves that switch dead
/// on hardware.
const BADGE_EDGE: [Key; 2] = [Key::new("Up", Button::Up), Key::new("Dn", Button::Down)];

/// The Tufty's body and its five buttons.
///
/// Body from Pimoroni's published dimensions: 65.2 x 52.7 mm. The panel is a
/// 2.4 inch display of roughly 48.9 x 36.7 mm active area.
///
/// Button placement is **estimated** from product photographs, as for the
/// Badger: A, B and C along the bottom, up and down on the right edge.
const TUFTY_PLAN: Plan = Plan::new((652, 527), (489, 367), 60)
    .footer(Run::new((95, 55), &BADGE_FOOTER))
    .right(Run::new((55, 55), &BADGE_EDGE));

const TUFTY_KEYS: [PhysicalButton; TUFTY_PLAN.count()] = TUFTY_PLAN.keys();
/// The Tufty 2040's body with its keys placed, for a simulator to draw and a
/// firmware to resolve pins from.
pub const TUFTY_BEZEL: Bezel = TUFTY_PLAN.bezel(&TUFTY_KEYS);

/// The Inky Frame's body: five keys along the footer, and nothing on the edges.
///
/// Body from Pimoroni's published dimensions: 131.4 x 127.5 mm. The panel is
/// 600x448 pixels at the 0.1915mm dot pitch they quote, which is 114.9 x 85.8
/// mm of active area — so 8.3mm of bezel each side, and the rest of the board
/// below the panel where the buttons and the Pico W sit.
///
/// **The forehead and the button size are estimated** from product
/// photographs, as the badges' are: Pimoroni publish the outline and the dot
/// pitch but not where the buttons sit. 8.3mm above the panel is the side
/// bezel repeated, which leaves a 33mm chin. The five keys and their shape —
/// a row along the footer, no edge keys — are not estimated: Pimoroni's own
/// module exposes `button_a` through `button_e`.
const INKY_FRAME_PLAN: Plan = Plan::new((1314, 1275), (1149, 858), 83).footer(Run::new(
    (60, 60),
    &[
        Key::new("A", Button::Back),
        Key::new("B", Button::Confirm),
        Key::new("C", Button::Left),
        Key::new("D", Button::Right),
        // The fifth key is the one a hint bar has no label for, so it takes the
        // job that needs none: these frames sleep between refreshes and wake on
        // a press.
        Key::new("E", Button::Power),
    ],
));

const INKY_FRAME_KEYS: [PhysicalButton; INKY_FRAME_PLAN.count()] = INKY_FRAME_PLAN.keys();
/// The Inky Frame's body with its keys placed, for a simulator to draw and a
/// firmware to resolve pins from.
pub const INKY_FRAME_BEZEL: Bezel = INKY_FRAME_PLAN.bezel(&INKY_FRAME_KEYS);

/// This vendor's 3 boards, so a caller can offer them without a table of
/// its own that would fall behind this one.
///
/// One vendor's list, not the framework's — there is no such thing. An
/// application that ships against more than one concatenates them; see
/// `xpui-gallery`'s `gallery/src/boards.rs`.
pub const ALL: [Board; 3] = [BADGER_2040, TUFTY_2040, INKY_FRAME];

/// Looks one of this vendor's boards up by its short name, for a command line.
///
/// `None` for a slug this vendor does not own, including one another vendor
/// does — a caller offering several asks each in turn.
///
/// The aliases exist because `badger` and `tufty` are what people say.
pub fn from_slug(slug: &str) -> Option<Board> {
    ALL.into_iter()
        .find(|board| board.slug == slug)
        .or(match slug {
            "badger" => Some(BADGER_2040),
            "tufty" => Some(TUFTY_2040),
            _ => None,
        })
}

/// The crate's prose, compiled: a page that does not build is worse than
/// none.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
}
