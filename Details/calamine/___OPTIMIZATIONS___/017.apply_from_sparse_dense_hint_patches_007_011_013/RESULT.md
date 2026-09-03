# Consolidate candidates 007 + 011 + 013 into one combined `from_sparse` dense-hint patch

`rustc 1.98.0`, `cargo 1.98.0` (same toolchain as prior candidates this session).

Candidates 007, 011, and 013 in this workspace each independently fixed the same underlying
sparse+dense buffer-overlap pattern in `Range::from_sparse()`, at three disjoint call sites:
XLSX/XLSB's `worksheet_range_ref()` (007), XLS's own inline BIFF-parsing loop via a new
push-based `DenseRangeBuilder` (011), and XLSX/XLSB's `worksheet_formula()` reusing 007's
`from_sparse_dense_hint()` helper (013). Per this workflow's "always revert" rule, each of the
three was authored and measured independently against the same clean base commit
(`5c02e86`) and its own `patch.diff` was never combined with the others - this candidate combines
all three into one diff and re-measures the combination to confirm they compose without
interaction effects, since 013 explicitly built on top of the exact same `from_sparse_dense_hint()`
addition 007 introduces.

## What was tried

Combining the three `patch.diff`s hunk-by-hunk (all three touch `src/lib.rs` to add
`from_sparse_dense_hint()`/`MAX_DENSE_RANGE_CELLS`/`DenseRangeBuilder`; 007 and 013 each add a
byte-for-byte identical copy of the `from_sparse_dense_hint()` addition, since 013 was authored
against a clean base without 007 applied) found exactly one thing to resolve: the duplicate
`from_sparse_dense_hint()`/`MAX_DENSE_RANGE_CELLS` addition in `src/lib.rs`, appearing once in
007's diff and once (byte-identical, confirmed via `diff`) in 013's diff. Kept one copy. 011's
`DenseRangeBuilder` addition (a separate struct, also in `src/lib.rs`) and its own
`MAX_DENSE_RANGE_CELLS` constant definition (identical name/value to 007's, also removed the
duplicate) merged in without any other conflict - the three patches touch four disjoint files
(`src/xls.rs`; `src/xlsx/mod.rs` + `src/xlsb/mod.rs`'s `worksheet_range_ref` bodies; the same two
files' `worksheet_formula` bodies) that only share the one `lib.rs` primitive.

| Variant | `big_300k_rows.xlsx` `worksheet_range()` peak RSS (2.4M cells) | `adhocallbabynames1996to2016.xls` `worksheet_range()` peak RSS (1.47M cells) | synthetic `big_formula.xlsx` `worksheet_formula()` peak RSS (2.4M cells) |
|---|---|---|---|
| baseline (`HEAD` 5c02e86), pinned x3 | 172,460 / 172,420 / 172,296 KB | 151,312 / 151,212 / 151,316 KB | 209,800 / 209,972 / 209,748 KB |
| baseline, unpinned x2 | 171,772 / 172,360 KB | 151,096 / 150,976 KB | 209,420 / 208,316 KB |
| combined patch, pinned x3 | 134,900 / 134,632 / 134,616 KB | 142,528 / 142,708 / 142,732 KB | 134,864 / 135,060 / 134,944 KB |
| combined patch, unpinned x2 | 134,536 / 133,916 KB | 142,060 / 141,964 KB | 134,112 / 133,468 KB |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (same harness/fixtures candidates
007/011/013 each used individually), release build, clean rebuild each time, `sig`/`cells`
compared every run.

Baseline means (5 runs: 3 pinned + 2 unpinned):
- `big_300k_rows.xlsx` range: **172,262 KB** -> combined patch **134,520 KB** (**-21.9%**)
- `adhocallbabynames1996to2016.xls` range: **151,182 KB** -> combined patch **142,398 KB**
  (**-5.8%**)
- `big_formula.xlsx` formula: **209,451 KB** -> combined patch **134,490 KB** (**-35.8%**)

`sig`/`cells` were bit-identical between baseline and combined-patch on every run, all three
fixtures. Elapsed time was within run-to-run noise both ways on all three fixtures (no metric of
record change claimed for wall-clock, consistent with 007/011/013's own individual results).

## Result

The three individually-measured reductions reproduce, essentially unchanged, when applied
together: **-21.9%** (XLSX `worksheet_range`), **-5.8%** (XLS `worksheet_range`), **-35.8%**
(XLSX `worksheet_formula`) peak RSS vs. candidates 007/011/013's own individually-measured
-21.8%/-5.9%/-35.5% - differences are within this harness's own run-to-run noise (a few hundred
KB to ~1MB on multi-hundred-MB peaks), not a new effect from combining them. This confirms the
three patches compose without a negative interaction, which was the open question this candidate
existed to answer (013 in particular reuses the exact same `from_sparse_dense_hint()` primitive
007 adds, so a subtle conflict there was the most plausible risk). Metric measured: **peak RSS**
(`VmHWM`) on all three fixtures/call sites.

## Does it change behavior/output?

- **Bit-exact?** Yes - `sig`/`cells` identical to baseline on all three real/synthetic fixtures
  tested (same fixtures each of 007/011/013 individually verified against, including 007's own
  12 hand-crafted wrong/missing/reversed/offset `<dimension>`-header edge cases and 011's 6
  synthetic dimension-hint edge cases and 013's 5 wrong-`<dimension>` variants - not independently
  re-run for this consolidation candidate, since the combined diff's actual code for each of the
  three call sites is byte-for-byte identical to what each candidate individually already verified
  against those edge cases; only the *composition* of the three diffs was new here, not the
  per-site logic).
- **Is this a breaking change?** No public API change - `from_sparse_dense_hint()`,
  `MAX_DENSE_RANGE_CELLS`, and `DenseRangeBuilder` are all `pub(crate)`, invisible outside the
  crate, same as when each was individually reviewed in 007/011/013.
- Project test suite (`cargo test --release --all-features`): baseline (`HEAD` 5c02e86) and
  combined-patched, identical both times: **45/45 unit tests, 165/166 integration tests** (1
  pre-existing `#[ignore]`, unrelated to this change), **67/67 doc tests**. `cargo clippy
  --release --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

Each of the three original patches touches a disjoint call site (XLS's BIFF loop; XLSX/XLSB's
`worksheet_range_ref`; XLSX/XLSB's `worksheet_formula`) with no runtime interaction between them -
the only thing they share is the `lib.rs` primitive(s), and 007/013's copies of
`from_sparse_dense_hint()` were already byte-identical (both diffs generated against the same
clean base, implementing the same function). Composing them was therefore expected to be
close to arithmetically independent, which the measurements confirm.

## What's not done / open questions

- This candidate is a mechanical consolidation + composition re-check, not new discovery - see
  007/011/013's own `RESULT.md`s for what each fix does, its own edge-case correctness testing,
  and its own "what's not done" notes (e.g. 007's flagged-but-unaddressed second dense-buffer
  overlap in `worksheet_range()`'s `DataRef -> Data` conversion, still present after this
  combination).
- Per this workflow's "the one rule that matters most", the combined patch was built, measured,
  and then reverted from this worktree like every other candidate - it was not left applied to
  any tree. `patch.diff` in this folder is the mechanism for a reader to actually apply it, as a
  separate, conscious action.
