//! Xteink's e-readers, described for `xpui`: the X3 and the X4, each with
//! four keys under the panel and page keys on the edges, and the X4 Pro, the
//! X4's panel with a touchscreen and no footer.
//!
//! Geometry and capabilities read from the firmware's own board configuration
//! and the simulator that ships beside it. None has been run from here: the
//! panels are driven by the firmware that ships on them, and a screen reaches
//! one by that firmware hosting `xpui` over the C ABI. `xpui-esp32` carries a
//! bare-metal image for the X3 that builds and links, with `Panel::present`
//! marked where a driver would go.
//!
//! All three scan their panel in landscape and are held in portrait, so the
//! canvas is the framebuffer turned a quarter. The framebuffer itself is never
//! rotated: a renderer transforms each pixel on its way out.

#![cfg_attr(target_os = "none", no_std)]
#![deny(missing_docs)]

use xpui::Button;
use xpui::host::KeyRow;
use xpui_boards_core::{Bezel, Board, Key, KeyAction, Orientation, PhysicalButton, Plan, Run};

/// Xteink X3 — ESP32-C3, a 792x528 panel held portrait, so 528x792.
///
/// No touchscreen. Four keys along the bottom edge and one on each side,
/// which is the arrangement the firmware calls edge side buttons.
///
/// The densest panel here at 257 ppi, and a button board, so it keeps the
/// baseline chrome: its 40px row is 3.9mm, the smallest of the seven.
/// Deliberate rather than overlooked — the firmware gives this profile
/// `uiScale = 1.0`, and a selection walked with a key does not have to be
/// finger-sized.
pub const X3: Board = Board {
    name: "Xteink X3",
    slug: "x3",
    width: 528,
    height: 792,
    framebuffer: (792, 528),
    orientation: Orientation::Portrait,
    // 3.7", the size Xteink sell it as. The only diagonal here not taken
    // from the firmware's board table, which records none for this profile.
    diagonal_hundredths_inch: Some(370),
    ui_scale_percent: 100,
    keys: KeyRow::READER,
    touch: false,
    refresh_ms: 1200,
    bezel: Some(X3_BEZEL),
};

/// Xteink X4 — ESP32-C3, an 800x480 panel held portrait, so 480x800.
///
/// No touchscreen, and its page keys are a rocker stacked on one side
/// rather than one per edge — the firmware's themes branch on exactly that
/// difference.
///
/// A button board, so the baseline chrome: 218 ppi makes its 40px row
/// 4.6mm.
pub const X4: Board = Board {
    name: "Xteink X4",
    slug: "x4",
    width: 480,
    height: 800,
    framebuffer: (800, 480),
    orientation: Orientation::Portrait,
    // 4.26", which is the figure the firmware's own board table quotes
    // where it explains the UI scale; 4.3" is the rounded selling size.
    diagonal_hundredths_inch: Some(426),
    ui_scale_percent: 100,
    keys: KeyRow::READER,
    touch: false,
    refresh_ms: 1200,
    bezel: Some(X4_BEZEL),
};

/// Xteink X4 Pro — the X4's panel with a touchscreen over it.
///
/// Back and Confirm come from the touch controller rather than from keys,
/// which is why only two navigation keys are wired: the reader's page pair,
/// one on each side edge. A capacitive Home key sits below the panel and is
/// reported by the touch controller too, not by a pin.
///
/// The X4's panel, so the X4's 218 ppi — but everything here is chosen with
/// a finger, so the chrome takes the touch scale and a row becomes 48px, or
/// 5.6mm.
pub const X4_PRO: Board = Board {
    name: "Xteink X4 Pro",
    slug: "x4pro",
    width: 480,
    height: 800,
    framebuffer: (800, 480),
    orientation: Orientation::Portrait,
    diagonal_hundredths_inch: Some(426),
    ui_scale_percent: 120,
    keys: KeyRow::READER,
    touch: true,
    refresh_ms: 1200,
    bezel: Some(X4_PRO_BEZEL),
};

/// The row every reader here carries along its footer.
///
/// Its pins are named back, confirm, left and right; the labels are what a list
/// screen prints above them, which is where Up and Down come from.
const READER_FOOTER: [Key; 4] = [
    Key::new("Back", Button::Back),
    Key::new("Select", Button::Confirm),
    Key::new("Up", Button::Left),
    Key::new("Down", Button::Right),
];

