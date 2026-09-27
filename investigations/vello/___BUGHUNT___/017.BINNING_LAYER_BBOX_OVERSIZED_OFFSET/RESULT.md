# BINNING_LAYER_BBOX_OVERSIZED_OFFSET

Assigned candidate for tracked issue [linebender/vello#1061](https://github.com/linebender/vello/issues/1061)
("Layer shapes are wrong"), reported from Servo's 2D canvas backend; reporter suspected the
binning stage. A smoke test for it already exists in-tree
(`vello_tests/tests/known_issues.rs::test_layer_size`, added by
[PR #1062](https://github.com/linebender/vello/pull/1062)), marked `#[should_panic]` - i.e. still
known-broken as of this session's `HEAD`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Run the project's own existing smoke test for this issue | `cargo test -p vello_tests --test known_issues test_layer_size -- --nocapture --include-ignored`, debug build, real wgpu GPU device (llvmpipe/GL software adapter available in this environment) | Reproduces: rendered image differs from expected by mean 0.38971454 (threshold 0.001). Deterministic across 3/3 runs. |
| Pixel-level analysis of the debug PNG the test writes (`vello_tests/current/gpu/smoke/layer_size.png`) | Decoded with Pillow, scanned for the bounding box of non-background pixels | The scene's `push_layer(Compose::Clear)` clip rect is `(20,20)-(40,40)` (20x20px), but the actually-cleared (fully transparent, `(0,0,0,0)`) region is `(16,16)-(47,47)` (32x32px) - exactly the rect's bbox rounded *out* to the 16px tile grid, with **zero** antialiased transition pixels anywhere in the block (matches the maintainer's own comment on the issue: "erroneous areas do line up with the 16x16 tile boundary"). |
| Manual code reading of the coarse-rasterization/fine-rasterization pipeline (`vello_shaders/shader/{coarse,fine}.wgsl`, `vello_encoding/src/encoding.rs`), guided by the above bbox evidence | N/A (static reading, cross-checked against the CPU mirror `vello_shaders/src/cpu/coarse.rs`) | Found the precise mechanism (see Root cause) and confirmed it is the same code path used by the standalone repro below. |
| Standalone repro (`repro/`) against a **fresh clone** of the real repo, driving only the public `vello` crate API, real wgpu device, no test-harness internals | `cargo run --release` in `repro/`, wired up via `just clone && just reproduce-before` | Reproduces identically: affected bbox `x=[16,47] y=[16,47]`, deterministic across 3/3 runs, both under the software Vulkan adapter used earlier in this session and under a software OpenGL (llvmpipe) adapter picked up by a later `wgpu::Instance::default()` call - not adapter-specific. |

## Status

**Confirmed.** Reproduces 3/3 runs (bit-for-bit identical affected-pixel bounding box each time)
under both the in-tree smoke test and the standalone `repro/`, on a deterministic GPU compute
pipeline with no seed/timing dependence.

**Fully reproducible**: **Yes**. `repro/` calls only the real public `vello` crate API
(`Scene::fill`/`push_layer`/`pop_layer`, `Renderer::render_to_texture`) against a real `wgpu`
device (no CPU fallback, no test-only internals), reading back the actual rendered texture - this
is the exact same entry point any real caller (e.g. Servo, the original reporter) uses. This is a
single-process rendering/logic bug, not a finding about one participant's reaction to another's
input, so CONFIRMATION.md section 11's "real second participant" bar does not apply here.

## Class

logic

## Root cause

`Scene::pop_layer()` takes no shape argument - by the time a layer is popped, the encoder no
longer has the clip shape at hand, so `Encoding::encode_end_clip` (`vello_encoding/src/encoding.rs`)
pushes a **dummy path with zero segments** for the `DRAWTAG_END_CLIP` draw object (the code's own
comment: `// This is a dummy path, and will go away with the new clip impl.`). Every draw object
(including this dummy one) still gets its own dedicated entry in the `paths`/tile-allocation
arrays (`DrawMonoid::path_ix` increments for every non-`NOP` tag, `vello_encoding/src/draw.rs`),
so tile allocation gives the dummy path a tile range from its bbox (correctly inherited from the
active clip), but because it has **no actual line segments**, every tile within that (tile-grid
-rounded) range ends up with `segment_count_or_ix == 0` and `backdrop == 0` everywhere - there is
no segment data anywhere to distinguish "inside the true shape" from "outside it".

In the coarse-rasterization stage (`vello_shaders/shader/coarse.wgsl`, mirrored in
`vello_shaders/src/cpu/coarse.rs`), `DRAWTAG_END_CLIP` unconditionally calls `write_path()` for
every tile it's assigned to (unlike `DRAWTAG_BEGIN_CLIP`, which has an explicit
"fully outside -> suppress" check). Since the dummy path always has zero segments, `write_path()`
always takes its `else` branch and emits `CMD_SOLID` - a PTCL command whose fine-rasterization
handler (`vello_shaders/shader/fine.wgsl`, `case CMD_SOLID:`) **unconditionally sets
`area[i] = 1.0`** for every pixel in the tile, with no way to encode "this solid tile is actually
uncovered." That assumption is safe for an ordinary fill (whose own tile-inclusion test in
`coarse.wgsl`, `backdrop_clear == is_clip`, only lets a zero-segment tile through when it's
genuinely fully-covered) but is violated here: the dummy path's own zero-segment tiles get
included regardless (since `is_clip` is true for `DRAWTAG_END_CLIP`, and separately `is_blend` -
true whenever the compose mode isn't the default clip blend - forces inclusion too), so
`CMD_END_CLIP` ends up compositing with `area[i] = 1.0` (fully covered) across the **entire**
tile-aligned bounding box of the dummy path, not the true sub-pixel shape of the original clip
rect.

For `Compose::Clear` specifically (`blend_compose` in `vello_shaders/shader/shared/blend.wgsl`),
`fa = fb = 0` unconditionally - the composited result is `(0,0,0,0)` regardless of source/backdrop
alpha - so even a *correctly-computed* `area` wouldn't, by itself, make the compose degrade
gracefully outside the true shape; the composite operation's own extent is controlled entirely by
which tiles receive the `CMD_END_CLIP` command at all, and that in turn is controlled by the dummy
path's coarse tile bbox (rounded out to the 16px tile grid) rather than the real antialiased shape.
This is exactly why the visible artifact is a hard-edged block that is both **oversized** (rounded
out to the tile grid on all four sides) and **offset** (the rounding is asymmetric relative to the
rect's actual position) - matching the issue title precisely.

This is a narrower case of a gap the vello maintainers are already aware of: `fine.wgsl` has an
existing `// TODO: ... See https://github.com/linebender/vello/issues/1061` comment in the
`LUMINANCE_MASK_LAYER` branch of the same `CMD_END_CLIP` handler, addressing only that one special
case (an explicit `area[i] == 0f` early-out) - the general-compose-mode case exercised by this
candidate's repro (`Compose::Clear`, and by construction any non-default compose/blend mode) has
no equivalent workaround.

## Introduced by

adc98117 "add clips and blends" (2022-11-18) - the original commit that introduced both general
compose/blend-mode support (`blend.wgsl`) and the `is_clip`/`is_blend`-aware tile-inclusion and
`write_path`/`CMD_SOLID` logic in `coarse.wgsl`/`fine.wgsl` that this bug depends on. The
zero-segment "dummy path" scheme for `encode_end_clip` predates this repo's current git history
(that logic was already present in some form in the earliest available history of
`vello_encoding`'s predecessor) and the two combined to produce this bug; `adc98117` is the
best-effort pin for when the specific mechanism described above (CMD_SOLID assuming full coverage,
applied to a clip's own end-of-layer draw object) became exploitable via a non-default compose
mode. Not spent further bisection time beyond this, per SKILL.md step 5's "best-effort, not
mandatory."

## The fix

**No bounded fix was attempted this session.** The root cause is architectural, not a local
off-by-one or missing check: `CMD_END_CLIP`'s own draw object structurally has no access to the
true clip shape's per-pixel antialiased coverage (only `BEGIN_CLIP`'s draw object does, and it is
a separate `path_ix` slot in a pipeline where every draw tag gets exactly one path slot via a
GPU-parallel prefix-sum monoid, with no existing mechanism for two draw objects to share one).
A real fix needs one of:
- making `END_CLIP` reference the same path/tile data as its matching `BEGIN_CLIP` (requires
  reworking the `DrawMonoid::path_ix` prefix-sum scheme so `END_CLIP` doesn't consume its own
  path slot, and reworking `coarse.wgsl`'s per-tile draw-object indexing to look up that shared
  data correctly), or
- carrying the clip mask's own coverage forward through `fine.wgsl`'s per-tile state independently
  of whatever the last inner draw command left in `area[]`.

Both are the kind of change the code's own comment already anticipates
(`// This is a dummy path, and will go away with the new clip impl.`), and the maintainers'
own comment on the issue agrees ("Fixing this is pretty hard; the data about clip regions/layer
boundaries is, I think, already discarded by the time that we're in fine rasterisation"). Forcing
a smaller patch (e.g. just making `write_path` report the dummy path's real, always-zero,
coverage) does not fix the underlying problem: it was traced through by hand and shown to just
turn the bug into a different wrong result (the compose op would then never fire anywhere, since
the dummy path's `backdrop` is uniformly `0`), not a correct one - see "Root cause" above for why.

- Project test suite: not run before/after a fix, since no fix was applied (source tree was never
  modified - reverting doesn't apply). `cargo test -p vello_tests --test known_issues
  test_layer_size` passes both before and "after" (identically) because the test is itself
  `#[should_panic]`, i.e. it already documents this exact bug as open.

## What's not done / open questions

- Not checked whether the same `CMD_SOLID`/dummy-path mechanism produces visible artifacts for
  other non-default compose modes besides `Compose::Clear` (e.g. `Compose::Copy`, `Compose::SrcIn`,
  `Mix::Multiply` etc.) - `Compose::Clear`'s `fa=fb=0` makes the effect maximally visible (a hard
  cleared block) but the same `area[i] = 1.0` overshoot would still be wrong (to varying visible
  degree) for most other non-default modes whenever the layer's clip rect doesn't happen to be
  nested inside an already-correct outer clip of the identical shape (see `vello_tests`'
  `clip_blends` test, which passes today only because its inner `Mix::Multiply` layer shares the
  exact same triangle as its own enclosing `push_clip_layer`, so the outer clip's correct
  `clip_zero_depth` suppression happens to mask the inner layer's own oversized-tile bug).
- Not checked whether `vello_hybrid`/`vello_cpu` (the sparse-strips CPU/GPU-hybrid renderer, a
  separate code path from the one this candidate covers) has an analogous issue for empty/immediate
  pop layers - it does not share this pipeline's per-draw-object monoid/path_ix scheme.
- A longer fuzzing/property-testing campaign specifically comparing "layer with any non-default
  compose + no content" against a reference CPU rasterizer across many rect positions/sizes (via
  `fuzz-bug-repro`, run separately) would be a reasonable way to characterize exactly which compose
  modes are affected and by how much, without requiring the full architecture fix.
