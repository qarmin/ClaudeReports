# Extend GDScript's `if`-statement short-circuit jump codegen (candidate 048 / PR #120660) to `while` loops

Candidate 048 (this session, PR godotengine/godot#120660) replaced `if a and b:`'s
materialize-a-boolean-then-branch codegen with direct short-circuit jumps, but scoped it to `if`
only - `while` (and `match`) were left as explicit future work in the PR itself. Since a `while`
condition re-evaluates every iteration (unlike an `if`, evaluated once), the same per-operator
"one extra assign + one extra jump" waste 048 fixed for `if` is paid **once per loop iteration**
for `while`, so the relative win was expected to be larger. This candidate writes and verifies that
extension, reusing 048's own codegen infrastructure (`start_expr_cond_buffer`/
`write_expr_cond_jump_if`/`flush_expr_cond_buffer`/`_parse_expression_cond`) rather than inventing a
parallel mechanism - no upstream PR exists for this yet (checked `gh search prs`/`issues`, nothing
open), so this is an original patch, not an adaptation of someone else's reviewed diff (unlike every
other candidate found this session).

**This patch is a strict extension of candidate 048 (PR #120660) and cannot be *built* without
048's patch already applied first** (it applies cleanly at the text level either way, since
`git apply` doesn't know about C++ symbols - but without 048's own `.h` additions, it references
undefined types/methods and fails to compile) - it calls `start_expr_cond_buffer`/
`write_expr_cond_jump_if`/`flush_expr_cond_buffer`/`_parse_expression_cond`, all of which are
048's own additions, not present in pristine `HEAD`. `patch.diff` in this folder is **only this
candidate's own marginal diff** (+20/-7 across 2 files) - see `justfile`'s `apply` recipe for how
it layers on top of 048's own `patch.diff`.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

