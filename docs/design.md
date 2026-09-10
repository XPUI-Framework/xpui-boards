# Design decisions

Where a comment in the code says what was decided, this is why. One section
per decision, named for the item that carries the sentence.

## `ui_scale_percent` is hand-tuned per board

A pixel is not a size. Across the 218–257 ppi the readers here sit at, a 40px
row is 3.9–4.6mm of glass: legible when a key walks the selection, small for a
finger. So the touch boards take 120 and the button boards stay at 100.

It is tuned by hand rather than derived, because the ppi alone cannot tell a
4.26" X4 from a 3.97" Sticky, and the values match the firmware's own
profiles. It is a percentage rather than an `f32` so that `Board` stays `Eq`,
every preset stays a `const`, and nothing needs an FPU the device does not
have.

## `has_left_right_keys` answers `false` when unsure

A board with no bezel answers `false`, and that is the safe direction rather
than the accurate one. A control told the pair exists when it does not cannot
be changed by any key, while one told it does not exist is entered and left
instead, at the cost of a Back press. Both keys are required, because one
without the other is a value that can be raised and never lowered.

## `BADGE_FOOTER`: a goes back, b confirms, c is bare

a goes back and b confirms, the order every preset in the organisation puts
them in, so a person moving between a reader and a badge presses the same
position for the same thing. c is left bare because `Right` without a `Left`
is a value that can be raised and never lowered, and walking the list is the
edge pair's job.

## `Board` is not a drawing concern

The vocabulary is separate from `xpui-chrome` because a panel size is not a
drawing concern: anything that wants to know how big a screen is can depend
on this without pulling in code that paints. It is also why `xpui` can forbid
naming a device — the names live in the vendor crates, and the framework
depends on none of them.
