# Reference

Every public name in the four crates, by what it describes.
[boards.md](boards.md) says what runs on each board, and
[adding-a-board.md](adding-a-board.md) writes a new one; this is what you reach
for to know what a field means or what a board measures.

## Topics

Each page gives every public name in its area its declaration, what it means,
and examples you can copy. The gate checks every page against the code, so what
a page says an item is, rustdoc agrees with, and every `rust` block on them is
compiled and run as a doctest.

| Page | Holds |
|---|---|
| [boards](reference/boards.md) | `Board` and `Orientation`: a panel's canvas and framebuffer, its keys, its size, and looking a board up |
| [bezel](reference/bezel.md) | `Bezel`, `PhysicalButton` and `KeyAction`, the `Plan`, `Run` and `Key` a body is written in, and the re-exported `KeyRow` and `RowKey` |
| [vendors](reference/vendors.md) | the eight boards, vendor by vendor: every number, the gallery's menu on each, and each crate's `ALL` and `from_slug` |

Names carry their crate, `xpui_boards_core::Board` rather than `Board`, because
`ALL` and `from_slug` exist in three crates and a bare name would not say which.
