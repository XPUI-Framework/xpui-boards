//! Seeed's Sticky.

use crate::Board;
use xpui_chrome::Tokens;

impl Board {
    /// Seeed Sticky — ESP32-S3, 800x480 monochrome e-ink (SSD1677).
    ///
    /// The one board here with a touchscreen, so it is the only one on which a
    /// tap-only control is reachable. Same panel size as the X4 and a different
    /// controller; the difference that matters to a screen is the touch.
    pub const STICKY: Board = Board {
        name: "Seeed Sticky",
        slug: "sticky",
        width: 800,
        height: 480,
        tokens: Tokens::DEFAULT,
        touch: true,
        refresh_ms: 1200,
    };
}
