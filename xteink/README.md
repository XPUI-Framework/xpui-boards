[![CI](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml/badge.svg)](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

# `xpui-boards-xteink`

Xteink's e-readers, described for xpui.

| Board | Panel | Keys |
|---|---|---|
| X3 | 528 × 792 e-ink, 257 ppi | `Back` `Select` `Up` `Down` along the bottom, `Prev` on the left edge, `Sleep` and `Next` on the right |
| X4 | 480 × 800 e-ink, 218 ppi | the same four along the bottom, `Sleep` and the page pair stacked on the right |
| X4 Pro | the X4's panel, with a touchscreen | **no footer at all** — `Prev` on the left edge, `Sleep` and `Next` on the right, and a capacitive `Home` pad below the panel |

The Pro is the one worth reading twice. It takes Back, Confirm and the pair
that walks a list from the touchscreen, so the four keys the other two carry
along the bottom are simply not there. `Home` is capacitive: the touch
controller reports it, which is why it arrives as a gesture rather than a key
press.

All three scan their panel in landscape and are held in portrait, so the canvas
is the framebuffer turned a quarter. The framebuffer itself is never rotated: a
renderer transforms each pixel on its way out.

## Using it

```toml
[dependencies]
xpui-boards-xteink = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
```

```rust
use xpui_boards_xteink as xteink;

let x4 = xteink::X4;
assert_eq!((x4.width, x4.height), (480, 800));
assert_eq!(x4.framebuffer, (800, 480));

// The Pro is the X4's panel with a finger on it, so it asks for bigger targets.
assert!(xteink::X4_PRO.touch);
assert!(xteink::X4_PRO.ui_scale_percent > x4.ui_scale_percent);

// And its body carries no footer, so no key on it sends Back.
let pro = xteink::X4_PRO.bezel.expect("the Pro has a body described");
assert!(pro.button_labelled("Back").is_none());
assert!(x4.bezel.unwrap().button_labelled("Back").is_some());

// This crate knows only Xteink.
assert_eq!(xteink::from_slug("x4pro"), Some(xteink::X4_PRO));
assert_eq!(xteink::from_slug("badger2040"), None);
```

It depends on [`xpui-boards-core`](../core/) and nothing else.

## Checking it

The gate is the repository's; run `./build-and-test.sh` from the root.

## Where next

| | |
|---|---|
| [`docs/boards.md`](../docs/boards.md) | how a screen reaches these panels, and what is proven on them |

## License

MIT — see [LICENSE](../LICENSE).
