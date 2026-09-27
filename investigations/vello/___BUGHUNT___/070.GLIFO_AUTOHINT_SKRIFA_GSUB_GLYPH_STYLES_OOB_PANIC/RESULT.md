# GLIFO_AUTOHINT_SKRIFA_GSUB_GLYPH_STYLES_OOB_PANIC

`projects/vello.md` flags `glifo/src` as the richest bug vein so far (030, 039, 040, 042, 044, 062
all confirmed panics from bare `.unwrap()`/`.expect()` on a `skrifa` `Result`/`Option`), and
candidate 062's own writeup says this specific literal-`unwrap`/`expect` pattern is "genuinely
exhausted" as of that session. This candidate was a fresh, independent re-sweep of `glifo/src` for
the same pattern, followed by a short one-off single-byte-mutation fuzz check once the sweep itself
confirmed nothing new. That fuzz check is what actually found something: not a new `.unwrap()` in
`glifo`'s own source, but a real, deterministic, out-of-bounds slice-index **panic inside the
pinned `skrifa` 0.44.0 dependency itself** (`outline::autohint::shape::GsubHandler::finish`),
reachable through `glifo`'s ordinary, public hinted-glyph-run code path with no `.unwrap()`/
`.expect()` anywhere on `glifo`'s side of the boundary.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Literal grep `\.unwrap()\|\.expect(` across all of `glifo/src` (11 files, 5406 lines) | Static, whole crate | Found exactly 5 call sites - identical to candidate 062's own finding: `colr.rs:167` (`stops.last().unwrap()`, protected - see below), `glyph.rs:1419` (`as_skrifa()`, = candidate 044), `glyph.rs:1629` (`.head().unwrap()`, = candidate 040), `glyph.rs:1985` (`outline_glyph.draw(...).unwrap()`, = candidate 039), `glyph.rs:2311` (inside `#[cfg(test)]` only). No new site. |
| Read `skrifa` 0.44.0's own `color/traversal.rs` (the pinned version this workspace actually builds against) to check whether `colr.rs:166/167`'s `stops[0]`/`stops.last().unwrap()` can really receive an empty slice | Static, source reading | Confirmed still safe: `skrifa`'s own `LinearGradient`/`RadialGradient`/`SweepGradient` traversal arms only ever call `ColorPainter::fill(...)` after a `match (resolved_stops.first(), resolved_stops.last()) { (None, _) \| (_, None) => {} ... }` guard (or an explicit `is_empty()` check for the sweep case) - `fill()` is never invoked with fewer than 1 stop, so `convert_stops`'s `stops` parameter can't be empty via the real call path. |
| Grep for adjacent panicking constructs (`panic!`, `unreachable!`, `todo!`, `unimplemented!`, `unwrap_unchecked`, `assert!`, `debug_assert!`) across `glifo/src` | Static, whole crate | One production (non-test) hit: `colr.rs:469`'s `unreachable!()` in the `SweepGradient` arm - already investigated and ruled genuinely unreachable by candidate 062 (skrifa's own traversal guards the condition before calling into this arm). Everything else was test-only (`renderer.rs`/`util.rs`/`glyph.rs` unit tests). |
| `cargo clippy -p glifo --no-deps --all-targets --all-features -- -W clippy::unwrap_used -W clippy::expect_used -W clippy::indexing_slicing -W clippy::panic -W clippy::unreachable -W clippy::todo` | Debug build, whole crate + tests | 20 warnings, all mapping onto already-catalogued sites (`insert_and_merge_range` in `glyph.rs` re-verified safe by construction - `merge_start`/`merge_end` are always in `0..=ranges.len()`; `atlas/cache.rs:576`'s `by_glyph[&glyph_id]` indexes a `BTreeMap` with a key freshly collected from its own `.keys()`, always present). No new site beyond what 062 already found (`atlas/cache.rs` `stats()`, separately Confirmed there). |
| Short one-off fuzz check: a scratch `#[test]` in `vello_cpu` (deleted before this candidate's `patch.diff` was captured, never part of the delivered fix) that mutates 1-4 random bytes of the project's own bundled `Roboto-Regular.ttf`/`NotoColorEmoji-Subset.ttf`, 3000 iterations each, driven through `vello_cpu::RenderContext::glyph_run(...).fill_glyphs(...)` (real public API) with `catch_unwind`, classifying distinct panic messages | Debug build, `cargo test -p vello_cpu --test tmp_fuzz_glifo`, 6000 total iterations (~8s) | Every panic message matched an already-confirmed `glifo` bug (`InvalidCollectionIndex`/`InvalidSfnt` = 044, `TableIsMissing(head)` = 040, `Read(MalformedData(...))`/`Read(OutOfBounds)` = 039, `index out of bounds: len is 30` = 042's CPAL palette-index bug) **except** two new message shapes: `range end index 1 out of range for slice of length 0` (16/6000 mutated `NotoColorEmoji` inputs) and `InvalidAnchorPoint(GlyphId(_), _)` (3/6000, a `ReadError` variant unwrapped at the same `glyph.rs:1985` site as candidate 039 - not investigated separately since it's the exact same call site/fix as that already-Confirmed candidate). |
| Backtrace capture (a `panic::set_hook` that stashes `Backtrace::force_capture()` at panic time, before unwind) on the "range end index" panics | Same harness, `RUST_BACKTRACE=1` | Traced to `skrifa::outline::autohint::shape::GsubHandler::finish` (`self.glyph_styles[range]`, no bounds check against `self.max_gid+1 <= self.glyph_styles.len()`), called from `HintingInstance::new`/`reconfigure` -> `glifo::glyph::HintKey::instance` (`glyph.rs:2058`) -> `find_hint_entry` -> `HintCache::get` -> `prepare_glyph_run` -> `GlyphRun::build`. Zero `.unwrap()`/`.expect()` anywhere in this chain on the `glifo` side - the panic is entirely inside `skrifa`'s own autohinting code. |
| Root-cause narrowing: byte-diffed 16 crashing mutated fonts against the pristine base font | `python3` diff | All corruptions cluster around the `maxp` table's sfnt-directory record (offset ~172-187: the `maxp` tag/checksum/offset/length fields) - corrupting `maxp`'s declared offset/length is enough to desynchronize `skrifa`'s internal glyph count from what a GSUB substitution rule reports touching, without necessarily breaking plain (unhinted) outline lookup for the same glyphs. |
| Isolated minimal repro construction: single-byte mutation (offset 185, `maxp` length field byte, `0x00`->`0xB3`) of the bundled `NotoColorEmoji-Subset.ttf`, requesting only glyph id 1 (not the multi-glyph fuzz set, to avoid also tripping candidate 039 on a different glyph in the same font) | Debug + release, `vello_cpu`'s real public `RenderContext::glyph_run(...).hint(true).fill_glyphs(...)` | Panics identically, 3/3 runs (debug), deterministic | 
| Second input in the same bug class (CONFIRMATION.md section 7): a different single-byte mutation of the same base font (offset 181, also part of `maxp`'s directory record, `0x00`->`0x51`), same single-glyph query | Debug build | Panics identically at the same `skrifa` location |
| Checked whether a newer `skrifa` already fixes this, using already-vendored crates.io source under `~/.cargo/registry/src` (0.43.2, 0.44.0, 0.46.0, 0.46.2) | Static source diff | `0.43.2`/`0.44.0` both have the unguarded `self.glyph_styles[range]`. `0.46.0`/`0.46.2` changed it to `if ... \|\| self.glyph_styles.is_empty() { return None; }` plus `self.glyph_styles.get_mut(range)?` (fallible, non-panicking) - and even ship a dedicated regression test, `finish_with_empty_glyph_styles`, for exactly this shape. Confirms this is a known, already-fixed-upstream defect, not something this session had to root-cause blind. |

## Status

**Confirmed.** Reproduces 3/3 debug-build runs (deterministic - a plain out-of-bounds slice index,
no seed/layout dependence) for the primary single-byte-mutation input, plus a second,
independently-constructed single-byte mutation in the same class (different byte offset within the
same `maxp` directory record) also reproducing. Additionally reproduced via `just test`'s
`reproduce-before` step against a completely fresh `git clone` of the real repository at this
candidate's `base_commit` (release build) - 1/1, matching the debug-build runs.

**Fully reproducible**: **Yes**. `repro/` drives `vello_cpu`'s real, public
`RenderContext::glyph_run(...).hint(true).fill_glyphs(...)` API end-to-end with a real font file
(the project's own bundled `NotoColorEmoji-Subset.ttf` test asset, with exactly one byte changed,
found by fuzzing rather than hand-crafted) - the same code path and API a real consumer embedding
`vello_cpu` for hinted text rendering with an externally-supplied font would use. This is a single
external artifact (a malformed font file) handed to a parser's real public entry point - there is
no second participant to fake, matching `CONFIRMATION.md` section 11's explicit carve-out (same
reasoning as candidates 044/062).

## Class

crash

## Root cause

This is **not** an unguarded `.unwrap()`/`.expect()` in `glifo`'s own source - the literal-pattern
sweep this candidate set out to redo confirms candidate 062's conclusion that vein is genuinely
exhausted. Instead, it's a real bug in the **pinned `skrifa` 0.44.0 dependency itself**, reachable
transitively through `glifo`'s ordinary hinting code:

`glifo::glyph::HintKey::instance` (`glyph.rs:2058`) calls `skrifa::outline::hint::HintingInstance::new(...)`
whenever a hinted glyph run (`.hint(true)`, the default transform-eligible path) is drawn for the
first time with a given font/size/coords combination. That call eventually reaches
`skrifa::outline::autohint::instance::Instance::new`, which builds a `GlyphStyleMap` sized to the
font's `maxp.numGlyphs`. While scanning GSUB substitution rules to determine which glyphs a given
autohint "style" (a Latin/writing-system classification) actually touches,
`GsubHandler::finish()` computes `range = self.min_gid..self.max_gid + 1` from the glyph IDs a GSUB
rule reports substituting, and then unconditionally does `for glyph in &mut self.glyph_styles[range]`
with **no check that `self.max_gid + 1 <= self.glyph_styles.len()`**. A font whose `maxp` table's
sfnt-directory record (its offset/length fields, not the table body itself) has been corrupted
desynchronizes `self.glyph_styles`'s actual length from the substitution rule's glyph-id range,
producing exactly this out-of-bounds slice index. `glifo` has no way to guard against this from its
own side - `HintingInstance::new`/`reconfigure` doesn't return a `Result` that could surface this as
a recoverable error; the panic happens deep inside `skrifa`'s own, non-fallible internal
construction logic.

Confirmed as a known, already-fixed-upstream defect: `skrifa` 0.46.0 (two minor versions past the
0.44.0 this workspace currently pins) changed the exact same line to
`self.glyph_styles.get_mut(range)?` (returning `None` instead of panicking) and added an explicit
`self.glyph_styles.is_empty()` guard, with a dedicated regression test
(`finish_with_empty_glyph_styles`) added in the same version.

## Introduced by

For this repository's own exposure to the bug: 0c5de377 "Bump skrifa (#1774)" (2026-07-29) is the
commit that most recently set the workspace's `skrifa` version requirement to `0.44.0` (the
version this candidate's repro is pinned against) - but the underlying defect itself is **not** a
regression introduced by that commit. The identical unguarded `self.glyph_styles[range]` line is
already present in `skrifa` 0.43.2 (the version pinned immediately before 0c5de377), so this
repository has been exposed to this exact bug for at least the last two `skrifa` version bumps, and
likely longer (not checked further back - `autohint_shaping` was already an enabled feature many
versions earlier per `git log -p -- Cargo.toml`). The code path that actually reaches this bug from
`glifo`'s side (`HintKey::instance`'s call into `HintingInstance::new`) has existed since
5796226b "Import glifo into the vello workspace (#1539)" (2026-03-27), the same commit already
cited as the origin point for candidates 039/040/044/062. Not determined: the exact upstream
`skrifa`/`fontations` commit or version that first introduced the unguarded indexing inside
`GsubHandler::finish` (would require bisecting `googlefonts/fontations`' own history, out of scope
for this session) or the exact upstream commit that fixed it in 0.45.x/0.46.0 (not looked up beyond
confirming the fixed source is present in the locally-cached 0.46.0/0.46.2 crate sources).

## The fix

Bumped the workspace's `skrifa` dependency requirement in the root `Cargo.toml` from `0.44.0` to
`0.46.2` (the newest version already present in the local cargo registry cache, confirmed via
direct source inspection to contain the upstream fix described above), and re-ran `cargo update -p
skrifa@0.44.0` to update `Cargo.lock` accordingly (`skrifa` 0.44.0 -> 0.46.2, its `read-fonts`
dependency 0.41.0 -> 0.43.3, `font-types` 0.12.1 -> 0.12.4 - transitive updates only, no other
workspace dependency changed). No source-code change was needed in `glifo`, `vello_cpu`, or
anywhere else in the workspace - the whole workspace (`cargo check --workspace --all-features`)
compiles cleanly against the bumped version with zero new warnings or errors.

This is sufficient because the fix lives entirely inside the dependency being bumped - it isn't
"papering over one specific malformed byte sequence" the way a `glifo`-side workaround would be;
verified against a second, independently-constructed input in the same bug class (a different
single-byte mutation of the same base font, at a different offset within the same `maxp` directory
record) - both no longer panic after the bump.

Before/after re-running `repro/` (per `just test`, both against a real fresh `git clone` of this
repository at `base_commit` and against the working checkout used during investigation): before the
bump, `repro` panics deterministically (`range end index 1 out of range for slice of length 0` at
`skrifa-0.44.0/src/outline/autohint/shape.rs:635:48`); after `git apply patch.diff` (which bumps
`Cargo.toml`/`Cargo.lock` only) and rebuilding, `repro` prints "did not reproduce - fill_glyphs
completed normally on the mutated font" and exits 0.

- Project test suite (`glifo`, `vello_cpu` - the two crates in the affected code path):
  `cargo test -p glifo`: 22 passed, 0 failed, both before and after the bump.
  `cargo test -p vello_cpu`: 81 passed + 1 doctest, 0 failed, both before and after the bump.
  Also spot-checked `cargo test -p vello_encoding` (31 passed) and `cargo test -p vello` (0 tests,
  1 ignored) after the bump for broader coverage of other `skrifa`-consuming crates in this
  workspace - both pass, no new failures. `cargo check --workspace --all-features` also succeeds
  cleanly after the bump (every crate in the workspace, not just the ones with test suites).

## What's not done / open questions

- This candidate's own literal-`.unwrap()`/`.expect()` sweep of `glifo/src` found nothing new
  beyond what candidate 062 already confirmed - that specific vein (bare `unwrap`/`expect` on a
  `skrifa` `Result`/`Option` inside `glifo`'s own source) does appear genuinely exhausted across two
  independent sessions now. Future sessions revisiting "glifo/src general unwrap/expect sweep"
  as a lead should treat it as closed unless `glifo`'s source changes meaningfully.
  `projects/vello.md`'s risk map should be updated to reflect this (not done as part of this
  candidate - deferred to whichever step transcribes this session's batch results).
  Instead, the same short fuzz check that found this candidate suggests the next fruitful step
  isn't re-grepping `glifo` but fuzzing/updating the pinned `skrifa`/`read-fonts` dependency
  version more broadly - other autohint/GSUB/GPOS code paths inside `skrifa` 0.44.0 may have
  similar unguarded indexing that a longer fuzz campaign (via `fuzz-bug-repro`, run separately)
  targeting hinted glyph runs specifically would likely surface faster than manual code reading.
- The `InvalidAnchorPoint(GlyphId(_), _)` panic message noticed during the same fuzz run (3/6000
  iterations) was not investigated separately - it unwraps at the exact same `glyph.rs:1985` call
  site as candidate 039 (`outline_glyph.draw(...).unwrap()`) and is almost certainly already
  covered by that candidate's fix (which changed the call site to handle any `DrawError`/`ReadError`
  variant generically, not just `TooManyPoints`), not a new bug class.
- Not checked: whether `skrifa`'s autohinting module has other similar unguarded-indexing bugs
  reachable via different corrupted tables (GPOS, other GSUB lookup subtable formats) - this
  candidate stopped once one clear, confirmed, root-caused, upstream-fixed example was found and
  verified, per the soft escalation cap; a longer fuzz campaign against `HintingInstance::new`
  specifically (varying which table gets corrupted, not just `maxp`) would likely find more.
- Whether bumping `skrifa` all the way to a version newer than 0.46.2 (the newest locally cached at
  investigation time) is desirable, or whether an even newer version exists upstream with further
  fixes/breaking changes, was not checked - this session picked the newest version already present
  in the local cargo registry cache specifically to keep the fix verifiable offline.
