# Zero-length subpath with Cap::Round/Cap::Square doesn't render the SVG-spec dot

`sparse_strips/vello_common/src/flatten.rs`'s `stroke()`/`expand_stroke()` delegate all stroke
expansion to `kurbo::stroke_with` (kurbo 0.13.1). This looked suspicious because vello issue #1450
has a concrete, already-written repro claiming a zero-length closed subpath with `Cap::Round`
renders nothing instead of the dot the SVG spec requires, and a second, independent report (#1055,
against an earlier vello_hybrid-era renderer) describes the same missing-dot symptom for an open
(non-closed) zero-length subpath - worth checking whether both stem from the same root cause.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: build issue #1450's exact repro (`move_to`+`line_to` to the same point +`close_path`, `Stroke::new(3.0).with_caps(Cap::Round)`) and render it through `vello_cpu`'s real public API (`RenderContext::stroke_path` -> `render_with` -> `Pixmap`), then inspect the pixel buffer | debug build, `cargo run -p vello_cpu --example` against this candidate's own git worktree | Reproduced immediately: 0 non-transparent pixels in the output, confirming #1450 as reported |
| Reading `kurbo` 0.13.1's `stroke_undashed`/`StrokeCtx::finish`/`finish_closed` (the actual dependency vello delegates to) | Registry source at `~/.cargo/registry/.../kurbo-0.13.1/src/stroke.rs` | Root cause found: `LineTo`/`QuadTo`/`CurveTo`/`ClosePath` handlers all skip `do_join`/`do_line` when the segment's endpoint equals the current point ("zero length"), so `forward_path` never gets a first `move_to`; `finish`/`finish_closed` then early-return (`if self.forward_path.is_empty() { return; }`) without ever calling `round_cap`/`square_cap` - this is unconditional, it doesn't matter whether the degenerate subpath was closed with `ClosePath` or left open, matching both #1450 and #1055 |
| Checked upstream kurbo for a fixed release/open issue | `cargo search kurbo` (0.13.1 is still latest), GitHub search via `gh api`/`gh pr diff` | Confirmed this is a known, currently-unfixed upstream bug: kurbo PR #597 ("Fix zero-length `stroke` and `dash` handling", open/unmerged as of this session) implements essentially the same diagnosis, via private `StrokeCtx` fields not reachable from outside the `kurbo` crate |
| Varied the input within the same bug class (CONFIRMATION.md section 7): `Cap::Square` (closed), `Cap::Butt` (closed, must stay empty per spec), a bare `move_to` with no drawing command at all (must stay empty - nothing was ever drawn), and a normal nonzero-length line stroke (must be unaffected) | Same debug-build example harness, both before and after the fix | Before fix: `Cap::Round` (open and closed) and `Cap::Square` all wrongly render nothing; `Cap::Butt` and the bare `move_to` correctly render nothing already; the normal line stroke is unaffected either way. After fix: `Cap::Round`/`Cap::Square` cases now render (center pixel opaque, full expected color), `Cap::Butt`/bare-`move_to`/normal-stroke cases are unchanged |
| Repeatability check (CONFIRMATION.md section 1) | Re-ran the debug-build example 3 times before the fix, 3 times after | 3/3 reproduces before the fix (deterministic, no randomness/seed involved), 3/3 fixed after - this is a pure logic bug with no layout/timing dependency, so determinism is expected and observed |

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, both debug and release builds, using the
real public `vello_cpu` API (`RenderContext::stroke_path`) - no sanitizer/miri needed, this is a
straightforward logic bug in stroke-expansion input handling, not memory unsafety.

**Fully reproducible**: **Yes**. The repro in `repro/` calls `vello_cpu`'s real, public
`RenderContext::stroke_path`/`render_with` entry points with a `BezPath` built the same way any
real caller would build one (byte-for-byte the paths from issues #1450 and #1055) - this is a
single-participant bug (a caller handing a degenerate path straight to the renderer's genuine
public API), so there is no second participant to fake here; CONFIRMATION.md section 11's bar is
satisfied trivially (the "malformed file handed to a parser's real public entry point" case it
describes as always fine).

## Class

logic

## Root cause

`vello_common::flatten::stroke()` (and `expand_stroke()`, which it calls) hands the input path
straight to `kurbo::stroke_with` (kurbo 0.13.1) and then flattens whatever `StrokeCtx::output()`
comes back with. Inside kurbo's `stroke_undashed`, every segment handler (`LineTo`, `QuadTo`,
`CurveTo`, `ClosePath`) explicitly skips calling `do_join`/`do_line` when the segment's endpoint(s)
equal the current point - i.e. whenever the segment has zero length. This means that for a subpath
whose *every* segment is zero-length (the "dot" idiom: `move_to(p)` + `line_to(p)` [+ `close_path`]),
`do_join` is never called even once, so `StrokeCtx::forward_path` never receives its initial
`move_to`. `StrokeCtx::finish`/`finish_closed` (called when the subpath ends, via the next
`MoveTo`/`ClosePath`/end of iteration) both start with `if self.forward_path.is_empty() { return; }`
- so they return immediately without ever reaching the `match style.end_cap { ... Cap::Round =>
round_cap(...), ... }` / `Cap::Square => square_cap(...)` branches that would otherwise paint the
cap. The subpath is silently dropped from the stroked output entirely, regardless of cap style,
which is exactly what the SVG spec says should *not* happen for `Cap::Round`/`Cap::Square` (only
`Cap::Butt` is supposed to produce no output for a degenerate subpath).

