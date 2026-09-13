[![CI](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml/badge.svg)](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

# `xpui-boards-core`

The vocabulary a device is described in.

A screen never knows which board it is on. What differs is a handful of
numbers — how big the panel is, whether there is a touchscreen, what the keys
along the bottom mean, how big the glass is in millimetres — and this crate is
the shape those are written in. **No device is described here.** They live one
crate per vendor, so a project takes the boards it targets and none of the
others:

| Crate | Boards |
|---|---|
| [`xpui-boards-pimoroni`](../pimoroni/) | Badger 2040, Tufty 2040, Inky Frame |
| [`xpui-boards-xteink`](../xteink/) | X3, X4, X4 Pro |
| [`xpui-boards-seeed`](../seeed/) | Sticky |

## Using it

```toml
[dependencies]
xpui-boards-core = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
xpui-boards-pimoroni = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
```

A vendor crate is data written in this vocabulary, and each answers for its
own boards only:

```rust
use xpui_boards_core::Board;
use xpui_boards_pimoroni as pimoroni;

let badger: Board = pimoroni::BADGER_2040;
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

It depends on [`xpui`](https://github.com/XPUI-Framework/xpui-framework) for
`Button` and `KeyRow`, and on nothing else.

## Checking it

The gate is the repository's; run `./build-and-test.sh` from the root.

## Where next

| | |
|---|---|
| [`docs/adding-a-board.md`](../docs/adding-a-board.md) | describing your own device: three measurements, a key row, a body, and the one check no script can run |
| [`docs/boards.md`](../docs/boards.md) | the seven that are described, and what runs on hardware |
| [`docs/design.md`](../docs/design.md) | why a board is not a drawing concern, and the other arguments |

## License

MIT — see [LICENSE](../LICENSE).
