//! The vocabulary a device is described in.
//!
//! A screen never knows which board it is on. What differs is a handful of
//! numbers — how big the panel is, whether there is a touchscreen, what the
//! keys along the bottom mean, how big the glass is in millimetres — and this
//! is the shape those are written in.
//!
//! **No device is described here.** The devices live in one crate per vendor —
//! `xpui-boards-pimoroni`, `xpui-boards-xteink`, `xpui-boards-seeed` — so a
//! project can take the boards it owns and none of the others. Anything with a
//! panel nobody has described reaches for [`Board::custom`] and needs no vendor
//! crate at all.
//!
//! The simulator and a real firmware read the same value, which is what makes
//! "develop in a window, then flash it" true rather than aspirational. Nothing
//! here touches hardware: it is a description, not a driver.
//!
//! Separate from the chrome crate because a panel size is not a drawing
//! concern. Anything that wants to know how big a screen is can depend on this
//! without pulling in code that paints.

#![cfg_attr(target_os = "none", no_std)]

use xpui::Button;

mod bezel;
mod plan;

pub use bezel::{Bezel, KeyAction, PhysicalButton};
pub use plan::{Key, Plan, Run};

pub use xpui::host::{KeyRow, RowKey};

/// Which way up a canvas sits on its framebuffer: a device scans its panel
/// in one order and is held in another.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    /// The canvas is the framebuffer turned a quarter: tall on a wide panel.
    Portrait,
    /// The canvas is the framebuffer as it is scanned.
    Landscape,
}

impl Orientation {
    /// The canvas a framebuffer of `size` presents in this orientation.
    pub const fn canvas(self, size: (i32, i32)) -> (i32, i32) {
        match self {
            Orientation::Portrait => (size.1, size.0),
            Orientation::Landscape => size,
        }
    }
}

/// A panel, its chrome, and what it can be driven with.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Board {
    /// What to call it, for a window title or a log line.
    pub name: &'static str,
    /// The short name a command line accepts. Stored rather than derived: the
    /// X4 and the Sticky are both 800x480, so a size cannot tell them apart.
    pub slug: &'static str,
    /// The canvas a screen is laid out against, in the orientation the device
    /// is normally held.
    pub width: i32,
    /// The canvas's height, in the same orientation.
    pub height: i32,
    /// The panel's own framebuffer, in the order the controller scans it.
    ///
    /// These readers all scan landscape and are held portrait, so the two
    /// differ: an X3 scans 792x528 and presents 528x792. The framebuffer is
    /// never rotated — a renderer transforms each pixel on its way out — so a
    /// backend that talks to real hardware needs this number, not the other.
    pub framebuffer: (i32, i32),
    /// How the canvas sits on the framebuffer, in the orientation a device is
    /// normally used in.
    pub orientation: Orientation,
    /// The panel's diagonal, in hundredths of an inch — 426 is 4.26" — or
    /// `None` for a panel whose physical size nobody has written down.
    ///
    /// Hundredths of an inch because that is the unit these panels are sold
    /// in, and an integer because `Board` is `Eq` and every preset is a
    /// `const`. [`ppi`](Board::ppi) is derived from it.
    pub diagonal_hundredths_inch: Option<u16>,
    /// How much larger this board's chrome is than the baseline, as a
    /// percentage: 100 keeps the pixel sizes, 120 turns a 40px row into 48.
    ///
    /// A pixel is not a size: across the 218–257 ppi the readers sit at, a
    /// 40px row is 3.9–4.6mm of glass — legible when a key walks the
    /// selection, small for a finger. So the touch boards take 120 and the
    /// button boards stay at 100, hand-tuned per board; `docs/design.md`
    /// says why the ppi alone cannot decide it.
    pub ui_scale_percent: u16,
    /// What the keys along its bottom edge mean, left to right. Inert on a
    /// touch board, whose consumers pass `hint_band = !touch`.
    pub keys: KeyRow,
    /// Whether a finger can reach it. A board with buttons and no touchscreen
    /// should not have its layout widened to finger-sized targets, and a screen
    /// can ask before offering a drag-only control.
    pub touch: bool,
    /// Roughly how long a full refresh takes, in milliseconds.
    ///
    /// E-ink is the reason the framework repaints only when something changed.
    /// A board that answers 0 is a display fast enough not to care.
    pub refresh_ms: u32,
    /// The body around the panel, when one has been described. `None` means
    /// the simulator opens a window that is exactly the panel.
    pub bezel: Option<Bezel>,
}

