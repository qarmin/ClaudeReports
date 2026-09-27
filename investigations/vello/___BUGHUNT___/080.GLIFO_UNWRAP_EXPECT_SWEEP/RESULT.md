# GLIFO_UNWRAP_EXPECT_SWEEP

`projects/vello.md`'s risk map calls `glifo/src` "the single richest vein" of confirmed bugs (030,
039, 040, 042, 044, 062, 068, 070 - unwrap/expect on skrifa/read-fonts `Result`/`Option`) and
directs a future session to grep `glifo/src` for any remaining unswept call site before assuming
the vein is exhausted. This candidate is exactly that generic sweep, done independently rather than
trusting the prior write-ups at face value.

`base_commit` for this session is `7516831426fb70f36d62c521c5477f6a7265b36a` - byte-for-byte
identical to the `base_commit` already recorded in candidates 062 and 070's own justfiles. `glifo/src`
has had zero commits since either of those sessions (`git log --oneline -- glifo/src` shows the last
touch was 80062c1a, already reflected in both prior sweeps). So this is a genuine re-verification of
already-examined code, not new ground by construction - the value of this candidate is in (a)
independently re-confirming 062/070's "exhausted" conclusion rather than taking it on faith, and (b)
trying one escalation neither of those sessions tried (see third row below).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Fresh literal grep `\.unwrap()\|\.expect(` across all 11 files / 5406 lines of `glifo/src`, plus a separate grep for `panic!`/`unreachable!`/`todo!`/`unimplemented!`/`unwrap_unchecked`/`debug_assert`/`unsafe` | Static, whole crate, this session's own worktree checkout | Identical results to candidates 062 and 070's own independent sweeps: exactly 5 `.unwrap()`/`.expect()` call sites, all already accounted for (`colr.rs:167` = investigated by 038/062, safe; `glyph.rs:1419` = 044; `glyph.rs:1629` = 040; `glyph.rs:1985` = 039; `glyph.rs:2311` = test-only). One production `unreachable!()` (`colr.rs:469`), already ruled genuinely unreachable by 062 given the pinned skrifa 0.44.0's own upstream guard. One `debug_assert!` (`util.rs:27`, `tolerance >= 0.0`), not reachable with a negative value from any real caller (traced its only call site, a hardcoded constant). Zero `unsafe` blocks anywhere in `glifo/src`. No new site. |
| Checked for `#[allow(clippy::...)]` suppressions in `glifo/src` that could have hidden a real site from 062's/070's own `clippy -W indexing_slicing/unwrap_used/expect_used` runs (`grep -rn "allow(clippy" glifo/src`) | Static | Zero matches - no lint-suppression attribute anywhere in the crate, so the two prior clippy sweeps were not blind to any locally-silenced warning. Not re-running the full clippy pass itself (062 and 070 already ran it at this exact `base_commit`, on default and `--all-features` respectively - no source change since to invalidate that result). |
| Manual boundary-value construction on a class neither 062 nor 070 targeted: `head.units_per_em()` (`glyph.rs:1628`, the `.unwrap()` at the exact call site of confirmed 040) is read as a raw, unvalidated `u16` from a real font's `head` table with no OpenType range check (spec requires 16-16384) before being divided into elsewhere (`font_size / upem` in `calculate_bitmap_transform`/`calculate_colr_metrics`/`GlyphScaleProperties::new`). Tested whether a font with a *present* `head` table but `unitsPerEm = 0` (bypassing 040's "table missing" trigger entirely) drives this division into `inf`/`NaN` far enough to hit a *different* panic downstream (capacity overflow, out-of-bounds atlas index, an assert on a huge/`NaN` transform) rather than the already-known 040 crash. Single-byte mutation (`unitsPerEm` field, offset 18 of the `head` table, located by parsing the real sfnt directory) of the project's own bundled `NotoColorEmoji-Subset.ttf` (COLR), `NotoColorEmoji-CBTF-Subset.ttf` (bitmap), and `Roboto-Regular.ttf` (plain outline), driven through `vello_cpu`'s real public `RenderContext::glyph_run(...).fill_glyphs(...)`/`.hint(true).fill_glyphs(...)` for every emoji glyph in each color font plus 3 Latin glyphs in Roboto, both hinted and unhinted | Debug build, a scratch standalone crate depending on the worktree's `vello_cpu`/`vello_common`/`skrifa` by path, ~15 runs total | No panic in any run. Added temporary `std::eprintln!` instrumentation (reverted before this candidate folder was written) to confirm the mechanism actually engages: `font_info.upem` does read back as `0`, `font_size_scale` does become `inf`, and (since the COLR glyph's font-unit bounding box is nonzero) `scale_factor_x`/`scale_factor_y`/`scaled_bbox` all become `NaN` (`0`-times-`inf`-adjacent arithmetic inside `x_y_advances`). But `NaN as u16`/`inf as u16` (Rust's saturating float-to-int cast, not a panic) turns the resulting pixel dimensions into `0`, and every downstream consumer (`GlyphAtlas::insert_entry`'s `checked_add`/`image_cache.allocate`, the bitmap-transform `Affine`) tolerates a `0`-sized or `NaN`-valued result without panicking - the glyph is silently rendered as invisible/degenerate instead. Same non-crash outcome for the bitmap-glyph font and for plain (non-color) outline glyphs in Roboto, hinted and unhinted. |

