//! Pimoroni's RP2040 badges.

use crate::Board;
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
        tokens: Tokens::SMALL,
        touch: false,
        // A full UC8151 update is close to a second; the partial modes are
        // faster but still nothing you would drive an animation with.
        refresh_ms: 900,
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
        tokens: Tokens::COMPACT,
        touch: false,
        refresh_ms: 0,
    };
}
