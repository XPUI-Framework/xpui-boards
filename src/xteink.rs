//! Xteink's e-readers.
//!
//! Geometry and capabilities read from the firmware's own board configuration
//! and the simulator that ships beside it.
//!
//! All three scan their panel in landscape and are held in portrait, so the
//! canvas is the framebuffer turned a quarter. The framebuffer itself is never
//! rotated: a renderer transforms each pixel on its way out.

use crate::{Bezel, Board, Orientation, PhysicalButton};
use xpui::Button;
use xpui_chrome::Tokens;

impl Board {
    /// Xteink X3 — ESP32-C3, a 792x528 panel held portrait, so 528x792.
    ///
    /// No touchscreen. Four keys along the bottom edge and one on each side,
    /// which is the arrangement the firmware calls edge side buttons.
    pub const X3: Board = Board {
        name: "Xteink X3",
        slug: "x3",
        width: 528,
        height: 792,
        framebuffer: (792, 528),
        orientation: Orientation::Portrait,
        tokens: Tokens::DEFAULT,
        touch: false,
        refresh_ms: 1200,
        bezel: Some(X3_BEZEL),
    };

    /// Xteink X4 — ESP32-C3, an 800x480 panel held portrait, so 480x800.
    ///
    /// No touchscreen, and its page keys are a rocker stacked on one side
    /// rather than one per edge — the firmware's themes branch on exactly that
    /// difference.
    pub const X4: Board = Board {
        name: "Xteink X4",
        slug: "x4",
        width: 480,
        height: 800,
        framebuffer: (800, 480),
        orientation: Orientation::Portrait,
        tokens: Tokens::DEFAULT,
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
    pub const X4_PRO: Board = Board {
        name: "Xteink X4 Pro",
        slug: "x4pro",
        width: 480,
        height: 800,
        framebuffer: (800, 480),
        orientation: Orientation::Portrait,
        tokens: Tokens::DEFAULT,
        touch: true,
        refresh_ms: 1200,
        bezel: Some(X4_PRO_BEZEL),
    };
}

/// The X3's body: four keys along the bottom, one on each side edge.
///
/// **The millimetres are estimated.** Xteink publish no mechanical drawing, so
/// the body is derived from the panel — 792x528 at roughly 257 ppi is about
/// 78 x 52 mm of glass — and the surround is scaled from photographs. Replace
/// them with measurements when somebody has the hardware to hand.
///
/// The arrangement is not estimated. `HalGPIO::hasEdgeSideButtons` names the X3
/// and the X4 Pro as the boards whose page keys sit on the screen's left and
/// right edges, and the themes lay out Up on the left and Down on the right
/// against exactly that.
pub const X3_BEZEL: Bezel = Bezel {
    body: (620, 1000),
    panel_origin: (75, 80),
    panel_size: (470, 705),
    buttons: &[
        PhysicalButton {
            label: "Up",
            button: Button::Up,
            centre: (37, 400),
            size: (50, 170),
        },
        PhysicalButton {
            label: "Dn",
            button: Button::Down,
            centre: (583, 400),
            size: (50, 170),
        },
        PhysicalButton {
            label: "Back",
            button: Button::Back,
            centre: (110, 890),
            size: (130, 60),
        },
        PhysicalButton {
            label: "OK",
            button: Button::Confirm,
            centre: (265, 890),
            size: (130, 60),
        },
        PhysicalButton {
            label: "<",
            button: Button::Left,
            centre: (400, 890),
            size: (90, 60),
        },
        PhysicalButton {
            label: ">",
            button: Button::Right,
            centre: (515, 890),
            size: (90, 60),
        },
    ],
    artwork: None,
};

/// The X4's body: four keys along the bottom, and a page rocker stacked on the
/// right rather than split across both edges.
///
/// **Estimated**, as the X3's is. The stacked rocker is not: the themes branch
/// on it, drawing both keys on one side for this board and one per edge for the
/// X3 and the X4 Pro.
pub const X4_BEZEL: Bezel = Bezel {
    body: (580, 1010),
    panel_origin: (60, 80),
    panel_size: (420, 700),
    buttons: &[
        PhysicalButton {
            label: "Up",
            button: Button::Up,
            centre: (545, 330),
            size: (50, 140),
        },
        PhysicalButton {
            label: "Dn",
            button: Button::Down,
            centre: (545, 490),
            size: (50, 140),
        },
        PhysicalButton {
            label: "Back",
            button: Button::Back,
            centre: (100, 900),
            size: (120, 60),
        },
        PhysicalButton {
            label: "OK",
            button: Button::Confirm,
            centre: (240, 900),
            size: (120, 60),
        },
        PhysicalButton {
            label: "<",
            button: Button::Left,
            centre: (360, 900),
            size: (90, 60),
        },
        PhysicalButton {
            label: ">",
            button: Button::Right,
            centre: (470, 900),
            size: (90, 60),
        },
    ],
    artwork: None,
};

/// The X4 Pro's body: a Home key below the panel, the page pair on the side
/// edges, and the touchscreen for everything else.
///
/// **Estimated**, as the others are. What is not: Back and Confirm are absent
/// as keys because the touch controller supplies them, the two wired keys are
/// the page pair one per edge, and the Home key is capacitive and sits below
/// the panel rather than being a pin.
pub const X4_PRO_BEZEL: Bezel = Bezel {
    body: (580, 1010),
    panel_origin: (60, 80),
    panel_size: (420, 700),
    buttons: &[
        PhysicalButton {
            label: "Up",
            button: Button::Up,
            centre: (30, 330),
            size: (44, 150),
        },
        PhysicalButton {
            label: "Dn",
            button: Button::Down,
            centre: (545, 330),
            size: (50, 150),
        },
        // Round on the device. It is the only key on the face, because the
        // touchscreen does everything the other readers give to buttons.
        PhysicalButton {
            label: "Home",
            button: Button::Back,
            centre: (270, 905),
            size: (100, 100),
        },
    ],
    artwork: None,
};
