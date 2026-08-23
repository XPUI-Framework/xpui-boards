# `xpui-boards-seeed`

> ⚠️ **Under heavy development.** Not production-ready. The API can break
> without notice. Use at your own risk.

Seeed's Sticky, described for xpui.

| Board | Panel | Driven by |
|---|---|---|
| Sticky | 480 × 800 e-ink, 234 ppi | a touchscreen, with three keys beside it |

The X4's framebuffer in a smaller body — 3.97" against 4.26" — so every pixel
here is about 7% smaller than the same pixel on an X4. That is why it asks for
a larger UI scale despite the identical resolution.

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

It depends on [`xpui-boards-core`](../core/) and nothing else.

**What this crate is for.** The Sticky's panel is driven by the firmware that
ships on it, and a screen reaches it by that firmware hosting `xpui` over the C
ABI — see [`xpui-cpp`](https://github.com/XPUI-Framework/xpui-cpp). These measurements are what lays a
screen out for it, in the simulator and in that firmware alike.
[`xpui-esp32`](https://github.com/XPUI-Framework/xpui-esp32) carries a bare-metal image for the same board,
with `Panel::present` marked where a driver would go.

## Using it

```toml
[dependencies]
xpui-boards-seeed = { git = "https://github.com/XPUI-Framework/xpui-boards", branch = "main" }
```

One vendor, so one crate: a project targeting a Sticky compiles this board's
dimensions and nothing else — no Pimoroni, no Xteink. That is why there is a
crate per manufacturer rather than one list, and why there is deliberately no
`ALL` across vendors. An application that ships against more than one
concatenates them.

