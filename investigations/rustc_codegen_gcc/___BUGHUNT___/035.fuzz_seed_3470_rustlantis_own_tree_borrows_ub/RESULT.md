# fuzz_seed_3470_rustlantis_own_tree_borrows_ub

Investigated `target/fuzz/runtime_error/fuzz3470.rs_reduced` (8908 lines), the second finding from
this session's `./y.sh fuzz -j 4` 2h rustlantis campaign (seed 3470) - the campaign's own log showed
`_56 = (1,)` under LLVM vs `_56 = (-9223372036854775807,)` under this backend.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Confirm the reported LLVM-vs-GCC divergence still reproduces | Direct compile+run of `fuzz3470.rs_reduced`, both backends, release mode | Reproduced: LLVM prints `fn17:_56 = (1,)`, this backend prints `fn17:_56 = (-9223372036854775807,)` (`i64::MIN + 1`) |
| `MIRIFLAGS=-Zmiri-tree-borrows cargo miri run` on `fuzz3470.rs_reduced` (the file the campaign actually reduced and reported) | Following candidate 034's established methodology (plain/default Miri gives false positives on rustlantis output - always use `-Zmiri-tree-borrows`, see that candidate's `RESULT.md`) | **Real Undefined Behavior detected even under Tree Borrows**: `error: Undefined Behavior: reborrow through <TAG> at allocN[0x28] is forbidden` |
| Regenerate the RAW, untouched seed-3470 program directly (`cd clones/rustlantis && cargo run --release --bin generate -- 3470`, no `--debug`, no reduction, no `dump_var` instrumentation - the literal, unmodified rustlantis output for this seed) and re-run the same Tree Borrows check | 12026 lines, saved as `raw_generated_seed3470.rs` in this folder | **Same class of UB, same offset**: `error: Undefined Behavior: reborrow through <TAG> at alloc5448[0x28] is forbidden` - full output in `miri_tree_borrows_output.txt`. This rules out the campaign's own reduction pipeline, or its `--debug`/`dump_var`-instrumented regeneration step, as the source of the UB - it is present in the literal, freshly-generated, untouched rustlantis output for this seed |
| Read the Miri diagnostic's own pointed-to lines in `raw_generated_seed3470.rs` | `_14 = &_26.1.1;` (line 3195) takes a shared reference to a struct field; `_26.1.1 = !_28;` (line 3441) later writes directly to that same field **not through `_14`** | This is exactly a Tree-Borrows "foreign write invalidates an outstanding shared reborrow" violation: Miri's own diagnostic confirms it explicitly - "the accessed tag <TAG> later transitioned to Disabled due to a foreign write access at offsets [0x28..0x29]" pointing at line 3441, "this transition corresponds to a loss of read permissions" - `_14` (or a further reborrow derived from it) gets used again after this write, which is the actual UB |

## Status

**Inconclusive.** The LLVM-vs-GCC output divergence reproduces reliably (checked twice, both on the
campaign's own reduced file and confirmed the underlying mechanism is present in the raw
regeneration), but the input program itself contains genuine Undefined Behavior under Tree Borrows -
the exact aliasing model rustlantis's own `config.toml` uses to validate its own "terminating,
UB-free by construction" guarantee (see candidate 034's `RESULT.md` for why Tree Borrows, not
Miri's Stacked-Borrows default, is the correct oracle for this generator's output). A backend
divergence on a program with real UB is not evidence of a `rustc_codegen_gcc` bug - both backends
are free to do anything once UB occurs, so the specific wrong-looking value (`i64::MIN + 1`) this
backend happens to produce carries no diagnostic weight.

**Fully reproducible**: N/A - not applicable, this is not being reported as a confirmed backend bug.

## Class

n/a (see Status - this is not a `rustc_codegen_gcc` finding)

## Root cause

Not a `rustc_codegen_gcc` root cause. The actual defect, as far as this session's investigation
went, is in **rustlantis's own MIR generator** (https://github.com/cbeuw/rustlantis) for this
specific seed: it generates a shared reference to a struct field (`_14 = &_26.1.1`) and later writes
directly to that same field through the original place expression (`_26.1.1 = !_28`) while the
reference (or a value derived from it) is still live and gets used afterward - a textbook
"foreign write invalidates an outstanding shared borrow" Tree Borrows violation. Since rustlantis's
own explicit guarantee (and its `config.toml`'s choice of Tree Borrows as its own validation
backend) is that its generated programs are UB-free under this exact model, this looks like a real,
reportable bug in rustlantis's own generation logic, not a downstream consumer's problem - though
this session did not attempt to minimize it down to a small enough standalone case to file upstream
(see "What's not done" below).

## Introduced by

N/A - not applicable to this repo (`rustc_codegen_gcc`). If reported upstream, it would need
`rustlantis`'s own `git blame`/`git log -S` on whatever generation rule allows a live shared
reference to coexist with a direct write to its referent place.

## The fix

No fix attempted or applicable to this repository - see Root cause. If this is worth pursuing
further, the fix belongs in `rustlantis`'s own generator (its aliasing-safety pass should reject or
avoid generating a direct write to a place while a reference derived from that place could still be
used afterward), not in `rustc_codegen_gcc`.

- Project test suite: N/A - no fix was applicable/attempted.

## What's not done / open questions

- This candidate's finding was NOT minimized down to a small, rustlantis-independent standalone
  Rust snippet suitable for filing as an upstream `rustlantis` issue - `raw_generated_seed3470.rs`
  (12026 lines) and the campaign's own reduced `fuzz3470.rs_reduced` (8908 lines, kept in
  `target/fuzz/runtime_error/`, not copied into this folder to avoid duplicating a large file this
  candidate isn't claiming ownership of a fix for) are both available as starting points for a
  future session that wants to file this upstream.
- Not checked whether this exact seed (3470) or generation configuration is already a known
  rustlantis issue (no GitHub issue search was done against `cbeuw/rustlantis` this session).
- Because this candidate turned out to be UB rather than a real backend bug, no attempt was made to
  determine what value GCC "should" have produced, why it produced `i64::MIN + 1` specifically, or
  whether that specific wrong-looking value hints at anything about this backend's own behavior
  under UB (deliberately not chased, since reasoning about "correct" behavior for UB code is not
  meaningful).
- This session's sibling candidates (034, and forks investigating seeds 7323/8691 from the same
  campaign) should each independently re-check with the same Tree-Borrows-verified methodology
  before being written up as Confirmed - this campaign has now produced at least one seed (3470)
  where the generated program itself was not actually UB-free, so "found by rustlantis" alone is
  not suffilient evidence of UB-freedom without an explicit Tree Borrows check on the specific
  seed in question.
