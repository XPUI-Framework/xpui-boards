# `xpui-boards-pimoroni`

> ⚠️ **Under heavy development.** Not production-ready. The API can break
> without notice. Use at your own risk.

Pimoroni's RP2040 boards, described for xpui.

| Board | Panel | Keys |
|---|---|---|
| Badger 2040 | 296 × 128 e-ink | `a` `b` `c` along the bottom, `Up` `Dn` stacked on the right edge |
| Tufty 2040 | 320 × 240 colour LCD | the same arrangement, in a different body |
| Inky Frame 5.7" | 600 × 448 seven-colour e-ink | `A` to `E` along the bottom, nothing on either edge |

The two badges share a key row of three — Back, Confirm, and one with no job —
because the pair that walks a list sits on the edge rather than in the row a
hint bar labels. The Inky Frame is the one with five in the row, and its fifth
takes the job that needs no label: these frames sleep between refreshes and
wake on a press.

```rust
use xpui_boards_pimoroni as pimoroni;

let badger = pimoroni::BADGER_2040;
assert_eq!((badger.width, badger.height), (296, 128));
assert!(!badger.touch);

// Three in the row on a frame, three on a badge — and the badge's third has
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
else that paints, so taking these three costs you no Xteink and no Seeed.

**The keys are not decoration.** `src/buttons.rs` in the RP2040 repository
resolves a
real GPIO pin by looking a key up **by its label** in the bezel here, so a
label changed here changes which switch does what on the hardware. The bezel
also carries the body in tenths of a millimetre, which is what lets the
simulator draw a device rather than a rectangle.

**Two of the three run.** The Badger 2040 and the Tufty 2040 have firmware in
[`xpui-rp2040`](https://github.com/XPUI-Framework/xpui-rp2040) and have been flashed over a
debug probe. The Inky Frame is described but not built: it exists so a screen
can be laid out and looked at for a 600 × 448 seven-colour panel in the
simulator.
