//! Pimoroni's RP2040 badges.

use crate::{Bezel, Board, KeyAction, Orientation, PhysicalButton};
use xpui::Button;
use xpui_chrome::Tokens;

impl Board {
    /// Pimoroni Badger 2040 — RP2040, 296x128 monochrome e-ink (UC8151).
    ///
    /// The panel that made small-panel tokens necessary: the default chrome
    /// leaves 28 pixels of content here, which is not enough for one list row.
    pub const BADGER_2040: Board = Board {
        name: "Badger 2040",
        slug: "badger2040",
        width: 296,
        height: 128,
        framebuffer: (296, 128),
        orientation: Orientation::Landscape,
        tokens: Tokens::SMALL,
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
    pub const TUFTY_2040: Board = Board {
        name: "Tufty 2040",
        slug: "tufty2040",
        width: 320,
        height: 240,
        framebuffer: (320, 240),
        orientation: Orientation::Landscape,
        tokens: Tokens::COMPACT,
        touch: false,
        refresh_ms: 0,
        bezel: Some(TUFTY_BEZEL),
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
pub const BADGER_BEZEL: Bezel = Bezel {
    body: (856, 487),
    panel_origin: (94, 60),
    panel_size: (669, 291),
    buttons: &[
        PhysicalButton {
            label: "A",
            action: KeyAction::Press(Button::Back),
            centre: (150, 430),
            size: (90, 60),
        },
        PhysicalButton {
            label: "B",
            action: KeyAction::Press(Button::Confirm),
            centre: (300, 430),
            size: (90, 60),
        },
        PhysicalButton {
            label: "C",
            action: KeyAction::Press(Button::PageForward),
            centre: (450, 430),
            size: (90, 60),
        },
        PhysicalButton {
            label: "Up",
            action: KeyAction::Press(Button::Up),
            centre: (800, 150),
            size: (70, 60),
        },
        PhysicalButton {
            label: "Dn",
            action: KeyAction::Press(Button::Down),
            centre: (800, 260),
            size: (70, 60),
        },
    ],
    artwork: None,
};

/// The Tufty's body and its five buttons.
///
/// Body from Pimoroni's published dimensions: 65.2 x 52.7 mm. The panel is a
/// 2.4 inch display of roughly 48.9 x 36.7 mm active area.
///
/// Button placement is **estimated** from product photographs, as for the
/// Badger: A, B and C along the bottom, up and down on the right edge.
pub const TUFTY_BEZEL: Bezel = Bezel {
    body: (652, 527),
    panel_origin: (81, 60),
    panel_size: (489, 367),
    buttons: &[
        PhysicalButton {
            label: "A",
            action: KeyAction::Press(Button::Back),
            centre: (130, 470),
            size: (80, 55),
        },
        PhysicalButton {
            label: "B",
            action: KeyAction::Press(Button::Confirm),
            centre: (250, 470),
            size: (80, 55),
        },
        PhysicalButton {
            label: "C",
            action: KeyAction::Press(Button::PageForward),
            centre: (370, 470),
            size: (80, 55),
        },
        PhysicalButton {
            label: "Up",
            action: KeyAction::Press(Button::Up),
            centre: (600, 160),
            size: (60, 55),
        },
        PhysicalButton {
            label: "Dn",
            action: KeyAction::Press(Button::Down),
            centre: (600, 270),
            size: (60, 55),
        },
    ],
    artwork: None,
};