/// The X3's body: four keys along the footer, Prev on the left edge, Sleep
/// and Next on the right. The arrangement is not estimated.
///
/// **The millimetres are estimated.** Xteink publish no mechanical drawing, so
/// the body is derived from the panel — 792x528 at roughly 257 ppi is about
/// 78 x 52 mm of glass — and the surround is scaled from photographs. Replace
/// them with measurements when somebody has the hardware to hand.
///
/// `HalGPIO::hasEdgeSideButtons` names the X3 and the X4 Pro as the boards whose page keys sit on the screen's left and
/// right edges, and the themes lay out Up on the left and Down on the right
/// against exactly that.
///
/// The side pair's pins are named up and down; on this board they sit on the
/// screen's left and right edges and turn pages. In a list they move the
/// selection, a row per tap and a page per hold.
const X3_PLAN: Plan = Plan::new((620, 1010), (470, 705), 80)
    .footer(Run::new((125, 60), &READER_FOOTER))
    .left(Run::new((50, 170), &[Key::new("Prev", Button::PageBack)]))
    .right(Run::new(
        (50, 170),
        &[
            Key::new("Sleep", Button::Power).spanning(110),
            Key::new("Next", Button::PageForward),
        ],
    ));

const X3_KEYS: [PhysicalButton; X3_PLAN.count()] = X3_PLAN.keys();
/// The X3's body with its keys placed, for a simulator to draw.
pub const X3_BEZEL: Bezel = X3_PLAN.bezel(&X3_KEYS);

/// The X4's body: the same footer, and a page rocker stacked on the right
/// rather than split across both edges.
///
/// **Estimated**, as the X3's is. The stacked rocker is not: the themes branch
/// on it, drawing both keys on one side for this board and one per edge for the
/// X3 and the X4 Pro.
const X4_PLAN: Plan = Plan::new((580, 1020), (420, 700), 80)
    .footer(Run::new((115, 60), &READER_FOOTER))
    .right(Run::new(
        (50, 140),
        &[
            Key::new("Sleep", Button::Power).spanning(110),
            Key::new("Prev", Button::PageBack),
            Key::new("Next", Button::PageForward),
        ],
    ));

const X4_KEYS: [PhysicalButton; X4_PLAN.count()] = X4_PLAN.keys();
/// The X4's body with its keys placed, for a simulator to draw.
pub const X4_BEZEL: Bezel = X4_PLAN.bezel(&X4_KEYS);

/// The X4 Pro's body: a Home key below the panel, the page pair on the side
/// edges, and the touchscreen for everything else.
///
/// **Estimated**, as the others are. What is not: there is no footer row
/// because this board leaves back, confirm, left and right unassigned and takes
/// them from the touchscreen, the two wired keys are the page pair one per
/// edge, and the Home key is capacitive rather than a pin.
///
/// What sits below the panel is the Home pad alone, in no row and no column —
/// the one-off [`Plan::loose`] exists for.
const X4_PRO_PLAN: Plan = Plan::new((580, 1020), (420, 700), 80)
    .left(Run::new((44, 160), &[Key::new("Prev", Button::PageBack)]))
    .right(Run::new(
        (44, 160),
        &[
            Key::new("Sleep", Button::Power).spanning(110),
            Key::new("Next", Button::PageForward),
        ],
    ))
    // Round, and capacitive: the touch controller reports it rather than a pin,
    // which is why it is a gesture and not a key press. Its centre is the
    // middle of the body and the middle of the chin, stated because a lone key
    // has no run to derive them from.
    .loose(&[PhysicalButton::round(
        "Home",
        KeyAction::Home,
        (290, 900),
        100,
    )]);

const X4_PRO_KEYS: [PhysicalButton; X4_PRO_PLAN.count()] = X4_PRO_PLAN.keys();
/// The X4 Pro's body with its edge keys and Home pad placed, for a simulator
/// to draw.
pub const X4_PRO_BEZEL: Bezel = X4_PRO_PLAN.bezel(&X4_PRO_KEYS);

/// This vendor's 3 boards, so a caller can offer them without a table of
/// its own that would fall behind this one.
///
/// One vendor's list, not the framework's — there is no such thing. An
/// application that ships against more than one concatenates them; see
/// `xpui-gallery`'s `gallery/src/boards.rs`.
pub const ALL: [Board; 3] = [X3, X4, X4_PRO];

/// Looks one of this vendor's boards up by its short name, for a command line.
///
/// `None` for a slug this vendor does not own, including one another vendor
/// does — a caller offering several asks each in turn. No aliases: nobody
/// shortens these names.
pub fn from_slug(slug: &str) -> Option<Board> {
    ALL.into_iter().find(|board| board.slug == slug)
}

/// The crate's prose, compiled: a page that does not build is worse than
/// none.
#[cfg(doctest)]
mod guides {
    #[doc = include_str!("../README.md")]
    pub mod readme {}
}
