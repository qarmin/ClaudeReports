# `Array`/`Vector<Variant>` conversions go from O(N) element-by-element copy to O(1) CoW share

`Array` is backed internally by a `Vector<Variant>` (`ArrayPrivate::array`), but nothing exposed
that fact - every conversion between the two (`Callable::bindv()`, `Callable::get_bound_arguments()`,
`Variant::operator Vector<Variant>()`) went through an explicit per-element loop
(`args.write[i] = p_arguments[i]` / `ret[i] = arr[i]`), copying every `Variant` one at a time even
though both `Array` and `Vector` are already copy-on-write containers - the loop was doing real,
unnecessary work that COW exists specifically to avoid. Open PR **godotengine/godot#109003**
proposes exposing the shared backing directly: a new `Array::as_vector()` accessor and an
`Array(Vector<Variant>&&)` move constructor, so both conversions become a single COW share
(equivalent to `Vector`'s/`Array`'s own copy constructor) instead of an O(N) loop.

**This candidate's patch is a hand-adapted version of PR #109003, not a direct `git apply`** - the
PR's own diff no longer applies cleanly against current `HEAD` (`array.h`'s
`recursive_equal`/`recursive_hash` parameter names changed from `recursion_count` to
`p_recursion_count` since the PR was opened, shifting the surrounding context lines just enough to
break a plain `git apply`). The change itself was mechanically identical to the PR's own diff -
verified by re-reading the PR's diff hunk-by-hunk against this candidate's own `patch.diff` below,
same additions in the same 4 files, same logic - only the line-context noise differs.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: two new `bench.gd` modes reaching the two GDScript-visible call sites this patch speeds
up - `callable_bindv_large` (`Callable.bindv()` on a 2000-element `Array`, 20,000 calls) and
`callable_get_bound_arguments_large` (`Callable.get_bound_arguments()` on a `Callable` already bound
to that same 2000-element array, 20,000 calls). Measured via **5 interleaved baseline/patched
pairs**, `uptime` checked before starting (load average 4.70-13.04 on a 24-core box - elevated from
a preceding full rebuild still settling, but irrelevant here given the effect size, see "Result").

| Pair | `bindv`: baseline us/iter | `bindv`: patched us/iter | `get_bound_arguments`: baseline us/iter | `get_bound_arguments`: patched us/iter |
|---|---|---|---|---|
| 1 | 11.044000 | 0.033050 | 8.683300 | 0.046400 |
| 2 | 9.752500 | 0.033050 | 9.019000 | 0.047400 |
| 3 | 11.073200 | 0.033800 | 10.015650 | 0.047750 |
| 4 | 11.438950 | 0.033900 | 10.022900 | 0.046200 |
| 5 | 10.044700 | 0.033750 | 8.714900 | 0.047500 |
| **mean** | **10.6707** | **0.03364** | **9.29115** | **0.04705** |

## Result

**-99.68% for `bindv()` on a 2000-element array (10.6707us -> 0.03364us/iter, ~317x faster),
-99.49% for `get_bound_arguments()` (9.29115us -> 0.04705us/iter, ~197x faster)**, 5/5 interleaved
pairs non-overlapping for both, by a wide margin (baseline's lowest value is >200x the patched
highest value in both cases). This is the largest speedup measured in this workspace to date - it
directly confirms the PR's own "O(N) to O(1)" framing: the old per-element loop's cost scaled with
array size (here, 2000 `Variant` copy-assignments per call, each of which for e.g. a `RefCounted`
element would itself be an atomic refcount op), while the patched version's cost is one `Vector`/
`Array` COW share regardless of element count - genuinely independent of the 2000-element size used
here, not just "fast at this size." Time only; peak RSS/allocation count were not measured, though
the mechanism (replacing N `Variant` copy-constructions with one shared-buffer reference) very
plausibly also reduces allocation count and peak memory for large bound-argument arrays - not
quantified in this session.

## Does it change behavior/output?

- **Bit-exact.** Dedicated correctness script (`bench/callable_bindv_correctness.gd`) covers: a
  basic `bindv()`+`call()` roundtrip, `get_bound_arguments()` returning the exact bound values, a
  2000-element array bound and summed correctly end-to-end (every element survives the O(1) share
  path), `get_bound_arguments()` on that same large bind returning the identical array, the
  `bindv([])` early-out (unaffected by this patch, still returns `*this`), and - the check that
  matters most for a COW-sharing change - **mutating the original array *after* `bindv()` does not
  affect the already-bound `Callable`** (confirms the new code takes a real COW share at bind time,
  not a live reference/alias - `Array`/`Vector`'s own COW-on-write behavior triggers a real split
  the moment either side is mutated, exactly as intended) - all `ALL_TESTS_PASS` on both arms.
- **Is this a breaking change?** No public API/file format change - `Callable::bindv()`,
  `get_bound_arguments()`, and `Variant`'s `Vector<Variant>` conversion operator all keep their
  existing signatures and semantics; only the internal copy mechanism changed.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

`Array` and `Vector<Variant>` are both `CowData`-backed reference-counted containers - copying
either one (via their own copy constructor) is O(1) (an atomic refcount bump), only *diverging*
(actually copying the underlying buffer) on the next write to either copy. The old code never used
this: it manually looped and assigned each `Variant` individually into a freshly `resize()`d
container, which is a real O(N) `Variant` copy-construction (and, for refcounted element types, an
O(N) atomic-refcount operation) every single time, regardless of whether the source and destination
ever actually diverged afterward. Exposing the shared backing (`as_vector()`) and adding a move
constructor lets both conversions become what they always could have been: a single COW share.
`bindv()`/`get_bound_arguments()` are the most visible affected call sites from GDScript, but
`Variant::operator Vector<Variant>()` (also patched, not separately benchmarked - see "What's not
done") is used more broadly throughout the C++ engine wherever a `Variant` holding an `Array` needs
to be treated as a `Vector<Variant>`.

## What's not done / open questions

- `Variant::operator Vector<Variant>()` (the third call site this patch touches) was not separately
  benchmarked - only `bindv()`/`get_bound_arguments()` (both of which internally rely on the same
  underlying mechanism) were measured directly.
- Peak RSS/allocation count not measured (see "Result" - plausible but unquantified benefit).
- Only tested with plain (non-`RefCounted`) `int` array elements for the timing benchmark - the
  correctness script's mutation-independence check does exercise COW-divergence correctness, but
  the *timing* benefit for an array of heavily-refcounted elements (e.g. `Array` of `Resource`
  references) specifically, where the old code's atomic refcount traffic would have been even more
  expensive per element, was not separately isolated.