No fourth approach was needed: the first two rows are clean re-confirmations of already-covered
ground (3 clean negatives against the literal pattern this candidate was named for), and the third
row - the one genuinely new angle tried this session - traced its own mechanism through to a
non-crashing dead end rather than leaving a partial/ambiguous signal, so there was nothing live left
to keep digging on.

## Status

**Not reproducible.**

The specific bug class this candidate was scoped to (a bare `.unwrap()`/`.expect()` on a
skrifa/read-fonts `Result`/`Option` in `glifo/src`, panicking on realistic font/scene input) is
confirmed - independently, at the identical `base_commit` - to already be fully accounted for by
existing candidates 030, 038/062, 039, 040, 042, 044: every live call site maps onto one of those,
and no new one exists in the current source. A further, more speculative escalation (whether
`units_per_em() == 0` - a boundary value the `head`-table-missing case investigated by 040 doesn't
cover - drives the same division into a different, not-yet-catalogued panic) was tried and did not
reproduce either; see "Root cause" for why.

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A (not Confirmed).

## Root cause (why this turned out to be safe, for now)

For the literal unwrap/expect sweep: nothing has changed in `glifo/src` since candidates 062 and
070 each independently swept it clean at this exact commit - there is no new code for a new bug to
live in.

For the `units_per_em() == 0` boundary-value escalation: `font_size / upem` becoming `inf` (or the
subsequent `0 * inf` becoming `NaN` once multiplied against a real bounding box) is real and does
happen with a crafted font - but every place that consumes the result happens to be tolerant of it
by construction, not by explicit defensive coding: Rust's `as u16`/`as usize` float-to-integer cast
has saturated (never panicked) since Rust 1.45, so `NaN`/`inf` degrades to `0`/`u16::MAX` rather
than raising; `GlyphAtlas::insert_entry` already uses `checked_add`/`.ok()?`/`.get()?` throughout
(itself hardened by earlier candidates in this same risk map, e.g. 052's `checked_mul`), so a `0`-
or maxed-out size either allocates a degenerate atlas slot or cleanly fails closed
(`CacheResult::AtlasFull`) instead of indexing out of bounds. The net effect is a glyph silently
rendered as invisible or wrongly-sized - a real but very mild correctness gap, not a crash - so it
does not meet this class's bar (a panic/crash) and isn't written up as its own candidate here (see
"What's not done" for where it's flagged instead).

## Introduced by

N/A (not Confirmed).

## The fix

No fix attempted - there is nothing panicking to fix. No source in the main project's working tree
was modified; the temporary `eprintln!` instrumentation used to trace the `units_per_em() == 0`
mechanism was added and reverted entirely inside a disposable worktree (`git worktree remove`
afterward), never touching this project's own checkout.

- Project test suite: N/A - no fix was made, no source change to verify against.

## What's not done / open questions

- The `units_per_em() == 0` (or, symmetrically, a `head` table present but with a wildly
  out-of-spec value, e.g. `1`) case is a real, previously-unflagged silent-degradation path (glyph
  renders invisible/wrong-size instead of at all) - worth a future candidate scoped explicitly as a
  *logic* bug (not a crash), since that's a different bug class than what this sweep targeted and
  wasn't chased further here once it was clear nothing panics. Flagging for `IDEAS.md`.
- This sweep, like 062's and 070's before it, targeted only `glifo/src` itself (bare
  unwrap/expect/indexing plus the one boundary-value class tried here). It does not re-litigate
  062's own already-Confirmed `atlas/cache.rs` `stats()` bug (still unfixed in the real tree, per
  "the one rule that matters most" - every candidate's fix is reverted after verification) or
  070's skrifa-internal autohint panic (upstream, already fixed in newer skrifa per that
  candidate's own findings) - both remain exactly as those candidates described them.
- Did not attempt a fresh mutation-fuzz campaign (070's own single-byte-mutation approach already
  covers that ground at this exact commit and found nothing beyond what it already reported) - a
  longer, dedicated campaign via `fuzz-bug-repro` might still be worth pointing at `glifo` broadly,
  but that's a different tool/scope than this skill's own shallow, few-minutes-per-candidate fuzz
  check.