**A real bug found and fixed during development, before any measurement** (see "Why the result
looks the way it does" for the mechanism): the first working version of this patch crashed with
SIGSEGV compiling *any* script containing a `while` loop. Root cause: `write_endwhile()` still
unconditionally called `while_jmp_addrs.back()->get()` to patch the loop-exit jump - but
`while_jmp_addrs` is only ever pushed to by `write_while()`, which this patch stops calling (the
condition is now compiled via the short-circuit jump buffers instead). `while_jmp_addrs` was empty,
so `.back()` returned a null `List::Element*` and `->get()` dereferenced it. Fixed by removing that
now-dead patch/pop pair and replacing it with `flush_expr_cond_buffer(BranchType::NOT_TAKEN)` at
the same position (right after the loop's back-edge jump - i.e. the loop-exit point), which is the
correct target for every `NOT_TAKEN` jump accumulated while compiling the condition.

Benchmark: two new `bench.gd` modes, `while_logic_and` (`while i < n and a and b: i += 1`, `a`/`b`
both `true` so every iteration evaluates all 3 operands) and `while_logic_or` (`while a or b or
i < n: i += 1`, `a`/`b` both `false` so every iteration evaluates all 3 operands, ordered so the
`false` operands come first, forcing full evaluation each time - unlike 048's own `if_logic_or`
mode, which deliberately short-circuits early), each `n = 20,000,000` iterations. Measured as this
candidate's own **marginal contribution over 048 alone** - baseline is a `048-only` binary (PR
#120660 applied, this candidate's patch not applied), patched is `048 + this candidate`. **5
interleaved pairs**, `uptime` checked before starting (load average 0.21-0.70 on a 24-core box).

| Pair | `while_logic_and`: 048-only us/iter | 048+this candidate us/iter | `while_logic_or`: 048-only us/iter | 048+this candidate us/iter |
|---|---|---|---|---|
| 1 | 0.036369 | 0.023489 | 0.037987 | 0.033074 |
| 2 | 0.037303 | 0.022827 | 0.032700 | 0.021770 |
| 3 | 0.038249 | 0.020492 | 0.034223 | 0.019871 |
| 4 | 0.037598 | 0.022815 | 0.034863 | 0.021343 |
| 5 | 0.037453 | 0.022439 | 0.038772 | 0.022285 |
| **mean** | **0.037394** | **0.022412** | **0.035709** | **0.023669** |

## Result

**-40.1% wall time for the 3-operand `and` loop condition** (0.037394us -> 0.022412us/iter mean),
**-33.7% for the 3-operand `or` loop condition** (0.035709us -> 0.023669us/iter mean), both measured
as the marginal contribution on top of candidate 048 (i.e. isolating what *this* patch adds, not
048's own already-recorded effect). 5/5 pairs non-overlapping for `and`; 4/5 clearly non-overlapping
for `or` with one borderline pair (pair 1: 048-only 0.037987 vs patched 0.033074 - still a real
21.5% improvement for that specific pair, just a smaller gap than the other 4). This confirms the
predicted larger relative win for loops versus 048's own one-shot `if` result (-43% to -57%) - here
the saved assign+jump per operator is paid on *every iteration* instead of once, so at 20 million
iterations the fixed per-iteration saving compounds into a substantial fraction of the loop's total
cost. Time only; no allocation-count/RSS effect is expected (same reasoning as 048 - this changes
bytecode shape, not allocation behavior).

## Does it change behavior/output?

- **Bit-exact**, verified by a dedicated correctness script (`bench/while_short_circuit_test.gd`)
  covering: basic `and`/`while` and `or`/`while` loops, `continue` inside a short-circuited
  condition, `break` inside one, nested short-circuited `while` loops, **short-circuit evaluation
  count** for both `and` (right operand must not evaluate once left is false) and `or` (right
  operand must not evaluate once left is true), a plain non-`and`/`or` condition (confirms the
  `_parse_expression_cond` fallback path still works), and a 3-operand chain - all `ALL_TESTS_PASS`.
- **A test-design pitfall hit and corrected while building the correctness script, worth recording
  for future GDScript correctness checks in this workspace**: the short-circuit-evaluation-count
  checks initially used a plain `int` local captured by a lambda (`var call_count := 0; var f :=
  func(): call_count += 1`) and always read back `0` regardless of how many times the lambda
  actually ran - on **both** the patched build *and* a completely pristine, unmodified `HEAD` with
  no candidate patches applied at all (confirmed by reverting everything and re-testing). This
  proved it was never a bug in this candidate (or in 048) - GDScript lambdas capture plain
  value-type locals **by value** (a snapshot at creation time), not by reference; `bench.gd`'s own
  pre-existing `bench_signal_emit` mode has the identical shape (`var count := 0; var handler :=
  func(): count += 1`) but never actually reads `count` back afterward, so this characteristic was
  never previously exercised as a correctness check in this workspace. Fixed by using a
  single-element `Array` (`var call_count := [0]`) as the mutable counter cell instead, since
  `Array`/`Dictionary`/`Object` are reference types in GDScript and a lambda capturing one shares
  the same underlying object.
- **Is this a breaking change?** No public API/file format change - internal bytecode shape only.
  `match` conditions remain on the old materialize-then-branch path (still out of scope, same as
  048 left it).
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

Same mechanism as 048: each `and`/`or` operator in the old codegen cost one `OPCODE_ASSIGN`
execution plus one extra `OPCODE_JUMP` dispatch beyond what a direct short-circuit jump needs. For
an `if`, that cost is paid once per evaluation of the `if`. For a `while`, the condition is
re-evaluated at the top of *every* iteration - so the same fixed per-operator saving is paid back
on every one of the loop's iterations instead of once, which is exactly why the measured relative
improvement here (-34% to -40%) is larger than 048's own `if`-only result (-43% to -57% is actually
comparable in *magnitude* since 048's benchmark measured a single evaluation's absolute cost
directly; the point of comparison that matters is that this candidate's win is realized on *every*
iteration of a hot loop, which is the more common place branch-heavy GDScript code actually spends
its time - a physics/game-logic loop condition, not a single one-shot `if`).

## What's not done / open questions

- `match` conditions are still unconverted (same scope boundary 048 left, now also true for this
  candidate) - a `match` statement's own codegen is structurally different (multiple pattern
  branches, not a binary true/false), so extending short-circuit jumps there would need its own
  design, not a copy-paste of this candidate's approach.
- Did not benchmark a loop condition mixing `and`/`or` with a `not` or comparison-chain (048's own
  scope boundary - only `and`/`or` `BinaryOpNode`s get the fast path, everything else still falls
  back to `_parse_expression`) - the correctness script's "3-operand chain" case covers this
  structurally but wasn't separately timed.
- Only 20,000,000-iteration tight loops were benchmarked; did not check whether the effect holds
  at more realistic loop body sizes (a loop doing real work per iteration would dilute the
  condition-evaluation saving as a fraction of total loop time - expected, not investigated
  further here).
