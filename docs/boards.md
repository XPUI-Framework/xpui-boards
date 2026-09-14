# The seven boards

What runs on each of the seven, and how a screen reaches each panel. Each
board's numbers — its panel, key row, refresh time and body — are in
[the vendors reference](reference/vendors.md); this page is what the numbers
cannot say.

The simulator and a real firmware read the same `Board`, which is what makes
"develop in a window, then flash it" true rather than aspirational. A board
also carries its **bezel** — the body around the panel in tenths of a
millimetre, every key placed — so the simulator can draw a device rather than
a rectangle.

Used by `xpui-simulator`, `xpui-gallery`, `xpui-rp2040` and `xpui-esp32` —
every one of them a caller, never a library below one.

## Pimoroni: Badger 2040, Tufty 2040, Inky Frame

**The keys are not decoration.** `xpui-rp2040`'s `src/buttons.rs` resolves a
real GPIO pin by looking a key up **by its label** in the bezel described here,
so a label changed here changes which switch does what on the hardware.
[`Bezel::button_labelled`](reference/bezel.md#xpui_boards_corebezelbutton_labelled)
is that lookup.

**Two of the three run.** The [Badger 2040](https://shop.pimoroni.com/products/badger-2040) and the [Tufty 2040](https://shop.pimoroni.com/products/tufty-2040) have firmware in
[`xpui-rp2040`](https://github.com/XPUI-Framework/xpui-rp2040) and have been
run over a debug probe. The [Inky Frame](https://shop.pimoroni.com/products/inky-frame-5-7) is described but not built: it
exists so a screen can be laid out and looked at for its seven-colour
panel in the simulator.

## Xteink: X3, X4, X4 Pro

**How a screen reaches one of these panels.** They are driven by the firmware
that ships on them, so the route is that firmware hosting `xpui` over the C
ABI — [`xpui-cpp`](https://github.com/XPUI-Framework/xpui-cpp) is that
boundary — rather than a second driver for the same glass.

[`xpui-esp32`](https://github.com/XPUI-Framework/xpui-esp32) carries a
bare-metal image for the [X3](https://www.xteink.com/products/xteink-x3) that builds and links, with `Panel::present`
marked where a panel driver would go. There is no published [Rust](https://rust-lang.org/) or C++
driver for these panels, so what reaches the glass on that route is not yet
proven; the geometry is, in the simulator and in the firmware's own board
configuration, and it is what lays a screen out correctly for all three.

## Seeed: Sticky

**What the crate is for.** The [Sticky](https://www.seeedstudio.com/reTerminal-Sticky-p-6861.html)'s panel is driven by the firmware that
ships on it, and a screen reaches it by that firmware hosting `xpui` over the
C ABI — see [`xpui-cpp`](https://github.com/XPUI-Framework/xpui-cpp). Its
[measurements](reference/vendors.md#seeed) are what lays a screen out for it,
in the simulator and in that firmware alike.

**Nothing has been run on it.** [`xpui-esp32`](https://github.com/XPUI-Framework/xpui-esp32)
carries a bare-metal image for the Sticky that builds and links and stops at
`Panel::present`, where a driver would go; what reaches the glass on either
route is not yet proven. The geometry is, in the simulator.

## No list of all seven

There is no crate here that knows all seven, because there is no such thing as
"every board" — a list is something an application assembles from the vendors
it ships against, each vendor's `ALL` concatenated. [`xpui-gallery`](https://github.com/XPUI-Framework/xpui-gallery/blob/main/gallery/src/boards.rs)'s
is one: seven entries, and a `const` assertion that a vendor cannot gain a
board without it noticing.

A board that is not one of these seven is [adding-a-board.md](adding-a-board.md);
why the vocabulary is a crate of its own is [design.md](design.md).
