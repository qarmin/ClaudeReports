# GDScript `if` statement short-circuit jump codegen for `and`/`or`

`GDScriptCompiler::_parse_block()`'s `IF` case compiles `if a and b:` by first materializing the
boolean result of the whole condition expression into a temporary stack slot (via
`_parse_expression()`, which itself emits a conditional-assign-then-jump sequence for each `and`/
`or`), then branches on that temporary with a single `write_if()`. This is open, still-unmerged
upstream PR **godotengine/godot#120660** (`MERGEABLE` against current `HEAD`, applies with
`git apply --check` cleanly at base_commit below) - it replaces this with direct short-circuit
jumps (a `jump-if-not`/`jump-if` chain straight to the true/false block), removing one boolean
assignment and one conditional jump per top-level `and`/`or` operator in an `if` condition. Only
`if` statements are touched (not `while`/`match`/ternary - explicitly out of scope per the PR's own
"Future Work"), and only `and`/`or` at the position that determines the branch; anything else
inside the condition still goes through the existing `_parse_expression()` path unchanged.

Flagged in this workspace's `IDEAS.md` on 2026-09-09 as real but deprioritized for being "too
large/risky without extra correctness infrastructure" - revisited this session because the PR ships
its own dedicated correctness test (`if_logical_op.gd`/`.out`, 8 combinations of nested `and`/`or`,
verified against a naive non-short-circuit reference computation in the same script) and because
this session's brief was explicitly to find real runtime/CPU wins outside the startup-RSS vein this
workspace had otherwise focused on for two sessions straight.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes` (debug symbols enabled this session for profiling reuse across candidates -
`-g2` does not change optimization level, `-O2` in both arms).

## What was tried

Benchmark: 4 new modes added to this workspace's existing `harness/gdbench/bench.gd` (`grow a
few, don't multiply`), matching PR #120660's own benchmark methodology (a raw `for` loop directly
timed with `Time.get_ticks_usec()`, not the `_timed()` closure-per-iteration helper other modes
use, since a closure call's own overhead would dominate the few-nanosecond effect being measured
here) - `if_logic_baseline` (empty loop, to subtract per-iteration loop overhead), `if_logic_and`
(`if a and b: pass`, both `false`), `if_logic_or` (`if a or b: pass`, both `false`), `if_logic_nested`
(`if (a and b) or (c and d): pass`, 3 operators). `n = 20,000,000` iterations each, no warmup
subtraction needed (single fixed-cost loop, not the `_timed()` 10%-warmup pattern). 3 runs per
arm, `--headless --path harness/gdbench --script bench.gd -- if_logic_baseline if_logic_and
if_logic_or if_logic_nested`, `uptime` checked before both baseline and patched runs (load average
0.49-1.48 on a 24-core box, both well under core count).

