# `xpui-boards`

The panels this framework has been run on, as data.

A screen never knows which board it is on. What differs is a handful of
numbers — how big the panel is, which token preset fits it, whether there is a
touchscreen, how long a refresh takes — and those are worth writing down once
rather than rediscovering per project.

```rust
use xpui_boards::Board;

let badger = Board::BADGER_2040;
assert_eq!((badger.width, badger.height), (296, 128));
assert!(!badger.touch);

// Seven of them, and a command line can name any one.
assert_eq!(Board::from_slug("x4pro").map(|board| board.name), Some("Xteink X4 Pro"));
```

The simulator and a real firmware read the same value, which is what makes
"develop in a window, then flash it" true rather than aspirational. Nothing
here touches hardware: it is a description, not a driver.

Separate from [`xpui-chrome`](../backend/chrome/) because a panel size is not a
drawing concern — anything that wants to know how big a screen is can depend on
this without pulling in code that paints. It is also why
[`crates/xpui`](../xpui/) can forbid naming a device: the names live here.

A board also carries its **bezel** — the body around the panel, in millimetres,
so the simulator can draw a device rather than a rectangle. See
[`docs/devices.md`](../../docs/devices.md) for every board's figures and what
has actually been proven about each.
