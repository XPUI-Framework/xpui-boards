[![CI](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml/badge.svg)](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

# `xpui-boards-pimoroni`

[Pimoroni](https://shop.pimoroni.com/)'s [RP2040](https://www.raspberrypi.com/products/rp2040/) boards, described for xpui.

| Board | Panel | Keys |
|---|---|---|
| [Badger 2040](https://shop.pimoroni.com/products/badger-2040) | 296 × 128 e-ink | `a` `b` `c` along the bottom, `Up` `Dn` stacked on the right edge |
| [Tufty 2040](https://shop.pimoroni.com/products/tufty-2040) | 320 × 240 colour LCD | the same arrangement, in a different body |
| [Inky Frame 5.7"](https://shop.pimoroni.com/products/inky-frame-5-7) | 600 × 448 seven-colour e-ink | `A` to `E` along the bottom, nothing on either edge |

The two badges share a key row of three — Back, Confirm, and one with no job —
because the pair that walks a list sits on the edge rather than in the row a
hint bar labels. The Inky Frame is the one with five in the row, and its fifth
takes the job that needs no label: these frames sleep between refreshes and
wake on a press.

## Using it

```toml
[dependencies]
xpui-boards-pimoroni = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
```

```rust
use xpui_boards_pimoroni as pimoroni;

let badger = pimoroni::BADGER_2040;
assert_eq!((badger.width, badger.height), (296, 128));
assert!(!badger.touch);

// Five in the row on a frame, three on a badge — and the badge's third has
// no job, so the hint bar has two words to paint.
assert_eq!(pimoroni::INKY_FRAME.keys.len(), 5);
assert_eq!(badger.keys.len(), 3);

// Three boards, each answering to its own short name. `badger` and `tufty`
// work too, because that is what people say.
assert_eq!(pimoroni::ALL.len(), 3);
assert_eq!(pimoroni::from_slug("tufty"), Some(pimoroni::TUFTY_2040));

// This crate knows only Pimoroni. Another vendor's slug finds nothing here.
assert_eq!(pimoroni::from_slug("x4"), None);
```

It depends on [`xpui-boards-core`](../core/) for the vocabulary and on nothing
else that paints, so taking these three costs you no [Xteink](https://www.xteink.com/) and no [Seeed](https://www.seeedstudio.com/).

## Checking it

The gate is the repository's; run `./build-and-test.sh` from the root.

## Where next

| | |
|---|---|
| [`docs/boards.md`](../docs/boards.md) | which of the three run on hardware, and why the key labels are load-bearing |

## License

MIT — see [LICENSE](../LICENSE).
