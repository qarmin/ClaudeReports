# `Projection::determinant()`: cofactor-expansion rewrite instead of the fully-expanded 4x4 formula

`core/math/projection.cpp`'s `determinant()` computed the 4x4 determinant as one fully-expanded
sum of 24 four-term products (the textbook Leibniz-formula expansion) - a lot of redundant
recomputation, since many 2x2/3x3 sub-determinants (e.g. `columns[2][2]*columns[3][3] -
columns[2][3]*columns[3][2]`) appear multiple times across those 24 terms but are recomputed from
scratch each time. This looked worth trying because it's independently verifying an already-open,
unmerged PR, [godotengine/godot#116493](https://github.com/godotengine/godot/pull/116493)
("Optimize `Projection::determinant()`"), using the exact GDScript benchmark methodology the PR
author posted in their own PR description (a tight loop calling `.determinant()` N times, timed
with `Time.get_ticks_usec()`) - re-run independently on this machine/toolchain rather than trusting
the author's own Windows/MinGW numbers.

Toolchain: `scons` 4.11.1, GCC 15.2.0 (Ubuntu 15.2.0-16ubuntu1), `platform=linuxbsd target=editor
tests=yes debug_symbols=yes`. Only the C++ core change was applied/measured here - the PR's C#
(`GodotSharp`) glue-code equivalent was not touched (this build has no mono/C# module enabled).

## What was tried

`___OPTIMIZATIONS___/harness/gdbench/projection_determinant_probe.gd` (new harness mode) builds
the same `Projection` matrix as the PR author's own benchmark script (`Vector4(1,5,9,13)`,
`Vector4(2,6,11,15)`, `Vector4(4,7,11,15)`, `Vector4(4,8,12,16)`) and calls `.determinant()` in a
tight loop (N=2,000,000 here, vs. the PR's N=10,000,000 x 30 repeats - fewer iterations, chosen to
keep this session's total time bounded, not for a measurement reason), printing total time and the
computed determinant value (for a correctness check) each run. Run via `bin/godot.linuxbsd.editor.x86_64
--headless --path harness/gdbench --script projection_determinant_probe.gd`. `uptime` load average
0.71-2.82 across all runs (higher toward the end from a concurrent `scons` background build in this
session, still well under the 24-core budget - noted per MEASUREMENT.md's "check `uptime` before
each measurement" discipline, not hidden).

| Variant | 5 runs, total us (N=2,000,000 calls) | us/call |
|---|---|---|
| baseline (unpatched, fully-expanded formula) | 39787, 39568, 39284, 40053, 40043 - mean 39747.0 | 0.019874 |
| patched (cofactor expansion) | 34795, 35495, 29842, 29784, 28942 - mean 31771.6 | 0.015886 |

## Result

**Time.** Mean 39747.0us -> 31771.6us for 2,000,000 calls, a **-20.1% reduction** (1.25x speedup).
This is in the same direction as, and somewhat larger than, the PR author's own claimed -13.6% (a
different OS/compiler/hardware combination - GCC 15.2 on this machine vs. their MinGW build - so an
exact match isn't expected, but the direction and rough magnitude both confirm the claim
independently). Peak RSS/allocation count not measured - this is a pure arithmetic rewrite with no
allocation involved either way.

## Does it change behavior/output?

- **Bit-exact?** Effectively yes for the tested input: both formulas printed the identical
  `determinant=-12.000000` for the benchmark's fixed matrix. Mathematically the two formulas compute
  the same determinant via different (but algebraically equivalent) expansions, so floating-point
  results could in principle differ by rounding in the last bit or two for some inputs (different
  operation order/associativity) - not exhaustively checked across many matrices, only the one used
  by both this benchmark and the PR author's own; the existing `tests/core/math/test_projection.cpp`
  suite (31 cases, 256 assertions) also passed unchanged (see below), which exercises `determinant()`
  indirectly via inversion (`Projection::inverse()` uses it) on several other real projection
  matrices without any tolerance-check failures.
- **Is this a breaking change?** No - internal computation only, same public signature/semantics.
- Project test suite (`--headless --test`, `XDG_DATA_HOME` set): **1427 test cases, 1426 passed, 1
  failed, 3 skipped** - the same one pre-existing `test_ip.cpp` IPv6 failure, identical before and
  after. The `*Projection*`-filtered subset specifically: **31 test cases, 31 passed, 0 failed, 256
  assertions, 256 passed** both before and after.

## Why the result looks the way it does

The original formula is the direct Leibniz-formula expansion of a 4x4 determinant: 24 terms, each a
product of 4 matrix entries, added/subtracted with the appropriate sign - a correct but naive
approach that does not reuse any of the repeated 2x2 sub-products between terms (e.g. `columns[2][2]
* columns[3][3] - columns[2][3] * columns[3][2]` is one of several 2x2 minors that would otherwise
need computing independently multiple times across different terms of the 24-term sum). The
cofactor-expansion rewrite instead computes each distinct 2x2 minor of the bottom-right 3x3
sub-block once (`m10*m15 - m11*m14`, `m6*m15 - m7*m14`, `m6*m11 - m7*m10`, and their column-shifted
counterparts) and reuses each one across the four top-level cofactor terms that need it, trading a
large multiply-heavy expanded sum for a smaller number of multiplications with each intermediate
computed only once. The measured ~20% reduction is consistent with removing that redundant
recomputation, not with any change in what's being computed.

## Reproduced end-to-end via `just test` (2026-09-10)

This candidate's `justfile` was run all the way through on this machine as the workspace's
end-to-end reproducibility check: fresh `git clone` of `godotengine/godot` at `base_commit`,
from-scratch baseline build, baseline bench, `git apply patch.diff`, patched build, patched bench -
**exit 0, both arms produced numbers** (full log in `just_test_run.log`, ~8 minutes total here).

The reproduction confirms the win and is *larger* than the figure this RESULT.md claims, in both
attempts:

| Measurement | baseline us/call | patched us/call | delta |
|---|---|---|---|
| original (this RESULT.md, 5 runs/arm) | 0.019874 | 0.015886 | **-20.1%** |
| `just test` run (3 runs/arm, other work on the machine) | 0.020433 | 0.014554 | -28.8% |
| bench recipes re-run on an idle machine (3 runs/arm, `bench_rerun_idle.log`) | 0.023160 | 0.014642 | -36.8% |

**The patched arm is stable across all three (0.0146-0.0159); it is the baseline arm that moves
(0.0199-0.0232).** So the -20.1% recorded above is the conservative end of the range and is left as
this candidate's claim rather than being restated upward - but the spread is a reminder that a
figure re-measured later on this container can differ by ~17 percentage points, and that both arms
must always come from the same session (see also candidate 032's XDG-state version of this).

## What's not done / open questions

- Did not test the PR's C# (`GodotSharp`) equivalent change (no mono module in this build).
- Did not exhaustively fuzz-check floating-point bit-exactness across many random matrices - only
  confirmed identical output on the one matrix used by both this benchmark and the PR author's own,
  plus a clean pass of the existing `test_projection.cpp` suite (which exercises `determinant()`
  indirectly through several fixed test matrices via `inverse()`, not directly with its own
  dedicated `determinant()`-value assertions against arbitrary matrices).
- Did not check whether `Basis::determinant()` or `Transform2D::determinant()` (both also bound in
  `core/variant/variant_call.cpp`, both 3x3/2x2 respectively) have any comparable naive-expansion
  inefficiency - not looked at this session, a plausible parallel lead for a future one.
