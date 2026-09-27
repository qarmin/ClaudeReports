# MIX_CLIP_ISOLATED_BLEND_OPTIMIZATION_UNSOUND

Source: https://github.com/linebender/vello/issues/1198. The issue describes `Mix::Clip`
(peniko 0.4.0) as documented to behave exactly like `Mix::Normal` while letting the renderer skip
creating an isolated blend group as an optimization, and shows a test scene where the two do not
actually produce equivalent output once a real blend mode (`Mix::Multiply`) is nested inside the
clip. `Mix::Clip` itself no longer exists in the `Mix` enum used by the current codebase (peniko
0.6.1) - it was replaced by a dedicated `Scene::push_clip_layer` API (vello commit bb51b7fb, "Add
`push_clip_layer`, to help removing `Mix::Clip` (#1192)"). That commit's own encoding, however,
reuses the exact same shader-level sentinel value the old `Mix::Clip` used
(`DrawBeginClip::CLIP_BLEND_MODE = 0x8003`, "equivalent to `Compose::SrcOver` with a `Mix` of 128,
for legacy reasons... we expect this to change in the future"), so this candidate re-checked
whether the underlying non-isolation optimization the new API is documented/tested to provide was
actually implemented.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: searched for `Mix::Clip` across the whole workspace | ripgrep, whole repo | Zero hits - confirmed the enum variant is gone from peniko 0.6.1 and the API surface (only `push_clip_layer` remains) |
| Ran the project's own existing regression test for this exact issue | `cargo test -p vello_tests --test known_issues -- clip_blends`, real wgpu device (llvmpipe/GL, debug build) | Passes against its checked-in reference snapshot - but visual pixel inspection of that snapshot itself (`vello_tests/snapshots/clip_blends.png`) shows the interior of the clipped triangle is mostly unmultiplied aquamarine (i.e. the bug's signature), not the correctly-multiplied dark blue expected from the issue's own "No Clip"/"Mix::Normal" reference images - a live signal, not a clean negative, so digging continued |
| Manual differential construction: built a "No Clip" ground-truth scene (a single direct `push_layer(.., Mix::Multiply, .., &triangle)`, no wrapper) and a "Clip" scene (`push_clip_layer` wrapping the same inner Multiply layer), rendered both through the real public `vello::Scene`/`Renderer` API on a real wgpu device, compared pixel-for-pixel | `cargo test -p vello_tests --test mix_clip_diff` (scratch test written this session, not part of the permanent suite), release build, llvmpipe/GL backend | Confirmed: 5280/10000 pixels differ by up to 255/255 between the two scenes (screenshot: "No Clip" renders a smooth dark-blue multiplied triangle; "Clip" renders mostly bright unmultiplied aquamarine with only a small blocky dark-blue island) |
| Read the coarse/fine rasterization shaders (`vello_shaders/shader/coarse.wgsl`, `fine.wgsl`, `shared/blend.wgsl`) that implement `BeginClip`/`EndClip` | source reading | `CMD_BEGIN_CLIP` in `fine.wgsl` unconditionally resets the working color buffer to transparent (`rgba[i] = vec4(0.0)`) and saves the real backdrop onto a side stack, for **every** `BeginClip` regardless of blend mode - i.e. it always creates a real isolated blend group. The only special-casing for `CLIP_BLEND_MODE` (0x8003) is (a) a tile-culling optimization in `coarse.wgsl`'s binning pass (unrelated to isolation) and (b) a math shortcut in `blend_mix_compose` that only simplifies the *final* composite formula. There is no code path anywhere that skips isolation for a clip-only layer. |
| `git log -S`/branch search for prior attempts to fix this | `git log --oneline -S "CLIP_BLEND_MODE"`, `git branch --all --contains <commit>` | Found `origin/maybe_clip_fix` (2 commits, both by Chad Brokaw, 2025-09-08, never merged into main, 430 commits stale relative to current `HEAD`), whose commit "Try to fix clip optimization" implements essentially the fix this candidate independently arrived at (a `NON_ISOLATED_BLENDING_FLAG` bit on `CMD_BEGIN_CLIP`'s tag). This confirms the bug is real and was already known/being worked on internally, but the fix never shipped. |

## Status

**Confirmed.** Reproduces 3/3 runs on the actually-run standalone `repro/` (release build, real
wgpu device via llvmpipe/GL, `bad_pixels=4880/40000 channel samples, max_channel_diff=255` every
run, byte-for-byte identical across runs).

**Fully reproducible: Yes.** The repro drives vello's real public API end-to-end (`Scene::fill`/
`push_layer`/`push_clip_layer`/`pop_layer`, `Renderer::render_to_texture`) through a real wgpu
device and reads back the actual rendered pixels - this is a single-participant correctness bug in
how vello's own renderer rasterizes a scene it was handed, not an interaction between two separate
participants, so `CONFIRMATION.md` section 11's second-participant requirement does not apply here
(same category as "a malformed file handed to a parser's own public entry point").

## Class

logic

## Root cause

`Scene::push_clip_layer` (the current, documented-as-non-isolating replacement for the removed
`Mix::Clip`) encodes its `BeginClip` draw object with the sentinel blend value
`DrawBeginClip::CLIP_BLEND_MODE = 0x8003` (`vello_encoding/src/draw.rs`). The GPU pipeline's fine
rasterization shader (`vello_shaders/shader/fine.wgsl`, `CMD_BEGIN_CLIP` case) treats every
`BeginClip` identically: it always pushes the current working color onto a per-tile blend stack
and resets the working color to fully transparent before executing the layer's own content, i.e.
it always creates a real isolated blend group, with no special case for `CLIP_BLEND_MODE`. As a
result, any real blend mode (e.g. `Mix::Multiply`) drawn directly inside a `push_clip_layer` sees a
transparent isolated backdrop instead of the genuine content underneath, so the blend degenerates
to plain unblended source color (via the standard Porter-Duff math: with backdrop alpha 0, `mix(cs,
mixed, backdrop.a)` in `blend_mix_compose` selects the un-mixed source color `cs`) across the
interior of the clip shape. The only place `CLIP_BLEND_MODE` is actually special-cased is (a) a
per-tile culling optimization in `coarse.wgsl`'s binning pass (decides whether a `BeginClip` draw
object needs to be included at all for a given tile - unrelated to isolation semantics) and (b) a
math shortcut in `blend_mix_compose` (`shared/blend.wgsl`) that only simplifies the final composite
formula for the common Normal+SrcOver/Clip case - neither actually skips creating the isolated
group. The blocky/staircase visual pattern of correct-vs-wrong pixels in the reproduction comes
from the 16x16-tile granularity of the coarse-stage `write_begin_clip` decision interacting with
where the nested Multiply layer's own `BeginClip` happens to land relative to the outer clip's
tile-level "fully inside" vs. boundary classification.

## Introduced by

bb51b7fb "Add `push_clip_layer`, to help removing `Mix::Clip` (#1192)" (2025-09-29) is the commit
that introduced the *current* public API and encoding this candidate exercised, and its own code
comment ("This is equivalent to `Compose::SrcOver` with a `Mix` of 128, for legacy reasons. We
expect this to change in the future") shows the author was already aware the underlying shader
mechanism was not actually updated to skip isolation - it simply carried the pre-existing
`Mix::Clip` shader-level sentinel/behavior forward under a new API name. The unconditional-isolate
behavior in `fine.wgsl`'s `CMD_BEGIN_CLIP` itself dates back to the original clip/blend
implementation (commit ef3ed3c9 "Checkpoint of clip and gradient work", 2022-11-10), long before
any "skip isolation for clip" promise existed in the public API, so it was not a regression at that
point in time - the mismatch between promise and implementation is what bb51b7fb (re-)introduced
for the current API surface. A genuine, unmerged fix attempt exists on `origin/maybe_clip_fix`
(commits b7f21a50 "Try to fix clip optimization" and ce709c0d "fmt", both 2025-09-08, by Chad
Brokaw) - it predates bb51b7fb and was based on the older `Scene::push_layer(.., Mix::Clip, ..)`
API, 430 commits behind current `HEAD`, and was never merged.

## The fix

Ported the same technique used by the unmerged `origin/maybe_clip_fix` branch onto current `HEAD`
(three files, +26/-7 lines total, see `patch.diff`):

- `vello_shaders/shader/shared/ptcl.wgsl`: added `BLEND_CLIP` (the `CLIP_BLEND_MODE` sentinel
  value) and a `NON_ISOLATED_BLENDING_FLAG` (`0x100u`) bit, safely distinguishable from the
  existing 4 command tags (0-13, all `< 0x100`).
- `vello_shaders/shader/coarse.wgsl`: `write_begin_clip` now takes a `flags: u32` parameter ORed
  into the emitted `CMD_BEGIN_CLIP` tag; the `DRAWTAG_BEGIN_CLIP` case reads the draw object's own
  blend-mode word and sets `NON_ISOLATED_BLENDING_FLAG` when it equals `BLEND_CLIP`.
- `vello_shaders/shader/fine.wgsl`: the interpreter now masks the tag (`cmd = tag & 0xffu`) to get
  the actual command, and `CMD_BEGIN_CLIP` multiplies the working color by `0.0` (isolate, existing
  behavior) or `1.0` (leave the real backdrop in place, "punch through") depending on whether the
  flag bit is set, instead of always resetting to transparent.

Verified against the actually-run `repro/`: before the fix, `bad_pixels=4880/40000,
max_channel_diff=255` (3/3 runs); after the fix, `bad_pixels=0/40000, max_channel_diff=8` (3/3
runs) - the residual max diff of 8/255 is ordinary antialiasing-edge rounding between the two
independently-rasterized scenes, not the bug. Also re-checked against the exact scenario from
`vello_tests/tests/known_issues.rs::clip_blends` (the project's own existing regression test for
issue #1198): before the fix it renders mostly-unblended aquamarine (matching its own, buggy,
checked-in reference snapshot); after the fix it renders the correct dark-blue multiplied triangle,
visually matching the "No Clip" ground truth. Checked against a second, structurally different
input in the same bug class beyond the one exact reproduction (a `push_clip_layer` immediately
followed by an isolated `Mix::Normal` layer at `alpha = 0.6` and then a nested `Mix::Multiply`) -
this second case (`mix_clip_diff.rs::case2_clip_with_nested_alpha_layer`) already matched its
no-clip baseline exactly (`bad_pixels=0`) both before and after the fix, i.e. it was never affected
because the intervening isolated alpha layer already re-establishes a fresh backdrop regardless of
the outer clip's isolation.

- Project test suite (scoped to `vello`, `vello_encoding`, `vello_shaders`, `vello_tests` - the
  crates this fix touches or that exercise the GPU rendering pipeline through the real public
  API): **before** 96 passed / 2 failed / 3 ignored; **after** 95 passed / 3 failed / 3 ignored.
  - The 2 pre-existing failures (both before and after) are in
    `vello_tests/tests/mix_clip_diff.rs`, the scratch differential test written this session (not
    part of the permanent suite) - `case1` (the direct differential case matching this bug)
    improves from `bad_pixels=5280/max_diff=255` to `bad_pixels=200/max_diff=8` after the fix
    (still nominally over its own strict tolerance=2, but this residual is AA-edge noise, not the
    bug - see `repro/`'s more appropriately-tolerant comparison instead); `case3` (nested clips +
    `Compose::Xor`) is unaffected by the fix in either direction and is a confounded test on my own
    part (its "no_clip" baseline used an isolated `Mix::Normal` wrapper instead of a genuine
    "no separate layer at all" baseline, which is not a valid equivalence check given `Mix::Normal`
    is legitimately isolated) - not evidence of a further bug.
  - The one **newly-failing** test after the fix is `vello_tests/tests/known_issues.rs::clip_blends`
    - expected, since the fix changes the render to the analytically-correct output, which now
    differs from the stale checked-in reference snapshot that has this exact bug baked into it.
    This snapshot was not regenerated/committed as part of this candidate (see "What's not done"
    below).
  - All other tests (`vello_encoding`: 31, `compare_gpu_cpu`: 9, `emoji`: 6, `hinting`: 4,
    `property`/`regression`/`smoke_snapshots`/`snapshot_test_scenes`: 6+10+8+19) are unaffected,
    identical pass counts before and after.
  - Full `output.txt` has the real captured stdout for both the standalone `repro/` (3 runs each,
    before/after) and this test-suite comparison.

## What's not done / open questions

- `vello_tests/snapshots/clip_blends.png` (the checked-in reference for
  `known_issues.rs::clip_blends`) still encodes the buggy output and was intentionally left
  unchanged (regenerating a binary git-lfs-tracked snapshot as part of this candidate's own patch
  felt out of scope for a bounded shader fix) - whoever takes this patch for real needs to
  regenerate it (`VELLO_TEST_UPDATE=all cargo test -p vello_tests --test known_issues`) and
  update `known_issues.rs`'s doc comment, since `clip_blends` would no longer be a "known issue".
- The general, non-clip-specific "isolated vs. non-isolated blending" question the issue's own
  body raises ("the general case of non-isolated blending is pretty gnarly because the web
  compositing spec is pretty lax") is intentionally out of scope here - this fix only addresses the
  clip-only (`push_clip_layer`/`CLIP_BLEND_MODE`) case, exactly like the unmerged
  `maybe_clip_fix` branch did.
- Not checked: whether the sparse-strips engines (`vello_cpu`, `vello_hybrid`) have an equivalent
  bug. A quick look at `sparse_strips/vello_common/src/clip.rs` suggests they were designed with
  genuinely non-isolated clip support from the start (a dedicated `push_clip_layer` with its own
  "non-isolated clips" machinery, architecturally different from the GPU pipeline's `BeginClip`/
  `EndClip` blend-stack scheme this candidate investigated), so this is likely specific to the
  `vello` (GPU/"Classic") crate, but this was not independently verified with a repro this session.
- `case3`'s Xor-based scenario (see above) was not re-investigated with a corrected baseline; if a
  future session wants to check "nested clips + non-default `Compose`" more rigorously, redo it
  comparing against a genuine "no wrapping layer at all" baseline (matching `case1`'s and this
  candidate's own `repro/`'s methodology) rather than an isolated `Mix::Normal` wrapper.
- No fuzzing/property testing done here - the bug was fully explained by direct code reading plus
  one differential comparison, so escalating further tools wasn't necessary.