| Mode | baseline (pristine `HEAD`) us/iter (3 runs) | patched (PR #120660) us/iter (3 runs) | delta (raw) | delta (minus loop overhead) |
|---|---|---|---|---|
| `if_logic_baseline` (empty loop) | 0.006603, 0.005528, 0.005583 (mean 0.00590) | 0.005376, 0.005353, 0.005393 (mean 0.00537) | -9.0% | n/a |
| `if_logic_and` | 0.020467, 0.014442, 0.014500 (mean 0.01647) | 0.009543, 0.009259, 0.009378 (mean 0.00939) | -43.0% | -64.0% (0.01057 -> 0.00402 us/iter) |
| `if_logic_or` | 0.023527, 0.024823, 0.021505 (mean 0.02329) | 0.012222, 0.011919, 0.012172 (mean 0.01211) | -48.0% | -63.5% (0.01739 -> 0.00674 us/iter) |
| `if_logic_nested` (3 operators) | 0.035775, 0.034802, 0.032902 (mean 0.03449) | 0.014891, 0.014885, 0.015224 (mean 0.01500) | -56.5% | -67.1% (0.02859 -> 0.00963 us/iter) |

Single-core-vs-multi-core is not a meaningful axis here: this is GDScript interpreter throughput
for one script's own execution, not a parallelizable workload - both arms ran unpinned on an
otherwise-idle box (see `uptime` figures above), matching how this workspace's other GDScript
interpreter-speed candidates (001, 004, 005) were measured.

## Result

Real, consistently reproduced speedup: **-43% to -57% raw time per `if a-and/or-b` evaluation**,
growing with the number of `and`/`or` operators in the condition (one operator: -43/-48%, three
operators: -56.5%), which matches the mechanism (each operator removes one fixed jump+assign pair,
so the win compounds). After subtracting the empty-loop's own overhead (measured identically both
before and after - 0.00590 vs 0.00537 us/iter, i.e. stable, not itself affected by this patch since
`for i in range(n): pass` contains no `if`), the net per-operator win is **-63% to -67%**, closely
matching PR #120660's own reported Windows/MSVC numbers (A-E: -69%, B-E: -48%) despite different
hardware/toolchain (this session: Linux/GCC vs. the PR's Windows/MSVC). No run overlap between
baseline and patched distributions in any of the three modes across 3 runs each.

This measures time only; the patch changes bytecode shape (fewer stack slots per `if`) but this
session did not measure peak RSS or allocation count, since the mechanism (removing jumps/temporary
stack assigns from generated bytecode, not changing anything at runtime allocation-wise) gives no
reason to expect a memory-dimension effect, and none was claimed by the PR either.

## Does it change behavior/output?

- **Bit-exact?** Yes, for the semantics that matter (the compiled program's observable behavior).
  The PR's own added test (`if_logical_op.gd`) verifies 12 distinct `and`/`or` nesting combinations
  (2 one-operator + 10 three-operator cases across `2**2` and `2**4` truth-table inputs) against a
  reference value computed independently in the same script, and this session ran it directly via
  a scratch `--script` runner confirming all 12 print `SUCCESS` (see `bench/` below) - not just via
  the full suite's own pass/fail count.
- Bytecode shape changes (fewer stack slots allocated per `if`/`and`/`or`), which is an internal
  compiler-output difference, not an observable one - no existing `.gdc`/bytecode-format
  compatibility promise is broken (compiled bytecode is not a stable cross-version format in Godot).
- **Is this a breaking change?** No - no public API, file format, or documented behavior changes.
  `while`/`match`/ternary conditions are untouched (PR explicitly scopes to `if` only), so no
  other statement type's codegen changes.
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped** both before and after (the
  same pre-existing `test_ip.cpp` IPv6 failure this container always shows, unrelated to this
  patch) - identical failing-test set (the empty set, modulo the one pre-existing failure).
  Assertion count went from 424066 (baseline, before the PR's new test file existed) to 424067
  (patched, with the new `if_logical_op.gd`/`.out` pair added) - the +1 assertion is the new
  script-output comparison, and it passed (no new `FAILURE`/diff printed).

## Why the result looks the way it does

The mechanism is exactly what the PR describes: `_parse_expression()` (the pre-existing path)
compiles `a and b` into "evaluate `a`, jump-if-not to a `false`-assign, evaluate `b`, jump-if-not
to the same `false`-assign, assign `true`, jump past, assign `false`, [continue]" - i.e. it always
materializes a boolean in a temp stack slot regardless of context, then the `if` statement's own
`write_if()` does one more conditional jump on that temp. When the temp is only ever used to decide
which of two blocks to run (the overwhelmingly common case for an `if` condition), that
materialize-then-branch is pure overhead: two extra bytecode ops (one assign, one jump) per
operator that a direct jump straight to the destination block doesn't need. The new
`_parse_expression_cond()` path recognizes exactly this context (condition position of an `if`) and
emits the direct jumps instead, falling back to the unchanged `_parse_expression()` for every case
it doesn't specifically handle (anything that isn't a top-level `and`/`or` `BinaryOpNode`). Because
each `and`/`or` operator removes one fixed unit of interpreter work (an `OPCODE_ASSIGN` execution
plus an extra `OPCODE_JUMP` dispatch) independent of operand cost, the saving is a per-operator
fixed cost, not dependent on the specific boolean values used - so the effect scales linearly with
operator count, matching the 1-operator vs. 3-operator gap measured above.

## What's not done / open questions

- `while` and `match` conditions still use the old materialize-then-branch path (the PR's own
  documented future work); this session did not attempt extending the same technique there.
- Ternary (`x if cond else y`), `not`, and binary comparison operators (`==`, `<`, etc.) as
  short-circuit-jump targets are also explicitly out of scope for this PR and untried here.
- Did not benchmark a realistic full game-logic script (only the isolated micro-benchmark
  matching the PR's own methodology) - real-world gain depends on what fraction of a script's
  total time is condition evaluation vs. everything else, as the PR's own description notes
  ("Real-world performance gains are likely lower").
- This session used a `debug_symbols=yes` build for both arms (for profiling reuse across the
  whole session) rather than this workspace's usual plain `tests=yes` build; `-g2` does not change
  `-O2` codegen, so this should not affect the relative comparison, but it means the absolute
  us/iter figures here are not directly comparable to other candidates' absolute numbers measured
  without debug symbols.
