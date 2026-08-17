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

/// A physical button: what it does, and where your thumb finds it.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PhysicalButton {
    /// What is printed beside it, or nothing if the device leaves it bare.
    pub label: &'static str,
    /// What the firmware receives when it is pressed.
    pub button: Button,
    /// Centre, in tenths of a millimetre from the body's top-left.
    pub centre: (i32, i32),
    /// Size, in the same units.
    pub size: (i32, i32),
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
