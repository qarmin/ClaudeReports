# `Node::get_path_to()` using cached tree depth instead of a `HashSet` - measured, no real-world win found

`Node::get_path_to()`'s common-ancestor search builds a `HashSet<const Node*>` of every ancestor of
`this` up to the root, then walks `node`'s ancestors checking set membership until a match is found;
the resulting path `Vector<StringName>` is grown via `push_back()` then reversed. Open PR
**godotengine/godot#111137** (`MERGEABLE` against current `HEAD`, applies clean, single file) adds a
second code path for the common case where both nodes are inside the same live tree: it uses each
node's already-maintained `data.depth` field to walk both nodes up to equal depth (no `HashSet`
needed) and pre-computes the exact output path length (`this`'s depth + `node`'s depth from the
common ancestor) to `resize()` the `Vector` once instead of growing it via `push_back()` + `reverse()`.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: two new `bench.gd` modes - `get_path_to_deep` (a single 300-node-deep chain under the
tree root, `get_path_to()` called from the shallow end to the deep end, 20,000 calls) and
`get_path_to_cross_branch` (two 150-node-deep branches from a common parent, `get_path_to()` called
between the two leaves - the shape that maximally stresses the old `HashSet` build, since both
branches contribute ancestors). Both node hierarchies are added under the real `SceneTree` root so
`is_inside_tree()` is true and `data.depth` is populated, matching the patch's fast-path precondition.
Measured via **5 interleaved baseline/patched pairs**, `uptime` checked before starting (load
average 0.65-0.96 on a 24-core box).

| Pair | deep baseline us/iter | deep patched us/iter | cross-branch baseline us/iter | cross-branch patched us/iter |
|---|---|---|---|---|
| 1 | 3.7247 | 3.6877 | 5.7295 | 5.8235 |
| 2 | 3.7645 | 3.8709 | 5.7954 | 5.6752 |
| 3 | 3.9284 | 3.8471 | 5.8880 | 5.8159 |
| 4 | 3.7896 | 3.7358 | 5.8733 | 5.7275 |
| 5 | 3.8305 | 3.7754 | 6.0327 | 6.0846 |
| **mean** | **3.8075** | **3.7834** | **5.8638** | **5.8253** |

## Result

**No change**: deep mean -0.6% (3.8075us -> 3.7834us), cross-branch mean -0.7% (5.8638us ->
5.8253us) - both deltas are smaller than the run-to-run spread visible within each arm's own 5 runs
(e.g. baseline deep alone spans 3.7247-3.9284, a 5.5% range with no patch involved), and neither
direction is consistent pair-to-pair (patched is faster in 3/5 deep pairs, 3/5 cross-branch pairs -
not the 5/5-style consistency this workspace's real wins show). An earlier, non-interleaved
single-run comparison at shallower trees (depth 40/20) suggested a ~5-7% win, but did not reproduce
once measured properly (interleaved, deeper trees, more runs) - recorded here as a caution about
trusting a single uncontrolled comparison, consistent with `MEASUREMENT.md`'s paired-comparison
discipline. Time only; peak RSS/allocation count were not measured (the mechanism trades one
allocation shape - `HashSet` build + `Vector::push_back` growth - for another - `Vector::resize`
upfront - rather than removing allocations outright, so no large delta was expected there either).

## Does it change behavior/output?

- **Bit-exact.** Dedicated correctness script (`bench/get_path_to_correctness.gd`) checks: a
  cross-branch path and its reverse, a root-to-descendant path, a descendant-to-root path (all
  `..`), a self-path (`"."`), and a not-inside-tree case (which always falls back to the unchanged
  `HashSet` path regardless of this patch, verified separately) - all match the expected `NodePath`
  string exactly on both arms.
- **Is this a breaking change?** No - internal algorithm change only, `NodePath` output is
  unchanged.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

At the depths tested (up to 300), Godot's `HashSet` on pointer keys is apparently cheap enough
(good hash distribution, small enough table to stay cache-resident) that its per-step cost is not
meaningfully different from the depth-comparison walk this patch replaces it with - the dominant
cost in both algorithms is very likely the same thing either way: chasing `data.parent` pointers
through the `Node` hierarchy, which both algorithms do an equal number of times. The patch's other
change (pre-sizing the output `Vector` instead of growing it) also doesn't show up measurably at
these path lengths (300 elements is not large enough for `Vector`'s own geometric-growth
reallocation cost to matter against the rest of the function). This is the kind of case
`HEURISTICS.md`'s anti-pattern section warns about: a plausible-sounding mechanism (swap a HashSet
for cached integers) that doesn't clear the noise floor once actually measured, rather than an
obviously-wrong assumption.

## What's not done / open questions

- Not tested at depths beyond 300 - it's possible a large enough tree (in the thousands) would
  separate the two algorithms more clearly if `HashSet` rehashing/resizing starts to matter, but
  that depth is unrealistic for a real Godot scene tree, so wasn't pursued.
- Not measured: peak RSS/allocation count (see "Result" for why no large delta was expected).
- The PR's own claimed benchmark numbers were not visible in its description body as fetched this
  session (only the benchmark *script* was included, not a results table) - unlike candidates
  109898/120100, this session cannot say whether its own "No change" result actually contradicts a
  specific claimed figure, or whether the PR's own numbers were similarly modest/noisy.
