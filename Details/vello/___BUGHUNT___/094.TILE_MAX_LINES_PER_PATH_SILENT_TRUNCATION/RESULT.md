# TILE_MAX_LINES_PER_PATH silent release-build truncation

`sparse_strips/vello_common/src/tile.rs`'s `make_tiles_analytic_aa_impl` (~line 543) and its
`make_tiles_msaa` MSAA sibling (~line 971) each follow a `debug_assert!(lines.len() <=
MAX_LINES_PER_PATH, ...)` immediately with `lines.iter().take(MAX_LINES_PER_PATH as usize)`.
`MAX_LINES_PER_PATH` is `1 << (32 - 5) = 134,217,728` (5 bits of the packed per-line winding/
intersection word are reserved). In a release build (`debug_assertions` off - vello_cpu/
vello_hybrid's normal deployment configuration) the assert compiles away entirely, so a single
flattened path whose line count exceeds this limit silently has its tail dropped, with no error,
panic, or log of any kind reaching the caller - the render simply "succeeds" while quietly
producing an incomplete/wrong shape. This candidate was flagged as a follow-on to this workspace's
own 004 (LARGE_CUBIC_UNBOUNDED_FLATTEN_ALLOC) and 012 (QUADTO_ZOOM_SUBDIVISION_HANG), which showed
the same unbounded, transform-scale-dependent flattening estimate can push a single path's line
count arbitrarily high; this candidate specifically targets the "large but not astronomical" zone
of that same input space, well below the scale that triggers 004's OOM abort or 012's practical
hang.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: reused 012's "extreme zoom on an ordinary `QuadTo`" mechanism, but searched for the transform scale that lands the flattened line count just *above* `MAX_LINES_PER_PATH` rather than at an astronomical scale, via a temporary `#[ignore]`d unit test in `strip_generator.rs` (added/removed in a disposable git worktree, never committed) | debug and release builds, `cargo test -p vello_common --lib --release -- --ignored --nocapture` | Found `Affine::scale(1.2e16)` on a single `QuadTo` curve produces exactly 134,744,919 lines (527,191 over the limit) in ~1.6s - fast enough to be an entirely ordinary, non-hanging, non-OOM-ing render |
| Real end-to-end confirmation: built a standalone `repro/` crate calling `vello_cpu`'s public, ordinary API (`RenderContext::fill_path` + `RenderContext::render`, exactly what any real caller uses to fill a path) with that same curve/transform | debug build (`cargo run`) and release build (`cargo run --release`), 3 repeat runs each of the release case | Debug build: deterministic panic inside `tile.rs`'s `debug_assert!` ("Max. number of lines per path exceeded. Max is 134217728, got 134744919."), 1/1. Release build: deterministic silent completion with no panic/error/log, 3/3 identical runs (~1.4-1.6s each), rendering a pixmap that looks like an ordinary successful render despite 527,191 of the path's lines never being tiled |

No `miri`/ASAN/fuzzing was needed - this is a plain unbounded-flattening-output-count question
(the same class already characterized by candidates 004/012), not a memory-safety/UB question;
manual boundary-value construction plus a real timed run of the public API fully characterized it
on the first attempt.

## Status

**Confirmed**. Reproduces 3/3 runs in a release build (fully deterministic - not layout/timing/
seed-dependent, the line count is a direct function of the curve shape and transform scale), and
1/1 in a debug build (also deterministic, same mechanism).

**Fully reproducible**: **Yes**. `repro/` calls the real, public `vello_cpu::RenderContext::fill_path`
+ `RenderContext::render` entry point - the same API surface every vello_cpu application uses to
fill a path - with an entirely ordinary single `QuadTo` curve and a plain `Affine::scale(...)`
transform. No private/internal function (`Tiles::make_tiles_analytic_aa`, `StripGenerator`, the
private `flatten_simd` internals) is called directly; `repro/`'s independent call to
`vello_common::flatten::fill` (also public) is used only to *report* the resulting line count for
the reader's benefit, not to trigger the bug. This is a single-participant case (the "attacker" is
just document/path data and a transform, not a second process/peer), so `CONFIRMATION.md` section
11's single-participant carve-out applies, the same as candidates 004/012.

## Class

logic

## Root cause

In both `Tiles::make_tiles_analytic_aa_impl` and `Tiles::make_tiles_msaa`
(`sparse_strips/vello_common/src/tile.rs`):

```rust
debug_assert!(
    lines.len() <= MAX_LINES_PER_PATH as usize,
    "Max. number of lines per path exceeded. Max is {}, got {}.",
    MAX_LINES_PER_PATH,
    lines.len()
);
...
for (line_idx, line) in lines.iter().take(MAX_LINES_PER_PATH as usize).enumerate() {
```

