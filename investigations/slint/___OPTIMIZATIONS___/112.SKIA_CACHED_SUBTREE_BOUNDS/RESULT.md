# 112 - Wire `cached_subtree_bounds` into Skia's `RendererSealed` impl

Candidate 051 added a `RendererSealed::cached_subtree_bounds(&ItemRc) -> Option<LogicalRect>`
accessor and implemented it for `SoftwareRenderer` by delegating to
`PartialRenderingState::cached_subtree_bounds` - the read that lets `send_mouse_event_to_item`
(`internal/core/input.rs`) skip descending into a subtree the cursor cannot be over. `SkiaRenderer`
also builds on `PartialRenderingState` (it's what backs its own partial-rendering dirty-region
pass), but 051 only wired the accessor into the software renderer's `impl RendererSealed`, leaving
Skia's own impl on the trait's conservative default (`None`) - so every mouse-move hit-test
dispatched through a Skia-backed window still blindly descends into every child, exactly as
described in candidate 015's original finding, regardless of 049/050/051 being applied.

## What was tried

Added one method to `impl RendererSealed for SkiaRenderer` (`internal/renderers/skia/lib.rs`):

```rust
fn cached_subtree_bounds(&self, item: &i_slint_core::item_tree::ItemRc) -> Option<LogicalRect> {
    self.partial_rendering_state()?.cached_subtree_bounds(item)
}
```

`partial_rendering_state()` is the same private accessor `free_graphics_resources`/
`clear_cache`/etc. already use - it returns `None` whenever a rendering notifier is registered
("we don't know where the application might render to, so disable partial rendering"), which
makes this delegation automatically conservative in that case too, with no extra logic needed.
Structurally identical to 051's software-renderer delegation, as expected - mechanical, one
method, ~7 lines including the signature.

**This candidate's `patch.diff` bundles 049+050+051's whole mechanism plus this one method**,
since none of it exists on plain `master` and the accessor being delegated has to exist for the
delegation to compile - same convention 051 itself used for 049+050.

