# fptoint_sat_nan_wrong_isize_result

Found by the `./y.sh fuzz` rustlantis campaign (2026-09-02 session, seed 4448), auto-reduced by the
project's own reducer (`target/fuzz/runtime_error/fuzz4448.rs_reduced`, 1992 lines), then minimized
further by hand (a UB-safety-checked ddmin: every candidate cut was verified to still reproduce the
LLVM-vs-GCC mismatch AND to still pass `cargo miri run` under `-Zmiri-tree-borrows` with no
Undefined Behavior - see "What was tried" below for why plain Stacked-Borrows Miri is *not* the
right oracle here) down to 181 lines - `repro/src/main.rs`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| rustlantis campaign (`./y.sh fuzz -j 4`, seeds 100+) | debug LLVM vs release GCC comparison, this project's own built-in fuzzer | Found this mismatch (seed 4448) within the first ~4500 of a 2h/4-thread run; project's own 8-pass auto-reducer (including this session's earlier `remove_dump_var_fast` speedup) took it from ~16k-equivalent generated size to 1992 lines automatically |
| Custom line-level ddmin (no semantic safety check) | Python script, chunk-halving bisection on raw lines, oracle = "LLVM/GCC stdout+exit-code differ" only | Reduced to 84 lines - but this candidate turned out to be **Undefined Behavior** (`cargo miri run` flagged "using uninitialized data" at the exact line the mismatch came from) - a false lead from over-aggressive reduction stripping necessary initialization; discarded, not used |
| Same ddmin, oracle strengthened to also require `cargo miri run` (default Stacked Borrows) to report no UB | - | The *original* 1992-line file itself failed this stricter check - Miri (default Stacked Borrows) reported a Tree-Borrows-specific aliasing violation ("that tag only grants SharedReadOnly permission") on the untouched original file |
| Re-checked with `MIRIFLAGS=-Zmiri-tree-borrows` (matching rustlantis's own `config.toml`, which validates all its generated programs under Tree Borrows, not Stacked Borrows - see `clones/rustlantis/config.toml.example`'s `[backends.miri]`/`[backends.miri-opt]` sections) | - | Original 1992-line file is clean under Tree Borrows and produces `_4 = 0`, matching LLVM exactly - confirms rustlantis's own "UB-free by construction" guarantee holds under the aliasing model it actually targets; Stacked Borrows is simply the wrong oracle for rustlantis output |
| ddmin re-run with the correct oracle (mismatch reproduces AND `MIRIFLAGS=-Zmiri-tree-borrows cargo miri run` reports no UB) | ~10 minutes total (compile+run+miri per candidate) | Converged to 181 lines (`repro/src/main.rs`), confirmed clean under Tree Borrows at every accepted step |
| Hand-construction of a simpler standalone repro (`f32::NAN as isize`, `x == x` on NaN, `inf + (-inf)` computed NaN, a manual squaring-then-cast chain) | Several small variants, both backends | All matched (`0` on both) - the simple/direct forms of this pattern do NOT reproduce the bug; something about the specific value-flow in the 181-line repro (a NaN value reaching `as isize` after being produced via a raw-pointer read/write/multiply chain: `_18 = _13 - _15; (*_38) = _18 - _13; _32 = (*_38) * (*_38); _4 = _32 as isize`) is necessary and not yet isolated further |
| Source reading: `Builder::fptoint_sat` (`src/builder.rs:1925-2011`) | - | Located the exact mechanism this bug lives in (see Root cause) - the Rust-level algorithm matches LLVM's own documented `fptosi.sat`/`fptoui.sat` fallback exactly, including its NaN-to-zero final step; the divergence is not in this Rust code's own logic (verified: plain `x == x` and `NAN as isize` through this same function give correct results in isolation) but in how it gets lowered to GCC IR for this specific value-flow shape |

## Status

**Confirmed.** Reproduces 3/3 runs (release-mode GCC-backend build, no fixed seed needed -
deterministic). `output.txt` has the real captured before/after run.

**Fully reproducible**: Yes - `repro/` compiles and runs the real, unmodified backend on ordinary
(if unusual-looking, since it's `custom_mir!`-authored) safe Rust, no second participant involved.

## Class

logic

## Root cause

`f32 as isize` (a signed saturating float-to-int cast, `BuilderMethods::fptosi_sat` ->
`Builder::fptoint_sat`, `src/builder.rs:1925`) is supposed to map a NaN input to `0` per Rust's
defined `as`-cast semantics (LLVM's own `fptosi.sat`/`fptoui.sat` behavior, which this function's
algorithm is a direct, documented port of - see the large comment block at `src/builder.rs:1943+`).
The algorithm's step 3 (`src/builder.rs:2078-2085`) does this via `let cmp = self.fcmp(RealOEQ, val,
val); self.select(cmp, s1, zero)` - for a real IEEE-754 NaN, `val == val` must be `false`, forcing
the final result to `zero`.

In this repro, the `isize`-typed cast target `_4 = _32 as isize` receives a NaN value that GCC
computes and stores correctly (confirmed via `output.txt`/the `dump_var` trace: `_32` and `_15` show
as `NaN` identically on both backends, and `_18` shows as `-inf` identically on both too) - the
inputs feeding the cast are byte-for-byte the same NaN on both backends. Only the *cast's own*
result differs: LLVM's backend correctly returns `0`; this backend returns `int_ty::MIN`
(`-9223372036854775808` for `isize`), which is exactly the value `fptoint_sat`'s step 2 alone (`s1`,
before the step-3 NaN override) would produce for a NaN input under the *unordered* `less_or_nan`
comparison - i.e. it looks exactly as if step 3's `val == val` NaN-detection check is not correctly
suppressing `s1` in this specific case, even though the same check (verified independently this
session with a standalone `x == x` test on a `black_box`-obtained NaN, and a standalone `f32::NAN as
isize` cast, both through this exact same `fptoint_sat` function) works correctly in isolation.

**Not fully pinned down**: exactly *why* this specific value-flow (a NaN produced via a chain of
float subtraction, a raw-pointer write-then-read-back (`(*_38) = ...; ... (*_38) * (*_38)`), and
squaring) causes the `fcmp(RealOEQ, val, val)` comparison (or the `select` built on top of it) to be
mis-lowered or mis-optimized specifically here, when the same Rust-level construct works correctly
for simpler value-flows. Plausible mechanisms considered but not confirmed: (a) GCC's own optimizer
applying an unsound value-range assumption specific to a squared value (`x*x`) being treated as
"provably non-NaN" when reached through this exact expression shape, confusing the later identical-
operand equality check; (b) some interaction between the raw-pointer aliasing pattern and how GCC
tracks whether the two `val` operands passed to `new_comparison` are recognized as "the same value"
across the intervening pointer read. Distinguishing between these (or finding another explanation)
needs deeper GCC-internals investigation (e.g. `-fdump-tree-all`/`-fdump-rtl-all` on the actual
generated GCC IR) that this session's time budget didn't allow.

## Introduced by

Not determined this session - `Builder::fptoint_sat` is long-standing code (a direct LLVM-algorithm
port, per its own comments) and `git log -S`/`git blame` on `src/builder.rs`'s `fptoint_sat`
function would need to be cross-referenced against exactly which lowering step is actually
miscompiling, which isn't pinned down yet (see Root cause) - blaming the function's own authorship
would misattribute this to whoever wrote the *correct*, LLVM-faithful algorithm, not necessarily
whoever's change (if any, as opposed to a pre-existing libgccjit/GCC limitation) actually caused the
divergence.

## The fix

No fix attempted this session - the exact mechanism causing the miscompilation isn't confirmed (see
Root cause's "not fully pinned down" section), and a patch to `fptoint_sat`'s Rust-level algorithm
without understanding *why* the existing, LLVM-faithful logic fails here risks papering over the
specific triggering input rather than fixing the actual defect (which may not even be in this
function's own code - it could be a real GCC/libgccjit optimizer bug reached via this call site,
similar in spirit to candidate 001's LTO crash being attributed to the bundled GCC fork rather than
this repo's own source). A future session with more time for GCC-internals debugging
(`-fdump-tree-all` on a minimal C reproduction of the same `fcmp`+`select` pattern, or a `git
bisect`-style narrowing against different libgccjit builds) is the recommended next step.

- Project test suite: N/A - no fix was applied this session, so there is no before/after to state.

## Corroborating evidence (seed 7323, added 2026-09-02, same session)

A **second, independently-generated** rustlantis mismatch from the same 2h/4-thread fuzzing
campaign (seed 7323, `target/fuzz/runtime_error/fuzz7323.rs_reduced`, 6564 lines) was investigated
in parallel and traced to the **exact same mechanism**: `Builder::fptoint_sat`'s NaN case returning
`isize::MIN` instead of `0`. Confirmed by instrumenting the file's own `dump_var` call to print the
intermediate values directly: `_22 = _12 - _12` (an `f32` self-subtraction, `_12` being infinite)
produces `NaN` identically on both backends; `_14 = _22 as isize` (the cast) then diverges exactly
as in this candidate's own repro (LLVM: `0`-equivalent contribution; GCC: `isize::MIN`-equivalent
contribution). The divergence surfaces in a numerically different-looking form here because the
buggy cast result is copied (`_42 = _14;`) and later combined with an unrelated correct value via
bitwise OR (`_14 = _62 | _42;`, `_62` holding `21025`) before being dumped - `21025 | isize::MIN` =
`isize::MIN | 0x5221` = `-9223372036854754783`, exactly the value observed. This is strong
independent confirmation that candidate 034's finding is a real, repeatable defect in this exact
code path, not an artifact specific to seed 4448's own particular value-flow.

**New negative result narrowing the trigger condition further**: a plain, minimal standalone
snippet mirroring exactly this seed's shape (`f32::INFINITY` in `black_box`, self-subtract to get
NaN, cast to `isize`, OR with a `black_box`ed correct value) does **NOT** reproduce - both backends
correctly give `0` for the cast and `21025` for the final OR. This means neither candidate's own
"needs a raw-pointer read/write/multiply chain" (034's own repro) nor "needs a self-subtraction
immediately followed by a same-basic-block OR" (this candidate's simplification attempt) is
sufficient in isolation to describe the trigger - something about the *surrounding* code volume/
complexity in the full custom_mir programs (many other basic blocks, functions, and live values
around the same cast) is necessary for GCC's optimizer to actually manifest the bug, consistent
with this RESULT.md's own "plausible mechanism (a): GCC's own optimizer applying an unsound
value-range assumption ... specific to this exact expression shape" hypothesis above - the shape
alone isn't enough without enough surrounding pressure on GCC's optimizer to trigger it.

Per this session's discipline (`SKILL.md`'s "the same underlying bug can live in more than one
place" - here, quite literally the *same* call site, not a different one), this was **not** written
up as a separate candidate/`SUMMARY.md` row - candidate number 036 (pre-assigned for this
investigation) was returned unused. The seed 7323 repro (UB-cleaned - 7 dead/decoy raw-pointer
writes removed that were genuine Tree-Borrows violations, not further minimized due to time) is
kept as supplementary material in `./corroborating_seed7323/` for reference, not as its own
`repro/`.

## What's not done / open questions

- The exact triggering condition (why this value-flow shape and not simpler ones) is not isolated -
  see Root cause. A future session should try intermediate value-flow shapes between the working
  simple cases and this 181-line repro (e.g., does removing just the raw-pointer read/write step,
  keeping the rest, still reproduce? Not yet tried - `fn2`'s call and several dead basic blocks in
  the current 181-line repro also weren't confirmed removable-or-not since ddmin's automated pass
  already converged and further manual restructuring wasn't attempted given the session's time
  budget).
- Not checked whether other saturating float-to-int cast destination widths (`i8`/`i16`/`i32`/`i64`/
  `i128`, or the unsigned `fptoui_sat` variant, which per the code comment doesn't need step 3 at
  all since unsigned's step 2 alone already yields `0` for NaN) share the same bug - only `isize`
  (64-bit signed) was exercised by this specific fuzzer find.
- The `___BUGHUNT___/tools/diffgen/` `floatint` category (this session's own differential harness)
  already covers plain `f32::NAN as isize`-style casts across many boundary values and found nothing
  - consistent with this bug needing the specific multi-step value-flow shape found here, not a
  simple boundary value. Worth adding a `diffgen` case mirroring this repro's exact shape (squared
  value via raw-pointer read/write, then cast) if `diffgen` is extended again in a future session.
