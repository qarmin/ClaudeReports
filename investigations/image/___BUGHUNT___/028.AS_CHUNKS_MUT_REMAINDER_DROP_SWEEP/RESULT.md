# AS_CHUNKS_MUT_REMAINDER_DROP_SWEEP

Follow-up from candidate 027 (`CHUNKS_MUT_SEPARATE_COUNTER_SWEEP`, Not reproducible), which
deliberately excluded `as_chunks_mut::<N>()` (const-generic slice method, `chunks_exact_mut`-like
semantics: remainder dropped via the `.1` tuple field, not shortened like `chunks_mut`). This
candidate sweeps every `as_chunks_mut::<N>()` call site in the repository for a separately-tracked
counter/index that assumes the const-generic `N` covers the whole buffer, desyncing against the
silently-dropped `.1` remainder.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep sweep of the whole repository for the literal `as_chunks_mut` | `grep -rn "as_chunks_mut" --include=*.rs .` (excluding `target/`) | 46 call sites across `src/primitive_sealed.rs`, `src/codecs/pnm/decoder.rs`, `src/codecs/farbfeld.rs`, `src/codecs/ico/decoder.rs`, `src/codecs/tiff.rs`, `src/codecs/avif/decoder.rs`, `src/codecs/avif/yuv.rs`, `src/codecs/avif/ycgco.rs`, `src/codecs/png.rs`, `src/codecs/tga/decoder.rs`, `src/utils/mod.rs`, `src/codecs/hdr/decoder.rs`, `src/metadata/cicp.rs`, `src/imageops/fast_blur.rs`, `src/codecs/bmp/decoder.rs`, `src/imageops/filter_1d.rs`, `src/imageops/sample.rs`, plus one in `benches/bgra_to_rgba.rs`. Only 3 sites (all in `src/imageops/filter_1d.rs:244/273/300`) ever read the `.1` remainder field at all - every other site uses `.0` (the full-chunks slice) exclusively, discarding `.1` implicitly. |
| Manual code reading of every call site for a separately-advanced counter/index used to write/read elsewhere, unaware some elements were dropped as an unprocessed remainder | Read each site (or its enclosing function) in full context | No instance of the suspected shape. The large majority pair `as_chunks_mut::<N>().0.iter_mut()` with `.zip(...)` against another iterator (a source slice, a `chunks_exact`/`chunks_exact_mut` row iterator, or a plain `.iter()`) - `zip` self-truncates to the shorter side, so there is no separate counter to desync; any genuine remainder just means fewer pairs get produced, not an out-of-bounds or misaligned write. Every site where `N` could plausibly not evenly divide the slice length is guarded by an explicit `assert_eq!` on the total length before chunking (`filter_1d.rs::make_arena_row`/`make_columns_arenas`, `tga/decoder.rs`'s RGB15->RGB24 expansion via paired `as_chunks::<2>`/`as_chunks_mut::<3>`), or the invariant is structural (RGBA buffers sized as `width*height*4`, so `as_chunks_mut::<4>()` never has a remainder: `ico/decoder.rs:411`, `bmp/decoder.rs:1863`). The one call site that actually consumes `.1` (`filter_1d.rs::filter_symmetric_column`'s manual SIMD-tail pattern: `as_chunks_mut::<32>()` -> `.1` -> `as_chunks_mut::<16>()` -> `.1` -> `as_chunks_mut::<4>()` -> `.1` -> per-element tail) tracks a separately-advanced `cx` counter alongside each stage, but `cx` is derived purely from `dst_rem`'s own actual length at each stage (re-derived from the live `dst_rem` slice, not a fixed nominal count), so it stays in lock-step with the real remainder by construction; the final per-element loop (`dst_rem.iter_mut().zip(cx..dst_stride)`) uses `cx` only to zip against a range for indexing into `arena_src`, which is itself always at least `dst_stride` long (arena rows are padded to `image_size.width + 2*pad_w`, always >= the unpadded row width). `avif/yuv.rs:606`'s odd-width chroma remainder handling (`rgb_chunks.into_remainder().as_chunks_mut::<CHANNELS>()`) uses `.rev()` iterators on the U/V planes zipped against a single-element remainder - deliberately correct (grabs the last chroma sample without computing an explicit index), not a counter-mismatch bug. `avif/decoder.rs`'s two alpha-compositing sites (lines 571, 772) are explicitly guarded by a `picture.width() != width \|\| picture.height() != height` size-equality check (with an inline comment already documenting why, from a prior fix) before the `zip`-based alpha copy, so a size mismatch is now rejected with an error rather than silently truncating. |
| Debug-assertions + overflow-checks build (`RUSTFLAGS="-C debug-assertions=on -C overflow-checks=on"`): (a) ran the existing `cargo test --lib` suite scoped to every codec/imageop module touched by an `as_chunks_mut` call site (`codecs::{tga,ico,pnm,farbfeld,hdr,bmp,tiff}`, `imageops::{filter_1d,sample,fast_blur}`, `metadata::cicp`); (b) wrote a standalone boundary-value test exercising `DynamicImage::blur`/`unsharpen` (drives `filter_1d.rs`'s `cx`-tracked SIMD-tail path, the one real `.1`-consuming call site) and `imageops::fast_blur` across a matrix of odd/prime widths (1, 3, 5, 7, 15, 17, 31, 33, 37, 63, 65, 97, 129, 161) x heights (1, 2, 5, 9) - deliberately targeting the 32/16/4/1-element chunk-size boundaries `filter_symmetric_column`/`filter_symmetric_row` step through, for `Rgba8`, `Luma8`, and `Rgb8` | `cargo test --lib -- codecs::tga codecs::ico codecs::pnm codecs::farbfeld codecs::hdr codecs::bmp codecs::tiff imageops::filter_1d imageops::sample imageops::fast_blur metadata::cicp` (114 tests) and a temporary `tests/scratch_as_chunks_boundary.rs` (deleted after the run, not committed) | All 114 existing tests passed. The boundary-value test completed all 14x4 = 56 width/height combinations across 3 pixel types (168 blur/unsharpen/fast_blur calls total) with no panic, no `assert_eq!` failure, and correct output dimensions at every combination - confirms `filter_1d.rs`'s `cx`/remainder bookkeeping holds even at chunk-size boundaries (e.g. width=32 exactly, width=33 crossing into the 16-chunk stage, width=4 landing exactly on the 4-chunk boundary with a zero-length tail). AVIF's `avif-native` alpha-compositing call sites (the only ones not exercised this way - `avif-native`/`dav1d` isn't buildable in this environment without the documented pkg-config shim per `___BUGHUNT___/README.md`) were only checked via code reading (see row above), not built/run. |

