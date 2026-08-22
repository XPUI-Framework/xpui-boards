//! The panels this framework has been run on, as data.
//!
//! A screen never knows which board it is on. What differs is a handful of
//! numbers — how big the panel is, which measurements fit it, whether there is
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

use xpui::Button;

mod bezel;
mod pimoroni;
mod plan;
mod seeed;
mod xteink;

pub use bezel::{Bezel, KeyAction, PhysicalButton};
pub use plan::{Key, Plan, Run};

pub use xpui::host::{KeyRow, RowKey};

/// Which way up a canvas sits on its framebuffer.
///
/// A device scans its panel in one order and is held in another. Nothing
/// rotates yet; this records which is which so that when something does, the
/// numbers are already here.
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
    pub height: i32,
    /// The panel's own framebuffer, in the order the controller scans it.
    ///
    /// These readers all scan landscape and are held portrait, so the two
    /// differ: an X3 scans 792x528 and presents 528x792. The framebuffer is
    /// never rotated — a renderer transforms each pixel on its way out — so a
    /// backend that talks to real hardware needs this number, not the other.
    pub framebuffer: (i32, i32),
    /// How the canvas sits on the framebuffer.
    ///
    /// Only `Portrait` and `Landscape` are described today, and only the
    /// orientation a device is normally used in. The field exists so adding
    /// the inverted pair, and letting a device turn, is a change of value
    /// rather than a change of shape — the X3 has a gyroscope and the touch
    /// readers rotate.
    pub orientation: Orientation,
    /// The panel's diagonal, in hundredths of an inch — 426 is 4.26" — or
    /// `None` for a panel whose physical size nobody has written down.
    ///
    /// Hundredths of an inch because that is the unit these panels are sold
    /// in, and an integer because `Board` is `Eq` and every preset is a
    /// `const`. It is here so [`ppi`](Board::ppi) can be derived rather than
    /// stored, and so a test can ask the only question that means anything
    /// about chrome on a 200-ppi panel: how big is it in millimetres.
    pub diagonal_hundredths_inch: Option<u16>,
    /// How much larger this board's chrome is than the button-era baseline, as
    /// a percentage: 100 keeps the original pixel sizes, 120 turns a 40px row
    /// into 48.
    ///
    /// It exists because a pixel is not a size. These panels run from 111 ppi
    /// on a Badger to 257 on an X3, and across the 218-257 ppi the readers sit
    /// at, **a 30px row is about 3mm of glass** — legible when a button walks
    /// the selection, and far too small to put a finger on. So the touch
    /// boards take 120 and the button boards stay at 100.
    ///
    /// Hand-tuned per board rather than derived from the ppi, because the ppi
    /// alone cannot tell a 4.26" X4 from a 3.97" Sticky, and because these are
    /// the numbers CrossPoint's own `BoardConfig` carries for each profile: a
    /// screen laid out here matches the firmware rather than merely resembling
    /// it.
    ///
    /// A percentage rather than an `f32` for the same reasons
    /// a chrome scale takes one: `Eq`, `const`, and no FPU on the device.
    pub ui_scale_percent: u16,
    /// What the keys along its bottom edge mean, left to right.
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
    pub const ALL: [Board; 7] = [
        Board::X3,
        Board::X4,
        Board::X4_PRO,
        Board::STICKY,
        Board::BADGER_2040,
        Board::TUFTY_2040,
        Board::INKY_FRAME,
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
            framebuffer: (width, height),
            orientation: Orientation::Landscape,
            // A panel nobody has measured cannot answer in millimetres, and
            // guessing an inch count would make `ppi` confidently wrong.
            diagonal_hundredths_inch: None,
            // No scaling. A panel nobody described cannot ask for bigger
            // targets on any grounds, and whoever wires the backend can pass
            // `Metrics` of their own if this is not what they wanted.
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
    /// the keys it describes — the same reason [`ppi`](Board::ppi) is derived
    /// from the diagonal. A key sending [`Button::Left`] and a key sending
    /// [`Button::Right`], both, because one without the other is a value that
    /// can be raised and never lowered.
    ///
    /// **A board with no bezel answers `false`**, which is the safe direction
    /// rather than a free one. A control told the pair exists when it does not
    /// cannot be changed by any key; a control told it does not exist is
    /// entered and left instead, which every device here can do — at the cost
    /// of a Back press being spent leaving the value rather than the screen.
    /// One is unusable, the other is a keystroke.
    ///
    /// The device's shape does not predict the answer. The X4 Pro takes back,
    /// confirm, left and right from its touchscreen — its wired keys turn pages
    /// and sleep it — so it answers `false`, while the Inky Frame's five-key
    /// footer carries both on its third and fourth keys and answers `true`.
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

/// The crate's prose, compiled.
///
/// A README that does not build is worse than none — and this crate's was
/// declared in `Cargo.toml` without existing at all, which made
/// `cargo package` fail on a crate meant to be published.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
}
