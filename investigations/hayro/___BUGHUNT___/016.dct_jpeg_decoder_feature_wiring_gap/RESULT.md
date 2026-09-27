# Open PR #1353's own `jpeg-decoder` feature never reaches the real `hayro`/`hayro-bench` build graph

Found while working `___OPTIMIZATIONS___/046.dct_target_dimension_scaled_decode` (perf-optimize
candidate testing open upstream PR [#1353](https://github.com/LaurenzV/hayro/pull/1353), "honor
`ImageDecodeParams::target_dimension` for DCTDecode and other filters"). That candidate found the
PR's own headline mechanism - a `jpeg-decoder`-based IDCT-scaled JPEG decode, meant to avoid
decoding an oversampled embedded image at full resolution - never actually executes in any real
build of `hayro`, `hayro-bench`, or `hayro-tests`, because the Cargo feature that gates it is
never requested along the real dependency edge. This candidate isolates and directly demonstrates
that specific gap on its own, independent of the rest of PR #1353's behavior/performance
characterization (already covered in `___OPTIMIZATIONS___/046`'s own `RESULT.md`).

This bug does not exist against plain `HEAD` - the `jpeg-decoder` feature this candidate is about
doesn't exist until PR #1353's own diff (`pr_1353.diff` here, identical to
`___OPTIMIZATIONS___/046.dct_target_dimension_scaled_decode/patch.diff`) is applied first. It is a
finding about an *open, unmerged PR*, not about anything currently shipping.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `pr_1353.diff` (`hayro-syntax/Cargo.toml`'s new `[features] default = [..., "jpeg-decoder"]`) alongside the root workspace `Cargo.toml` (`hayro-syntax = { path = "hayro-syntax", ..., default-features = false }`) and every real consumer's own feature list (`hayro-interpret/Cargo.toml`, `hayro-bench/Cargo.toml`, `hayro-tests/Cargo.toml`) | Static read | `default-features = false` at the workspace-dependency level suppresses `hayro-syntax`'s own `default` list for every consumer that references it via `workspace = true`; none of the three real consumers' own explicit feature lists mentions `"jpeg-decoder"` |
| `cargo tree -p hayro-bench` on a fresh clone with `pr_1353.diff` applied | `cargo tree -p hayro-bench 2>/dev/null \| grep -i jpeg-decoder` | No output - confirms `jpeg-decoder` never becomes an edge of `hayro-bench`'s real dependency graph, even though the PR is fully applied and `jpeg-decoder` is a real dependency of `hayro-syntax` in that same build (`cargo tree -p hayro-syntax`, run standalone, does show it - not shown here, already established in `___OPTIMIZATIONS___/046`) |
| One-line fix: add `"jpeg-decoder"` to `hayro-interpret/Cargo.toml`'s own `hayro-syntax` feature list, re-run the same `cargo tree -p hayro-bench` check | Same clone, `patch.diff` applied on top of `pr_1353.diff` | `jpeg-decoder v0.3.2` now appears in the tree - confirms this one-line change is sufficient to close the gap |
| End-to-end reproduction via this candidate's own `justfile` | `just test`: fresh clone, apply `pr_1353.diff`, `cargo tree` check (empty), apply `patch.diff`, `cargo tree` check again (present) | Reran cleanly from `rm -rf clone && git clone` - both steps behave as described above |

## Status

**Confirmed.**

**Fully reproducible**: **Yes** - `cargo tree`, a standard Cargo command, run against the real
workspace's own `-p hayro-bench` package (the actual project binary this PR is meant to speed up),
not a fabricated or hand-constructed scenario. `just test` in this folder reproduces it end-to-end
from a completely fresh clone.

## Class

logic (a Cargo feature-gating/build-configuration oversight, not a crash or memory-safety issue -
its effect is that PR #1353's own performance claim doesn't hold for any real consumer as
submitted, not incorrect output).

## Root cause

PR #1353 adds a new `jpeg-decoder` optional dependency + Cargo feature to `hayro-syntax`, and lists
it in that crate's own `[features] default = [...]`. But the root workspace `Cargo.toml`'s
`[workspace.dependencies]` entry for `hayro-syntax` is `default-features = false`
(`Cargo.toml:34`), which - per normal Cargo feature-unification rules - means every consumer that
pulls in `hayro-syntax` via `{ workspace = true }` must *explicitly* list any feature it wants,
including ones that are `default` in `hayro-syntax`'s own manifest; `hayro-syntax`'s own default
list is simply never consulted for such consumers. `hayro-interpret/Cargo.toml:14` (the only real
path from the published `hayro` crate down to `hayro-syntax`) lists `features = ["std", "images",
"unsafe"]` - the PR adds `"jpeg-decoder"` to `hayro-syntax`'s own default list, but never adds it
here, so it's compiled out of `hayro-interpret`, and therefore out of `hayro`, `hayro-bench`, and
`hayro-tests` too (all three ultimately depend on `hayro-syntax` only through
`hayro-interpret`). `hayro-syntax/src/filter/dct.rs`'s new `try_scaled_decode` fast path is dead
code in every real binary as a direct result.

## Introduced by

The PR itself: not yet merged, so there is no commit hash on `hayro`'s own history to cite - the
gap is in PR #1353's diff as currently posted (`pr_1353.diff` in this folder).

## Fixed by

Not fixed anywhere (upstream or otherwise) as of this writing - the PR is still open. This
candidate's own `patch.diff` is a proposed one-line fix, not something already applied.

## The fix

`patch.diff`: one line in `hayro-interpret/Cargo.toml`, adding `"jpeg-decoder"` to the existing
`hayro-syntax` feature list (`features = ["std", "images", "unsafe"]` ->
`features = ["std", "images", "unsafe", "jpeg-decoder"]`). Verified sufficient via `cargo tree -p
hayro-bench` (see "What was tried"). Likely also worth adding to `hayro-bench`/`hayro-tests`
directly if either is meant to *independently* exercise the feature without going through
`hayro-interpret` - not needed for `hayro-bench`'s own `render_bench`, since that already reaches
`hayro-syntax` transitively through `hayro-interpret` via the published `hayro` crate.

- Project test suite: not re-run for this candidate specifically - `___OPTIMIZATIONS___/046`'s own
  `RESULT.md` already has a full before/after test-suite comparison for "PR + this exact one-line
  fix" (its own "secondary check"), including the resulting `hayro-tests` snapshot regressions
  that come from the scaled-decode path actually running once wired (5-6 tests, matching the PR's
  own disclosed list) - not duplicated here to avoid two slightly-differently-worded accounts of
  the same numbers; see that file for the full table.

## What's not done / open questions

- Whether this gap was intentional (kept opt-in pending review) or an oversight was not confirmed
  with the PR author - not investigated further here, same open question `___OPTIMIZATIONS___/046`
  already noted.
- Not reported on the PR itself (e.g. as a review comment) - this candidate only records the
  finding locally; posting it would need the `gh-issue-style`/PR-review workflow run separately,
  and only makes sense while the PR is still open.
- `hayro-bench`/`hayro-tests`' own `Cargo.toml`s were not checked for whether they'd also need
  `"jpeg-decoder"` added directly (vs. inheriting it transitively via the `hayro-interpret` fix
  above) if a future change ever has either of them depend on `hayro-syntax` directly instead of
  through `hayro-interpret` - not relevant to the current dependency graph, just an unverified
  assumption about how the fix generalizes if the graph changes.
