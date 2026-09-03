# GLIFO_UNWRAP_EXPECT_GENERAL_SWEEP

General sweep of `glifo/src/**/*.rs` for `unwrap()`/`expect()` calls on `skrifa`/`read-fonts`
`Result`/`Option` values, per this candidate's own brief (five confirmed panics of this pattern so
far: 030, 039, 040, 042, 044; `projects/vello.md`'s risk-map note for this repo states the pattern
"almost certainly recurs elsewhere" and "not yet swept exhaustively"). That note turned out to be
stale: this exact sweep (a literal grep for `.unwrap()`/`.expect()`, a related-construct grep, and
a pedantic `clippy` pass with `unwrap_used`/`expect_used`/`indexing_slicing`/`panic`/`unreachable`/
`todo` all promoted to warnings) was already run twice before, independently, by candidates 062
(`GLIFO_UNWRAP_EXPECT_SWEEP_CONTINUATION`) and 080 (`GLIFO_UNWRAP_EXPECT_SWEEP`), and a third,
narrower variant by 070. This session's own `git rev-parse HEAD` (75168314) is byte-identical to
candidate 080's own recorded `base_commit` - i.e. zero commits have landed in this repository
between 080's sweep and this one - so re-running the identical static checks cannot possibly turn
up a call site those two earlier sessions missed; this candidate exists to verify that explicitly
rather than assume it, and to record why the underlying `projects/vello.md` lead should now be
retired as a "general sweep" target.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Literal grep `\.unwrap()\|\.expect(` across all of `glifo/src` (in an isolated `git worktree` at HEAD `75168314`, the same commit as 080's `base_commit`) | Static, whole crate | Exactly 5 call sites, identical set and identical line numbers to 062's and 080's own independent sweeps: `colr.rs:167` (`stops.last().unwrap()`, ruled safe by 038/062/070 - skrifa's own traversal never calls the paint callback with an empty stop slice); `glyph.rs:1419` (`FontRef::from_index(...).unwrap()` in `as_skrifa()`) = candidate 044 (Confirmed); `glyph.rs:1629` (`.head().map(...).unwrap()`) = candidate 040 (Confirmed); `glyph.rs:1985` (`outline_glyph.draw(...).unwrap()`) = candidate 039 (Confirmed); `glyph.rs:2311` (`font.as_skrifa().charmap().map(ch).unwrap()`) is inside the `#[cfg(test)]` module only, not reachable from any external input. No new site. |
| Grep for adjacent panicking constructs (`panic!`, `unreachable!`, `todo!`, `unimplemented!`, `unwrap_unchecked`, `debug_assert!`) and, separately, for any `unsafe` block or `#[allow(clippy::...)]` suppression that could hide a site from the clippy pass below | Static, whole crate | Same set as 062/070/080: one production `unreachable!()` (`colr.rs:469`, `SweepGradient` arm) already ruled genuinely unreachable by 062 given the pinned `skrifa` 0.44.0's own upstream guard; one `debug_assert!` (`util.rs:27`, `tolerance >= 0.0`) not reachable with a negative value from any real caller (062/080 already traced its only call site to a hardcoded constant); zero `unsafe` blocks anywhere in `glifo/src`; zero `#[allow(clippy::...)]` suppressions anywhere in the crate (so neither this nor the prior clippy passes could have been blind to a locally-silenced warning). |
| `cargo clippy -p glifo --no-deps --all-targets --all-features -- -W clippy::unwrap_used -W clippy::expect_used -W clippy::indexing_slicing -W clippy::panic -W clippy::unreachable -W clippy::todo` | Debug build, whole crate + test target, same worktree | 20 warnings (19 lib + 1 duplicate in the test target), same 19 distinct source locations 062/070/080 already catalogued: the same 4 confirmed `.unwrap()` sites above; `atlas/cache.rs:329`/`:576` (indexing/slicing already read in full and confirmed safe by construction - a preceding `resize_with` / a `BTreeMap` key freshly taken from its own `.keys()`); `atlas/cache.rs:481`/`:490` (`GlyphAtlas::stats()` indexing `subpixel_dist[key.subpixel_x as usize]` - this is candidate 062's own already-Confirmed bug, still present in the real tree since every candidate's fix is reverted after verification per this workspace's own rule); `colr.rs:132/166/167/391/420/454/469` (all previously investigated and ruled safe by 038/062/070); `glyph.rs:1001/1005/1011/1018` (`insert_and_merge_range`'s internal indexing, re-verified safe by construction - `merge_start`/`merge_end` are always derived to stay in `0..=ranges.len()`); `glyph.rs:765` (a `clippy::manual_clear` style suggestion on `exclusions.truncate(0)`, not a panic risk at all). Full raw output captured in `output.txt`. No new site beyond what 062/070/080 already found. |

