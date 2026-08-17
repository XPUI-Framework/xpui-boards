//! What a device looks like around its panel.
//!
//! Measured in tenths of a millimetre, not pixels, so the description is of the
//! *device* rather than of one zoom level. A simulator converts to window pixels
//! at draw time from the panel's scale, and zooming changes how big the body is
//! without changing where anything sits on it.
//!
//! Published dimensions where the manufacturer states them, and a documented
//! estimate where they do not. An estimate says so at its definition; none of
//! them is a silent guess.

use xpui::Button;

/// What pressing a key means.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyAction {
    /// A logical button the firmware reads.
    Press(Button),
    /// Go home.
    ///
    /// Not a button: on the reader that has one this is a capacitive pad below
    /// the panel, reported by the touch controller rather than by a pin, and
    /// the framework receives it as a gesture.
    Home,
}

/// A physical key: what it does, and where your thumb finds it.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PhysicalButton {
    /// What is printed beside it, or nothing if the device leaves it bare.
    ///
    /// This is what the key *does*, which is not always what its pin is called.
    /// A reader's side keys sit on pins named up and down and turn pages.
    pub label: &'static str,
    /// What the firmware receives when it is pressed.
    pub action: KeyAction,
    /// Centre, in tenths of a millimetre from the body's top-left.
    pub centre: (i32, i32),
    /// Size, in the same units.
    pub size: (i32, i32),
}

impl PhysicalButton {
    /// A round key of `diameter`, placed by hand.
    ///
    /// For a one-off that is in no row and no column — the reader with a
    /// capacitive Home pad below its panel is the one here. Square by
    /// construction, because a round key described as a rectangle one unit off
    /// square is drawn as an ellipse, and that is a mistake nobody sees until
    /// it is rendered.
    pub const fn round(
        label: &'static str,
        action: KeyAction,
        centre: (i32, i32),
        diameter: i32,
    ) -> PhysicalButton {
        PhysicalButton {
            label,
            action,
            centre,
            size: (diameter, diameter),
        }
    }
}

/// The body around a panel.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bezel {
    /// Body size in tenths of a millimetre.
    pub body: (i32, i32),
    /// Where the panel's top-left sits in the body, same units.
    pub panel_origin: (i32, i32),
    /// The panel's own size, same units. Together with the panel's pixel
    /// dimensions this gives the scale everything else is drawn at.
    pub panel_size: (i32, i32),
    pub buttons: &'static [PhysicalButton],
    /// A licensed image of the device, when there is one.
    ///
    /// `None` everywhere today: manufacturer photography is not licensed for
    /// reuse, and nothing public covers the rest. The slot exists so an image
    /// somebody holds the rights to can be dropped in later without the layout
    /// being redesigned around it.
    pub artwork: Option<&'static [u8]>,
}

impl Bezel {
    /// Whether `point`, in tenths of a millimetre, is on a button.
    pub fn button_at(&self, point: (i32, i32)) -> Option<PhysicalButton> {
        self.buttons.iter().copied().find(|button| {
            let (cx, cy) = button.centre;
            let (w, h) = button.size;
            point.0 >= cx - w / 2
                && point.0 <= cx + w / 2
                && point.1 >= cy - h / 2
                && point.1 <= cy + h / 2
        })
    }

    /// The panel's rectangle in body units, as `(x, y, width, height)`.
    pub const fn panel_rect(&self) -> (i32, i32, i32, i32) {
        (
            self.panel_origin.0,
            self.panel_origin.1,
            self.panel_size.0,
            self.panel_size.1,
        )
    }
}
