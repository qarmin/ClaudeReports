# GLIFO_UNWRAP_EXPECT_SWEEP_CONTINUATION

`projects/vello.md` flags `glifo/src` as a recurring vein of `unwrap()`/`expect()` on `skrifa`
`Result`/`Option` values that panic on malformed fonts (5 already confirmed: 030, 039, 040, 042,
044) and explicitly says the vein "is not yet exhausted" - directing the next session to grep
`glifo/src` for remaining unswept call sites before assuming otherwise. This candidate is that
follow-up sweep: an exhaustive re-check of all 5406 lines of `glifo/src` (`glyph.rs`, `renderer.rs`,
`colr.rs`, `interface.rs`, `util.rs`, `lib.rs`, `atlas/*.rs`) for any panicking call site reachable
from real usage that isn't already covered by an existing candidate.

For the literal "bare `.unwrap()`/`.expect()` on a skrifa `Result`/`Option`" bug class the previous
5 candidates were about, this sweep found the vein genuinely exhausted (see "What was tried" -
every such call site maps onto an already-Confirmed candidate or is test-only code). But the same
sweep's third escalation tool (`cargo clippy` with `indexing_slicing` enabled) surfaced a different,
previously-unswept panic in adjacent code: `GlyphAtlas::stats()`
(`glifo/src/atlas/cache.rs:472-499`) - `glifo`'s own public, debug-only glyph-cache diagnostics
API - indexes a fixed 4-element array with a cache key's raw `subpixel_x` field with no bounds
check. Every COLR or bitmap glyph that gets cached (an entirely ordinary thing to happen - no
malformed font needed at all) is tagged with a sentinel `subpixel_x` value (4 or 5) specifically
chosen to be *outside* that array's valid range (0-3), so calling `.stats()` after caching even one
COLR/bitmap glyph panics.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Literal grep for `.unwrap()` / `.expect(` across all of `glifo/src` (`grep -rn '\.unwrap()\|\.expect(' .`) | Static, whole crate | Found exactly 5 call sites, all already accounted for: `colr.rs:167` (`stops.last().unwrap()`) = the exact code already investigated as 038 (Not reproducible - skrifa's own COLR traversal never calls the paint callback with 0 stops); `glyph.rs:1419` (`FontRef::from_index(...).unwrap()` in `FontDataExt::as_skrifa`) = the exact call site of 044 (Confirmed); `glyph.rs:1629` (`.head().map(...).unwrap()` in `prepare_glyph_run`) = the exact call site of 040 (Confirmed); `glyph.rs:1985` (`outline_glyph.draw(...).unwrap()`) = the exact call site (`glyph.rs:1985:69`) of 039 (Confirmed); `glyph.rs:2311` (`font.as_skrifa().charmap().map(ch).unwrap()`) is inside the `#[cfg(test)]` module's own test helper - not reachable from any external font input. `renderer.rs`/`interface.rs`/`util.rs`/`lib.rs`/`atlas/*.rs` have zero `.unwrap()`/`.expect()` calls. |
| Grep for adjacent panicking constructs the plain-text unwrap/expect search could miss: `panic!`, `unreachable!`, `todo!`, `unimplemented!`, `try_into().unwrap`, `unwrap_unchecked` | Static, whole crate | Found exactly one: `colr.rs:469`, an `unreachable!()` in the `Brush::SweepGradient` arm. Traced the actual guarantee by reading the pinned `skrifa` 0.44.0 source (`src/color/traversal.rs:462`): skrifa's own traversal explicitly checks `if start_angle_scaled == end_angle_scaled && extend != &Extend::Pad { return Ok(()); }` *before* ever invoking the `ColorPainter::fill` callback with a `SweepGradient` brush - i.e. skrifa itself guarantees this `unreachable!()`'s guard condition can never hold. Genuinely unreachable given the pinned skrifa version, not a live bug (see "What's not done" for a related, unconfirmed float-equality suspicion noticed while reading this code). |
| `cargo clippy -p glifo --no-deps --all-targets -- -W clippy::unwrap_used -W clippy::expect_used -W clippy::indexing_slicing -W clippy::panic` | Debug build, whole crate + test target | 21 warnings. Most were the same already-confirmed/investigated sites above, or provably-safe indexing (`colr.rs:391/420/454`'s `stops[0]`, each guarded by an immediately-preceding `if stops.len() == 1` check; `glyph.rs`'s private `insert_and_merge_range` helper, whose `merge_start`/`merge_end` are bounds-derived by construction, read in full and confirmed safe; `atlas/cache.rs:329`'s `pending_atlas_commands[idx]`, preceded by a `resize_with` that guarantees `idx` is in range). But 3 warnings (`atlas/cache.rs:481`, `:490`) pointed at a genuinely new, previously-unswept site: `GlyphAtlas::stats()` indexing `subpixel_dist[key.subpixel_x as usize]` where `subpixel_dist: [usize; SUBPIXEL_BUCKETS as usize]` (a 4-element array), for every key in `static_entries`/`variable_entries`. |
| Read `atlas/key.rs` to understand `subpixel_x`'s actual value range | Static | `SUBPIXEL_BUCKETS = 4` (valid outline range `0..4`). `SUBPIXEL_COLR = SUBPIXEL_BUCKETS = 4` and `SUBPIXEL_BITMAP = SUBPIXEL_BUCKETS + 1 = 5` are **sentinel** values, documented as "values above that range can never appear in an outline key" and used *specifically* so COLR/bitmap cache keys can never collide with an outline key. `glyph.rs:474` and `glyph.rs:574` (real, unmodified production code, not test-only) construct exactly these keys for every real COLR/bitmap glyph rendered with atlas caching enabled (the default: `colr_bitmap_cache_enabled` only requires `font_size <= config.max_cached_font_size`, default 128.0 - an entirely ordinary configuration). |
| Standalone runtime repro (`repro/`), driving `glifo`'s own public, non-vello_cpu-specific API (`GlyphRunBuilder`/`GlyphRunBackend`/`GlyphRenderer`/`GlyphAtlas`) directly with the project's real bundled `NotoColorEmoji-Subset.ttf` COLR test font, debug build | `cargo run` (debug), 5 runs total across two harness instances (one against a reused worktree checkout, one against this candidate's own fresh `just clone`) | Panicked identically every time: `index out of bounds: the len is 4 but the index is 4` at `glifo/src/atlas/cache.rs:481:13`. 5/5 runs, deterministic (fixed font bytes, fixed glyph id, fixed config, no seed/layout dependence). Does **not** reproduce in `--release`: `GlyphCacheStats`/`GlyphAtlas::stats` are themselves `#[cfg(all(debug_assertions, feature = "std"))]`-gated in `glifo`, so the whole diagnostics API compiles away in release - see `output.txt`. |
| Second input in the same bug class (CONFIRMATION.md section 7): a real bitmap-glyph font (`NotoColorEmoji-CBTF-Subset.ttf`, the project's own bundled `png`-feature test asset) via a temporary in-tree unit test reusing `glifo`'s own `TestResources`/`draw_test_glyph` test harness | `cargo test -p glifo --features png`, debug build | Panics identically: `index out of bounds: the len is 4 but the index is 5` (the `SUBPIXEL_BITMAP` sentinel) - confirms the same defect, not something specific to COLR or to the one font used in `repro/`. Temporary test removed again before `patch.diff` was captured (it was a verification aid, not part of the delivered fix). |

## Status

**Confirmed.** Reproduces 5/5 debug-build runs (deterministic - a plain, unconditional, safe-Rust
out-of-bounds slice index, no seed/layout dependence), plus a 6th confirmation via a second input
in the same bug class (a bitmap glyph instead of a COLR glyph, hitting the sibling
`SUBPIXEL_BITMAP` sentinel). Does not reproduce in a release build - see "Root cause" for why that
is itself expected, not evidence the bug is narrower than described.

**Fully reproducible**: **Yes**, with one honest scope caveat. `repro/` drives `glifo`'s own real,
genuinely public API (`GlyphRunBuilder`, `GlyphRunBackend`, `GlyphRenderer`/`DrawSink`, `GlyphAtlas`,
`GlyphCacheConfig`, `AtlasCacher`) end-to-end with a real, ordinary, well-formed COLR font -
`glifo` is a separately-published crate (`description = "Glifo provides APIs for efficiently
rendering text."`) whose own advertised purpose includes exactly the diagnostics API this bug is
in, so a third-party consumer depending on `glifo` directly (not through `vello_cpu`/`vello_hybrid`)
and calling `GlyphAtlas::stats()` for its documented purpose is a genuine, real caller - no second
participant is faked anywhere in this repro (matching CONFIRMATION.md section 11's carve-out: the
font file is the one external artifact, exactly like "a malformed file handed to a parser's real
public entry point"). The caveat: as of this exact commit, neither `vello_cpu` nor `vello_hybrid`
currently exposes this diagnostics API through their own *public* surface - see "What's not done"
for exactly what would need to change for an application embedding one of those two crates
specifically (rather than `glifo` directly) to trigger this today.

## Class

crash

## Root cause

`GlyphAtlas::stats()` (`glifo/src/atlas/cache.rs:472-499`) builds
`let mut subpixel_dist = [0; SUBPIXEL_BUCKETS as usize];` (a 4-element array, since
`SUBPIXEL_BUCKETS = 4`) and then, for every cached `GlyphCacheKey` in `static_entries` and every
variable-font `GlyphCacheKey` in `variable_entries`, does
`subpixel_dist[key.subpixel_x as usize] += 1` with no bounds check.

`subpixel_x: u8` is documented (`atlas/key.rs`) to hold either a genuine outline subpixel bucket in
`0..SUBPIXEL_BUCKETS` (produced by `quantize_subpixel`, which is itself correctly clamped to
`SUBPIXEL_BUCKETS - 1`), *or* one of two sentinel values reserved specifically to be **outside**
that range: `SUBPIXEL_COLR = SUBPIXEL_BUCKETS = 4` for COLR glyphs, and
`SUBPIXEL_BITMAP = SUBPIXEL_BUCKETS + 1 = 5` for bitmap glyphs. This isn't a theoretical edge
case - `glyph.rs`'s real, unmodified glyph-rendering code (the same code every real render of a
COLR/bitmap glyph through `vello_cpu`/`vello_hybrid` executes) constructs and inserts exactly such
a key for every COLR glyph (`glyph.rs:468-479`) and every bitmap glyph (`glyph.rs:568-579`) it
caches, whenever atlas caching is enabled (the default) and the font size is within the cache's
configured `max_cached_font_size` (default 128.0 - an entirely ordinary, unremarkable font size).

So the two sentinel values that `atlas/key.rs`'s own doc comments say exist *specifically so they
can never collide with a valid outline bucket index* are exactly the values that make `stats()`
index out of bounds - the sentinel design (deliberately placing COLR/bitmap keys outside the valid
outline range) and `stats()`'s indexing (assuming every key's `subpixel_x` is a valid outline
bucket index) were never reconciled. `stats()`'s own doc comment ("Distribution of entries across
subpixel buckets") describes a concept that only applies to outline glyphs in the first place -
the fix is to simply exclude non-outline (sentinel) keys from this particular distribution rather
than trying to give them a bucket of their own.

## Introduced by

5796226b "Import glifo into the vello workspace (#1539)" (2026-03-27). `git blame -L 481,481` and
`git blame -L 490,490` on `glifo/src/atlas/cache.rs` (the two panicking indexing sites) both
attribute the unguarded `subpixel_dist[key.subpixel_x as usize] += 1` line to this exact commit,
with no later commit having touched either line since. `git log -S "SUBPIXEL_COLR" --
glifo/src/atlas/key.rs` confirms the `SUBPIXEL_COLR`/`SUBPIXEL_BITMAP` sentinel constants (the
values that make the indexing panic) were introduced in this same commit - i.e. the mismatch
between the sentinel design and `stats()`'s indexing has existed since `glifo` was first imported
into this workspace, the same commit already cited for candidates 039 and 044.

## The fix

Changed both `subpixel_dist[key.subpixel_x as usize] += 1` call sites (the `static_entries` loop
and the `variable_entries` loop) to
`if let Some(bucket) = subpixel_dist.get_mut(key.subpixel_x as usize) { *bucket += 1; }` - i.e.
silently exclude a sentinel (COLR/bitmap) key from the outline-only `subpixel_distribution`
histogram instead of indexing out of bounds. This is sufficient and doesn't lose real information:
`static_glyphs`/`variable_glyphs`/`unique_glyph_ids`/`sizes_used` (`stats()`'s other fields) already
count every cached entry regardless of glyph kind, using `key.glyph_id`/`key.size_bits` rather than
`key.subpixel_x`, so a COLR/bitmap glyph is still fully represented in the returned
`GlyphCacheStats` - it just correctly contributes nothing to the one field
(`subpixel_distribution`) that is conceptually meaningless for it. Verified against both inputs in
the same bug class (CONFIRMATION.md section 7): the COLR sentinel (4) via `repro/`, and the bitmap
sentinel (5) via the temporary unit test described above - both stop panicking after the fix, and
`subpixel_distribution` correctly stays `[0, 0, 0, 0]` in both cases while the entry is still
counted everywhere else in the returned struct.

Before/after re-running `repro/` (both via the reused worktree and via this candidate's own
`just clone`d fresh checkout): before the patch, `cargo run` panics
(`index out of bounds: the len is 4 but the index is 4`, exit 101); after the patch, the exact same
input completes normally and prints
`did not reproduce: GlyphCacheStats { static_glyphs: 1, ..., subpixel_distribution: [0, 0, 0, 0], ... }`.

- Project test suite (`cargo test -p glifo`, default features, no source changes beyond
  `patch.diff`): **22 passed, 0 failed** both before and after (identical - the existing suite
  never calls `GlyphAtlas::stats()` at all, so this is a genuine, previously-untested gap, not a
  regression risk either way).
- With `--features png` (while the temporary verification test existed): **24 passed, 1 failed**
  before (the temporary test itself, demonstrating the bug) -> **25 passed, 0 failed** after.
- `cargo test -p vello_cpu --lib`: **81 passed, 0 failed** both before and after (identical -
  `vello_cpu`'s own wrapper around this API, `text_debug.rs`'s `GlyphAtlasResources::stats()`, is
  `pub(crate)` and marked `#![allow(dead_code, reason = "only used for debugging purposes")]` -
  nothing in `vello_cpu`'s own code or test suite calls it, so this run is an unaffected sanity
  baseline, not evidence about this specific bug either way).

See `output.txt` for the full captured before/after transcripts (including the second-input
verification and the release-build non-reproduction).

## What's not done / open questions

- **Reachability through `vello_cpu`/`vello_hybrid` specifically, as opposed to through `glifo`
  directly**: as of this commit, `vello_cpu`'s only caller of this diagnostics API
  (`sparse_strips/vello_cpu/src/text_debug.rs`'s `GlyphAtlasResources::stats()`/
  `log_atlas_stats()`/`log_keys_grouped()`) is `pub(crate)`-only and is not called from anywhere
  else in `vello_cpu`'s own code (confirmed by grep) - so an application that only depends on
  `vello_cpu`'s (or `vello_hybrid`'s) own public surface cannot reach this panic *today* without a
  further change exposing that wrapper publicly (e.g. a `--stats`/debug-overlay feature, or simply
  making the existing `pub(crate)` methods `pub`). This candidate's repro instead depends on
  `glifo` directly, which is a real, separately-published, intentionally-public crate whose own
  documented purpose includes this exact diagnostics API - see "Fully reproducible" above for why
  this still meets CONFIRMATION.md section 11's bar without needing a `vello_cpu`-specific path.
  A maintainer who *does* wire up `text_debug`'s existing dead code (which would be a small,
  natural change - the plumbing already exists) would immediately hit this same panic the first
  time a COLR/bitmap glyph gets cached.
- The `unreachable!()` in `colr.rs`'s `Brush::SweepGradient` handling (found during this same
  sweep, see "What was tried") is confirmed safe given the current pinned `skrifa` 0.44.0, but
  while reading that code a *different*, unconfirmed suspicion was noticed: the equality check
  guarding it (`colr.rs:459`, `if start_angle == end_angle`) uses plain float equality on values
  that are the result of several chained float operations, both in `glifo` and in skrifa's own
  upstream traversal (`start_angle_scaled == end_angle_scaled`). If any variable-font delta or
  malformed-COLR-table input could ever drive these particular angle computations to produce two
  values that are conceptually equal but not bit-identical (or `NaN`), both guards (skrifa's own
  and `glifo`'s `unreachable!()`) would fail to trigger, letting a `SweepGradient` brush with
  unusable angles through silently instead of being rejected - a possible silent-garbage-rendering
  logic bug (not a crash), structurally similar to the already-confirmed 036
  (`GLYPH_HINT_UNIFORM_SCALE_FLOAT_EQ`). Not investigated further this session (a different bug
  class - float-equality logic, not unwrap/expect/indexing - and establishing whether any
  realistic input can actually drive this to non-bit-identical-but-conceptually-equal values is a
  nontrivial reachability question of its own) - flagged as a new suspect for a future candidate.
- This sweep specifically targeted "bare `.unwrap()`/`.expect()`/unchecked indexing reachable from
  real usage" in `glifo/src`. For that class, `glifo/src` is now swept clean as of this session
  (`projects/vello.md` should be updated to record this, per the batch-transcription step, so a
  future session doesn't repeat the identical grep). It did not attempt a broader logic-bug audit
  of the same files (COLR gradient math correctness), which is a different bug class already
  tracked separately in `IDEAS.md`'s backlog ("COLR/COLRv1 rendering vs. a reference renderer").
- `GlyphAtlas::log_atlas_stats()` calls `self.stats(page_count)` internally, so it shares the exact
  same defect and is fixed by the same patch (verified: it's a thin `log::debug!` wrapper around
  `stats()`'s already-fixed return value, nothing further to change there). `log_keys_grouped()` is
  independent - it iterates/formats keys directly without ever indexing a fixed-size array by
  `subpixel_x`, so it was never affected and needed no change.
