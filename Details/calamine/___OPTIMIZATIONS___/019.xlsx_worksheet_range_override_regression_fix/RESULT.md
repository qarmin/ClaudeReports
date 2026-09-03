# Fix candidate 018's XLSX regression: delete the stale `Xlsx::worksheet_range()` override

`rustc 1.98.0`, `cargo 1.98.0`.

Candidate 018 found that layering candidate 006's `Data::String` -> `Arc<str>` patch on top of
candidate 017's combined `from_sparse_dense_hint()` restructuring regresses XLSX peak RSS all the
way back to the pre-017 baseline, because candidate 006 shipped its own hand-written
`Xlsx::worksheet_range()` override (authored before candidate 017's `from_sparse_dense_hint()`
existed) that duplicates the old collect-a-sparse-`Vec<Cell>`-then-`Range::from_sparse()` pattern
instead of delegating to `worksheet_range_ref()`. That override makes candidate 017's fix
completely unreachable for XLSX's `worksheet_range()` (XLSB and XLS have no such override and are
unaffected). Candidate 018's own "What's not done" section flagged two options and named "(a)
delete the override, let XLSX fall back to the generic default (`worksheet_range_ref()` +
`.map(Into::into).collect()`)" as the simpler, higher-confidence fix, since candidate 012 already
proved that path doesn't pay for a second dense buffer (`Vec::from_iter`'s in-place-collect
specialization reuses the same allocation for same-size/same-align `DataRef`->`Data`). This
candidate applies exactly that option and re-measures.

## What was tried

