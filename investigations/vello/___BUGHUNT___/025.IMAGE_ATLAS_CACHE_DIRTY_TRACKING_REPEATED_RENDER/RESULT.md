# Image-atlas cache dirty-tracking silently loses an image across repeated `render_to_texture` calls

`vello_encoding::resolve::Resolver::resolve`'s "patchless" fast path (an encoding with no
gradients/images/glyph-runs at all - e.g. a scene that only fills solid-color paths) returns
`Images::default()` (a 0x0 atlas, no uploads) without ever touching the image-residency cache
(`ImageCache::begin_resolve`/`finish_resolve`). Since commit c55a2b5e made the renderer's GPU image
atlas texture *persistent* across renders (reusing it whenever the resolver reports the same atlas
size, freeing/recreating it otherwise), a 0x0 report from an intervening patchless render is
misread by the renderer as "no atlas needed any more", so it frees the real, populated atlas
texture and replaces it with a fresh, empty one - while the resolver's own cache still believes
every previously-uploaded image is clean and skips re-uploading it on the next render that needs
it. This matches upstream issue linebender/vello#1809 ("repeatedly rendering the same `ImageData`
does not occur any error but the image does not appear except the first rendering") and is
root-caused, independently confirmed, and already has an open (unmerged as of this session) fix
proposed upstream in PR #1664 "fix: preserve image atlas across patchless resolves" - this
candidate independently re-derived, reproduced, and verified that same mechanism and fix from
first principles against this checkout, rather than just trusting the PR description.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the upstream issue (linebender/vello#1809) via `gh api`, and the two open PRs it points at (#1664, #1739) and the merged-but-still-open #1862, to identify the exact real-world trigger | `gh api repos/linebender/vello/issues/1809`, `gh pr view`/`gh pr diff` on #1664 | #1664's own PR description exactly named the mechanism: a patchless resolve's `Images::default()` return frees/replaces the renderer's persistent GPU atlas texture while the resolver's residency cache still believes existing entries are clean, so the next image render silently samples a blank/newly-recreated atlas |
| Manual boundary-value construction through the real public API (`Scene`/`Renderer::render_to_texture`, debug reasoning, no sanitizer needed - this is a logic/caching bug, not memory-unsafety), first tried without an intervening patchless render (matching the *literal* code snippet in the issue, same `Scene` re-rendered, or `scene.reset()`+redraw) | `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1 cargo run --release` against the real `vello::Renderer::render_to_texture` API, 16x16 solid-red `ImageData`, 2 renders (same scene reused, and separately: reset+redraw) | Did **not** reproduce either variant - a 2nd render of the same image alone is safe, because the persistent-atlas "Reused" path only triggers when the reported atlas size changes, which it doesn't for two image-only renders in a row |
| Re-read `render_encoding_coarse` (`vello/src/render.rs`) and `Resolver::resolve` (`vello_encoding/src/resolve.rs`) together, focusing on what makes the reported atlas size change between renders | Static code reading | Found the exact trigger: an intervening render whose *encoding has no patches at all* takes `resolve`'s early-return branch and reports a 0x0 atlas, which differs from a previous real atlas size and causes the renderer to free+recreate the GPU atlas texture (`vello/src/render.rs`'s `atlas_proxy_action` match) |
| Standalone `repro/` crate driving vello's real public API end-to-end (`Scene::draw_image` + `Renderer::render_to_texture`, driven by a real wgpu device via `RenderContext`) with the corrected 3-render trigger (image render, then a patchless solid-fill-only render, then the same image again), reading back the center pixel of each output texture | `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1 cargo run --release`, 16x16 solid-red `ImageData`, against this checkout's pristine worktree (`75168314`) | Reproduced: render 1 shows the correct opaque red pixel; render 3 (after the intervening patchless render) shows `[0, 0, 0, 0]` (the scene's transparent background) instead - no error/panic anywhere |

No `miri`/ASAN/fuzzing was used - this is a stateful cross-render caching/logic bug (HEURISTICS.md's
"logic bugs" category), not a memory-safety/UB question; differential comparison against the
correct expected pixel value through the real public API fully demonstrates it.

## Status

**Confirmed.** Reproduces 3/3 runs (release build, `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`
software GL rasterizer, no fixed seed needed - this is fully deterministic, not layout/seed
dependent) at the original 16x16 image size, and additionally reproduces (separately verified 1/1,
alongside the deterministic 16x16 case) with a varied 40x40 image (CONFIRMATION.md section 7) -
both against the pristine worktree and, after applying `patch.diff`, confirmed **not** to reproduce
for either size (3/3 clean runs at 16x16, 1/1 at 40x40). See `output.txt` for the full captured
transcripts, including a from-scratch `just test` run (fresh clone at `base_commit`, no dependency
on any pre-existing worktree).

**Fully reproducible**: Yes. `repro/` drives vello's real public API end-to-end - a real `Scene`
with a real `Renderer` and a real wgpu `Device`/`Queue` (via `vello::util::RenderContext`), calling
the actual `Renderer::render_to_texture` entry point three times in a row exactly as a real
application would, with no hand-constructed internal state anywhere. This is a single-process,
single-caller finding (an application repeatedly calling its own `Renderer`), not an
inter-participant network/peer scenario, so CONFIRMATION.md section 11's "no downgrade needed" case
applies directly - there is no second participant to fake here.

## Class

logic

## Root cause

`Resolver::resolve` (`vello_encoding/src/resolve.rs`) has a fast path for a "patchless" encoding:

```rust
let resources = &encoding.resources;
if resources.patches.is_empty() {
    let layout = resolve_solid_paths_only(encoding, packed);
    return (layout, Ramps::default(), Images::default());
}
```

`Images::default()` reports a `0x0` atlas with no pending uploads, and critically never calls
`ImageCache::begin_resolve`/`finish_resolve` - the image-residency cache (which tracks, per image
blob id, its atlas coordinates and whether it needs re-uploading) is not touched at all for this
render.

`vello::render::render_encoding_coarse` (`vello/src/render.rs`) uses the reported atlas
width/height to decide what to do with the renderer's *persistent* GPU atlas texture proxy
(`self.image_atlas: Option<ImageProxy>`, kept across renders since c55a2b5e "Keep image atlas
residency across renders"): if the new reported size differs from the currently-held proxy's size,
it frees the old GPU texture and allocates a fresh one of the new size. A `0x0` report from a
patchless render differs from any previous non-trivial atlas size, so this fires: the real,
already-populated atlas texture is destroyed and replaced by an empty `1x1` texture (via
`recording.free_image`+a new `ImageProxy::new`).

The *next* render that draws an image again does have patches, so it takes the normal path.
`ImageCache::get_or_insert` finds the image's residency entry still present in the cache's
`HashMap` (it was never touched, let alone evicted, by the intervening patchless render) with its
`dirty` flag still `false` (it was already uploaded and marked clean by an earlier render) - so it
is *not* re-queued for upload, even though the GPU texture that's supposed to contain it was just
discarded. The resolver does correctly report the atlas's *real* (non-zero) size again at this
point (since `ImageCache::images()` always reads `self.atlas.size()`, which was never changed by
the patchless render), so the renderer dutifully resizes its persistent proxy *back* to that size -
but since `images.images` is empty, no upload command is issued into the newly-recreated,
zero-initialized GPU texture. The image is therefore missing from the render, silently, with no
error or panic anywhere: the bug is a state desynchronization between two independently-persistent
pieces of state (the resolver's logical residency cache, and the renderer's physical GPU atlas
texture) that are only kept in sync by every render going through the same code path - and the
patchless fast path is a code path that bypasses that synchronization entirely.

## Introduced by

c55a2b5e "vello: Keep image atlas residency across renders (#1558)" (2026-04-28). Before this
commit, the renderer recreated and re-uploaded the *entire* atlas texture on every single render
(no persistence at all), so a `0x0`/no-op report from a patchless render was harmless - there was
never anything persistent to lose. This commit added persistent GPU atlas-texture residency to
`vello/src/render.rs` (reusing the same `ImageProxy` across renders, freeing/recreating it only on
a size change) and persistent logical residency tracking to `vello_encoding/src/image_cache.rs`
(entries survive across resolves, keyed by blob id, with a `dirty` flag), but did not update the
pre-existing `if resources.patches.is_empty() { ... return (..., Images::default()); }` early
return in `resolve.rs` to account for the new persistence - that line is untouched in this commit's
own diff. The bug has therefore been present in every release since this commit landed (matches
the upstream issue reporter's own account of a regression between vello 0.7.0 and 0.9.0).

## The fix

Mirrors the mechanism (independently re-derived, not blindly copied) of the still-open upstream PR
#1664 "fix: preserve image atlas across patchless resolves":

1. `vello_encoding/src/image_cache.rs`: add `ImageCache::images_for_patchless_resolve(&self) ->
   Images<'_>`, which returns `Images::default()` only if the residency cache is actually empty
   (nothing to preserve), and otherwise returns the same `Images` a normal resolve would (the real
   current atlas size, no pending uploads).
2. `vello_encoding/src/resolve.rs`: in the patchless fast path, call `self.image_cache.begin_resolve()`
   (so generation bookkeeping/eviction stays consistent across mixed patchless/patched renders) and
   return `self.image_cache.images_for_patchless_resolve()` instead of unconditionally
   `Images::default()`.

This is sufficient because it removes the actual cause (the renderer being told "no atlas" when a
real, populated one exists) rather than patching a symptom: a patchless render now correctly
reports the atlas's real size whenever there's anything resident in it, so the renderer's "size
changed" reuse/free/resize decision no longer misfires just because the current render happens not
to draw any images. An image-free frame with *no* prior resident images still correctly reports
`0x0` (verified: the `self.map.is_empty()` check), so image-free applications that never draw any
image still never allocate an atlas at all, matching the pre-existing intent of the original code
(per PR #1664's own description).

Verified against `repro/`: before the patch, reproduces 3/3 (16x16) and separately with a varied
40x40 image (CONFIRMATION.md section 7 - not just the one exact triggering input); after the patch,
does not reproduce for either size, confirmed 3/3 (16x16) and separately for 40x40. Full transcripts
in `output.txt`.

- Project test suite:
  - `cargo test -p vello_encoding --release`: **31 passed / 0 failed / 0 ignored**, identical
    before and after the patch.
  - `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1 VELLO_SKIP_LFS_SNAPSHOTS=all cargo test -p vello_tests
    --release` (this checkout's snapshot PNGs are unresolved git-lfs pointers, hence the env var -
    unrelated to this patch): **64 passed / 0 failed / 1 ignored**, identical before and after the
    patch, across all 8 test binaries (`compare_gpu_cpu`, `emoji`, `hinting`, `known_issues`,
    `property`, `regression`, `smoke_snapshots`, `snapshot_test_scenes`).

## What's not done / open questions

- Did not port the additional shader-hot-reload fix from PR #1664's own diff
  (`Renderer::reload_shaders` also drops the GPU atlas by replacing the engine while keeping clean
  resolver residency, addressed there by resetting `self.resolver` on reload) - that is a related
  but distinct trigger (shader hot reload, a dev-only, `hot_reload`-feature-gated code path) for the
  same underlying "renderer GPU state and resolver logical cache can independently lose sync" class
  of bug, not the one this candidate's repro demonstrates or that issue #1809 reports. Worth a
  follow-up candidate if that code path is considered in scope.
- Did not check whether `vello_hybrid`/`vello_cpu` (the newer `sparse_strips`-based CPU/WebGL
  renderers, PRs #1739/#1862) have an analogous cache/atlas-lifecycle desync - their image cache
  (`sparse_strips/vello_common/src/image_cache.rs`) is architecturally different (caller-managed
  explicit `ImageId` allocate/deallocate, not a content-hash-keyed residency cache with a dirty
  flag), so this exact mechanism does not directly transfer, but a similar class of "two
  independently-persistent caches losing sync across specific render sequences" issue was not ruled
  out there this session.
- Did not investigate whether other early-return/fast-path branches elsewhere in `resolve.rs` (or a
  similar early return in a different resolver, if any) have the same "reports a stale/default
  value without touching a persistent cache" shape - only the one branch this issue actually
  reports was investigated.