impl Board {
    /// A board of an arbitrary size.
    ///
    /// For a panel no vendor crate describes — the point of the framework is
    /// that there will be many. It has no bezel and no measured diagonal, so
    /// the simulator opens a window that is exactly the panel and nothing can
    /// ask it for millimetres.
    pub const fn custom(name: &'static str, width: i32, height: i32, touch: bool) -> Board {
        Board {
            name,
            // Nothing must match `"custom"`: it is no real board's slug.
            slug: "custom",
            width,
            height,
            framebuffer: (width, height),
            orientation: Orientation::Landscape,
            diagonal_hundredths_inch: None,
            // Whoever wires the backend can pass `Metrics` of their own.
            ui_scale_percent: 100,
            keys: KeyRow::READER,
            touch,
            refresh_ms: 0,
            bezel: None,
        }
    }

    /// Whether this board has a Left/Right pair to nudge a value with.
    ///
    /// Derived from the bezel rather than stored, so it cannot disagree with
    /// the keys it describes. Both keys, because one without the other is a
    /// value that can be raised and never lowered.
    ///
    /// **A board with no bezel answers `false`**, the safe direction. The
    /// shape does not predict the answer: the X4 Pro takes Left and Right
    /// from its touchscreen and answers `false`; the Inky Frame's five-key
    /// footer carries both and answers `true`.
    pub fn has_left_right_keys(&self) -> bool {
        let sends = |wanted: Button| {
            self.bezel.is_some_and(|bezel| {
                bezel
                    .buttons
                    .iter()
                    .any(|key| key.action == KeyAction::Press(wanted))
            })
        };
        sends(Button::Left) && sends(Button::Right)
    }

    /// The panel's pixel density, or `None` when its physical size is unknown.
    ///
    /// Derived from the diagonal rather than stored, so the two cannot drift
    /// apart — and because the diagonal is the number a datasheet prints.
    pub const fn ppi(&self) -> Option<i32> {
        let Some(diagonal) = self.diagonal_hundredths_inch else {
            return None;
        };
        if diagonal == 0 {
            return None;
        }
        let (width, height) = self.framebuffer;
        // Integer square root, not `f32::sqrt`: this is a `const fn`, and the
        // targets it compiles for have no floating-point unit.
        let diagonal_px = (width * width + height * height).isqrt();
        Some(diagonal_px * 100 / diagonal as i32)
    }

    /// What `pixels` of this panel measure, in tenths of a millimetre.
    ///
    /// The question the chrome is actually judged by. A row is 40 pixels on
    /// two of the readers here and 4.6mm on one, 3.9mm on the other, because
    /// the panels differ in density — and 3mm is where a finger stops finding
    /// it.
    ///
    /// Tenths because a millimetre is a coarse unit for a 4mm row, and because
    /// tenths keep this integer on a device with no FPU.
    pub const fn tenths_of_a_mm(&self, pixels: i32) -> Option<i32> {
        let Some(ppi) = self.ppi() else {
            return None;
        };
        if ppi <= 0 {
            return None;
        }
        // 254 tenths of a millimetre to the inch.
        Some(pixels * 254 / ppi)
    }
}

/// The crate's prose, compiled: a README that does not build is worse than
/// none.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
    #[doc = include_str!("../../docs/adding-a-board.md")]
    pub mod adding_a_board {}
}
