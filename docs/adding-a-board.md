# Adding a board

You have a device nobody here describes, and you want your screens laid out for
it. This is what that takes: three measurements, a key row, a body, and two
checks a script can run plus one only you can.

A board is **data**. Nothing in it is code, nothing in it draws, and every
number in it comes off a datasheet or a ruler.

## 1. Where it goes

| Your device is… | Put it in |
|---|---|
| a Pimoroni, Xteink or Seeed board not listed | that vendor's crate here, beside its siblings |
| anything else | a crate of your own, depending on `xpui-boards-core` |
| a panel with no case worth drawing | nowhere — call `Board::custom` at the call site |

`core` is the vocabulary and describes no device. A vendor crate is data
written in it and knows nothing of the other vendors, which is the whole reason
they are separate: a project takes the boards it targets and none of the rest.

The shortcut first, because most people want it:

```rust
use xpui::host::KeyRow;
use xpui_boards_core::Board;

// A name, a size, and whether a finger drives it. No bezel, no millimetres.
let mine = Board::custom("my reader", 480, 800, false);

assert_eq!((mine.width, mine.height), (480, 800));
assert!(!mine.touch);

// Nobody measured it, so nothing can ask it for millimetres.
assert_eq!(mine.ppi(), None);

// A board with keys gets the reader's row, because something has to answer;
// a finger brings its own Back and gets none.
assert_eq!(mine.keys, KeyRow::READER);
assert!(Board::custom("my tablet", 480, 800, true).keys.is_empty());
```

The row matters more than it looks. `custom` hands a board with keys
`KeyRow::READER` — Back, Confirm, and a pair of page keys — so a device without
those keys gets a hint bar naming keys nobody can press, which §4 is about.
Write the row out and skip `custom` if yours differs.

That is enough to open a window and lay out every screen. The rest of this page
is what you gain by measuring.

## 2. Three numbers, and everything else follows

From the datasheet, in **tenths of a millimetre** — never pixels:

| | What |
|---|---|
| **body** | the case, width × height |
| **panel** | the glass, width × height |
| **forehead** | how much case sits *above* the glass |

Only the forehead is asked for separately, and only because a device's chin is
deeper than its forehead by an amount that is a fact about the device. Guessing
it symmetrical puts every key in the wrong place. Everything else — the
margins, the spacing, the key centres, where the panel sits between them —
derives from those three.

Tenths of a millimetre and not pixels, because a body drawn in pixels stops
being right the moment somebody zooms.

## 3. The panel, as a `Board`

Here is the Seeed Sticky, which is the smallest complete example in this
repository:

```rust
use xpui::host::KeyRow;
use xpui_boards_core::{Board, Orientation};
use xpui_boards_seeed::STICKY;

let mine = Board {
    name: "Seeed Sticky",
    slug: "sticky",
    // What a screen is laid out against.
    width: 480,
    height: 800,
    // What the driver scans. These differ when the panel is held turned.
    framebuffer: (800, 480),
    orientation: Orientation::Portrait,
    // The diagonal, in hundredths of an inch. `None` if nobody measured it,
    // and then `ppi` and `tenths_of_a_mm` both answer `None` rather than
    // guessing. §5's panel has to agree with it.
    diagonal_hundredths_inch: Some(397),
    // How much larger this board's chrome should be than the button-era
    // baseline. Hand-tuned, because pixels per inch alone cannot tell a
    // 4.26" X4 from a 3.97" Sticky.
    ui_scale_percent: 120,
    // Its keys are a column on the right, so there is no row to describe.
    keys: KeyRow::new(&[]),
    touch: true,
    refresh_ms: 1200,
    bezel: None,
};

// This is the shipped Sticky, field for field — so the page cannot drift from
// the crate without going red. Its bezel is blanked here and nowhere else:
// that is the one field §5 fills in, and it is `None` until you measure a body.
assert_eq!(mine, Board { bezel: None, ..STICKY });
assert_eq!(mine.ppi(), Some(234));
```

`width`/`height` against `framebuffer` is the pair to get right. A screen is
laid out against the first; a driver scans the second. On every reader here
they differ, because the panel is held a quarter turn from the way it is
scanned.

## 4. The key row, and why a blank key matters

`keys` says what the row along the bottom **means**, left to right. The Badger
has three:

