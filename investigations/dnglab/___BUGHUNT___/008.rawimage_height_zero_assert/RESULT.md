# `RawImage::calc_black_levels` panics on a decoded image with `height == 0`

Sibling bug to candidate 004 (`decompress_lines_zero_width_panic`): the same file-tag-derived-
dimension-with-no-minimum-size-validation class, but a different code path/assertion, hit only when
`ImageLength` (height) is 0 while width is non-zero - the value candidate 004's boundary sweep
surfaced but didn't investigate (see that candidate's "What's not done").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `RawImage::calc_black_levels`/`RawImage::new_with_data` (`rawler/src/rawimage.rs`), starting from the exact `rawimage.rs:287` location candidate 004 recorded | - | Found `new_with_data` (called from `DngDecoder::raw_image`, and `RawImage::new` the same way) calls `Self::calc_black_levels(...)` to compute a fallback black level whenever the camera config/hints/caller don't already supply one and `dummy` is false. `calc_black_levels` checks `cfa.width * cfa.height == 0` (returns `None` early) but then unconditionally `assert!(!image.is_empty())` - with no check that the *image* itself is non-empty, only that the CFA pattern is non-degenerate |
| Standalone repro: a hand-built, spec-shaped uncompressed DNG with `ImageWidth=5, ImageLength=0` (via rawler's own low-level `formats::tiff::writer::TiffWriter`, decoded via the real, public `rawler::analyze::raw_to_srgb` - the same entry point `dnglab analyze --srgb` uses) | debug build, 3 runs; release build, 1 run | Deterministic panic at `rawimage.rs:287`, 3/3 debug + 1/1 release, identical location/message every run |
| Same repro via `rawler::decode_file` (a different real public entry point - returns `RawImage` directly, stopping right at construction) | debug build, 1 run | Also panics at `rawimage.rs:287` internally, but `RawLoader::decode` wraps decoding in `panic::catch_unwind` (`rawler/src/decoders/mod.rs:1096`), so the panic is caught and turned into a graceful `Err("...Caught a panic while decoding...")` rather than an uncaught process crash - noted below, doesn't change that the assert itself is a real defect, just that its externally-visible severity depends on which entry point the caller uses |
| Boundary-value variant: same shape, `ImageWidth=200, ImageLength=0` (a different width in the same "height=0" class, via `decode_file`) after the fix | debug build | Fix holds - `decode_file ok: 200x0`, no panic (CONFIRMATION.md section 7) |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |
| Standalone `repro/`'s own `justfile` against a genuine fresh clone of `base_commit` | release build | `just reproduce-before`: panics identically against the pristine clone (recipe fails with exit 101, as expected - proves the bug independent of this checkout). `just apply && just reproduce-after`: patched clone decodes cleanly, `decoded ok (bug not present): 5x0` |

## Status

**Confirmed.** Reproduces 3/3 debug runs and 1/1 release run via `raw_to_srgb`, fully deterministic,
identical panic location (`rawimage.rs:287`) and message every time. Not gated by
`overflow-checks`/build profile - `assert!` (not `debug_assert!`) fires unconditionally in both
debug and release.

**Fully reproducible**: **Yes**. `repro/` writes a real, spec-shaped DNG file to disk (differing
from a normal file only in its `ImageLength=0` tag value) and decodes it through the unmodified,
public `rawler::analyze::raw_to_srgb` - the same function `dnglab analyze --srgb <file>` calls.
Single-participant case (a malformed file handed to a parser's real public entry point is the
genuine external input), no second participant to fake, per `CONFIRMATION.md` section 11.

## Class

`crash`.

## Root cause

`RawImage::calc_black_levels` (`rawler/src/rawimage.rs:279-287`):

```rust
pub fn calc_black_levels<T>(cfa: &CFA, blackareas: &[Rect], width: usize, _height: usize, image: &[T]) -> Option<BlackLevel>
where
  T: SubPixel,
{
  let x = cfa.width * cfa.height;
  if x == 0 {
    return None;
  }
  assert!(!image.is_empty());
  ...
```

is reached from `RawImage::new_with_data` (used by `DngDecoder::raw_image`, the DNG decode path)
and `RawImage::new` (used by several other format decoders), both of which call it as a fallback
when computing `blacklevel`:

```rust
.or_else(|| {
  if dummy {
    Some(BlackLevel::default())
  } else {
    match &image {
      RawImageData::Integer(pix) => Self::calc_black_levels(&cam.cfa, &blackareas, sample_width, height, pix),
      ...
```

`sample_width`/`height` (and therefore the length of `image`) come straight from the file's own
`ImageWidth`/`ImageLength` tags with no minimum-value validation anywhere upstream - the same
"file-tag-derived dimension, no minimum-size validation" root shape as candidate 004, just at a
later pipeline stage (black-level computation during `RawImage` construction, not decode). For a
normal Bayer camera, `cfa.width * cfa.height` is 4 (non-zero), so the `x == 0` early return doesn't
fire - the function proceeds straight to `assert!(!image.is_empty())`, which fails whenever
`width * height * cpp == 0`, i.e. whenever either dimension is 0 (candidate 004's decode-stage bug
already independently guards against `width == 0`, but nothing guards `height == 0`, and this
`assert!` is a separate code path entirely, unaffected by candidate 004's fix).

Note this is functionally reachable in two different severities depending on which public entry
point the caller uses: through `rawler::analyze::raw_to_srgb` (used by `dnglab analyze --srgb`),
the panic is genuinely uncaught and crashes the calling process. Through `rawler::decode_file`
(used by `RawLoader::decode`), the same panic is internally caught by
`panic::catch_unwind` (`rawler/src/decoders/mod.rs:1096`) and turned into a graceful `Err` - so it
does not crash that caller, only wastes the panic-unwind machinery and prints a panic message to
stderr. Both are real, but `raw_to_srgb`'s uncaught crash is the more severe manifestation and is
what `repro/` demonstrates.

## Introduced by

0977872e "Add missing CR2 and CRW cameras and testdata" (2022-06-12) - `git log -S'assert!(!image.is_empty())' -- rawler/src/rawimage.rs`
shows this is the commit that introduced `calc_black_levels` (and this exact `assert!`) in the
first place; it has been present, unchanged in this respect, ever since.

## The fix

Replaced the unconditional `assert!(!image.is_empty())` with an early `return None` folded into the
existing `x == 0` guard right above it - `calc_black_levels` already returns `Option<BlackLevel>`
and every caller already handles `None` via `.unwrap_or_else(|| BlackLevel::zero(1, 1, cpp))`, so
this needs no signature/caller change, just extending the existing "can't compute a black level for
this degenerate case" early-exit to also cover an empty image, not just a degenerate CFA pattern:

```rust
let x = cfa.width * cfa.height;
if x == 0 || image.is_empty() {
  return None;
}
```

Verified: `repro/` (`ImageWidth=5, ImageLength=0`) no longer panics at `rawimage.rs:287` in either
debug or release, in both the `raw_to_srgb` and `decode_file` variants. Re-verified against a
`width=200, height=0` variant (still cleanly handled by the same guard) to confirm the fix isn't
narrowly matched to the exact triggering byte values (CONFIRMATION.md section 7).

In the `raw_to_srgb` (debug build) path specifically, after this fix the pipeline proceeds further
and hits a *different*, already-known, already-documented (and already fixed-and-reverted per this
workspace's own policy) bug: candidate 001's `ppg.rs` demosaic underflow (`attempt to subtract with
overflow` at `ppg.rs:120`, gated by `overflow-checks`/debug build only - it doesn't fire in release,
where integer subtraction just wraps instead of panicking). This is expected, not a failure of this
fix: it's the same confound candidate 004's own RESULT.md already recorded when it found this exact
input (`width=5, height=0`) hitting *this* candidate's assert - now that this assert is fixed, the
next bug in the same tiny image's pipeline is candidate 001's, not a new one. In release mode (and
via the `decode_file` entry point in either build), the fix produces a fully clean, panic-free
result end to end - see `output.txt`.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises a zero-height image, so this fix is
  additive coverage, not a regression risk.

## What's not done / open questions

- Same as candidate 004's own note: this is the file-tag-controlled-dimension class of bug (no
  central validation of `ImageWidth`/`ImageLength`/`SamplesPerPixel` against a sane minimum
  anywhere before `RawImage` construction or decode). A single shared validation point would likely
  close most of this bug class at once rather than patching each downstream consumer (this
  candidate's `calc_black_levels`, candidate 004's `decompress_lines`/`decompress_strips` family)
  separately - worth considering as a follow-up architectural fix, out of scope for this session's
  bounded patch.
- `calc_black_levels` is only reachable from decoders that go through `RawImage::new`/
  `RawImage::new_with_data` with `dummy == false` and no camera-config-supplied black level/hint -
  not independently re-checked against every other format decoder sharing this constructor (ARW,
  NEF, RW2, IIQ, etc., the same list candidate 004 named) for whether each can independently produce
  a `height == 0` `RawImage` the same way the DNG path does; the fix itself is shape-independent of
  which decoder called in, so it protects all of them identically, but this session only built a
  DNG-shaped repro to demonstrate it.
- Not investigated: whether an `ImageWidth == 0` (rather than `ImageLength == 0`) input can also
  still reach this exact assert independently of candidate 004's own fix (candidate 004's fix
  already rejects `width == 0` earlier, at decode time, before a `RawImage` is even constructed for
  most decoders - this candidate's fix is a second, independent layer of defense for whichever
  dimension is zero, reached if a decoder path ever bypasses candidate 004's decode-stage guard).
