# `JSON::_from_native()` pre-sizes the output `Array` instead of repeated `push_back()`

`JSON::_from_native()` (`core/io/json.cpp`, backs the GDScript-bound `JSON.from_native()`) converts
a native packed array (`PackedVector3Array`, `PackedColorArray`, `PackedFloat32Array`, ...) or
`Dictionary`/`Array` into a JSON-safe representation by building an `Array` one `push_back()` at a
time - for a `PackedVector3Array` this means 3 separate `push_back()` calls per source element (one
per component). Open PR **godotengine/godot#109898** (`MERGEABLE` against current `HEAD`, applies
clean) replaces every one of these loops with a single `args.resize(final_size)` up front plus a
direct `Array::Iterator` fill, for every packed-array/Dictionary/Array branch in the function.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: new `json_from_native_packed` mode added to `harness/gdbench/bench.gd` - builds a
1,000,000-element `PackedVector3Array`, then times a single `JSON.from_native(arr)` call directly
(not amortized over many iterations - matches this operation's own O(n) single-call shape, and the
PR's own benchmark methodology). Measured via **interleaved baseline/patched pairs** (5 pairs, A/B/
A/B/.../A/B using two saved binaries so no rebuild is needed between measurements), `uptime` checked
before starting (load average 2.10-2.26 on a 24-core box).

| Pair | baseline us | patched us |
|---|---|---|
| 1 | 48286 | 35340 |
| 2 | 48561 | 36311 |
| 3 | 46090 | 34359 |
| 4 | 46561 | 33446 |
| 5 | 45741 | 35572 |
| **mean** | **47047.8** | **35005.6** |

## Result

**-25.6% wall time** (47047.8us -> 35005.6us mean, 1,000,000-element `PackedVector3Array` ->
`JSON.from_native()`), **5/5 interleaved pairs non-overlapping** (min baseline 45741 > max patched
36311) - a real, reproducible, moderate win.

**This is far smaller than the PR's own claimed number** (its own table reports `PackedVector3Array`
going from 11715ms to 1519ms at 10M elements, a 7.7x speedup / -87%, versus this session's -25.6% at
1M elements against current `HEAD`). Not investigated further to explain the gap (see "What's not
done"), but the most likely explanation is that `Array`'s own growth strategy has changed since the
PR was opened (its geometric-growth amortization may have been weaker, or absent, in whatever base
commit the PR's own numbers were measured against) - i.e. the PR's claimed baseline may no longer
match current `HEAD`'s `push_back()` behavior, the same class of "an old PR's numbers may be stale
against current `HEAD`" caveat this workspace has seen before (candidate 004's `just test`
reproduction landing at -36.8% instead of the recorded -20.1%, in the other direction). The
mechanism (avoiding N `push_back()` calls' amortized-but-nonzero per-call overhead by pre-sizing
once) is still real and the effect is still real, just smaller here than claimed. Time only; this
session did not measure peak RSS/allocation count (the patch doesn't change the number or type of
allocations, only when/how the final `Array`'s backing buffer is grown, so no allocation-count
delta is expected).

## Does it change behavior/output?

- **Bit-exact?** Yes - verified with a dedicated roundtrip script (`bench/json_roundtrip_test.gd`)
  covering every packed-array type this patch's diff touches (`PackedVector3Array`,
  `PackedColorArray`, `PackedInt32Array`, `PackedStringArray`) plus a `Dictionary`: for each,
  `JSON.to_native(JSON.from_native(x)) == x`, run against **both** the baseline and patched builds,
  all `ALL_TESTS_PASS` on both.
- **Is this a breaking change?** No - internal conversion-function change only, output format
  (`Array`/`Dictionary` shape) is unchanged, only how it's built internally.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.
- **Testing-methodology note, recorded so it isn't re-triggered**: an earlier attempt to run the
  patched binary from a copy in `/tmp/` crashed with a C++ backtrace inside `modules/mbedtls/tests/
  test_crypto_mbedtls.cpp` (missing crypto test-data files via a broken relative path). This is
  **not a bug in this patch** - it is the already-documented `projects/godot.md` gotcha ("a binary
  must be run from `bin/`, not copied to `/tmp`"), reproduced here as a fresh reminder of why that
  rule exists: running the *exact same* patched binary from `bin/` instead produces the clean
  1427/1426/1/3 result above with no crash. Caught before being mis-reported as a regression only
  because the test suite was re-run from the correct location before concluding anything.

## Why the result looks the way it does

`Array::push_back()` already amortizes its own backing-buffer growth (geometric doubling), so this
patch's saving isn't about avoiding reallocation - it's about avoiding N repeated bounds-check/
capacity-check branches and N separate `Variant` move-assignment call sites (one per `push_back()`)
in favor of one upfront `resize()` and a straight iterator walk. That is a real but modest
per-element constant-factor win, consistent with the -25.6% measured here, rather than the
order-of-magnitude change a broken (non-amortized) growth strategy would produce - which is why this
session's number is real but much smaller than the PR's own claimed figure (see "Result").

## What's not done / open questions

- **The gap between this session's -25.6% and the PR's claimed -87% was not root-caused.** Would
  need to check out the PR's actual base commit (not stated in the PR itself, and not investigated
  here) and compare `Array::push_back()`'s growth strategy at that point in history against current
  `HEAD`, or re-run the PR author's own exact benchmark methodology (10M elements, `Time.get_ticks_msec()`
  wall time from a `_ready()`-triggered button press per their description) rather than this
  session's 1M-element headless `--script` measurement, to rule out a benchmark-methodology
  difference rather than a baseline-drift one.
- Only `PackedVector3Array` was benchmarked (this session's harness); the PR's diff touches 9 other
  branches (`Dictionary`, plain `Array`, `PackedByteArray`, `PackedInt32Array`, `PackedInt64Array`,
  `PackedFloat32Array`, `PackedFloat64Array`, `PackedStringArray`, `PackedVector2Array`,
  `PackedColorArray`, `PackedVector4Array`) not individually timed here, though the roundtrip
  correctness check does cover a representative subset of them.
- Not measured: peak RSS/allocation count (see "Result" for why no delta is expected).
