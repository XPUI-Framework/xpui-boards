//! Xteink's e-readers.
//!
//! Geometry and capabilities read from the CrossPoint simulator's own
//! `BoardConfig.h`, which is what the firmware compiles against.

use crate::Board;
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
    };
}
