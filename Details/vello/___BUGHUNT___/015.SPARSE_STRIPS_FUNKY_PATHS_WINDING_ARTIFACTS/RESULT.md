# SPARSE_STRIPS_FUNKY_PATHS_WINDING_ARTIFACTS

Investigates issue https://github.com/linebender/vello/issues/1240 ("FunkyPathsScene renders
incorrectly in Vello Sparse Strips"): `sparse_strips/vello_example_scenes/src/path.rs`'s
`FunkyPathsScene` exercises a "funky" `BezPath` with a subpath that continues via `line_to` right
after a `close_path` (no intervening `move_to`) and is never itself explicitly closed - a
path-stress case meant to test self-intersecting/degenerate-subpath handling. The scene still
carries a `// TODO: fix issue https://github.com/linebender/vello/issues/1240` comment directly
above its `impl ExampleScene for FunkyPathsScene` at current HEAD, suggesting the bug is still
considered open.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Geometric dump: called `vello_cpu::kurbo::stroke` directly on the scene's exact `missing_movetos` `BezPath` vs. a hand-constructed "PathEncoder-equivalent" version (with the implicit `move_to` that `vello_encoding::PathEncoder::line_to` inserts after a `close_path`, matching `vello_encoding/src/path.rs` line 526-533's own documented mimicry of kurbo's behavior) | Standalone example (`vello_cpu/examples/funky_repro.rs`, deleted after use - not part of the final deliverable), debug build, printed `PathEl` dumps | **Byte-for-byte identical** stroke outline `PathEl` sequences between the raw scene path and the PathEncoder-equivalent one. Stroke was not the site of any divergence; kurbo already implicitly restarts a subpath at the just-closed point per its own documented semantics, matching what `vello_encoding::PathEncoder` does for strokes (it does *not* force-close a trailing open subpath - only fills get that in `PathEncoder::finish()`). |
| Direct rendering of the real scene via the real, public API: `FunkyPathsScene::render()` through `vello_cpu::RenderContext` (real SIMD level via `Level::new()`, real `render_with`), saved to PNG and pixel-diffed | Standalone examples, `vello_cpu` release/debug, `Level::new()` (auto-detected AVX2 on this host), single-threaded | Rendered **without any visible defect** at current HEAD - a correctly bounded pair of triangles, no unbounded fill, no gap. |
| Fetched the actual image attached to issue #1240 (`https://github.com/user-attachments/assets/28a7ceea-87e8-435d-bcbc-07be70b80513`) and compared visually against the current-HEAD render | `curl` download, visual inspection | The attached screenshot shows the second ("bottom") triangle's fill leaking **unbounded to the right edge of the viewport** (a wide trapezoid instead of a triangle) - a classic symptom of a subpath whose closing edge never got emitted, breaking the winding-number invariant the sparse-strip rasterizer relies on. This does **not** match the current-HEAD render, which is correctly bounded. |
| Git archaeology: `git log --oneline --since="2025-09-20" -- sparse_strips/vello_common/src/flatten.rs sparse_strips/vello_common/src/flatten_simd.rs sparse_strips/vello_common/src/tile.rs sparse_strips/vello_common/src/strip.rs`, then inspected the most promising hit | `git show` | Commit b373c2ca "vello_common: Correctly handle subpaths without initial `MoveTo` (#1544)" (2026-03-29) rewrote exactly the function responsible (`flatten_simd.rs`'s `flatten()`), removing a `closed: bool` flag whose bug matches the screenshot symptom precisely (see Root cause below), and added its own regression test (`implicit_subpaths` in `vello_sparse_tests/tests/basic.rs`). |
| Causal verification: reintroduced the exact pre-b373c2ca `closed`-flag logic into current HEAD's `flatten_simd.rs` (adapted only for the unrelated `RectU16`/`Affine`-parameter API changes made in later, unrelated commits), rebuilt, re-ran the real-scene repro | Same standalone example, debug and release builds, isolated `git worktree`, reverted immediately after | **Reproduced the exact issue-#1240 artifact** (unbounded trapezoid fill, pixel-identical composition to the attached screenshot) with the old logic in place, and confirmed clean/correct rendering again immediately after reverting. This directly demonstrates the fix commit is what resolved the symptom, not a coincidental unrelated change. |

## Status

**Not reproducible** (at current HEAD, base_commit 7516831426fb70f36d62c521c5477f6a7265b36a,
2026-08-31).

The bug **was real** and is fully root-caused (see below) via a genuine historical regression that
matches the GitHub issue's own attached screenshot pixel-for-pixel in composition. It has already
been fixed upstream - independently of this bug-hunt workspace - by commit b373c2ca (merged
2026-03-29, roughly 5 months before this session), as an incidental side effect of an unrelated
subpath-handling rewrite (motivated by a Zulip discussion about left-edge culling, not this issue).
The `// TODO: fix issue #1240` comment directly above `FunkyPathsScene`'s `impl ExampleScene` in
`sparse_strips/vello_example_scenes/src/path.rs` is now **stale** - the scene renders correctly via
`vello_cpu` and nothing gates it out of the default scene list (it is rendered by default in both
the native winit demo and the wasm/webgl demo), so end users are not currently seeing this defect.

Verified 3/3 times against a real, standalone `repro/` (see below): byte-identical PASS at
base_commit, and byte-identical FAIL when the pre-fix logic is deliberately reinstated - fully
deterministic, no randomness/threading/seed involved.

**Fully reproducible**: N/A (no bug to reproduce at base_commit - see Status). The investigation
itself (both the "not reproducible at HEAD" finding and the historical-bug causal verification) is
fully reproducible on demand via `just test` / `just test-historical-bug` against a genuinely fresh
clone of base_commit (verified in this session - see `output.txt`).

## Class

N/A (not Confirmed - the underlying bug class, for the historical record, would have been
**logic** - a missing closing edge silently breaking the winding-number invariant sparse-strip
rendering depends on, not a crash/panic/UB).

## Root cause

**Historical, already fixed - documented here for completeness since the mechanism precisely
explains the GitHub issue's screenshot.**

Before commit b373c2ca, `sparse_strips/vello_common/src/flatten_simd.rs`'s `flatten()` function
(shared by both fill and stroke flattening, and shared by both the `vello_cpu` and `vello_hybrid`
backends) tracked subpath-closure state with a `closed: bool` that started `true` and was only
ever reset to `false` by an **explicit** `PathEl::MoveTo`:

```rust
let mut closed = true;
// ...
PathEl::MoveTo(p) => { /* ... */ closed = false; /* ... */ }
// ...
PathEl::ClosePath => {
    closed = true;
    if last_pt != start_pt { callback.callback(LinePathEl::LineTo(start_pt)); }
}
// ... end of function:
if !closed && last_pt != start_pt {
    callback.callback(LinePathEl::LineTo(start_pt));
}
```

When a subpath continues via `LineTo`/`QuadTo`/`CurveTo` immediately after a `ClosePath` with **no**
intervening explicit `MoveTo` (exactly `FunkyPathsScene`'s `missing_movetos` path: `close_path()`
followed directly by `line_to((0.0, 400.0))`), `closed` incorrectly stayed `true` for the rest of
the path, because nothing ever executes the `MoveTo` arm's `closed = false` again. If that
continued subpath was itself never explicitly closed with its own trailing `close_path()` (as is
the case here - the scene's last two `line_to` calls have no closing `close_path()` at all), the
function reached its end with `closed` still `true`, so the final
`if !closed && last_pt != start_pt` check was **skipped**, and the subpath's closing edge back to
its own start point was **never emitted**.

A sparse-strip rasterizer computes fill coverage from a running winding number as it scans each
row left-to-right; that winding number must return to 0 by the time it reaches an edge of the
fillable region for the fill to stop. With the closing edge missing, the winding number for that
subpath never returns to 0 within the shape's own bounds, so the fill continues all the way to the
tile/viewport's right edge - producing exactly the wide unbounded trapezoid in the issue's
screenshot (and reproduced verbatim in this session - see `repro/funky_scene_HISTORICAL_BUG_reference.png` next to `repro/issue_1240_original_screenshot.png`).

Commit b373c2ca replaced this entirely: it removed the `closed` flag and instead tracks closure
purely via direct point comparison (`last_pt != start_pt`) that doesn't depend on any state
surviving across a subpath boundary, and additionally reset `last_pt` back to `start_pt` right
after emitting a `ClosePath`'s closing edge (matching kurbo's own documented behavior: "If `quad_to`
is called immediately after `close_path` then the current subpath starts at the initial point of
the previous subpath"). Both of these fix the exact defect above as an incidental consequence,
alongside the commit's stated (unrelated) motivation.

## Introduced by

Not determined precisely within this session's scope (not required - Status is not `Confirmed`).
The `closed`-flag design was present in `flatten_simd.rs` since at least the file's introduction of
the shared SIMD flattening rewrite; `git log -S"let mut closed"` on that file was not run to pin
down the exact originating commit, since the fix (and thus the practical "when did this stop
mattering" question) was the useful fact for this candidate, not the original introduction date.

**Fixed by**: b373c2ca "vello_common: Correctly handle subpaths without initial `MoveTo` (#1544)"
(2026-03-29).

## The fix

No fix needed/applied - already fixed upstream at base_commit, independently of this workspace.
See "Status" above. `historical_bug.patch` in this folder is **not** a fix; it is the *inverse* -
a deliberate, temporary reintroduction of the pre-fix defect, included solely so `just
test-historical-bug` can demonstrate the causal mechanism against a fresh clone (see `output.txt`).
It was applied to this session's isolated `git worktree` only, verified, and immediately reverted
(`git checkout --`) before this folder was written - the main project tree and this session's
worktree are both clean of it.

- Project test suite (base_commit, release, no patch applied - there is nothing to fix):
  `cargo test -p vello_common --release` -> **159 passed; 0 failed; 1 ignored**.
  `cargo test -p vello_cpu --release --lib` -> **81 passed; 0 failed**.
  `cargo test -p vello_sparse_tests` (the real rendering-snapshot suite, closest in kind to this
  candidate) -> **N/A this session**: its reference PNGs are Git-LFS-tracked and this sandbox has
  no `git-lfs` binary, so no worktree/clone in this session could check them out at all (see
  `___BUGHUNT___/README.md`). The `vello_common`/`vello_cpu` unit-test numbers above, plus this
  candidate's own real-scene `repro/`, are the available evidence of current-HEAD correctness for
  this code path in this environment.

## What's not done / open questions

- The stale `// TODO: fix issue https://github.com/linebender/vello/issues/1240` comment in
  `sparse_strips/vello_example_scenes/src/path.rs` (directly above `impl ExampleScene for
  FunkyPathsScene`) was not removed - this bug-hunt workspace does not modify the main project's
  source as a side effect of an investigation (see `SKILL.md`'s "the one rule that matters most").
  Worth a small follow-up PR removing/updating that comment, since the scene renders correctly now.
- `vello_hybrid` (the GPU/WGSL-shader sparse-strips backend) was not independently exercised this
  session - it shares the exact same `vello_common::flatten`/`flatten_simd` code that was fixed, so
  the same fix should apply equally to it, but this was not verified end-to-end (would need a real
  or software Vulkan/GL adapter, not attempted - see `___BUGHUNT___/README.md`).
- `only_movetos` (two consecutive `move_to` calls, no `line_to` at all) and the fully `empty`
  `BezPath` - the other two "funky" cases in the same scene - were traced by hand through
  `flatten_simd.rs`'s current logic and confirmed to produce no output (correctly invisible), but
  were not independently pixel-verified via a render the way `missing_movetos` was.
