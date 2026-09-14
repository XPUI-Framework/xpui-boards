# `xpui-boards`

## What this is, and what it may not become

Seven devices — six e-ink panels and the Tufty's LCD — as data: four crates,
one vocabulary and three vendors.
`xpui-boards-core` is the shape a device is written in — `Board`,
`Orientation`, `Bezel`, and the `Plan`/`Run`/`Key` const DSL that places keys
in a body — and describes no device. Each vendor crate is `const` data written
in it: a panel size, a key row, a refresh time, a body in tenths of a
millimetre.

**It depends on `xpui` alone, and nothing here draws.** `core` may not
describe a device; a vendor crate names one vendor's hardware and no other,
and knows nothing of its siblings. A board is data — every number comes off a
datasheet or a ruler, and an estimate says so where it is written. There is
no list across vendors, and adding one is not a change to make here: an
application assembles its own.

## The gate

```bash
./build-and-test.sh          # everything below
./build-and-test.sh fix      # the same, formatting in place first
```

```text
format · file sizes · crates are tested · READMEs warn · prose is compiled · documented paths resolve · rustdoc links resolve · the reference mirrors rustdoc · documented commands resolve · lint · tests · doctests · README sections · AGENTS.md · published crates deny missing_docs · comment blocks · comment narration
```

There is no `all` mode; this list is the whole of it, and a last stage,
`the gate is documented`, compares it to what ran. Run it before saying a
change is done, and read the real exit code.

## What only this repository checks

Nothing a sibling lacks. The two bare-metal clippy runs under `lint`, on
`riscv32imc` and `thumbv6m`, lint all four crates (`LINT_CRATES`); they are
the only checks that reach their `no_std` paths before a firmware build does.
Five READMEs are checked against the standard, the root's order and the four
crates' nested order.

## Style that bites here

- **`no_std`, and `const` everywhere.** Every preset is a `const`, so the DSL
  is `const fn` only: integer arithmetic, an integer square root, no `f32` —
  the targets have no FPU, and `Board` is `Eq`.
- **A number in a comment is checked against the constant beside it.** The
  millimetre figures, the ppi, the seven-percent claims: each recomputes
  from the constants, or the docs-reviewer catches it.
- **The key labels are what the hardware sends.** `xpui-rp2040` looks a pin
  up by its label in the bezel; a renamed label is a silent switch.
- **`core`'s dev-dependencies on the three vendor crates stay path-only**, or
  the crates cannot be published in any order.
- **Every `pub` item is documented.** `#![deny(missing_docs)]` is on in all
  four crates.
- **A file under `src/` is at most 400 lines.**

## Where the documentation lives, and what proves each piece

| Document | Proven by |
|---|---|
| [`README.md`](README.md) | its `rust` fence is a doctest, mounted by `core/src/lib.rs` |
| [`docs/README.md`](docs/README.md) | its paths resolve; the README-heading check exempts it, because it is the index of `docs/`, not a front page |
| [`core/README.md`](core/README.md), [`pimoroni/README.md`](pimoroni/README.md), [`seeed/README.md`](seeed/README.md), [`xteink/README.md`](xteink/README.md) | doctests, each mounted by its own crate's `src/lib.rs` |
| [`docs/adding-a-board.md`](docs/adding-a-board.md) | doctests, mounted by `core/src/lib.rs`; every snippet copies a real constant |
| [`docs/reference.md`](docs/reference.md), [`docs/reference/`](docs/reference/) | doctests, mounted by `core/src/lib.rs`; every public name in the four crates, crate-prefixed, mirrored by `the reference mirrors rustdoc` |
| [`docs/boards.md`](docs/boards.md), [`docs/design.md`](docs/design.md) | mounted by `core/src/lib.rs`; neither carries a `rust` fence, so what is checked is their paths |
| [`docs/contributing.md`](docs/contributing.md) | every path and command it gives resolves; the umbrella command is `xpui-dev`'s |
| `AGENTS.md` | the stage list above is compared to what the gate runs |
| every `///` and `//!` | `rustdoc links resolve`, and the two comment checks |

## Git

Never stage, never commit, never push without being asked, each time. The
index is the reviewer's queue; leave new work unstaged. No self-attribution
in a commit message. Never rewrite a commit that exists; a correction is a new
commit. The rules that apply to all ten repositories, and the five review
steps, are in [`xpui`'s `docs/orientation.md`](https://github.com/XPUI-Framework/xpui-framework/blob/main/docs/orientation.md);
how a change is built and reviewed here is in
[`docs/contributing.md`](docs/contributing.md).