`MAX_LINES_PER_PATH` (`1 << (32 - INT_MASK_SHIFT)` = 134,217,728) exists because each line's
packed winding/intersection word only has `32 - 5 = 27` bits free to hold a line index. The
`debug_assert!` is the *only* signal that a path exceeded this limit - it is compiled out entirely
in release builds (`cfg(debug_assertions)`, off by default in a release profile). The `.take(...)`
call right below it is what actually enforces the limit at runtime in every build, silently
discarding every line past index `MAX_LINES_PER_PATH - 1` with no error, panic, return value
change, or log message of any kind. The caller (`StripGenerator::generate_with_clip`, called from
`generate_filled_path`/`generate_stroked_path`) has no way to detect this happened - it receives
exactly the same "success" it would for a well-formed path, just missing however much of the
path's geometry landed past the cutoff.

This is architecturally identical to candidates 004 and 012 (an unbounded, transform-scale-
dependent subdivision estimate in `flatten_simd.rs`'s cubic/quad flattening), but is the next
layer down the pipeline: even if 004/012's own subdivision estimates were bounded/clamped rather
than left fully unbounded (as their own patches propose), a sufficiently large but still-bounded
subdivision count for a single path can still land above `MAX_LINES_PER_PATH`, and this exact
silent-truncation behavior would still fire.

## Introduced by

7606c5b4 "Pack `Tile` into 64 bits (#852)" (2025-03-18) - found via `git log -S
"MAX_LINES_PER_PATH" -- sparse_strips/vello_common/src/tile.rs`. This commit introduced
`MAX_LINES_PER_PATH` itself (replacing an effectively-unreachable `u32::MAX`-based line-index
scheme) together with the `debug_assert!` + `.take(MAX_LINES_PER_PATH as usize)` pattern in the
(then-unified) AA/MSAA tiling function - i.e. the release-silent-truncation behavior has existed,
unchanged in mechanism, since this commit. The later split into separate AA
(`make_tiles_analytic_aa_impl`) and MSAA (`make_tiles_msaa`) functions (4ea47b47, "Separate
make_tiles into AA and MSAA versions (#1319)") duplicated the same pattern into both, which is why
this candidate names two locations rather than one.

## The fix

Replaced the `debug_assert!` in both locations with an always-on `if lines.len() >
MAX_LINES_PER_PATH as usize { warn!(...) }` check (using the `log` crate, already a dependency of
`vello_common` and already used for an analogous "malformed input, falling back" case in
`flatten.rs`'s NaN-path handling). This makes the exact same condition visible in every build
profile via the standard `log` facade, instead of only in builds with `debug_assertions` enabled.
The `.take(MAX_LINES_PER_PATH as usize)` truncation behavior itself is left unchanged - the doc
comment on `MAX_LINES_PER_PATH` already states truncation "may result in visual artifacts" as a
documented, structural ceiling of the packed bit layout (27 bits available for a line index); a
real fix that avoids the truncation would require widening the packed representation or exceeding
the render, which is a larger design decision out of scope for a bounded session fix. Making the
existing, intended fallback path observable in every build is the bounded fix this session
targets.

Verified: `repro/` re-run against the patched tree (`just apply && (cd repro && cargo run --release
&& cargo run)`) no longer panics in the debug build, and both debug and release builds now print
`[WARN vello_common::tile] Path has 134744919 lines, exceeding the max of 134217728. The remaining
527191 lines will be ignored, which may result in visual artifacts.` - full output in `output.txt`.

- Project test suite: `cargo test -p vello_common --lib`: 159 passed, 0 failed, 1 ignored, both
  before and after the patch (the 1 ignored test predates this session and is unrelated).
  `cargo test -p vello_cpu --lib`: 81 passed, 0 failed, both before and after. `cargo clippy -p
  vello_common --all-features` shows no new warnings in `tile.rs` (pre-existing warnings in
  unrelated files `probe.rs`/`record.rs` are unchanged). `cargo build -p vello_common --release
  --no-default-features --features libm` (the `no_std` configuration) also builds cleanly with the
  patch applied.

## What's not done / open questions

- The `MAX_LINES_PER_PATH` structural ceiling itself (27 bits for a line index) is unchanged by
  this fix - a path that genuinely needs more than 134M lines still renders incompletely, just
  with a visible warning now instead of total silence. Whether that ceiling should be raised (at
  the cost of fewer bits for winding/intersection flags, or a wider packed word) is a design
  decision for the project maintainers, not attempted here.
- `make_tiles_msaa` (the MSAA sibling) currently has no caller anywhere in this workspace outside
  its own tests/benches (`vello_bench`) - it is public API but not wired into any of the shipped
  renderer backends (`vello_cpu`, `vello_hybrid`) at this commit. The fix was still applied there
  for consistency (same bug, same fix, same file), but the primary confirmed end-to-end
  reproduction targets the AA path (`make_tiles_analytic_aa_impl`), which is the one every shipped
  renderer actually exercises.
- Not re-checked: whether 004's or 012's own (unapplied, per this workspace's "always revert"
  rule) proposed fixes would change what scale/curve shape is needed to land just above
  `MAX_LINES_PER_PATH` - this candidate's repro constructs the over-the-limit line count directly
  against the current, unpatched flattening code, independent of whether 004/012 are ever fixed.
