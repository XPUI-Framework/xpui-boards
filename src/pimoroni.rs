//! Pimoroni's RP2040 boards.

use crate::{Bezel, Board, Key, Orientation, PhysicalButton, Plan, Run};
use xpui::Button;
use xpui_chrome::{RowKey, Tokens};

/// A, B and C along the bottom edge, with a dedicated up/down pair elsewhere
/// on the board — so unlike a three-key badge, there *is* a key for Back, and
/// it is the first one. C has nothing on it; giving it a job is one entry
/// here and one line in the firmware's `Buttons::new`.
const BADGE_ROW: &[RowKey] = &[RowKey::Back, RowKey::Confirm, RowKey::Unassigned];

/// Five keys along the bottom and nothing down either edge, so the pair that
/// walks the list lives in the row too. The fifth has nothing on it.
const INKY_ROW: &[RowKey] = &[
    RowKey::Back,
    RowKey::Confirm,
    RowKey::Previous,
    RowKey::Next,
    RowKey::Unassigned,
];

impl Board {
    /// Pimoroni Badger 2040 — RP2040, 296x128 monochrome e-ink (UC8151).
    ///
    /// The panel that made small-panel tokens necessary: the default chrome
    /// leaves 28 pixels of content here, which is not enough for one list row.
    ///
    /// Buttons only, so the baseline scale — and at 111 ppi it needs no help:
    /// a 24px row on this strip is 5.5mm, wider than a 40px row on a reader.
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
        tokens: Tokens::SMALL.with_row(BADGE_ROW),
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
        tokens: Tokens::COMPACT.with_row(BADGE_ROW),
        touch: false,
        refresh_ms: 0,
        bezel: Some(TUFTY_BEZEL),
    };

    /// Pimoroni Inky Frame 5.7" — RP2040 with a Pico W aboard, 600x448
    /// seven-colour e-ink (E Ink Gallery Palette 4000).
    ///
    /// The first board here whose keys are only a footer: five of them, A to E,
    /// and nothing down either edge. Buttons only, so the baseline scale, and
    /// at 131 ppi a 40px row is 7.7mm — the roomiest chrome of the seven.
    ///
    /// Five keys, and the standard row names four of them, so the last is
    /// [`RowKey::Unassigned`] rather than shifting every label one key left.
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
        tokens: Tokens::DEFAULT.with_row(INKY_ROW),
        touch: false,
        // **A documented estimate.** Pimoroni quote "about 30 seconds"; a
        // seven-colour panel cycles through each colour, and the figure people
        // report runs from twenty to forty. Two orders of magnitude slower than
        // the readers, which is the fact a screen would need to know.
        refresh_ms: 30_000,
        bezel: Some(INKY_FRAME_BEZEL),
    };
}

/// The Badger's body and its five front buttons.
///
/// Body from Pimoroni's published dimensions: 85.6 x 48.7 mm, the size of a
/// credit card. The panel is a 2.9 inch strip roughly 66.9 x 29.1 mm of active
/// area, centred across the width and sitting above the button row.
///
/// Button placement is **estimated** from product photographs: A, B and C run
/// along the bottom edge, with up and down stacked on the right-hand side.
/// Pimoroni publish the GPIO map but not the millimetre positions.
///
/// `a` goes back and `b` confirms, matching the row painted above them and the
/// firmware that reads the pins. `c` has nothing on it yet.
const BADGER: Plan = Plan::new((856, 487), (669, 291), 60)
    .footer(Run::new((110, 60), &BADGE_FOOTER))
    .right(Run::new((60, 60), &BADGE_EDGE));

const BADGER_KEYS: [PhysicalButton; BADGER.count()] = BADGER.keys();
pub const BADGER_BEZEL: Bezel = BADGER.bezel(&BADGER_KEYS);

/// a, b and c along the footer, as the silkscreen has them, on both badges.
///
/// What each sends has to match [`BADGE_ROW`], which is what the hint bar
/// paints over them, and `Buttons::new` in the rp2040 firmware, which is what
/// the hardware sends. They disagreed once, and every label sat one key off
/// what it named.
///
/// `a_boards_keys_match_the_row_it_paints` holds this and the row together.
/// **Nothing holds the firmware to either** — it compiles only for the board,
/// so no host test parses it. Closing that is spec 32.
const BADGE_FOOTER: [Key; 3] = [
    Key::new("a", Button::Back),
    Key::new("b", Button::Confirm),
    Key::unassigned("c"),
];

/// Up and down beside the screen, stacked on the right edge.
const BADGE_EDGE: [Key; 2] = [Key::new("Up", Button::Up), Key::new("Dn", Button::Down)];

/// The Tufty's body and its five buttons.
///
/// Body from Pimoroni's published dimensions: 65.2 x 52.7 mm. The panel is a
/// 2.4 inch display of roughly 48.9 x 36.7 mm active area.
///
/// Button placement is **estimated** from product photographs, as for the
/// Badger: A, B and C along the bottom, up and down on the right edge.
const TUFTY: Plan = Plan::new((652, 527), (489, 367), 60)
    .footer(Run::new((95, 55), &BADGE_FOOTER))
    .right(Run::new((55, 55), &BADGE_EDGE));

const TUFTY_KEYS: [PhysicalButton; TUFTY.count()] = TUFTY.keys();
pub const TUFTY_BEZEL: Bezel = TUFTY.bezel(&TUFTY_KEYS);

/// The Inky Frame's body: five keys along the footer, and nothing on the edges.
///
/// Body from Pimoroni's published dimensions: 131.4 x 127.5 mm. The panel is
/// 600x448 pixels at the 0.1915mm dot pitch they quote, which is 114.9 x 85.8
/// mm of active area — so 8.3mm of bezel each side, and the rest of the board
/// below the panel where the buttons and the Pico W sit.
///
/// **The forehead and the button size are estimated** from product
/// photographs, as the badges' are: Pimoroni publish the board's outline and
/// the dot pitch but not where the buttons sit on it. 8.3mm above the panel is
/// the side bezel repeated, which leaves a 33mm chin.
///
/// The five keys are not estimated — Pimoroni's own module exposes
/// `button_a` through `button_e` — and neither is the shape: they are a row
/// along the footer with no edge keys at all, which is the whole reason this
/// board is here. Five centres, and not one of them written down.
const INKY_FRAME: Plan = Plan::new((1314, 1275), (1149, 858), 83).footer(Run::new(
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

const INKY_FRAME_KEYS: [PhysicalButton; INKY_FRAME.count()] = INKY_FRAME.keys();
pub const INKY_FRAME_BEZEL: Bezel = INKY_FRAME.bezel(&INKY_FRAME_KEYS);
