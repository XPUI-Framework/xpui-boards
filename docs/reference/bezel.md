# Bezel

The body around a panel, measured in tenths of a millimetre, with every key
placed on it, and the `const` plan a board is written in to place them. A
simulator draws a device from this rather than a bare rectangle, and a firmware
finds out what the switch on a pin sends by looking its key up here.

Tenths of a millimetre, not pixels, so the description is of the device rather
than of one zoom level: a simulator converts at draw time from the panel's
scale, and zooming changes how big the body is without moving anything on it.
Dimensions are published ones where the manufacturer states them, and an
estimate says so where it is written.

[Adding a board](../adding-a-board.md#5-the-body-if-you-want-the-simulator-to-draw-a-device)
builds a body step by step. This page is what each piece does.

## Topics

| | |
|---|---|
| [`xpui_boards_core::Bezel`](#xpui_boards_corebezel) | The body around a panel. |
| [`xpui_boards_core::PhysicalButton`](#xpui_boards_corephysicalbutton) | A physical key: what it does, and where your thumb finds it. |
| [`xpui_boards_core::KeyAction`](#xpui_boards_corekeyaction) | What pressing a key means. |
| [`xpui_boards_core::Plan`](#xpui_boards_coreplan) | A body, a panel, and the shapes the keys are arranged in. |
| [`xpui_boards_core::Run`](#xpui_boards_corerun) | A row of keys along the footer, or a column of them down an edge. |
| [`xpui_boards_core::Key`](#xpui_boards_corekey) | A key in a row or a column: what it is, and nothing about where it goes. |
| [Re-exports](#re-exports) | `KeyRow` and `RowKey`, from `xpui` |

## `xpui_boards_core::Bezel`

The body around a panel.

```text
pub struct Bezel
```

A board holds one in [`Board::bezel`](boards.md#xpui_boards_coreboard). It is
rarely written as a literal: [`Plan::bezel`](#xpui_boards_coreplanbezel) builds
it, with every key's centre derived.

| Field | | Type |
|---|---|---|
| `xpui_boards_core::Bezel::body` | Body size in tenths of a millimetre. | `(i32, i32)` |
| `xpui_boards_core::Bezel::panel_origin` | Where the panel's top-left sits in the body, same units. | `(i32, i32)` |
| `xpui_boards_core::Bezel::panel_size` | The panel's own size, same units. | `(i32, i32)` |
| `xpui_boards_core::Bezel::buttons` | Every physical key, placed in the body. | `&'static [PhysicalButton]` |
| `xpui_boards_core::Bezel::artwork` | A licensed image of the device, when there is one. | `Option<&'static [u8]>` |

`panel_size`, together with the panel's pixel dimensions, gives the scale
everything else is drawn at. `artwork` is `None` on every board here:
manufacturer photography is not licensed for reuse, and the slot exists so an
image somebody holds the rights to can be dropped in without redesigning the
layout.

### Finding a key

#### `xpui_boards_core::Bezel::button_at`

Whether `point`, in tenths of a millimetre, is on a button.

```text
pub fn button_at(&self, point: (i32, i32)) -> Option<PhysicalButton>
```

The point is measured from the body's top-left, and a key's edges count as on
it. Every key is tested as a rectangle, a round one included, and the first
match in `buttons` wins. A simulator turns a click into a key press with this.

#### `xpui_boards_core::Bezel::button_labelled`

The key with `label` printed beside it, or `None` when the body has no such key.

```text
pub fn button_labelled(&self, label: &str) -> Option<PhysicalButton>
```

Matched exactly. The label is the board's own name for the key: the silkscreen
where it prints one, the direction where it prints an arrow. A firmware asks
this what the switch beside a pin sends, rather than declaring that a second
time and letting the two drift.

> [!WARNING]
> **A label is what the hardware sends.** `xpui-rp2040`'s `src/buttons.rs`
> resolves a real GPIO pin by looking a key up by its label, so a label changed
> in a vendor crate changes which switch does what on the hardware. The badges
> print an arrow and name that key `Dn`: spelling it `Down` compiles, changes
> nothing the panel paints, and leaves that switch dead.

```rust
use xpui::Button;
use xpui_boards_core::KeyAction;
use xpui_boards_pimoroni::BADGER_BEZEL;

let down = BADGER_BEZEL.button_labelled("Dn").expect("the Badger's lower edge key");
assert_eq!(down.action, KeyAction::Press(Button::Down));
assert_eq!(BADGER_BEZEL.button_labelled("Down"), None, "never the nearest match");
```

### The panel

#### `xpui_boards_core::Bezel::panel_rect`

The panel's rectangle in body units, as `(x, y, width, height)`.

```text
pub const fn panel_rect(&self) -> (i32, i32, i32, i32)
```

`panel_origin` and `panel_size` in one tuple, for a caller drawing the glass.

**See also:** [`Plan`](#xpui_boards_coreplan),
[`PhysicalButton`](#xpui_boards_corephysicalbutton)

## `xpui_boards_core::PhysicalButton`

A physical key: what it does, and where your thumb finds it.

```text
pub struct PhysicalButton
```

| Field | | Type |
|---|---|---|
| `xpui_boards_core::PhysicalButton::label` | What is printed beside it, or nothing if the device leaves it bare. | `&'static str` |
| `xpui_boards_core::PhysicalButton::action` | What the firmware receives when it is pressed. | `KeyAction` |
| `xpui_boards_core::PhysicalButton::centre` | Centre, in tenths of a millimetre from the body's top-left. | `(i32, i32)` |
| `xpui_boards_core::PhysicalButton::size` | Size, in the same units. | `(i32, i32)` |

The label is what the key *does*, which is not always what its pin is called:
a reader's side keys sit on pins named up and down and turn pages. Almost every
key is placed by [`Plan::keys`](#xpui_boards_coreplankeys) from a
[`Key`](#xpui_boards_corekey); a key in no row and no column is written with
[`round`](#xpui_boards_corephysicalbuttonround).

### Placing a key by hand

#### `xpui_boards_core::PhysicalButton::round`

A round key of `diameter`, placed by hand.

```text
pub const fn round(label: &'static str, action: KeyAction, centre: (i32, i32), diameter: i32) -> PhysicalButton
```

| Parameter | Meaning |
|---|---|
| `label` | The key's name, as a firmware looks it up. |
| `action` | What pressing it means. |
| `centre` | Tenths of a millimetre from the body's top-left, stated because a lone key has no run to derive it from. |
| `diameter` | The same units. The key's `size` is this in both directions. |

Square by construction, because a round key described as a rectangle one unit
off square is drawn as an ellipse, and nobody sees that mistake until it is
rendered. The [X4 Pro](https://www.xteink.com/products/xteink-x4-pro-pocket-ereader)'s capacitive Home pad is the one here, handed to
[`Plan::loose`](#xpui_boards_coreplanloose).

## `xpui_boards_core::KeyAction`

What pressing a key means.

```text
pub enum KeyAction
```

| Variant | |
|---|---|
| `xpui_boards_core::KeyAction::Press` | A logical button the firmware reads. |
| `xpui_boards_core::KeyAction::Home` | Go home. |
| `xpui_boards_core::KeyAction::Unassigned` | A key the board has with nothing mapped to it. |

`Press` carries an `xpui::Button`. `Home` is not a button: on the reader that
has one it is a capacitive pad below the panel, reported by the touch
controller rather than by a pin, and the framework receives it as a gesture.
`Unassigned` is still described because it is still under a thumb: a hint bar
divides its band by the keys that exist, so a key left out shifts every label
after it onto the wrong neighbour.

### Reading an action

#### `xpui_boards_core::KeyAction::button`

The logical button this key sends, or `None` when it sends none.

```text
pub const fn button(self) -> Option<Button>
```

`Home` and `Unassigned` both answer `None`: a firmware reading switches cares
only that neither arrives as a `Button`.

```rust
use xpui::Button;
use xpui_boards_core::KeyAction;

assert_eq!(KeyAction::Press(Button::Confirm).button(), Some(Button::Confirm));
assert_eq!(KeyAction::Home.button(), None);
assert_eq!(KeyAction::Unassigned.button(), None);
```

## `xpui_boards_core::Plan`

A body, a panel, and the shapes the keys are arranged in.

```text
pub struct Plan
```

Almost every device is a **row** of keys along the footer and a **column** of
them down an edge. A plan takes the three numbers a board cannot derive — the
body, the panel, and how much bezel sits above the panel — says which keys are
in which shape, and works out the rest:

- **The panel** is centred across what the edge keys leave, not across the body.
- **A row** spreads evenly across the body's width, each key in the middle of
  its own share, and sits centred in the chin below the panel.
- **A column** stacks its keys with one key's width of shell between them,
  centred on the panel's middle and across the margin beside it.

| Builder | Sets | When not called |
|---|---|---|
| [`Plan::new`](#xpui_boards_coreplannew) | the body, the panel, and the bezel above it | — |
| [`footer`](#xpui_boards_coreplanfooter) | the row along the bottom | no footer keys |
| [`left`](#xpui_boards_coreplanleft) · [`right`](#xpui_boards_coreplanright) | a column down that edge | no keys on that edge |
| [`loose`](#xpui_boards_coreplanloose) | keys placed by hand | none |

A plan is used in three `const` steps: the plan, an array of exactly
[`count`](#xpui_boards_coreplancount) keys from [`keys`](#xpui_boards_coreplankeys),
and the [`bezel`](#xpui_boards_coreplanbezel) over that array. Every vendor
crate keeps its plans and key arrays private and exports the bezel.

**Example — a bezel plan**

```rust
use xpui::Button;
use xpui_boards_core::{Bezel, Key, PhysicalButton, Plan, Run};

// A body 60 x 100 mm holding a 40 x 70 mm panel, 8 mm below its top edge.
const PLAN: Plan = Plan::new((600, 1000), (400, 700), 80)
    .footer(Run::new(
        (100, 60),
        &[Key::new("Back", Button::Back), Key::new("Select", Button::Confirm)],
    ))
    .right(Run::new((50, 150), &[Key::new("Next", Button::PageForward)]));
const KEYS: [PhysicalButton; PLAN.count()] = PLAN.keys();
const BEZEL: Bezel = PLAN.bezel(&KEYS);

// The right-hand column takes 50 of the 200 spare, and the panel is centred in the rest.
assert_eq!(BEZEL.panel_origin, (75, 80));

// The footer first, left to right, in the middle of each half; then the column.
assert_eq!(KEYS.map(|key| key.centre), [(150, 890), (450, 890), (537, 430)]);
assert_eq!(BEZEL.button_at((537, 430)).map(|key| key.label), Some("Next"));
```

### Creating a plan

#### `xpui_boards_core::Plan::new`

A body of `body`, holding a panel of `panel`, with `forehead` of bezel above it.

```text
pub const fn new(body: (i32, i32), panel: (i32, i32), forehead: i32) -> Plan
```

| Parameter | Meaning |
|---|---|
| `body` | The case, width by height. |
| `panel` | The glass, width by height. |
| `forehead` | How much case sits above the glass. A chin is deeper than a forehead by an amount that is a fact about the device, so it is asked for rather than guessed. |

Tenths of a millimetre throughout. The plan starts with no keys.

### Arranging keys

#### `xpui_boards_core::Plan::footer`

The row along the bottom, left to right.

```text
pub const fn footer(self, run: Run) -> Plan
```

#### `xpui_boards_core::Plan::left`

The column down the left edge, top to bottom.

```text
pub const fn left(self, run: Run) -> Plan
```

#### `xpui_boards_core::Plan::right`

The column down the right edge, top to bottom.

```text
pub const fn right(self, run: Run) -> Plan
```

#### `xpui_boards_core::Plan::loose`

Keys that are in neither shape, placed by hand: one reader carries a round Home pad below its panel that is in no row and no column, and the next odd device will have something else.

```text
pub const fn loose(self, keys: &'static [PhysicalButton]) -> Plan
```

Each is written with [`PhysicalButton::round`](#xpui_boards_corephysicalbuttonround)
or as a literal, and copied into the bezel as it is.

### Building the bezel

#### `xpui_boards_core::Plan::count`

How many keys this describes, so the array holding them can be declared `[PhysicalButton; PLAN.count()]` rather than counted by hand.

```text
pub const fn count(&self) -> usize
```

#### `xpui_boards_core::Plan::keys`

Every key, placed: the footer left to right, then each column top to bottom, then the one-offs.

```text
pub const fn keys<const N: usize>(&self) -> [PhysicalButton; N]
```

`N` comes from the array being assigned to. A plan that describes a different
number of keys fails to compile, rather than at run time, because the array is
a `const`. The left column is placed before the right.

#### `xpui_boards_core::Plan::panel_origin`

Where the panel's top-left corner lands: centred in what the edge keys leave — the middle of the body when neither edge carries keys, half a column over when one does.

```text
pub const fn panel_origin(&self) -> (i32, i32)
```

The vertical position is the forehead. A panel and edge keys wider than the
body stop the `const` compiling.

#### `xpui_boards_core::Plan::bezel`

The bezel this describes, over an array built by [`keys`](#xpui_boards_coreplankeys).

```text
pub const fn bezel(&self, buttons: &'static [PhysicalButton]) -> Bezel
```

The array is passed in rather than returned because a `Bezel` borrows its
buttons for `'static`, and a `const fn` cannot hand out a reference to
something it made itself. The bezel has no artwork.

**See also:** [`Run`](#xpui_boards_corerun), [`Key`](#xpui_boards_corekey),
[`Bezel`](#xpui_boards_corebezel)

## `xpui_boards_core::Run`

A row of keys along the footer, or a column of them down an edge.

```text
pub struct Run
```

The keys are named and the size they share is stated once; every centre is
derived. `size` is `(width, height)` of one key. In a footer a key's width lies
along the run; in a column its height does, and its width is the rail's
thickness, which is also the gap between keys. A key with a
[`span`](#xpui_boards_corekeyspanning) of its own overrides whichever of the two
lies along the run.

### Creating a run

#### `xpui_boards_core::Run::NONE`

No keys at all — a footer on a touch device, or an edge with nothing on it.

```text
pub const NONE: Run
```

What each shape of a new [`Plan`](#xpui_boards_coreplan) holds until a builder
replaces it.

#### `xpui_boards_core::Run::new`

A run of keys, each `size` unless it states otherwise.

```text
pub const fn new(size: (i32, i32), keys: &'static [Key]) -> Run
```

| Parameter | Meaning |
|---|---|
| `size` | One key, as `(width, height)` in tenths of a millimetre. |
| `keys` | In order: left to right for a footer, top to bottom for a column. |

## `xpui_boards_core::Key`

A key in a row or a column: what it is, and nothing about where it goes.

```text
pub struct Key
```

Its label and action become a [`PhysicalButton`](#xpui_boards_corephysicalbutton)
once a [`Plan`](#xpui_boards_coreplan) places it. Its fields are private: a key
is written with one of the three below.

### Creating a key

#### `xpui_boards_core::Key::new`

A key that presses a logical button.

```text
pub const fn new(label: &'static str, button: Button) -> Key
```

#### `xpui_boards_core::Key::unassigned`

A key with nothing mapped to it yet.

```text
pub const fn unassigned(label: &'static str) -> Key
```

Pressing it does nothing. It is written because the board has it, and a row
that leaves it out puts every later label over the wrong key.

#### `xpui_boards_core::Key::spanning`

The same key, shorter or longer along its run than its neighbours.

```text
pub const fn spanning(self, span: i32) -> Key
```

| Parameter | Meaning |
|---|---|
| `span` | Its length along the run, in tenths of a millimetre. |

A sleep key stacked above a page pair is the case that needs it: the same
width, and shorter. The [X3](https://www.xteink.com/products/xteink-x3), the X4 and the X4 Pro each carry one at 110, and the
[Sticky](https://www.seeedstudio.com/reTerminal-Sticky-p-6861.html)'s OK key is 120 above two keys of 140.

## Re-exports

Defined in `xpui`, re-exported so a board can be written with this crate alone.
Both are described in `xpui`'s
[reference](https://github.com/XPUI-Framework/xpui-framework/blob/main/docs/reference.md), under input.

| Name | |
|---|---|
| `xpui_boards_core::KeyRow` | What the keys along a device's bottom edge mean, left to right; [`Board::keys`](boards.md#xpui_boards_coreboard) is one. |
| `xpui_boards_core::RowKey` | One slot in that row: `Back`, `Confirm`, `Previous`, `Next`, or `Unassigned`, drawn blank. |