## Status

**Not reproducible.** An exhaustive grep of the whole repository for `as_chunks_mut` (46 call
sites, all non-test/non-bench source) found no instance of a separately-tracked counter desyncing
from the actual (possibly-fewer) chunks `as_chunks_mut::<N>()` produces when the input isn't an
exact multiple of `N`. The overwhelming majority of call sites use `zip`-based iteration, which
self-truncates safely by construction rather than relying on an independently-computed count; the
few sites with a fixed-size invariant enforce it via `assert_eq!` before chunking or via a
structurally-guaranteed exact-multiple buffer size; and the one call site that manually tracks a
counter across successive remainder-narrowing stages (`filter_1d.rs`'s SIMD-tail pattern) derives
that counter from the live remainder's own length at each stage, not from a fixed nominal count,
so it cannot desync. No signal (panic, wrong output, sanitizer complaint) was found in code
reading, in the existing test suite run under debug-assertions + overflow-checks, or in a
dedicated boundary-value test sweeping width/height combinations chosen to land exactly on and
just past every chunk-size boundary the SIMD-tail code steps through.

**Fully reproducible**: n/a (not Confirmed).

## Class

n/a (not Confirmed).

## Root cause

n/a - the suspected bug shape does not manifest anywhere in the codebase's `as_chunks_mut::<N>()`
call sites. Unlike the hypothesis in the backlog entry (a counter that "assumes the const-generic
`N` covers the whole buffer"), every site either avoids needing such an assumption entirely
(`zip`-based truncation), enforces the assumption explicitly before chunking (`assert_eq!` on
total length), or is structurally guaranteed to hold (fixed-channel-count image buffers sized as
an exact product of width/height/channel-count), except for `filter_1d.rs`'s one genuine
remainder-consuming site, which re-derives its counter from the actual remainder length at each
stage rather than from a fixed nominal count.

## The fix

Not applicable - nothing to fix.

- Project test suite: 114/114 relevant tests passed under
  `RUSTFLAGS="-C debug-assertions=on -C overflow-checks=on"` (no source change was made; the only
  file added was a temporary, uncommitted scratch test under `tests/`, deleted after use, and
  `git status` was confirmed clean before finishing).

## What's not done / open questions

- `avif-native`'s two alpha-compositing `as_chunks_mut::<4>()` call sites
  (`src/codecs/avif/decoder.rs:571,772`) and every `as_chunks_mut::<CHANNELS>()` site in
  `src/codecs/avif/yuv.rs`/`src/codecs/avif/ycgco.rs` were only checked via code reading this
  session, not empirically exercised - `dav1d`/`avif-native` isn't buildable in this environment
  without the pkg-config shim documented in `___BUGHUNT___/README.md`, which wasn't set up this
  session (would need a fresh, fairly large `avif-native` build under a disk-space-constrained
  session). Code reading found the size-mismatch guard already added ahead of both alpha-copy
  sites and consistently `zip`-based iteration elsewhere, so no signal is expected, but this is a
  code-reading-only conclusion for that specific feature, not an empirically verified one.
- This sweep only covered the literal `as_chunks_mut(` textual call shape, same limitation noted
  by candidates 026/027 for their own respective APIs: a hand-rolled loop achieving the same
  "chunk into fixed-size const-generic-like groups, silently drop the remainder" effect via raw
  slice indexing (e.g. `for i in (0..(len / N) * N).step_by(N) { ... }`) without ever calling
  `as_chunks_mut` would not have been caught by this sweep.
- This closes out the "chunking API" bug-class sweep started by candidate 026
  (`chunks_exact_mut`), continued by candidate 027 (`chunks_mut`), and finished here
  (`as_chunks_mut`/`as_chunks`) - all three textual API shapes for the "nominal vs. actual chunk
  count" bug class have now been swept across the whole repository with no instance found. A
  fourth, structurally different sweep (hand-rolled raw-indexing loops achieving the same effect,
  noted as unswept by both candidate 027 and this candidate) remains open in `IDEAS.md` if a
  future session wants full bug-class coverage.
