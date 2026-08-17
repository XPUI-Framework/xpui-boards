//! Xteink's e-readers.
//!
//! Geometry and capabilities read from the CrossPoint simulator's own
//! `BoardConfig.h`, which is what the firmware compiles against.

use crate::{Bezel, Board, PhysicalButton};
use xpui::Button;
use xpui_chrome::Tokens;

impl Board {
    /// Xteink X3 — ESP32-C3, 792x528 monochrome e-ink, landscape.
    ///
    /// No touchscreen: `FREEINK_CAP_TOUCH` is 0 for this profile. Navigation is
    /// the side buttons and the four front ones, which is why a screen that
    /// only offers a drag has nothing to offer here.
    pub const X3: Board = Board {
        name: "Xteink X3",
        slug: "x3",
        width: 792,
        height: 528,
        tokens: Tokens::DEFAULT,
        touch: false,
        refresh_ms: 1200,
        bezel: Some(X3_BEZEL),
    };

    /// Xteink X4 — ESP32-C3, 800x480 monochrome e-ink, landscape.
    ///
    /// The default profile, and no touchscreen either. Shares a panel size with
    /// the Sticky and differs in exactly that, which is why identity is a slug
    /// rather than a pair of dimensions.
    pub const X4: Board = Board {
        name: "Xteink X4",
        slug: "x4",
        width: 800,
        height: 480,
        tokens: Tokens::DEFAULT,
        touch: false,
        refresh_ms: 1200,
        bezel: None,
    };
}

/// The X3's body, with its side buttons.
///
/// **Every dimension here is an estimate.** Xteink publish no mechanical
/// drawing, so the body is derived from the panel: a 792x528 e-ink panel at
/// roughly 150 dpi is about 134 x 89 mm of active area, and the surrounding
/// body is scaled from product photographs. Replace these with measurements
/// from the hardware when someone has one to hand.
///
/// The arrangement is what matters and is not in doubt: Up and Down are on the
/// **side**, under the thumb of the hand holding it, and the four front buttons
/// run along the bottom edge below the panel.
pub const X3_BEZEL: Bezel = Bezel {
    body: (1560, 1090),
    panel_origin: (100, 80),
    panel_size: (1320, 890),
    buttons: &[
        PhysicalButton {
            label: "Up",
            button: Button::Up,
            centre: (1490, 380),
            size: (90, 190),
        },
        PhysicalButton {
            label: "Dn",
            button: Button::Down,
            centre: (1490, 640),
            size: (90, 190),
        },
        PhysicalButton {
            label: "Back",
            button: Button::Back,
            centre: (300, 1030),
            size: (200, 70),
        },
        PhysicalButton {
            label: "OK",
            button: Button::Confirm,
            centre: (620, 1030),
            size: (200, 70),
        },
        PhysicalButton {
            label: "Prev",
            button: Button::PageBack,
            centre: (940, 1030),
            size: (200, 70),
        },
        PhysicalButton {
            label: "Next",
            button: Button::PageForward,
            centre: (1260, 1030),
            size: (200, 70),
        },
    ],
    artwork: None,
};
