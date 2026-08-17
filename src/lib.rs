//! The panels this framework has been run on, as data.
//!
//! A screen never knows which board it is on. What differs is a handful of
//! numbers — how big the panel is, which token preset fits it, whether there is
//! a touchscreen — and those are worth writing down once rather than
//! rediscovering per project.
//!
//! The simulator and a real firmware read the same value, which is what makes
//! "develop in a window, then flash it" true rather than aspirational. Nothing
//! here touches hardware: it is a description, not a driver.
//!
//! Separate from the chrome crate because a panel size is not a drawing
//! concern. Anything that wants to know how big a screen is can depend on this
//! without pulling in code that paints.

#![cfg_attr(target_os = "none", no_std)]

mod bezel;
mod pimoroni;
mod seeed;
mod xteink;

pub use bezel::{Bezel, PhysicalButton};
pub use xpui_chrome::Tokens;

/// A panel, its chrome, and what it can be driven with.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Board {
    /// What to call it, for a window title or a log line.
    pub name: &'static str,
    /// The short name a command line accepts. Stored rather than derived: the
    /// X4 and the Sticky are both 800x480, so a size cannot tell them apart.
    pub slug: &'static str,
    pub width: i32,
    pub height: i32,
    /// The chrome sized for this panel.
    pub tokens: Tokens,
    /// Whether a finger can reach it. A board with buttons and no touchscreen
    /// should not have its layout widened to finger-sized targets, and a screen
    /// can ask before offering a drag-only control.
    pub touch: bool,
    /// Roughly how long a full refresh takes, in milliseconds.
    ///
    /// E-ink is the reason the framework repaints only when something changed.
    /// A board that answers 0 is a display fast enough not to care.
    pub refresh_ms: u32,
    /// The body around the panel, when one has been described.
    ///
    /// `None` means the simulator opens a window that is exactly the panel, as
    /// it always did. Bezels arrive one device at a time rather than all five
    /// at once.
    pub bezel: Option<Bezel>,
}

impl Board {
    /// Every board, so an example can offer them all without a table of its own
    /// that would fall behind this one.
    pub const ALL: [Board; 5] = [
        Board::X3,
        Board::X4,
        Board::STICKY,
        Board::BADGER_2040,
        Board::TUFTY_2040,
    ];

    /// Looks a board up by its short name, for a command line.
    ///
    /// The aliases exist because `badger` and `tufty` are what people say.
    pub fn from_slug(slug: &str) -> Option<Board> {
        Board::ALL
            .into_iter()
            .find(|board| board.slug == slug)
            .or(match slug {
                "badger" => Some(Board::BADGER_2040),
                "tufty" => Some(Board::TUFTY_2040),
                _ => None,
            })
    }

    /// A board of an arbitrary size, with the chrome that fits it.
    ///
    /// For a panel not listed here — the point of the framework is that there
    /// will be many.
    pub const fn custom(name: &'static str, width: i32, height: i32, touch: bool) -> Board {
        Board {
            name,
            // Deliberately not any real board's slug. An earlier version
            // matched on size and fell through to a default, so every custom
            // board claimed to be one of the presets.
            slug: "custom",
            width,
            height,
            tokens: Tokens::for_panel(width, height),
            touch,
            refresh_ms: 0,
            bezel: None,
        }
    }

    /// How many list rows this board's content band holds. The number that
    /// decides whether a screen is usable on it at all.
    pub const fn list_rows(&self) -> i32 {
        self.tokens.list_rows_for(self.height)
    }
}
