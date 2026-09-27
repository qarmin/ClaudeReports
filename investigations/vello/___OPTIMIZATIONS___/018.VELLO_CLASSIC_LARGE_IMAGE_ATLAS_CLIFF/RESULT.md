# Vello Classic large-image atlas cliff (issue #1545)

Investigates linebender/vello#1545: a full-screen ~1835x8192 image on an iPad Pro reportedly
couldn't sustain 120fps, but the same image scaled to height 4096 could - a sharp, sustained
frame-rate cliff exactly at the 4096/4097 boundary, plus a separate report that images taller
than 8192 silently fail to render. `vello_encoding`'s image atlas (`ImageCache`, backed by
`guillotiere::AtlasAllocator`) starts at 1024x1024 and doubles up to a hard `MAX_ATLAS_SIZE` of
8192, which is exactly the kind of "atlas/texture-tiling limit hit at a size boundary" this
candidate was asked to check.

Toolchain: `cargo 1.98.0 (797e8a9bc 2026-08-05)`, `rustc 1.98.0 (88d9e12ae 2026-08-18)`.

## What was tried

Reading the issue thread (13 comments, fetched via `gh api repos/linebender/vello/issues/1545/comments`)
first: a Vello maintainer (waywardmonkeys) diagnosed the *sustained* cliff during the same thread
as the atlas being rebuilt from scratch and every resident image being re-uploaded to the GPU
**every frame** (not just when something actually changed), and landed a fix for it,
`c55a2b5e` "vello: Keep image atlas residency across renders (#1558)" - making the GPU atlas
texture and CPU-side residency map persistent across frames, only re-uploading images that are
new or explicitly marked dirty. That commit (and a related dirty-tracking fix,
`4b70f8c8`/`___BUGHUNT___/025`) is already present at this workspace's `base_commit`
(`75168314`, well after both). Bicubic filtering (the thread's second complaint, "`ImageQuality::High`
does not yield bicubic filtering") was also separately landed (`1ba46680` #1557) and is present
at `base_commit` too. So the two mechanisms actually root-caused in the issue thread are already
fixed upstream, before this candidate started - no source change was applied.

To confirm the *current* code's behavior at exactly the reported boundary, and check whether any
smaller cliff remains, a standalone bench crate was written (`bench/`, depending on
`{ path = "../clone/vello" }`) - no existing project benchmark reaches classic `vello`'s GPU
image-atlas path. It drives the real `vello::Renderer` (`render_to_texture`, real WGSL compute
dispatch, not the CPU-only echo path) in two modes:

- `--mode cliff --height H`: draws one `(H*1835/8192)x H` image (matching the issue's aspect
  ratio) scaled to fill a 1024x1024 canvas, repeatedly. Reports the first ("cold") frame
  separately from 20 steady-state frames (image already resident, atlas already at final size),
  so a genuine per-frame cliff can be told apart from a one-time atlas-creation cost.
- `--mode growth`: places 8 small (128x128) images (comfortably resident in the default
  1024x1024 atlas), runs 15 steady frames, then adds one 920x4096 image that forces
  `ImageCache::bump_size` to grow and repack the atlas, and reports that one "growth" frame
  against the steady frames before/after it.

This sandbox has no real GPU (`/dev/dri` doesn't exist); `wgpu` falls back to a software
Vulkan/GL adapter (llvmpipe, consistent with candidates 003/007/etc. in this workspace) -
absolute millisecond values are software-rasterizer numbers, not real-hardware timings, but the
*relative* deltas (steady-state flat vs. first-frame/growth-frame spikes) directly reflect the
CPU->GPU upload volume mechanism the issue is about, independent of GPU brand.

24 logical cores were available; "single-core" pins the whole process with `taskset -c 0`,
"multi-core" is unpinned. Two full runs were taken for the `cliff` mode to check the noise floor.

### `cliff` mode: does the sustained per-frame cost still cliff at 4096 -> 4097?

| height | first frame (cold) | steady p50, multi-core (2 runs) | steady p50, single-core |
|---|---|---|---|
| 4096 (fits a 4096x4096 atlas) | 112.9 ms / 112.8 ms | 3.18 ms / 2.79 ms | 19.98 ms |
| 4097 (needs an 8192x8192 atlas) | 185.9 ms / 189.5 ms | 2.98 ms / 2.76 ms | 19.69 ms |
| 8192 (max size; width also doubles to 1835) | 213.8 ms / 215.1 ms | 4.02 ms / 4.12 ms | 20.79 ms |

(Full `min/p50/mean/max` for every row, plus the fresh-clone `just test` re-run reproducing the
same numbers, is in the bench's own stdout - not re-transcribed here for brevity.)

### `growth` mode: cost of one atlas-growth event with pre-existing residents

| | multi-core p50 (2 runs) |
|---|---|
| steady, before growth (8 small images, all resident+clean) | 3.46 ms / 3.31 ms |
| **growth frame** (9th image added, forces `bump_size`) | **117.2 ms / 111.4 ms** |
| steady, after growth (all 9 resident+clean again) | 2.91 ms / 2.99 ms |

## Result

Time only (no memory/allocation metric instrumented). Two distinct findings, both wall-clock:

1. **The reported sustained cliff at the 4096/4097 boundary is gone at `base_commit`.** Steady-state
   per-frame time is ~2.6-3.5 ms regardless of whether the image needed a 4096x4096 or an
   8192x8192 atlas (4096 vs. 4097 rows differ by less than run-to-run noise, on both multi-core
   and single-core, across two independent runs). The 8192 row is a little higher, but that's
   because its *width* is also doubled (1835 vs. 918) to keep the issue's aspect ratio, i.e. it's
   sampling roughly 4x the source pixels per frame - not a boundary effect. This matches: the
   mechanism the issue thread diagnosed (non-persistent atlas -> full re-upload every frame) was
   fixed by `c55a2b5e` (#1558), already present here.
2. **What differs at the boundary now is a one-time cost, not a sustained one**: the first frame
   costs 112.9 ms at height 4096 vs. 185.9-189.5 ms at 4097 and 213.8-215.1 ms at 8192 (consistent
   across repeats) - because reaching an 8192-sized atlas from the 1024 default takes one more
   doubling step than reaching 4096, and because more actual image bytes get uploaded. This only
   happens once (or once per newly-introduced large image), not every frame - it would show up as
   a single dropped frame on first appearance, not a sustained frame-rate ceiling.
3. **A related, currently-unavoidable one-time cost was found and quantified**: when the atlas
   *does* need to grow, `ImageCache::bump_size` -> `repack_to_size` marks **every already-resident
   image** dirty, not just the newly-added one, because `render.rs`'s resize path frees the old
   GPU atlas texture and allocates a brand new one (`recording.free_image` + `ImageProxy::new`)
   with no content-preserving copy - so every previously-uploaded image genuinely has to be
   re-uploaded into the fresh texture regardless of whether its CPU-side packed position changed.
   The `growth` mode bench measured this directly: the frame that introduces a 9th image forcing
   growth costs ~111-117 ms against a ~3 ms steady frame (~35-40x), even though only 1 of the 9
   resident images is actually new content. Like finding 2, this is a one-time-per-growth-event
   cost, not sustained - but it scales with however many *unrelated* images happen to be resident
   when growth is triggered, which a sustained-heavy-animation scene (e.g. one that keeps adding
   distinct images over time, each triggering another doubling) could hit repeatedly.

## Does it change behavior/output?

- **No source patch was applied** - the mechanism the issue thread root-caused is already fixed
  upstream at `base_commit` (see "What was tried"). This is a measurement/investigation-only
  candidate: N/A, no `patch.diff` for the tested project, by design (see SKILL.md's "patch.diff is
  mandatory" section - this is squarely the "pure measurement/investigation" case, not the "real
  fix" case).
- Pixel hash (`edf6bce5ccdd0383` for every `cliff` height, verified across both runs and the
  fresh-clone `just test` re-run) confirms the bench itself renders deterministically - not a
  correctness claim about vello's source, since nothing there was touched.
- Project test suite: not run. Per SKILL.md's "Correctness" section, the test-suite requirement's
  only stated exception is "a pure investigation with no patch," which this is - N/A.

## Why the result looks the way it does

The specific mechanism reported in #1545 (full atlas rebuild + full re-upload *every frame*,
regardless of whether anything changed) was already found and fixed by a Vello maintainer within
the same GitHub issue thread (#1558), before this workspace's `base_commit`. Re-deriving that same
fix here would have duplicated already-merged work; instead this candidate's value is confirming,
with an actual GPU-driven measurement, that the fix holds (finding 1) and precisely characterizing
what cost *does* remain at that boundary (findings 2-3) - both one-time, not sustained, so neither
looks like the "can't sustain 120fps" symptom described in the original report anymore.

## What's not done / open questions

- **The "images above 8192 silently fail to render" complaint from the same issue is still real
  and still present at `base_commit`** (`vello_encoding/src/resolve.rs`, `resolve_pending_images`
  around line 528 sets the image's atlas slot to `None` when `bump_size` can't grow past
  `MAX_ATLAS_SIZE`, and the patch-writer around line 309 then zeroes the image's draw data instead
  of erroring - both sites already carry a `// TODO: a better strategy: texture array? downsample
  large images?` comment). This is not a new discovery: it's explicitly acknowledged, still open,
  in the issue thread itself ("And I do want to deal with the silent failure for 8192 images. Just
  haven't gotten to it yet!" - waywardmonkeys). Not filed as a new `___BUGHUNT___/` candidate by
  this session for that reason - noted here for the record instead.
- Finding 3 (atlas-growth marks all residents dirty) was measured but **not patched**: a real fix
  would mean switching `ImageCache`'s grow path to `guillotiere::AtlasAllocator::grow()` (which
  preserves existing allocation positions, unlike `repack_to_size`'s full rebuild) *and* adding a
  GPU texture-to-texture copy of the old atlas content into the new, larger texture on resize
  (there is currently no such command in `vello`'s `Recording`/`Command` enum - `wgpu_engine.rs`
  only ever frees-and-recreates). That is a real engine feature, not a bounded one-file patch, and
  risks a subtle content-copy-region bug if rushed - out of scope for this candidate's session;
  left as a lead for a future candidate instead of attempting a partial version of it here.
- The `growth` mode's 8-small-images setup is a synthetic worst case chosen to make the mechanism
  visible cheaply; it wasn't tuned to reproduce any specific real scene from the issue thread's
  later comments (the "3 rectangles, each <=4096px, still only 60fps" follow-up) - that follow-up
  report (post-#1558) points at ongoing compute-pipeline-dispatch and/or fine-rasterization
  sampling cost for large images, a different mechanism from the atlas-sizing one this candidate
  was scoped to, and is still open upstream as of the last comment in the thread.
