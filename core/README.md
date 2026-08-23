# `xpui-boards-core`

> ⚠️ **Under heavy development.** Not production-ready. The API can break
> without notice. Use at your own risk.

The vocabulary a device is described in.

A screen never knows which board it is on. What differs is a handful of
numbers — how big the panel is, whether there is a touchscreen, what the keys
along the bottom mean, how big the glass is in millimetres — and this crate is
the shape those are written in.

**No device is described here.** They live one crate per vendor, so a project
takes the boards it targets and none of the others:

| Crate | Boards |
|---|---|
| [`xpui-boards-pimoroni`](../pimoroni/) | Badger 2040, Tufty 2040, Inky Frame |
| [`xpui-boards-xteink`](../xteink/) | X3, X4, X4 Pro |
| [`xpui-boards-seeed`](../seeed/) | Sticky |

```rust
use xpui_boards_core::Board;
use xpui_boards_pimoroni as pimoroni;

let badger = pimoroni::BADGER_2040;
assert_eq!((badger.width, badger.height), (296, 128));
assert!(!badger.touch);

// Each vendor answers for its own boards, and for no others.
assert_eq!(pimoroni::from_slug("badger").map(|b| b.name), Some("Badger 2040"));
assert_eq!(pimoroni::from_slug("x4pro"), None);
```

A panel nobody has described needs no vendor crate at all:

```rust
use xpui_boards_core::Board;

let mine = Board::custom("my reader", 480, 800, false);
assert_eq!(mine.slug, "custom");
// Nothing has measured it, so nothing can ask it for millimetres.
assert_eq!(mine.ppi(), None);
```

There is no crate that knows all seven, because there is no such thing as
"every board" — a list is something an application assembles from the vendors
it ships against. `examples/gallery/src/boards.rs` is one: seven entries, and a
`const` assertion that a vendor cannot gain a board without it noticing.

The simulator and a real firmware read the same value, which is what makes
"develop in a window, then flash it" true rather than aspirational. Nothing
here touches hardware: it is a description, not a driver.

Separate from [`xpui-chrome`](../../backend/chrome/) because a panel size is
not a drawing concern — anything that wants to know how big a screen is can
depend on this without pulling in code that paints. It is also why
[`crates/xpui`](../../xpui/) can forbid naming a device: the names live in the
vendor crates.

A board also carries its **bezel** — the body around the panel, in millimetres,
so the simulator can draw a device rather than a rectangle. See
[`docs/devices.md`](../../../docs/devices.md) for every board's figures and what
has actually been proven about each.
