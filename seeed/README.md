[![CI](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml/badge.svg)](https://github.com/XPUI-Framework/xpui-boards/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

# `xpui-boards-seeed`

[Seeed](https://www.seeedstudio.com/)'s [Sticky](https://www.seeedstudio.com/reTerminal-Sticky-p-6861.html), described for xpui.

| Board | Panel | Driven by |
|---|---|---|
| Sticky | 480 × 800 e-ink, 234 ppi | a touchscreen, with three keys beside it |

The X4's framebuffer in a smaller body — 3.97" against 4.26" — so every pixel
here is about 7% smaller than the same pixel on an X4. That is why it asks for
a larger UI scale despite the identical resolution.

## Using it

```toml
[dependencies]
xpui-boards-seeed = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
```

```rust
use xpui_boards_seeed as seeed;

let sticky = seeed::STICKY;
assert_eq!((sticky.width, sticky.height), (480, 800));
assert!(sticky.touch);

// Back and the directions come from touch, so the three wired keys are a
// shared OK/Power and a page pair.
assert_eq!(seeed::ALL.len(), 1);
assert_eq!(seeed::from_slug("sticky"), Some(sticky));
assert_eq!(seeed::from_slug("x4"), None);
```

It depends on [`xpui-boards-core`](../core/) and nothing else. One vendor, so
one crate: a project targeting a Sticky compiles this board's dimensions and
nothing else.

## Checking it

The gate is the repository's; run `./build-and-test.sh` from the root.

## Where next

| | |
|---|---|
| [`docs/boards.md`](../docs/boards.md) | how a screen reaches the Sticky's panel, and that nothing has been run on it yet |

## License

MIT — see [LICENSE](../LICENSE).
