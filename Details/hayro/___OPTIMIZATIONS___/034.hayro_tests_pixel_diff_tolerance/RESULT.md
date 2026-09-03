# `hayro-tests`' exact-pixel-diff harness has zero tolerance for any rounding difference

**No patch attempted - investigation-only, confirming and documenting an already-observed
finding.** This is not a performance candidate in the usual sense (it touches no code on
`hayro`'s render path, so there is nothing whose speed or memory use could be measured); it is a
test-infrastructure observation, first surfaced as a side effect of candidate `009`'s own
correctness check, backlog-worthy on its own. Per `SKILL.md`'s "patch.diff is mandatory..."
section, a `patch.diff` against `hayro-tests`' own test harness (rather than the tested project's
library source) would be a mistake, not a completeness gap - none is included here, by design.

## What was checked

Read `hayro-tests/tests/mod.rs`'s `check_render`/`get_diff`/`is_pix_diff` (the code path every
snapshot-based render/SVG/write test in the suite goes through):

- `check_render` (around line 100): `let (diff_image, pixel_diff) = get_diff(&reference,
  &actual); if pixel_diff > 0 { *failed = true; ... panic!("at least one page had a pixel
  diff") }` - any nonzero pixel-diff count fails the test, no threshold.
- `get_diff` (line 360): counts a pixel as differing via `is_pix_diff`.
- `is_pix_diff` (line 422): `pixel1.0[0] != pixel2.0[0] || pixel1.0[1] != pixel2.0[1] ||
  pixel1.0[2] != pixel2.0[2] || pixel1.0[3] != pixel2.0[3]` (with a special-case skip only when
  both pixels are fully transparent) - i.e. exact byte-for-byte RGBA8 equality per channel, no
  epsilon.
- Searched the rest of `hayro-tests/` (`grep -rn "tolerance\|threshold"`) - no existing
  configurable-tolerance or perceptual-diff mechanism exists anywhere in the crate; `is_pix_diff`
  is the only comparison predicate used by every snapshot test (`render.rs`, `svg.rs`, `write.rs`
  all route through `check_render`/`get_diff`).

This confirms the candidate description exactly: a single-LSB antialiasing/rounding difference on
one pixel is treated identically to a large, visibly-wrong region - both fail the same way, with
no way to distinguish "cosmetically different" from "actually broken" from the test result alone.

## The evidence this candidate is built on (already gathered by candidate `009`, not re-run here)

Candidate `009` (`../009.vello_fill_path_glyph_batching/RESULT.md`) measured this directly while
validating an unrelated glyph-batching change: applying that patch flipped `cargo test --release
-p hayro-tests --test tests -- render` from 1259 failing to 1331 failing (**72 new failures**,
stable across repeated runs both sides), even though the actual pixel deviation ranged from
sub-visible to clearly-visible depending on the file:
- `font_cid_3`: 2399/500395 pixels differ, max channel deviation **63/255** (a real, visible
  difference).
- `issue994`: 6/484704 pixels differ, max channel deviation 10/255.
- `password_1`, `xref_invalid_offsets`, `color_space_lab`: 5-6 pixels differing each.
- The broader `img.pdf` case in the same candidate: 47/500395 pixels differ, max channel
  deviation **1/255** - a sub-visible antialiasing rounding difference, but it fails the harness
  identically to `font_cid_3`'s 63/255 case above.

No new measurement was run for this candidate - the numbers above are candidate `009`'s own,
cited here rather than duplicated with a fresh run, since nothing about the harness or the
underlying render code has changed since.

## Result

Not applicable - no code was changed, nothing was benchmarked. This candidate is a documented
observation plus a design-level recommendation, not a measured delta.

## Does it change behavior/output?

Not applicable - no code was changed on either the `hayro` render path or `hayro-tests` itself.

## Why the result looks the way it does

`is_pix_diff`'s exact-equality check is deliberate strictness for catching real regressions, and
that's a reasonable default for a test suite whose primary job is "did rendering change at all."
The cost is that it can't distinguish "changed by 1/255 on 47 pixels out of 500k, invisible to
the eye" from "changed by 63/255 on 2399 pixels, a real visible defect" - both produce the exact
same panic message shape (`"pixel diff was {pixel_diff}"`) and the exact same test-suite-wide
signal (one more failing test). This makes any optimization or refactor that legitimately changes
rounding (FMA vs. non-FMA float ops, a different but equally-valid antialiasing tie-break, a
changed default precision parameter - all real, expected patterns per `SKILL.md`'s own
"Characterized-difference" correctness section) look, from the test-suite pass/fail count alone,
indistinguishable from an actual correctness bug. Concretely, this session's own candidate `009`
had to hand-verify each of its 72 new failures individually (spot-checking diff triptychs) to
confirm they were genuine, rather than being able to tell from the test output whether any of them
crossed a "this is actually wrong" line.

## What's not done / open questions

Not attempted here (would be `hayro-tests`-only work, no `hayro` library involvement, and
explicitly out of scope for a `patch.diff` per the note at the top):
- A configurable per-pixel channel-deviation tolerance (e.g. an env var or `REPLACE`-style
  build-time knob, defaulting to today's exact-match behavior so nothing currently passing could
  start silently passing-with-drift) plus an image-wide summary stat (max deviation, % of pixels
  over a threshold, mean deviation) reported alongside `pixel diff was {pixel_diff}` so a
  developer can eyeball "is this actually a rounding nit or a real break" without manually opening
  the saved diff triptych every time.
- A separate `--perceptual`/second test mode (structural-similarity or a small per-channel
  epsilon) that runs alongside, not instead of, the existing exact-match mode, so the strict mode
  keeps catching real regressions while an epsilon mode can be used to gate purely-cosmetic
  changes (float-rounding-driven optimizations in particular) without needing hundreds of
  reference-image regenerations (`REPLACE=1`) each time.
- Whether such a change belongs in `hayro-tests` itself or as a wrapper script that post-processes
  the existing `diffs/` folder `check_render` already writes on failure (the diff/reference/actual
  triptych images are already saved there - a tolerance-aware summary could be computed from those
  without touching `mod.rs` at all, which would sidestep the "no patch against the harness" rule
  entirely by living outside the crate as a standalone reporting tool instead).
