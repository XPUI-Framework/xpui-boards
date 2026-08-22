# `xpui-boards-seeed`

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

**It has not been run.** There is no published panel driver for it;
[`examples/esp32`](../../../examples/esp32/) marks the seam. See
[`docs/devices.md`](../../../docs/devices.md).
