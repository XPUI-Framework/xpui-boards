# Contributing to `xpui-boards`

## Building it

`rust-toolchain.toml` pins the toolchain and the two bare-metal targets, so
`cargo build` on a fresh clone installs what it needs. The only dependency
outside this workspace is `xpui`, fetched from its repository on `main`.

```bash
cargo test --workspace                # the suite, on a laptop
./build-and-test.sh                   # everything CI checks
```

## The gate

A change is not finished until `./build-and-test.sh` passes. It is the same
command CI runs, so a green run locally means what a green tick means there.
The checks are listed in [`AGENTS.md`](../AGENTS.md) and implemented in
[`xtask/`](../xtask/); `./build-and-test.sh fix` formats in place first.

Three things bite here more than anywhere else:

- **A board is data.** Every number in it comes off a datasheet or a ruler,
  and a figure in a comment is checked against the constant it describes. An
  estimate says so at its definition.
- **The key labels are load-bearing.** `xpui-rp2040` resolves a GPIO pin by
  looking a key up by its label in the bezel; renaming one silences a switch
  on real hardware.
- **`core`'s dev-dependencies on the three vendor crates stay path-only.** A
  versioned dev-dependency there is a publish cycle; `core/Cargo.toml` says
  why beside them.

## Adding a board

[adding-a-board.md](adding-a-board.md) walks the whole of it against a real
device. A board goes in its vendor's crate, or a new vendor crate; never in
`core`, which describes no device.

## The review

Five steps, in order, none skipped:

1. The gate passes, with the real exit code read.
2. The [code-reviewer](../.claude/agents/code-reviewer.md) agent reviews the
   change — every finding resolved, not noted.
3. The [docs-reviewer](../.claude/agents/docs-reviewer.md) agent reviews the
   prose, last: it runs every command a document gives and resolves every
   snippet against the API.
4. The author reviews the code and looks at the board in the simulator, or
   holds one.
5. They say commit.

A test that cannot fail is worse than no test. Before adding one, break the
code on purpose and confirm the test notices.

## Commits

The subject says what was done — imperative, under fifty characters, one
concern. The body says what changed and why, in under about ten lines,
carrying the fact that is not in the diff. Nothing about how the bug was
found. No self-attribution.

## Working across the repositories

The simulator, the gallery and both firmwares depend on these crates through
a `git` dependency on `main`, so a changed number reaches all of them. Before
pushing one, run the umbrella:

```bash
for d in ../xpui*/; do git -C "$d" fetch --quiet --all; done
cd ../xpui-dev && ./build-and-test.sh cross
```

It builds every crate from the sibling checkouts on disk and says which one
broke. `cross` is that repository's gate, not this one's — run it from there,
not here. The fetch first, because its link check resolves every
`github.com/XPUI-Framework/…` URL against each sibling's `origin/main`, and a
stale remote is a stale answer. `xpui`'s [`docs/orientation.md`](https://github.com/XPUI-Framework/xpui-framework/blob/main/docs/orientation.md)
describes the layout it expects.
