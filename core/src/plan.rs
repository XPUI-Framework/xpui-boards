//! Describing a body by the shapes on it, rather than by every coordinate.
//!
//! Almost every device here is a **row** of keys along the footer and a
//! **column** of them down an edge. A [`Plan`] says which keys are in each
//! and derives the rest — margins, spacing, centres, and where the panel
//! sits between them — in tenths of a millimetre, as [`Bezel`] is.
//!
//! Three numbers are a board's own: how big the body is, how big the panel
//! is, and how much bezel sits above it. The panel is centred in what the
//! edge keys leave, not in the body; a row spreads evenly across the body and
//! sits centred in the chin; a column stacks beside the panel with one key's
//! width of shell between its keys, centred on the panel's middle. A key
//! that is the odd one out states its own [`span`](Key::spanning).
//!
//! ```
//! use xpui::Button;
//! use xpui_boards_core::{Bezel, Key, PhysicalButton, Plan, Run};
//!
//! // A body 60 x 100 mm, a panel 40 x 70 mm, 8 mm of bezel above it.
//! const PLAN: Plan = Plan::new((600, 1000), (400, 700), 80).footer(Run::new(
//!     (100, 60),
//!     &[
//!         Key::new("Back", Button::Back),
//!         Key::new("Select", Button::Confirm),
//!     ],
//! ));
//! const KEYS: [PhysicalButton; PLAN.count()] = PLAN.keys();
//! const BEZEL: Bezel = PLAN.bezel(&KEYS);
//!
//! // Two keys, in the middle of each half of the body.
//! assert_eq!(BEZEL.buttons[0].centre.0, 150);
//! assert_eq!(BEZEL.buttons[1].centre.0, 450);
//! ```

use xpui::Button;

use crate::{Bezel, KeyAction, PhysicalButton};

/// A key in a row or a column: what it is, and nothing about where it goes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Key {
    label: &'static str,
    action: KeyAction,
    /// How far this key reaches along its run, or 0 to take the run's own
    /// length.
    span: i32,
}

impl Key {
    /// A key that presses a logical button.
    pub const fn new(label: &'static str, button: Button) -> Key {
        Key {
            label,
            action: KeyAction::Press(button),
            span: 0,
        }
    }

    /// A key with nothing mapped to it yet.
    ///
    /// Pressing it does nothing. It exists here because the board has it, and
    /// a row that omits it puts every later label over the wrong key.
    pub const fn unassigned(label: &'static str) -> Key {
        Key {
            label,
            action: KeyAction::Unassigned,
            span: 0,
        }
    }

    /// The same key, shorter or longer along its run than its neighbours.
    ///
    /// The case that needs it is a sleep key stacked above a page pair: same
    /// width, and about two thirds the height.
    pub const fn spanning(self, span: i32) -> Key {
        Key {
            label: self.label,
            action: self.action,
            span,
        }
    }

    const fn span_or(&self, run: i32) -> i32 {
        if self.span > 0 { self.span } else { run }
    }
}

/// A row of keys along the footer, or a column of them down an edge. The keys
/// are named and the size they share is stated once; every centre is derived.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Run {
    keys: &'static [Key],
    /// What one key measures, as `(width, height)` in tenths of a millimetre.
    /// A key with a [`span`](Key::spanning) of its own overrides whichever of
    /// the two lies along the run.
    size: (i32, i32),
}

impl Run {
    /// No keys at all — a footer on a touch device, or an edge with nothing on
    /// it.
    pub const NONE: Run = Run {
        keys: &[],
        size: (0, 0),
    };

    /// A run of keys, each `size` unless it states otherwise.
    pub const fn new(size: (i32, i32), keys: &'static [Key]) -> Run {
        Run { keys, size }
    }

    /// How much of a margin this run needs beside the panel.
    const fn margin(&self) -> i32 {
        if self.keys.is_empty() { 0 } else { self.size.0 }
    }

    /// How far this run reaches with its keys stacked end to end, one key's
    /// width of shell between them.
    ///
    /// The gap is the rail's own thickness because that is the only length a
    /// column has to hand, and because it keeps a two-key rocker and a
    /// three-key one looking like the same family of device.
    const fn stack(&self) -> i32 {
        let mut total = 0;
        let mut index = 0;
        while index < self.keys.len() {
            if index > 0 {
                total += self.size.0;
            }
            total += self.keys[index].span_or(self.size.1);
            index += 1;
        }
        total
    }
}

/// A body, a panel, and the shapes the keys are arranged in.
///
/// [`new`](Plan::new) takes the three numbers a board cannot derive; the
/// builders beside it say which keys sit in which shape, and everything else
/// follows from those. Tenths of a millimetre throughout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    body: (i32, i32),
    panel: (i32, i32),
    forehead: i32,
    footer: Run,
    left: Run,
    right: Run,
    loose: &'static [PhysicalButton],
}

impl Plan {
    /// A body of `body`, holding a panel of `panel`, with `forehead` of bezel
    /// above it. Tenths of a millimetre throughout.
    pub const fn new(body: (i32, i32), panel: (i32, i32), forehead: i32) -> Plan {
        Plan {
            body,
            panel,
            forehead,
            footer: Run::NONE,
            left: Run::NONE,
            right: Run::NONE,
            loose: &[],
        }
    }