```rust
use xpui::host::{KeyRow, RowKey};
use xpui_boards_pimoroni::BADGER_2040;

const BADGE_ROW: KeyRow = KeyRow::new(&[RowKey::Back, RowKey::Confirm, RowKey::Unassigned]);

assert_eq!(BADGE_ROW.len(), 3);
assert!(BADGE_ROW.contains(RowKey::Back));

// And it is the row the Badger actually ships with, not one that resembles it.
assert_eq!(BADGE_ROW, BADGER_2040.keys);
```

**`Unassigned` is not padding.** It holds a position. The hint bar paints one
word per key, in order, so a key with no job that is simply left out shifts
every label after it one key to the left — and then the label above a key names
what its neighbour does. That shipped once, on two boards, with the suite green
and a person finding it by pressing a key.

A board driven by a finger takes its Back and its directions from the
touchscreen and often has no row at all. Give it an empty one,
`KeyRow::new(&[])`, and a consumer reserves no band for hints, because a hint
names a key and naming one that is not there sends a person looking for it.

## 5. The body, if you want the simulator to draw a device

A `Plan` is a body, a panel, and the shapes the keys are arranged in. Almost
every device is the same two shapes — a **row** along the footer and a
**column** down an edge:

```rust
use xpui::Button;
use xpui_boards_core::{Key, Plan, Run};

// body, panel, forehead — all in tenths of a millimetre.
const STICKY: Plan = Plan::new((629, 1125), (519, 865), 95).right(Run::new(
    (44, 140),
    &[
        Key::new("OK", Button::Confirm).spanning(120),
        Key::new("Prev", Button::PageBack),
        Key::new("Next", Button::PageForward),
    ],
));

assert_eq!(STICKY.count(), 3);

// Every label here is one the shipped bezel answers to. That is the check the
// next paragraph is about, run against the real thing rather than this copy.
for label in ["OK", "Prev", "Next"] {
    assert!(
        xpui_boards_seeed::STICKY_BEZEL.button_labelled(label).is_some(),
        "the shipped Sticky has no key labelled {label}"
    );
}
```

`Run::new` takes the size of one key and the keys in it; the plan spaces them
and centres the panel in what the edge keys leave. A key that is taller or
wider than its neighbours says so with `spanning`.

**The panel here and the diagonal in §3 describe the same glass**, and they
must agree: every millimetre a screen asks for comes from the diagonal, and the
simulator draws this. Work the panel out from the diagonal — the pixel count
over `ppi()`, times 254 — and measure only the case around it. A panel copied
from a photograph that disagrees makes a row look finger-sized in one and not
the other; `xpui-gallery` checks every board it ships to within two percent.

**A key's label is not decoration.** `Bezel::button_labelled` matches on it, and
a firmware resolves a real GPIO pin by looking a key up *by that string* —
`src/buttons.rs` in the RP2040 repository does exactly that.
Spelling `Dn` where the firmware says `Down` compiles, changes nothing the
panel paints, and leaves that switch dead on hardware.

## 6. What proves it

Two checks a script runs:

- **This repository's own tests** — the census, and that every key label the
  bezel carries is one the firmware can find. `cargo test` here.
- **`xpui-gallery`'s `gallery/tests/chrome_for_a_board.rs`** — that your panel holds at
  least three list rows, and that a board with keys reserves a band to name
  them. Those are about the *chrome* derived from your panel, so they live
  where a board and a backend meet.

Then look at it. The gallery opens a window through SDL2, which
[its requirements](https://github.com/XPUI-Framework/xpui-gallery#requirements)
say how to install, and two things stand between a fresh clone and your board:

- **It builds these crates from pushed `main`**, not from your edit. Point it at
  your checkout with a `[patch]` table, as
  [`xpui-dev`'s guide](https://github.com/XPUI-Framework/xpui-dev/blob/main/docs/working-across-repositories.md)
  shows.
- **Its `gallery/src/boards.rs` lists every board it ships**, and a `const`
  assertion stops it compiling the moment a vendor crate gains one. Add yours
  to that list.

```bash
cd .. && git clone https://github.com/XPUI-Framework/xpui-gallery
cd xpui-gallery && cargo run -p xpui-gallery -- --board <your-slug>
```

The simulator draws the body from your millimetres with its keys where a thumb
would find them, and they are clickable. A key in the wrong place is obvious in
a second and invisible in a test.

**And one check no script can run.** Nothing here knows that the key you
labelled `Down` is wired to the switch a person would call Down. That is a
person, with the board, pressing it.