Base: candidate 018's own combined patch (`017` `from_sparse_dense_hint` + `006` `Arc<str>`
interning, adapted - see that candidate's `RESULT.md` for how the merge was resolved), applied
verbatim. On top of that base, one change: replaced `Xlsx::worksheet_range()`'s entire hand-rolled
body (87 lines: its own `HeaderRow::FirstNonEmptyRow`/`HeaderRow::Row` cell-collection loops, then
a bare `Range::from_sparse(cells)`) with a 7-line delegation to `worksheet_range_ref()` +
`.into_iter().map(Into::into).collect()` - i.e. deleted the override entirely and let it fall back
to the exact same pattern XLSB already uses (no override there) and the exact pattern this
function itself used before candidate 006 existed. Net diff for this candidate's own increment on
top of 018: `src/xlsx/mod.rs` only, +13/-75 lines (see `incremental_xlsx_mod.diff` in this folder);
`patch.diff` in this folder is the full combined diff (018's patch + this increment) against
`base_commit`, since a candidate's `patch.diff` must apply standalone against pristine
`base_commit`.

Fixture: `___OPTIMIZATIONS___/testdata_external/big_300k_rows.xlsx` (20 MB, 300k rows x 8 cols,
2.4M cells, ~75% shared-string - same fixture candidates 007/017/018 used for XLSX). Measured with
`harness/rss_probe` (`range` mode), release build.

**Peak RSS, XLSX** (`big_300k_rows.xlsx`, 7 samples: 5 pinned `taskset -c 3` + 2 unpinned):

| Variant | peak_rss_kb range | mean | vs. original baseline | vs. 017-alone floor |
|---|---|---|---|---|
| baseline (no patches, from candidate 018's own table) | 172,080-172,768 | 172,373 | - | - |
| 017-alone (from candidate 018's own table) | 134,368-134,844 | 134,610 | -21.9% | - |
| 018 (017+006, override still present - the regression) | 172,104-172,516 | 172,314 | -0.03% | +28.0% |
| **019 (this candidate: 017+006, override deleted)** | **78,004-78,784** | **78,520** | **-54.4%** | **-41.7%** |

**Peak RSS, XLSB and XLS - unaffected by this candidate's own change (re-confirmed, not just
assumed)**, since the increment touches only `src/xlsx/mod.rs`'s `worksheet_range()` body and
XLSB/XLS have no equivalent override:

| Format / fixture | 018's own mean | this candidate's re-measurement (3 pinned samples) |
|---|---|---|
| XLSB (`018/testdata/big_300k_rows.xlsb`, synthetic, 2.4M cells) | 78,437 | 78,428-78,632 (unchanged) |
| XLS (`tests/adhocallbabynames1996to2016.xls`, real, 1.47M cells) | 95,284 | 95,100-95,332 (unchanged) |

**Paired alternating wall-clock + peak-RSS, XLSX, 018(A) vs. 019(B)**, same process, same moment,
`taskset -c 5`, 10 rounds (no warmup discarded - the very first pair already shows the full,
stable effect and every subsequent pair confirms it, nothing to discard):

| round | A (018) elapsed_ms / peak_rss_kb | B (019) elapsed_ms / peak_rss_kb |
|---|---|---|
| 1 | 726 / 172,456 | 570 / 78,772 |
| 2 | 605 / 172,292 | 583 / 78,656 |
| 3 | 607 / 172,540 | 572 / 78,596 |
| 4 | 634 / 172,516 | 591 / 78,608 |
| 5 | 606 / 172,504 | 572 / 78,536 |
| 6 | 610 / 172,588 | 572 / 78,640 |
| 7 | 605 / 172,472 | 571 / 78,604 |
| 8 | 616 / 172,340 | 582 / 78,568 |
| 9 | 622 / 172,540 | 575 / 78,784 |
| 10 | 615 / 172,592 | 576 / 78,640 |

B (019) is lower on both peak RSS and wall-clock in **10 of 10** rounds - decisive, non-overlapping
ranges (`peak_rss_kb`: 172,292-172,592 vs. 78,536-78,784; `elapsed_ms`: 605-726 vs. 570-591). `sig`
(860639953605989024) and `cells` (2,400,000) identical in every one of the 20 runs above.

## Result

**Peak RSS on XLSX: -54.4% vs. the original unpatched baseline (172,373 -> 78,520 KB mean),
-41.7% below even candidate 017's own already-Strong-win floor (134,610 KB)** - fully recovers
candidate 018's regression and goes further, because XLSX now also gets candidate 006's
Arc-interning benefit for its ~1.8M shared-string cells, the same benefit XLSB/XLS already had.
Wall-clock also improves, decisively (10/10 paired rounds faster). XLSB and XLS are unaffected (re-
confirmed, not just reasoned about), since this candidate's own diff touches only
`Xlsx::worksheet_range()`. Metric measured: **peak RSS** (`VmHWM`, this workspace's metric of
record) as primary, wall-clock as secondary, both on the real 2.4M-cell XLSX fixture.

## Does it change behavior/output?

- **Bit-exact?** Yes - `sig` and `cells` identical across baseline, 017-alone, 018, and 019 in
  every run shown above (20+ runs total this candidate, plus the runs already recorded in
  candidates 007/017/018).
- **Is this a breaking change?** No new breaking change from this candidate's own increment (a
  pure deletion/simplification of one function's body, same public signature, same behavior). It
  still carries candidate 006's own already-documented breaking API change (`Data::String` ->
  `Arc<str>`, `DataRef` loses its lifetime parameter) since this candidate's `patch.diff` is 018's
  patch plus this increment, not a standalone patch against pristine `base_commit` for just this
  fix in isolation.
- `HeaderRow::FirstNonEmptyRow` and `HeaderRow::Row(_)` handling is preserved exactly: the deleted
  override's two branches are subsumed by `worksheet_range_ref()`'s own `from_sparse_dense_hint()`
  call (already handles both `HeaderRow` variants, verified bit-exact for the `DataRef` path by
  candidates 007/017) - confirmed by re-running the project's own `HeaderRow`-specific
  `#[rstest]` cases in `tests/test.rs` (`header-row.xlsx`/`temperature.xlsx`/
  `temperature-in-middle.xlsx`, all three `HeaderRow` variants), all passing identically.
- Project test suite (`cargo test --release --all-features`): **45/45 unit tests, 165/166
  integration tests (1 pre-existing `#[ignore]`, unrelated), 67/67 doc tests** - identical
  pass/fail counts to every prior candidate in this workspace, no new failures.
- `cargo build --release --all-features` and `cargo clippy --release --all-features --all-targets`
  both clean (one `unused import: Cell` warning surfaced by this candidate's own deletion, fixed
  in the same patch by dropping `Cell` from `src/xlsx/mod.rs`'s import list - it's still used
  elsewhere in the file via `Cell<DataRef>`/`Cell<Data>` return types from other functions, so this
  is a real now-unused import, not an accidental removal of something still needed elsewhere -
  confirmed by the clean build after removing it).

## Why the result looks the way it does

Exactly as candidate 018 diagnosed: `Xlsx::worksheet_range()`'s override was the *only* reason
candidate 017's `from_sparse_dense_hint()` fix was unreachable for XLSX's default value-reading
API. Deleting it routes XLSX through the same generic default path XLSB already uses (no override
there), which candidate 012 already confirmed does not pay for a second dense buffer when
`DataRef`/`Data` are same-size/same-align (`Vec::from_iter`'s in-place-collect specialization).
With the override gone, XLSX's `worksheet_range()` becomes structurally identical in cost to
XLSB's - which is exactly why XLSX's number (78,520 KB mean) now lands within noise of XLSB's own
017+006 number (78,437 KB mean, candidate 018's own table) rather than being a separate, worse
number as it was in 018.

## What's not done / open questions

- This candidate's own `patch.diff` is 018's full patch plus this increment (a complete, working
  017+006+override-fix combination), not a from-scratch patch built directly against pristine
  `base_commit` in isolation - the `incremental_xlsx_mod.diff` file in this folder isolates just
  this candidate's own 13/-75 line change for a reader who wants to see it apart from 006/017.
- Only `taskset -c 3`-pinned + 2 unpinned samples were taken for the XLSB/XLS re-confirmation
  table (3 samples each), not the full 15-sample/alternating-pair treatment candidate 018 gave
  XLSB originally - acceptable here since the claim being checked is "unchanged from 018's own
  number" (which it is, within the same noise band 018 itself measured), not a new independent
  finding about those two formats.
- Same open items 006/017/018 already carried forward and not re-litigated here: the XLSB fixture
  is synthetic (no real large `.xlsb` with heavy shared-string reuse found yet, per `README.md`);
  `Data::String` -> `Arc<str>` is still an unapplied, breaking API change (design decision for a
  maintainer, not something this workspace applies for real).
- Once this fix (or 006/017 individually) is ever actually applied to real source, it would be
  worth grepping for any other format-specific override in the codebase that similarly predates
  `from_sparse_dense_hint()` and could have silently bypassed it the same way - only XLSX's
  `worksheet_range()` was checked here (the one 018 flagged); `Xlsb`/`Xls`/`Ods`'s own
  `worksheet_range()` implementations were not re-audited for the same shape as part of this
  candidate (XLSB has no override at all per candidate 018's own analysis; XLS's dense-hint path
  is candidate 011's own newer code, unlikely to have the same staleness, but not independently
  re-checked here).
