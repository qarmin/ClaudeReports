# PPG demosaic: `usize` underflow on raw images narrower/shorter than 6px

`rawler::imgop::sensor::bayer::ppg` implements the PPG debayer algorithm used both by
`RawDevelop::develop_intermediate` (the fallback preview-generation path, `rawler::analyze::raw_to_srgb`,
i.e. `dnglab analyze --srgb`) and by `dng::convert`'s Fuji-rotation path. Its three interior-pixel
passes bound their loops with `.take(h - 6)` / `.take(w - 6)` to stay clear of a 3px border handled
separately - `HEURISTICS.md`'s "arithmetic on sizes/lengths/indices derived from input" pattern:
`height`/`width` come straight from the decoded `RawImage`'s dimensions, which for a TIFF/DNG-family
decoder are read directly from the file's own `ImageWidth`/`ImageLength` tags with no minimum-size
validation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading against `HEURISTICS.md`'s "arithmetic on sizes derived from input" pattern | Read `interpolate_green`/`interpolate_rb_at_green`/`interpolate_rb_at_non_green` in `ppg.rs` | Found `.take(h - 6)` / `.take(w - 6)` with no guard against `h`/`w` < 6 |
| Standalone repro: hand-built tiny (5x5) but spec-valid uncompressed RGGB DNG via rawler's own public `DngWriter`, decoded via the real public `rawler::analyze::raw_to_srgb` (same fn `dnglab analyze --srgb` uses) | debug build (`cargo run`, default profile - debug-assertions on) | Deterministic panic, 3/3 runs, `attempt to subtract with overflow` at `ppg.rs:120` |
| Same repro, `cargo run --release` (release profile: `overflow-checks = false`, matching dnglab's own `[profile.release]`) | release build | No panic ("decoded ok: 5x5") - the subtraction silently wraps to `usize::MAX` instead, see Root cause |
| Boundary-value sweep after applying the fix (1x1, 8x4, 10x3, 3x10, 6x6, 7x7, 2x2, 20x1, 1x20) | debug build | All succeed, no panic, no behavior change for the already-safe cases |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after (unaffected - no existing test exercises PPG on an image this small) |
| Same 5x5 repro rebuilt with `RUSTFLAGS="-Z sanitizer=address" cargo +nightly run --release --target x86_64-unknown-linux-gnu` against pristine (unpatched) code | release build + AddressSanitizer | **heap-buffer-overflow**, 1/1 run (ASAN aborts on first detection so a repeat-run count isn't meaningful here - see `CONFIRMATION.md` section 5): "READ ... 44 bytes after 300-byte region" inside `interpolate_green`'s inlined `rayon` `for_each` closure, called from `PPGDemosaic::demosaic` <- `RawDevelop::develop_intermediate` <- `raw_to_srgb` - full report in `output_asan_release.txt`. 300 bytes = 5×5×3×4 (the 5×5 `Color2D<f32,3>` buffer exactly); 44 bytes past it lands ~11 `f32`s into the next heap object/redzone, consistent with the `row+2`/`col+2` OOB reads predicted by the Root cause below |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic, debug build (`cargo run`, default profile with
debug-assertions enabled - the same profile `cargo test`/`cargo fuzz` use). Does not panic in a
`--release` build because dnglab's own `[profile.release]` sets `overflow-checks = false` (this is
also plain Rust's default `--release` behavior even without that explicit setting) - see Root cause
for why the release-mode silent wraparound is not a "the bug goes away in release" case.

**Fully reproducible**: **Yes**. `repro/` builds a genuinely tiny-but-valid DNG file on disk with
rawler's own public `DngWriter` (a different subsystem than the one being tested), then decodes it
through the real, public, unmodified `rawler::analyze::raw_to_srgb` entry point - the exact function
the `dnglab analyze --srgb <file>` CLI command calls. This is a single-participant case (a malformed/
unusual file handed to a parser's real entry point) - no second participant to fake, per
`CONFIRMATION.md` section 11.

## Class

`memory-safety` - confirmed as a heap-buffer-overflow **read** under AddressSanitizer in a release
build (see "What was tried" and `output_asan_release.txt`); also `crash` in a debug build (panics
instead, before any unsound memory access happens).

## Root cause

`interpolate_green`, `interpolate_rb_at_green`, and `interpolate_rb_at_non_green` (`ppg.rs:120`,
`179`, `216`) each do:

```rust
img.pixels_mut().par_chunks_exact_mut(w).enumerate().skip(3).take(h - 6).for_each(|(row, buf)| {
  for (col, pixel) in buf.iter_mut().enumerate().skip(3).take(w - 6) {
```

`.skip(3).take(h - 6)` is meant to restrict `row` to `[3, h-4]` (leaving a 3px border on each side,
which `interpolate_borders` handles separately with its own, correctly `saturating_sub`-guarded
bounds). When `h < 6` (or `w < 6`), `h - 6` (`usize`) underflows:

- **Debug build**: panics immediately with "attempt to subtract with overflow" - a crash, but a
  safe one (the process aborts before anything unsound happens).
- **Release build** (`overflow-checks = false`, dnglab's actual shipped configuration): wraps to a
  huge value (`usize::MAX - 5` or similar). `.take()` on that huge count does not itself panic or
  read out of bounds - it's simply not the limiting factor anymore, since `par_chunks_exact_mut(w)`
  already bounds `row` to the image's *actual* number of complete rows. The real consequence is that
  the intended upper bound `row <= h - 4` silently stops being enforced: for a 5-row image, `row` can
  now reach `3` and `4` (the last two chunks `skip(3)` leaves), and the loop body unconditionally
  reads `dataptr.at(row + 1, col)` / `dataptr.at(row + 2, col)` (and the `col`-direction equivalents)
  via `Color2DPtr::at`, whose only bounds check is `debug_assert!(row < self.height && col <
  self.width)` (`pixarray.rs:586`) - compiled out in release. For `row = 4, height = 5`,
  `row + 2 = 6` is one-plus-one past the last valid row: `ptr.add(row * width + col)` computes a
  pointer 1-2 full rows past the end of the `Color2D`'s backing `Vec<[f32; 3]>` and dereferences it
  unchecked. That is out-of-bounds pointer arithmetic and an out-of-bounds read - undefined behavior
  regardless of whether it happens to land inside the same heap allocation's slack space (as it did
  in this session's release-mode run, which returned a plausible-looking image with no visible
  crash) or reads/crashes on adjacent heap memory (allocator- and build-dependent).

## The fix

Changed `.take(h - 6)` -> `.take(h.saturating_sub(6))` and `.take(w - 6)` -> `.take(w.saturating_sub(6))`
in all three functions (`ppg.rs:120,121,179,180,216,217`). When `h`/`w` >= 6 this is identical to the
original expression (no behavior change for normal-sized images - confirmed via the existing 66-test
suite passing unchanged before/after). When `h`/`w` < 6, `.take(0)` makes the interior pass a no-op,
which is correct: `interpolate_borders` (already `saturating_sub`-guarded, unaffected by this bug)
is the only pass whose output is defined for pixels this close to every edge of a sub-6px image.

Verified: `repro/` (5x5) no longer panics in debug and produces a `5x5` image (`decoded ok: 5x5`).
Re-verified against the boundary sweep in "What was tried" (1x1 up to 20x1/1x20, spanning `w`/`h`
both above and below 6) - all succeed with no panic and no change to the already-correct large-image
codepath.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no test in the suite exercises PPG demosaic below 6px in either
  dimension, so this fix is additive coverage, not a regression risk to existing behavior.

## What's not done / open questions

- Not independently re-run under `miri` this session (ASAN's release-mode heap-buffer-overflow
  finding above already meets `CONFIRMATION.md`'s bar for `memory-safety`; `miri` would be
  redundant confirmation, not new information, for this specific candidate).
- A second, independent bug in the same file was noticed while boundary-testing the fix (see
  "Critical pitfalls" #7 - same suspect area, different mechanism): `interpolate_borders`'s
  "jump to right border" logic (`ppg.rs:79-81`) hangs (infinite loop, not a panic) for
  `width` in `{4, 5}` combined with `height >= 7`. This is **not** fixed by this candidate's patch
  (`interpolate_borders` is untouched) and is filed separately - see `002.ppg_interpolate_borders_infinite_loop/`.
- Other `Color2DPtr::at`/raw-pointer-based callers with similar `debug_assert!`-only bounds
  (`imgop/sensor/bayer/bilinear.rs`, `decoders/iiq.rs`, `imgop/fuji_rotate.rs`,
  `decompressors/radc.rs`, `imgop/sensor/xtrans/markesteijn.rs`) were noticed but not checked this
  session for the same underflow-adjacent class of bug - see `IDEAS.md`.