This is an **upstream bug in the `kurbo` crate** (pinned at 0.13.1 in this workspace's
`Cargo.lock`), not a vello-authored regression: `vello_common`'s own code has always simply
delegated stroke expansion to `kurbo::stroke_with` since the sparse-strips renderer's stroke
support was first written, and kurbo's `stroke_undashed` has apparently always had this
zero-length-skip behavior (there is no newer kurbo release that fixes it - `cargo search`/crates.io
still show 0.13.1 as latest, and a GitHub search turned up an **open, unmerged** upstream PR,
kurbo#597 ("Fix zero-length `stroke` and `dash` handling"), attempting essentially this exact fix
via private `StrokeCtx` fields (`degenerate_subpath`, `mark_degenerate`) that are not visible or
patchable from outside the `kurbo` crate itself). Both #1450 (closed subpath) and #1055 (open
subpath, no `close_path`) are the same root cause: the zero-length check in `stroke_undashed` does
not distinguish open vs. closed termination, it only checks whether a nonzero-length segment was
ever seen in the subpath.

## Introduced by

Not a vello-side regression to bisect in the usual sense - the defect lives entirely inside the
external `kurbo` crate's `stroke_undashed`, which vello_common has depended on unchanged since
stroke support was first added. Best-effort vello-side attribution: 7a8d8f39 "Add the code for
`vello_common` (#828)" is the commit that first wired `vello_common::flatten::stroke()` to
`kurbo::stroke_with` (per `git log --follow`/`git log -S` on `flatten.rs`); every kurbo version
that provides `stroke_with` (there is no fixed release yet, only the open kurbo#597 PR) has this
same zero-length-subpath-drops-caps behavior, so there is no single vello commit that "introduced"
the bug in the sense of a prior version being correct.

## The fix

Since the actual defect is inside `kurbo::stroke_with`'s private `StrokeCtx` state (not reachable
from vello's own source, and the upstream fix is itself still an open, unmerged PR), the bounded
fix implemented here works around it entirely within `vello_common::flatten`, without touching
`kurbo`:

- `stroke()` now collects the input path into a `Vec<PathEl>` once (needed since it's scanned
  twice: once handed to `kurbo::stroke_with` as before, once by the new degenerate-subpath scan
  below).
- A new `degenerate_cap_dots()` walks the collected elements and identifies any subpath whose every
  segment (including `ClosePath`'s implicit return-to-start) has zero length. For each such subpath,
  `append_cap_shape()` synthesizes the correct fill geometry directly (bypassing `kurbo::StrokeCtx`
  entirely for this part): a full circle of radius `width / 2` for `Cap::Round`, a full
  axis-aligned square of side `width` for `Cap::Square`, nothing for `Cap::Butt` - matching the SVG
  spec's degenerate-subpath cap rule. When the start and end caps differ, `Cap::Round` takes
  priority over `Cap::Square` (documented as a known simplification: a true half-shape split needs
  a tangent direction a zero-length subpath doesn't have; this asymmetric-cap combination isn't
  part of either reported issue).
- This synthetic geometry is chained onto `kurbo`'s own (correct, for every non-degenerate subpath)
  output before the existing `fill()` call, so ordinary strokes are completely unaffected.

Verified against more than the one triggering input (CONFIRMATION.md section 7): both issues'
exact reproductions (#1450 closed, #1055 open), `Cap::Square`, `Cap::Butt` (must stay empty - and
does, both before and after), a bare `move_to` with no other drawing command (must stay empty - and
does), and an ordinary nonzero-length line stroke (unaffected, still renders correctly) - see
`repro/src/main.rs` and `output.txt`.

- Project test suite: **3983 passed / 86 failed both before and after the patch, identical set of
  86 failing test names** (`cargo test --workspace`, this candidate's own worktree). All 86
  failures are pre-existing `*_hybrid` (GPU/wgpu-backed renderer) reference-image mismatches
  unrelated to this change (e.g. `mix::mix_normal_hybrid`, `image::image_bicubic_2x_scale_hybrid`)
  - confirmed identical failing-test-name sets before/after via `diff`, and none of the 86 involve
  stroking/caps/dots (the one stroke-adjacent name, `glyph::glyphs_colr_noto_stroked_hybrid_cached`,
  is a hybrid-renderer glyph test, not a CPU-renderer stroke-cap test). These 86 look like a
  headless-sandbox GPU/driver limitation in this environment, not something this session's change
  touches. No CPU-renderer (`vello_cpu`) or `vello_sparse_tests` non-hybrid test changed outcome.

## What's not done / open questions

- Asymmetric caps (`start_cap != end_cap`, e.g. `Cap::Round` + `Cap::Square` on the same degenerate
  subpath) are handled by a documented simplification (round wins outright) rather than the
  spec-accurate half-shape split kurbo#597 attempts - neither #1450 nor #1055 exercises this case.
- `vello_hybrid` (the GPU-backed renderer #1055 was originally filed against) wasn't independently
  checked for the same bug class - this session only touched `vello_common::flatten::stroke()`,
  which is shared by `vello_cpu`'s stroke path; whether `vello_hybrid` reaches the same code or has
  its own separate stroke-expansion path wasn't verified.
- The proper long-term fix is upstream in `kurbo` (either merging kurbo#597 or an equivalent), at
  which point vello could drop this workaround; this candidate's fix is a stopgap scoped to what's
  reachable from vello's own source tree.
- Dashed strokes (`style.dash_pattern` non-empty) go through a different kurbo code path
  (`dash_iter`) before `stroke_undashed` and weren't specifically tested for the same degenerate-
  subpath dot behavior.
