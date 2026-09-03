# 81 - weak-reference side table for IMAGE_CACHE, scoped to `VRc`-backed variants (Svg)

`IDEAS.md`'s "weak-reference side table for `IMAGE_CACHE`" entry (narrowed after candidate 052)
asked for a table that dedupes a live `Image` regardless of `IMAGE_CACHE`'s byte bound, framed as
candidate 038's lowest-risk fix direction ("cannot regress memory, weak refs cost nothing once
the strong owner drops them"). That framing assumed the fix could be added as a same-file side
table with upgrade-weak-to-strong lookup logic, the way `alloc::rc::Weak` would work for a plain
`Rc`. Investigating showed that assumption doesn't hold for the case candidates 038/052 actually
measured (a decoded raster `EmbeddedImage`, e.g. a 1080p PNG/JPEG): its data lives in
`SharedVector` (`internal/core/sharedvector.rs`), which only tracks a single atomic strong
refcount, no weak count, no dual strong/weak allocation-lifetime split - the mechanism a real weak
reference requires to safely tell "still alive elsewhere" from "already freed" apart. Only the
`Svg` variant (`vtable::VRc`/`VWeak`, which already has that split, see
`helper_crates/vtable/src/vrc.rs:240,332`) can be given a true weak side table without deeper
surgery. This candidate implements that narrower, correctly-scoped version and measures it - see
"Why the result looks the way it does" for why `EmbeddedImage` isn't included and what it would
take.

## What was tried

Extended `perf_test`'s existing `image_probe`/`make_assets` harness (from candidates 038/052) with
an SVG source large enough to exceed `IMAGE_CACHE`'s 5 MB bound on its own
(`large_shapes.svg`, 12000 paths, 604 KB source -> `source_size*16+8192` ~= 9.5 MB weight, see
`internal/core/graphics/image/svg.rs:37`), and a `probe_svg()` that runs the same
load/hold/reload/drop/reload sequence as the existing raster `probe()`. This binary is
single-threaded and does no concurrent work, so pinned and unpinned numbers coincide - both are
reported for completeness per the loop's own discipline, not because concurrency is exercised.

| Variant | single-core (`taskset -c 3`), `large_shapes.svg` while-held reload | multi-core (unpinned) | notes |
|---|---|---|---|
| baseline, run 1 | 8.427 ms (RE-PARSED) | - | cold parse was 10.134 ms; while-held reload pays nearly the full cost again |
| baseline, run 2 | 8.556 ms (RE-PARSED) | - | consistent with run 1 |
| baseline, unpinned | - | 8.322-9.407 ms (RE-PARSED) | two runs, same pattern |
| patched, run 1 | 0.014 ms (deduped) | - | cold parse was 8.274 ms |
| patched, run 2 | 0.013 ms (deduped, `pixels match`) | - | consistent with run 1 |
| patched, unpinned | - | 0.018-0.027 ms (deduped) | two runs, same pattern |

`shapes.svg` (20 KB source, ~0.3 MB weight, well under the bound) shows "deduped while held" in
both baseline and patched - it was already served from the strong `IMAGE_CACHE` before this patch,
unaffected either way, included as a control.

Raster images (`large.png`, `large.jpg`, `large_alpha.png`, `smooth_large.png`,
`smooth_large.jpg`, all >5 MB decoded) are **unaffected in both baseline and patched builds** -
still "RE-DECODED" every time, matching candidate 038's original numbers (~13-24 ms per reload
while held). This patch does not touch that path; see below for why.

## Result

For an `Svg`-backed `Image` heavier than `IMAGE_CACHE`'s 5 MB bound, a reload while the first
`Image` is still alive drops from ~8.3-9.4 ms (a full re-parse of the SVG tree, indistinguishable
from a cold load) to ~0.01-0.03 ms (an atomic increment via `VWeak::upgrade`), a real and
reproducible effect (2 paired runs pinned, 2 unpinned, same direction and magnitude each time). A
reload once nothing else holds the image still re-parses at the same cost in both baseline and
patched builds - a weak reference cannot and is not meant to help there. Raster images (the case
candidates 038/052 actually quantified) are completely unaffected by this patch, in either
direction - see below.

## Does it change behavior/output?

- **Bit-exact?** Yes. `probe_svg`'s pixel-hash check (FNV-1a over `Image::to_rgba8()` bytes,
  same helper the raster `probe()` already used) reports "pixels match" for every load path (cold,
  while-held, after-drop) on both `shapes.svg` and `large_shapes.svg`, before and after the patch.
  The patch changes whether a re-parse happens, never what the parsed/rendered result is.
- **Is this a breaking change?** No public API changes. `WeakImageInner` and the new
  `HashMap<ImageCacheKey, WeakImageInner>` field are private to `cache.rs`. The only externally
  observable difference is that an `Image` constructed from an SVG source heavier than the cache
  bound can now come back identical (`==` via `ImageInner`'s existing `VRc::ptr_eq`-based
  `PartialEq`) to a still-alive sibling `Image` for the same path/data, instead of always being a
  freshly-reparsed, distinct allocation - a strictly closer-to-the-doc-comment behavior (the
  pre-existing comment on `lookup_image_in_cache_or_create` already claimed this dedup happens;
  before this patch it did not for anything over the bound).