## Status

**Not reproducible** - no new bug found. This specific bug class ("bare `.unwrap()`/`.expect()` on
a `skrifa`/`read-fonts` `Result`/`Option`, reachable from real external font input, inside
`glifo/src`") is exhausted: three independent sweeps (062, 070, this one) at the exact same source
state (070's own commit was also unchanged from 062's, and this candidate's HEAD is byte-identical
to 080's `base_commit`) all find precisely the same 5 literal `.unwrap()`/`.expect()` call sites,
all already accounted for by existing candidates (4 Confirmed panics: 039/040/044, plus one
Confirmed-but-different-class indexing bug caught by the same clippy pass: 062's `atlas/cache.rs`
`stats()` bug), and one test-only site that's unreachable from real input. No fuzzing was run this
session specifically - 070 and 080 already ran short mutation-fuzz/single-byte-mutation checks
against this exact code at the same commit and found nothing beyond what static analysis already
covered (070's fuzz-of-fuzz did surface one genuinely new panic, but it was inside `skrifa`'s own
autohinting code, not a `glifo`-side `.unwrap()`/`.expect()`, so it's out of scope for this
candidate's specific brief). Given three consecutive negative results at unchanged source, a fourth
fuzzing pass this session would not add signal proportionate to the time it costs, per the soft
escalation cap - this candidate stopped after re-confirming the static result rather than repeating
070/080's own fuzz runs a third time.

## Class

N/A (not reproducible).

## Root cause

The candidate's own premise - drawn from `projects/vello.md`'s risk-map note ("Same pattern almost
certainly recurs elsewhere in glifo's font-table access code not yet swept") - was accurate when
that note was first written (before candidate 062 existed) but is now stale: candidates 062 and 080
already performed the exact sweep this candidate was asked to do, at the same commit, and found the
same exhaustive, already-accounted-for result. There is no remaining unswept `.unwrap()`/`.expect()`
call site in `glifo/src` for this bug class to hide in - the file only has 5 such call sites total,
by direct count, and all 5 have been individually triaged across candidates 038-044/062/070/080.

## Introduced by

N/A (not reproducible - no new bug to attribute).

## The fix

N/A (not reproducible - nothing to fix). No `patch.diff` in this folder.

- Project test suite: N/A - no fix was attempted, so no before/after test run was needed for this
  candidate. (For reference, `cargo clippy -p glifo` and the crate's own build both completed
  cleanly at this commit during the checks above - see `output.txt`.)

## What's not done / open questions

- This sweep, like the two before it, targeted only the literal "unwrap/expect/indexing/panic
  macro on a skrifa/read-fonts value" bug class inside `glifo/src` itself. It does not re-litigate
  062's own separately-Confirmed `atlas/cache.rs` `stats()` bug (still unfixed in the real tree,
  per "the one rule that matters most" - every candidate's fix is reverted after verification), or
  039/040/044's already-Confirmed panics - all three remain exactly as those candidates described
  them.
- `projects/vello.md`'s current risk-map text (lines 16-21 at the time this candidate ran) still
  says this pattern "almost certainly recurs elsewhere... not yet swept" - that statement is now
  incorrect as of candidate 062 and should be corrected/updated by whichever step transcribes this
  session's batch results, so a future session doesn't propose this exact candidate a fourth time.
  (Not edited directly by this candidate, to avoid a concurrent-write race with any other candidate
  from the same batch also touching that file.)
- Did not run a fresh fuzz/mutation campaign this session (070's single-byte-mutation approach and
  080's own follow-up already cover that ground at this exact commit and found nothing new beyond
  what's catalogued above) - a longer, dedicated campaign via `fuzz-bug-repro` might still be worth
  pointing at `glifo` broadly, especially at the `skrifa`/`read-fonts` boundary itself rather than
  `glifo`'s own call sites (070's one real finding was inside `skrifa`, not `glifo`).
- Did not check whether a newer `skrifa`/`read-fonts` version changes any of the ruled-safe
  assumptions above (e.g. whether `colr.rs`'s "traversal never calls `fill` with empty stops"
  guarantee still holds in a version newer than the pinned 0.44.0) - out of scope for a pure
  same-commit re-sweep.
