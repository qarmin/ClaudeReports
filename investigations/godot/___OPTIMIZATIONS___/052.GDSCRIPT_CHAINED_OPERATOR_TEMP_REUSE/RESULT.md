# GDScript reduces temporary-stack-slot count for chained operators, shrinking function stack frames

`GDScriptCompiler` manages compile-time temporaries in a strictly LIFO (stack-like) fashion: a
temporary reserved before another must also be freed after it. For a chained binary expression like
`p0 + p1 + p2 + ... + p9`, the result of each `+` must be reserved *before* the next operand is
parsed (since the result outlives the operand), which pins that slot for the rest of the chain and
forces every subsequent partial sum into a brand-new slot - a chain of N operators ends up needing
N-1 live temporaries simultaneously, growing the compiled function's total declared stack size (the
`GDScriptFunction`'s `Variant` stack array, constructed/destructed via `memnew_placement` for every
declared slot on every call, independent of how many are actually touched by a given call path -
established by this workspace's own candidate 005 investigation into `Object`/stack costs). Open PR
**godotengine/godot#117472** (`MERGEABLE` against current `HEAD`, applies clean) reworks temporary
management to allow freeing any temporary at any time, so a chained expression's intermediate
results can reuse a small, constant number of slots (2, alternating) instead of growing linearly
with chain length - shrinking the compiled function's stack size and therefore its per-call
setup/teardown cost.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: new `chained_operator_call` mode added to `harness/gdbench/bench.gd` - a dedicated
`_chained_add()` function (`return _c0 + _c1 + ... + _c9`, 10 script-level integer members, matching
the PR author's own benchmark shape) called 5,000,000 times in a raw `for` loop (manual
`Time.get_ticks_usec()` timing, not the `_timed()` closure helper, for the same reason as candidate
048's `if_logic_*` modes - the call itself is what's being measured, a closure-call wrapper would
add its own overhead on both sides equally but at this small a magnitude it's cleaner to avoid).
Measured via **6 interleaved baseline/patched pairs** using two saved binaries, `uptime` checked
before starting (load average 0.79-1.46 on a 24-core box).

| Pair | baseline us/iter | patched us/iter |
|---|---|---|
| 1 | 0.081222 | 0.071812 |
| 2 | 0.080497 | 0.071653 |
| 3 | 0.081375 | 0.074480 |
| 4 | 0.079846 | 0.070746 |
| 5 | 0.081357 | 0.072733 |
| 6 | 0.081164 | 0.071393 |
| **mean** | **0.080910** | **0.072136** |

## Result

**-10.8% wall time per call to a function with a 10-operand chained-`+` expression**
(0.080910us -> 0.072136us/iter mean), **6/6 interleaved pairs non-overlapping** (min baseline
0.079846 > max patched 0.074480) - a real, modest, reproducible win, in the same direction and
rough magnitude as the PR author's own reported "~6% reduction in call overhead" (this session's
number is a bit larger, on different hardware/toolchain, matching this workspace's general pattern
of PR-claimed numbers not exactly transferring - see `projects/godot.md`). Time only; this session
did not measure peak RSS/allocation count - the mechanism reduces per-call stack-frame *setup* cost
(fewer `Variant` placement-constructions/destructions per call), which is a time effect, not an
allocation-count one (the stack array itself is one fixed-size allocation per `GDScriptFunction`
regardless of declared size, only *smaller* per this patch, not fewer in count).

## Does it change behavior/output?

- **Bit-exact.** Dedicated correctness script (`bench/chained_op_correctness_test.gd`) covering:
  a 5-operand int chain, a mixed-type float chain (`+`/`*`/`-`/`/`), a 5-operand string
  concatenation chain, a `Vector2` chain, a 10-operand chain (the exact shape being benchmarked),
  a nested/parenthesized chain (`(a+b)*(c+d)-(e-a)`), and a comparison-chain feeding into `and`
  (deliberately exercising both this candidate's temp-reuse mechanism and candidate 048's
  short-circuit-jump mechanism together, since both touch overlapping codegen paths) - all
  `ALL_TESTS_PASS` on both arms.
- **Is this a breaking change?** No public API/file format change - compiled bytecode's temporary
  slot assignment is an internal compiler detail with no external contract.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

A `GDScriptFunction`'s call setup constructs (`memnew_placement`) a `Variant` for every declared
stack slot, and tears them down again on return, regardless of how many of those slots the
particular call path actually uses - this is the same "fixed cost proportional to declared size, not
usage" shape candidate 005 characterized for `Object`'s own per-instance costs. A 10-operand chain
under the old LIFO-only temporary allocator needs 9 simultaneously-live result slots (one per partial
sum, each pinned until the whole chain resolves); under the patched allocator it needs at most 2,
alternating - a real reduction in the compiled function's total declared stack size, which directly
shrinks the fixed per-call construct/destruct cost measured here. The size of the win scales with how
many "extra" slots a function's worst chain used to need, which is why the PR author's own benchmark
found a slight *regression* for a 2-operand chain (no chain-depth benefit, so only measurement noise
remains) but a real win for the 10-operand case.

## What's not done / open questions

- Only integer chains were benchmarked for timing (the correctness script covers other types, but
  their timing wasn't separately measured) - other Variant types resolve their `+`/`*`/etc. through
  different operator-evaluator code paths with different absolute costs, so the *relative* saving
  from fewer stack slots plausibly differs by type, not measured here.
- Not measured: peak RSS/allocation count (see "Result" for why no delta is expected).
- Did not check whether this patch changes anything about GDScript's runtime stack-overflow
  behavior for pathologically long chains (fewer declared slots for the same chain could plausibly
  shift where a stack-depth limit is hit, if one exists) - not investigated.