    /// The row along the bottom, left to right.
    pub const fn footer(self, run: Run) -> Plan {
        Plan {
            footer: run,
            ..self
        }
    }

    /// The column down the left edge, top to bottom.
    pub const fn left(self, run: Run) -> Plan {
        Plan { left: run, ..self }
    }

    /// The column down the right edge, top to bottom.
    pub const fn right(self, run: Run) -> Plan {
        Plan { right: run, ..self }
    }

    /// Keys that are in neither shape, placed by hand: one reader carries a
    /// round Home pad below its panel that is in no row and no column, and the
    /// next odd device will have something else.
    pub const fn loose(self, keys: &'static [PhysicalButton]) -> Plan {
        Plan {
            loose: keys,
            ..self
        }
    }

    /// How many keys this describes, so the array holding them can be declared
    /// `[PhysicalButton; PLAN.count()]` rather than counted by hand.
    pub const fn count(&self) -> usize {
        self.footer.keys.len() + self.left.keys.len() + self.right.keys.len() + self.loose.len()
    }

    /// Where the panel's top-left corner lands: centred in what the edge keys
    /// leave — the middle of the body when neither edge carries keys, half a
    /// column over when one does.
    pub const fn panel_origin(&self) -> (i32, i32) {
        let left = self.left.margin();
        let spare = self.body.0 - self.panel.0 - left - self.right.margin();
        assert!(
            spare >= 0,
            "the panel and its edge keys are wider than the body"
        );
        (left + half(spare), self.forehead)
    }

    /// The bezel this describes, over an array built by [`keys`](Plan::keys).
    ///
    /// The array is passed in rather than returned because a [`Bezel`] borrows
    /// its buttons for `'static`, and a `const fn` cannot hand out a reference
    /// to something it made itself.
    pub const fn bezel(&self, buttons: &'static [PhysicalButton]) -> Bezel {
        Bezel {
            body: self.body,
            panel_origin: self.panel_origin(),
            panel_size: self.panel,
            buttons,
            artwork: None,
        }
    }

    /// Every key, placed: the footer left to right, then each column top to
    /// bottom, then the one-offs.
    ///
    /// `N` comes from the array being assigned to, and a plan that describes a
    /// different number of keys fails to compile rather than at run time.
    pub const fn keys<const N: usize>(&self) -> [PhysicalButton; N] {
        assert!(
            N == self.count(),
            "the array is not the size this plan describes"
        );

        let (panel_x, panel_y) = self.panel_origin();
        let mut placed = [BLANK; N];
        let mut at = 0;

        // The footer: each key in the middle of its own share of the body's
        // width, and the row centred in the chin the panel leaves below it.
        let along = self.footer.keys.len() as i32;
        let below = panel_y + self.panel.1;
        let line = below + half(self.body.1 - below);
        let mut index = 0;
        while index < self.footer.keys.len() {
            let key = self.footer.keys[index];
            placed[at] = PhysicalButton {
                label: key.label,
                action: key.action,
                centre: (share(self.body.0, index as i32, along), line),
                size: (key.span_or(self.footer.size.0), self.footer.size.1),
            };
            at += 1;
            index += 1;
        }

        // The two columns: a stack centred on the panel's middle, and centred
        // across the margin beside the panel.
        let mut side = 0;
        while side < 2 {
            let (run, column) = if side == 0 {
                (self.left, half(panel_x))
            } else {
                let margin = self.body.0 - panel_x - self.panel.0;
                (self.right, self.body.0 - half(margin))
            };

            let mut top = panel_y + half(self.panel.1 - run.stack());
            let mut index = 0;
            while index < run.keys.len() {
                let key = run.keys[index];
                let span = key.span_or(run.size.1);
                placed[at] = PhysicalButton {
                    label: key.label,
                    action: key.action,
                    centre: (column, top + span / 2),
                    size: (run.size.0, span),
                };
                top += span + run.size.0;
                at += 1;
                index += 1;
            }
            side += 1;
        }

        let mut index = 0;
        while index < self.loose.len() {
            placed[at] = self.loose[index];
            at += 1;
            index += 1;
        }

        placed
    }
}

/// The middle of key `index` of `count`, in a run `extent` long.
///
/// Every key gets an equal share and sits in the middle of it, so a run of
/// three and a run of five differ by a number rather than by a table of
/// positions.
const fn share(extent: i32, index: i32, count: i32) -> i32 {
    extent * (2 * index + 1) / (2 * count)
}

/// Half of `value`, rounded up, so a margin split between two sides differs by
/// at most one tenth of a millimetre.
const fn half(value: i32) -> i32 {
    (value + 1) / 2
}

/// What [`Plan::keys`] fills its array with before placing anything. Every slot
/// is overwritten — the length is checked against the plan first — so this is
/// never what a caller sees.
const BLANK: PhysicalButton = PhysicalButton {
    label: "",
    action: KeyAction::Press(Button::Back),
    centre: (0, 0),
    size: (0, 0),
};