- Project test suite: `cargo test -p i-slint-core --lib graphics::image::cache --features
  image-decoders,svg` - **1 passed, 0 failed before; 1 passed, 0 failed after**
  (`test_path_cache_invalidation`, the module's only test, uses a 10x10 image well under the
  bound in both cases, unaffected). Also re-checked `cargo check -p i-slint-core` with
  `--no-default-features --features image-decoders,std` (no `svg`) and with `svg` added - both
  configurations compile cleanly (the `svg`-gated `WeakImageInner` correctly degrades to an
  uninhabited enum when the feature is off; see "Why the result looks the way it does" for a
  compiler quirk that surfaced here).
- Memory: a `HashMap<ImageCacheKey, WeakImageInner>` entry (one `ImageCacheKey` plus one
  `VWeak`, a single `NonNull` pointer) per distinct SVG source ever loaded, for as long as its
  cache key is never reused for a different value - this cannot regress *decoded pixel* memory
  (weak refs hold nothing alive), but it is an unbounded-by-count side table today: a stale entry
  is only removed lazily, the next time the same key is looked up after its `VRc` has died. An
  application that streams through many distinct large SVG paths without ever repeating one would
  accumulate small (`ImageCacheKey` + pointer-sized) dead entries indefinitely. Not measured here
  since the harness only exercises 2 distinct SVG keys; flagged in "What's not done" below.

## Why the result looks the way it does

`ImageInner`'s variants split into two families depending on what backs their data: `Svg`,
`HTMLImage` (wasm only), `BackendStorage` and `NineSlice` are `vtable::VRc<OpaqueImageVTable, _>`,
which already maintains separate strong and weak atomic counts (`VRcInner::strong_ref`/
`weak_ref`, `helper_crates/vtable/src/vrc.rs`) - `VRc::downgrade`/`VWeak::upgrade` is exactly the
"cannot regress memory, tells alive-elsewhere from freed apart" mechanism the idea's justification
described. `EmbeddedImage` (the raster PNG/JPEG case that candidates 038 and 052 actually
quantified - the 1080p "large.png"/"large.jpg" cliff) is backed by `SharedImageBuffer` ->
`SharedPixelBuffer` -> `SharedVector` (`internal/core/sharedvector.rs`), which implements only a
single atomic strong refcount (plus a `-1`-sentinel static for the shared-empty singleton) with no
weak slot at all. Building a genuine weak reference into that type - the only way to safely
observe "is this still alive somewhere" without either keeping it alive (defeating the point) or
risking a read of already-freed memory - would mean giving `SharedVectorHeader` a second atomic
count and reworking every one of its ~15 unsafe pointer-arithmetic methods (`drop`, `clone`,
`detach`, `push`, `remove`, `insert`, `resize`, `shrink`, `clear`, `into_iter`, ...) around the new
two-count lifetime rule, while keeping the type's existing FFI surface
(`slint_shared_vector_allocate`/`slint_shared_vector_free`, `#[repr(C)]` header layout mirrored on
the C++ side) compatible - a change with a blast radius across the whole codebase (`SharedVector`
backs paths, gradient stops, model rows, and more, not just images), not the "one file, cannot
regress memory" scope the idea described. A structurally different route - wrapping
`EmbeddedImage`'s two fields in an `Rc<...>` so `Image::clone()` shares one allocation instead of
deep-cloning `SharedVector`'s pieces - was also considered and rejected: `ImageInner` carries a
`cbindgen:prefix-with-name` annotation and is mirrored in generated C++ headers
(`api/cpp/include/private/slint_image.h`'s `ImageInner_EmbeddedImage(cache_key, buffer)`
two-argument constructor); reshaping that variant's fields breaks the C++ FFI layer, again far
outside "one file, lowest risk."

A minor Rust quirk surfaced while implementing the `svg`-feature-off path: `match self { }` over
zero arms fails to compile for `&WeakImageInner` even when `WeakImageInner` has zero variants
under `#[cfg(not(feature = "svg"))]`, because (per rustc's own diagnostic) "references are always
considered inhabited" for exhaustiveness purposes - a `#[cfg(not(feature = "svg"))] _ => None`
wildcard arm was needed in `upgrade()` (absent from the `svg`-enabled build, where the single real
arm is exhaustive on its own).

## What's not done / open questions

- **`EmbeddedImage` (the raster/decoded-bytes case, i.e. what 038/052's own numbers are about) is
  not covered.** This is the main practical gap left by this candidate: an application that cycles
  through distinct 1080p+ photos while holding more than one at a time still re-decodes on every
  load past the 5 MB bound, exactly as before. Closing it needs either (a) a strong/weak dual-count
  redesign of `SharedVector` (see above - large, cross-cutting, FFI-sensitive), or (b) raising the
  bound itself (candidate 052, already measured, orthogonal and composable with this patch - they
  don't conflict, since 052 changes the strong-cache admission threshold and this patch adds an
  independent weak layer below it).
- **`HTMLImage` (wasm), `BackendStorage`, `NineSlice`** also use `vtable::VRc` and could in
  principle get the same weak-dedup treatment, but none of them are ever constructed inside
  `lookup_image_in_cache_or_create` under a non-wasm, `image-decoders` build (`HTMLImage` is
  wasm-only; `BackendStorage`/`NineSlice` are produced elsewhere, e.g.
  `internal/renderers/skia/cached_image.rs`'s `replace_cached_image` call, not through this lookup
  path) - not reachable from this fix's call site as written, and not measured here.
- **Unbounded weak-table growth by key count** (see "Does it change behavior/output?" above): the
  side table only prunes a dead entry lazily, on next lookup for that same key. A periodic sweep,
  or capping the table by count, was not implemented - not measured to matter for this harness's
  2-key SVG workload, but worth a look before shipping if `Svg` images are loaded from many
  distinct, never-repeated paths in a long-running session.