**Harness work needed**: `perf_test`'s existing `--skia-bench` mode (`skia_headless.rs`, added for
candidate 066) never dispatched any input event at all - it only timed `render()` in a loop over
scroll/rects content. Added `--pointer-move` support to it (previously a `--bench`-only flag,
already present on `BenchOptions`): dispatches the same per-frame circular `PointerMoved` pattern
`bench.rs` uses, and widens the timed window to start right before that dispatch (instead of
`draw_if_needed`'s internal `Instant::now()`, which only covered `render()`) so the hit-test cost
itself is included - **only when `--pointer-move` is passed**; without it, timing is byte-for-byte
the same code path as before, so candidates 066/068's existing recorded numbers are unaffected.
`--row-style` was already forwarded generically for `scroll`/`list`, so `touch`/`clipped`/`bare`
needed no further changes.

Baseline for this candidate = `master` + 049/050/051 applied, **without** this one-line Skia
delegation (i.e. `cached_subtree_bounds` still `None` for Skia, same as today). Patched = the same
tree with this candidate's method added. This isolates exactly this candidate's own contribution,
the same way 051 isolated itself from 049/050.

`--skia-bench --mode scroll --count 100000 --frames 20 --pointer-move`, `rdebug` profile, paired
interleaved runs (`tools/paired_skia_headless.sh`, `RUNS=9`), machine load (`uptime`) 3.9-4.5 out
of 24 cores (well under core count, no re-baseline needed):

| row style | single-core (`taskset -c 2`), run 1 | single-core, repeat | multi-core | notes |
|---|---|---|---|---|
| `layout` (5 items/row) | **-12.3% (9/9)** | **-10.2% (8/9)** | **-15.0% (8/9)** | matches 051's own headline scenario |
| `touch` (`layout` + `TouchArea`, hover feedback) | - | - | **-18.1% (7/9)**, `taskset -c 2` | exercises the exact code path a real clickable row uses |
| `bare` (1 item/row, no children) | - | - | -3.2%, **not consistent (5/9)**, `taskset -c 2` | expected: nothing for the pruning to skip, matches 051's own `bare` result |

## Result

The same shape 051 found for the software renderer shows up for Skia too, once the accessor is
actually wired up: a real, consistent win at every row style with children to prune (-10.2% to
-18.1%, single- and multi-core, 7-9 of 9 paired runs), and a wash at `bare` (5/9, no descendants to
skip). Two single-core repeats of `layout` (the standard noise-floor check) agree in direction and
rough magnitude (-12.3% then -10.2%).

## Does it change behavior/output?

- **Bit-exact?** Characterized-difference, but **the difference predates this patch and this
  patch adds none of it**: `--skia-bench`'s PPM snapshots are not bit-exact run-to-run for *any*
  binary in this family, including two runs of the exact same binary with no code change at all -
  confirmed directly. Four same-binary reruns of the baseline binary (`--mode list --count 5000
  --frames 10`) against each other differ by 126-327 bytes out of 3 302 400 (~0.01-0.01%); four
  same-binary reruns of the patched binary differ by 0-291 bytes; baseline-vs-patched pairs differ
  by 0-327 bytes - all three groups overlap in the same range, including some baseline-vs-patched
  pairs landing at exactly 0. The same magnitude (~130-490 bytes) showed up identically on
  `scroll --row-style layout/touch/clipped/bare` too, `--pointer-move` on or off. This is a
  pre-existing nondeterminism in `--skia-bench`'s headless rendering path (likely a font-hinting or
  Skia-internal-cache effect not controlled by this harness's synthetic clock; candidates 099-106
  already found and characterized unrelated nondeterminism sources in other `perf_test` paths -
  this looks like the same family, just not previously noticed for `--skia-bench` specifically,
  since candidate 066's own bit-exact check happened not to hit it). Not investigated further here
  (out of scope for a one-line delegation candidate) - added to `IDEAS.md`.
- **Is this a breaking change?** No - `RendererSealed` is sealed, `SkiaRenderer`'s new method
  return type/signature matches the trait exactly, and every other renderer backend is untouched.
  Semantically this only ever *removes* otherwise-always-`true` descent into subtrees the cursor
  provably cannot be over (same invariant 051 already established and pixel-verified for the
  software renderer, reusing the identical `PartialRenderingState` machinery Skia already runs for
  its own rendering pass) - no new staleness window beyond the one 051's `RESULT.md` already
  documents for the software renderer, since both backends read the same struct.
- `cargo test -p i-slint-core --lib`: **307 passed, 0 failed** before and after (same suite 051
  itself reported; this candidate's only source changes are in `internal/renderers/skia/lib.rs`,
  which this crate's tests don't cover).
- `cargo test -p i-slint-renderer-software --lib`: **8 passed, 0 failed**, unchanged (software
  renderer's own delegation, from 051, is unaffected by this candidate).
- `cargo test -p i-slint-renderer-skia`: **N/A, could not build in this environment** - its
  default (`softbuffer`) feature fails with a set of `E0004`/`E0282`/`E0392` compile errors in the
  `softbuffer` dependency itself, confirmed identical on pristine `master` with no changes applied
  at all (`git stash` + rerun, same errors). Unrelated to this candidate. The code path this
  candidate touches was instead confirmed to compile and run correctly via `perf_test --features
  skia` (a real, working build of the same `i-slint-renderer-skia` crate with a feature
  combination that does build in this environment) - see "What was tried" above; all measurements
  and the pixel-diff characterization above ran against that binary.

## Why the result looks the way it does

Identical mechanism to 051, just previously unreachable for Skia because `RendererSealed`'s
default (`None`) is what every backend gets unless it opts in - `PartialRenderer`/
`PartialRenderingState` themselves are backend-agnostic (Skia's `create_partial_renderer` call in
`render_components_to_canvas` already uses the exact same struct the software renderer does), so
there was nothing backend-specific to design; the accessor just wasn't exposed on that one
`RendererSealed` impl.

## What's not done / open questions

- FemtoVG remains out of scope, as 051 already noted: it moved off `PartialRenderer` onto a
  different (`anyrender`-based) abstraction with no equivalent per-item subtree cache to delegate
  to - giving it this optimization needs its own mechanism, not a delegation.
- The pixel-snapshot nondeterminism described above is real but predates this candidate and isn't
  caused by it; flagged in `IDEAS.md` as its own investigation (same family as 099-106, not
  previously known to affect `--skia-bench`).
- 051's own still-open items (dirty-region walk's bookkeeping, `Text::layout_info`'s consumer) are
  unaffected by this candidate and remain open per its own `RESULT.md`.

## Reproduce

```sh
tools/paired_skia_headless.sh <baseline_bin> <patched_bin> --skia-bench --mode scroll --row-style layout --count 100000 --frames 20 --pointer-move
```

`<baseline_bin>` = `master` + `051.SUBTREE_BOUNDS_HIT_TEST_PRUNE/patch.diff` applied (no further
change). `<patched_bin>` = the same tree + this candidate's `patch.diff` (which is 051's patch plus
this candidate's one method - apply this candidate's `patch.diff` alone against plain `master` to
get the full patched tree in one step). Both built with `cargo build --bin perf_test --profile
rdebug --features skia` inside `perf_test/`.
