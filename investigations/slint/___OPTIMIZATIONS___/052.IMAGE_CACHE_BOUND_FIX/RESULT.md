# 52 - raise the image cache bound and stop discarding the insert failure

Follow-up to candidate 038, which measured (but did not patch) the effect of `IMAGE_CACHE`'s
5 MB decoded-byte bound in `internal/core/graphics/image/cache.rs`. This candidate applies the
two directly-implementable fixes from that backlog entry: raise the fixed 5 MB `clru` capacity to
128 MB (the exact figure 038 already validated as a what-if), and stop silently discarding a
rejected `put_with_weight` insert - log it via `crate::debug_log!` instead so a developer whose
assets still exceed the (now much larger) bound gets a signal. The weak-reference dedup side
table (038's fix direction 1, lower risk but a larger change) and deriving the bound from actual
display resolution (fix direction 2, a bigger design question - see "What's not done" below) are
left as open backlog items, not part of this patch.

## What was tried

Harness: `perf_test/src/bin/image_probe.rs` (existing, from candidate 038's session), extended
in this candidate with a pixel-hash correctness check (FNV-1a over `Image::to_rgba8()` bytes,
comparing the first cold decode against whatever a same-path re-load next returns, cached or
freshly decoded). `cargo run --profile rdebug --bin make_assets` generates the fixed test assets
in `perf_test/assets/` (36 MB, gitignored).

Single-core: `taskset -c 3 ./target/rdebug/image_probe`. This binary is single-threaded and does
no concurrent work, so "multi-core" (plain unpinned invocation) and "single-core" numbers are
expected to and do coincide - both are reported below for completeness per the loop's own
discipline, not because concurrency is exercised here.

| Variant | single-core (`large.png`, cold->reload) | single-core (`large.jpg`) | single-core (`large_alpha.png`) | notes |
|---|---|---|---|---|
| baseline (5 MB bound), run 1 | 20.43 ms -> 17.59 ms (RE-DECODED) | 29.69 ms -> 27.67 ms (RE-DECODED) | 28.61 ms -> 26.90 ms (RE-DECODED) | reload still pays ~full decode cost |
| baseline, run 2 | 18.10 ms -> 17.60 ms (RE-DECODED) | 27.03 ms -> 27.30 ms (RE-DECODED) | 27.28 ms -> 26.91 ms (RE-DECODED) | consistent with run 1 and with candidate 038's original numbers |
| patched (128 MB bound), run 1 | 19.01 ms -> 0.00 ms (cached) | 27.48 ms -> 0.00 ms (cached) | 25.27 ms -> 0.00 ms (cached) | reload is free |
| patched, run 2 | 18.49 ms -> 0.00 ms (cached) | 28.13 ms -> 0.00 ms (cached) | 24.73 ms -> 0.00 ms (cached) | consistent with run 1 |

`while held` (second load while the first `Image` is still alive - the deduplication path) shows
the same pattern: baseline re-decodes every time (~17-27 ms even with a live reference, since the
cache is the only dedup mechanism, per 038), patched serves the same clone in ~0.01-0.02 ms.

Every image at or under the old 5 MB bound (`small.png`, `medium.png`, `medium.jpg`,
`cliff_800x600.png` through `cliff_1600x1000.png`) was already cached before this patch and stays
cached after it - the patch only changes behavior for images that cross the old cliff.

## Result

Reload of a 5.9 MB decoded image (1920x1080 RGB8/RGBA8, PNG or JPEG) drops from ~18-29 ms
(re-decoded every time, cache silently rejecting the insert) to ~0.00-0.02 ms (served from
cache), on both the "still held" and "after drop" paths. This reproduces candidate 038's own
128 MB what-if measurement against the real applied patch, not just the synthetic bound-only
test: same magnitude, same direction, no new noise introduced. Every image that was already under
the old 5 MB bound is unaffected (still ~0.00-0.01 ms on reload, as before).

## Does it change behavior/output?

- **Bit-exact?** Yes. The pixel-hash check added to `image_probe` (FNV-1a over
  `Image::to_rgba8()` bytes) confirms the reloaded image is byte-identical to the first decode,
  both before this patch (where the reload is a fresh decode of the same file - "pixels match" on
  every asset, including the ones that were already re-decoding) and after it (where the reload
  is served from the cache - "pixels match" on every asset, including the six that crossed the
  old cliff). The patch changes *when* a decode happens and whether an error is logged, not what
  gets decoded.
- **Is this a breaking change?** No public API changes. The only externally observable difference
  is a new `debug_log!` line when an image still exceeds the (now 128 MB, was 5 MB) cache bound -
  `debug_log!` is a no-op in release builds without the relevant feature/backend configured for
  it (same mechanism already used elsewhere in this crate, e.g. `model.rs`, and for the existing
  SVG/decode-error logging two lines above the patched line in this same file).
- Project test suite: `cargo test -p i-slint-core --lib graphics::image::cache --features
  image-decoders` - 1 passed before and after (`test_path_cache_invalidation`, the module's only
  test, unaffected since it uses images well under both the old and new bound).
- Memory: the cache can now hold up to 128 MB of decoded image bytes instead of 5 MB, worst case
  (an application that churns through many large, distinct images) trading that much more
  resident memory for not re-decoding. This is the same tradeoff candidate 038 already flagged:
  5 MB may be intentional for embedded targets, so a blanket 128 MB default is a size choice, not
  free - see "What's not done" below.

## Why the result looks the way it does

The bound is a single `clru::CLruCacheConfig::new(NonZeroUsize)` argument gating every
`put_with_weight` call in `lookup_image_in_cache_or_create`; raising it does exactly what
candidate 038 already predicted since nothing else in the decode/cache path changed. The
`debug_log!` addition is a pure observability change on the (now much rarer) rejection path and
has no measurable performance effect of its own - it only fires when an image still exceeds
128 MB decoded, which none of this harness's assets do.

## What's not done / open questions

- **Weak-reference dedup side table (038's fix direction 1, lowest risk of the three).** Not
  implemented here. It would fix the "while held" case unconditionally (regardless of the bound,
  cannot regress memory since it's weak), whereas this patch only fixes it up to 128 MB. Left in
  `IDEAS.md`'s backlog as its own item, separate from this candidate, since it's a structurally
  different (and larger) change - a new side table keyed the same way as `ImageCacheKey`, plus
  upgrade-weak-to-strong logic at the lookup site.
- **Deriving the bound from display resolution instead of a fixed constant.** Not attempted - it
  needs a design decision (per-window? global? what happens before any window exists, e.g. during
  headless use or interpreter startup?) and access to display info at `IMAGE_CACHE`'s
  `thread_local!` initialization point, which doesn't have one today. Raising the constant is the
  directly-measurable subset of this backlog item; making it resolution-aware and configurable is
  left open.
- **128 MB was chosen to match candidate 038's own validated figure**, not derived from any
  display or platform analysis - a real fix would likely want this configurable (e.g. an
  application-set capacity) rather than a different fixed constant, especially since the original
  5 MB may have been deliberately sized for embedded targets with much less RAM.
- Not measured: GPU renderers' own texture caches (femtovg/skia), which may sit above this one and
  already avoid the problem for on-screen images - candidate 038 flagged this as unmeasured too.
