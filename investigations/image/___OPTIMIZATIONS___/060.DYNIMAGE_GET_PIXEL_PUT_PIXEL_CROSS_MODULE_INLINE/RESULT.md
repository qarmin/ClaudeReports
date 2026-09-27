# `DynamicImage::get_pixel`/`put_pixel`: `#[inline(always)]`/`#[inline]` across the module boundary into `imageops::overlay`

`DynamicImage`'s `GenericImageView::get_pixel` (`src/images/dynimage.rs:1754`) and
`GenericImage::put_pixel` (`:1760`) are un-annotated 12-arm `dynamic_map!`/match shims. They are
called from a different module (`src/imageops/mod.rs`'s `overlay`, and transitively `tile`,
`replace`, `vertical_gradient`, `horizontal_gradient`) every time a user composites onto or reads
from a `DynamicImage` - the crate's own primary public type. This is the same cross-module
small-accessor inlining pattern candidate 054 already confirmed for `colorops.rs`'s
`brighten_in_place`, one abstraction level up (the crate's own top-level enum type rather than a
single `ImageBuffer` instantiation), and was flagged as an unconverted lead in candidate 059's
own backlog note (`IDEAS.md` row for 2026-08-29).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. No existing bench exercised
`DynamicImage`-driven `overlay`/`tile` (per row 14 of `IDEAS.md`), so a `"dyn image overlay"` case
was added to `benches/imageops.rs` (bundled into `patch.diff`, same `iter_batched(..,
BatchSize::LargeInput)` shape candidate 054 used): a 1920x1080 `DynamicImage::ImageRgba8` bottom
overlaid with a 300x300 `DynamicImage::ImageRgba8` top that has partial alpha (180/255), so the
blend actually mixes colors rather than just overwriting - both operands are real, separately
allocated `DynamicImage` enum values (not the same value reused), so the enum discriminant is a
genuine runtime unknown to the compiler at the `overlay` call site, not something LLVM could
constant-fold away up front.

Compared two prebuilt `--release` `imageops` bench binaries (baseline vs. patched source) via 10
alternating paired runs per mode, each `--warm-up-time 1 --measurement-time 1.2`
(`paired_overlay.sh` in this folder, same script shape as candidate 054's
`paired_colorops.sh`). A same-binary self-check (baseline binary vs. an independently-rebuilt,
byte-identical copy of itself) was run first to establish the noise floor. Full raw output:
`raw_pairs.txt`.

| Variant | single-core (`taskset -c 3`) p50 | multi-core (unpinned) p50 | notes |
|---|---|---|---|
| noise floor: baseline vs. rebuilt-identical baseline (two independent 8-pair self-checks) | run 1: 0.9432 -> 0.9431 ms (+0.0%, 5/8 pairs); run 2: 0.9644 -> 0.9537 ms (-1.1%, 3/8 pairs) | not separately re-checked (single-core check sufficed - see MEASUREMENT.md) | md5-identical binaries; noise floor is inconsistent in direction and up to ~1.1% in magnitude, well under the -6.8% patched delta |
| baseline (unpatched, un-annotated `get_pixel`/`put_pixel`) | 0.9406 ms | 0.9391 ms | both accessors have standalone `nm` symbols; overlay's hot loop makes 2 indirect (`call r12`, a jump-table dispatch over the 12 `dynamic_map!` arms) + 1 direct call per pixel |
| patched (`#[inline(always)]` on `get_pixel`, `#[inline]` on `put_pixel`) | **0.8767 ms (-6.8%)**, 10/10 pairs favor patch | **0.8764 ms (-6.7%)**, 10/10 pairs favor patch | neither accessor has a standalone `nm` symbol; `objdump` shows the per-pixel calls in `overlay`'s hot loop replaced by direct calls to `Rgba<u8>::blend` (still present, expected - the loop's own arithmetic step) with the two former dynamic-dispatch calls gone entirely |

## Result

**A real, consistent, moderate win for `imageops::overlay` on `DynamicImage` operands: -6.8%
single-core / -6.7% multi-core median wall-clock, 10/10 paired runs favoring the patch in both
modes** (well outside the ~1.1%-magnitude same-binary noise floor measured above). This bench is single-threaded (no
`rayon` involvement in `overlay`), so the single-/multi-core numbers track each other closely, as
expected - "multi-core" here means "unpinned, realistic scheduling", not "uses more cores".
Smaller in magnitude than the fully-specialized, same-enum-variant throwaway example from the
pre-verification note (which collapsed the loop to raw arithmetic with *zero* per-pixel calls or
branches) - that example's magnitude was itself flagged there as an artifact of both operands
being literally the same runtime value; this measurement uses two independently-allocated
`DynamicImage`s of the same variant but is a fairer, more realistic proxy for real `overlay` usage
between two independently-constructed images.

Time was the only dimension measured; the mechanism (removing indirect/direct per-pixel function
calls) has no plausible effect on allocation count or peak RSS (`overlay` does no per-pixel
allocation in either variant), so no RSS/allocation measurement was taken.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_check_060.rs`, not
  part of `patch.diff`, removed before reverting) exercised `overlay` (RGB8 bottom/RGBA8 top,
  RGBA8 bottom/RGBA8 top, negative and out-of-bounds-clipped offsets), `tile`, `replace`,
  `vertical_gradient`, and `horizontal_gradient` on `DynamicImage`s, then read every output pixel
  back via `DynamicImage::get_pixel` directly and combined an FNV-1a hash of every buffer. The
  combined hash (`b3782e04fb902881`) matched byte-for-byte between an unpatched-source build and
  this candidate's patched-source build.
- **Is this a breaking change?** No - no public signature, panic behavior, or documented result
  changed; only `#[inline]`/`#[inline(always)]` attributes were added to two existing trait-impl
  methods.
- Project test suite (`cargo test --release`): **550 passed / 1 failed, identically before and
  after.** The one failure is the same pre-existing, unrelated `tests/regression.rs::
  resizing_with_catmul` every prior BMP/`pic-scale-safe`-adjacent candidate in this workspace has
  already documented (independently confirmed on this session's own unpatched baseline too, not
  something this patch introduced).

## Why the result looks the way it does

Confirmed via `nm -C`/`objdump -d -C` on the bench binary before and after. Baseline: both
`<DynamicImage as GenericImageView>::get_pixel` and `<DynamicImage as GenericImage>::put_pixel`
have standalone symbols (LLVM's ordinary inlining heuristic declines to inline the 12-arm
`dynamic_map!`/match body into `overlay`'s call sites). Inside `overlay`'s compiled body, the
per-pixel loop makes 2 indirect calls (`call r12` - a computed jump table over the 12
`dynamic_map!` arms sharing a signature, for the two `get_pixel` calls) plus 1 direct call (a GOT
entry to `put_pixel`) - 3 real dynamic-dispatch-shaped calls per pixel, matching the
pre-verification note's own count. After adding `#[inline(always)]` to `get_pixel` and `#[inline]`
to `put_pixel`: neither symbol exists standalone anymore, and `overlay`'s own disassembly shows
those 3 calls replaced by direct calls into `<Rgba<u8> as Pixel>::blend` (12 call sites - one per
possible resulting variant combination the inliner produced while expanding both match arms
in-line, plus a handful of cold `slice_index_fail`/`panic_fmt` calls that are compiled-in bounds
checks, never hot). This matches candidate 054's own finding for `brighten_in_place`: a small,
frequently-called cross-module accessor whose own body is otherwise cheap gets a real,
measurable win once LLVM's default inlining threshold (lost due to the accessor's own body size -
here, the 12-arm match plus, for `get_pixel`, a `to_rgba().into_color()` conversion) is overridden
explicitly.

`get_pixel` needed the stronger `#[inline(always)]` (not just `#[inline]`) because its body is
larger than `put_pixel`'s (the `to_rgba().into_color()` conversion after the per-variant
`get_pixel` call) - this matches the pre-verification note's own finding that plain `#[inline]`
alone inlined `put_pixel` but left `get_pixel` as a standalone symbol.

## What's not done / open questions

- `tile`/`replace`/`vertical_gradient`/`horizontal_gradient` were not separately benchmarked (no
  existing bench cases for them either) - they call the same `get_pixel`/`put_pixel` accessors
  (`tile` via `overlay` itself; the gradient functions directly) and were exercised only for
  correctness, not timed. A plausible small follow-up: add bench cases for these too, though the
  underlying mechanism is identical to what's measured here, so a materially different result
  seems unlikely.
- The related, lower-confidence `SubImageInner::get_pixel`/`put_pixel` forwarding-shim candidate
  (`src/images/sub_image.rs:205`/`:242`, flagged in the same `IDEAS.md` backlog entry as this one)
  was not investigated in this session - left in the backlog as its own, separate item.
- The pre-verification note's fully-specialized (zero-call, zero-branch) throwaway example used
  two operands that were the *same* `DynamicImage` runtime value; this candidate's measurement
  intentionally uses two independent values instead for realism, which is why the measured -6.8%
  is smaller than that note's qualitative description - not re-measured against the artificial
  same-value scenario here, since it isn't representative of real `overlay` call sites.
