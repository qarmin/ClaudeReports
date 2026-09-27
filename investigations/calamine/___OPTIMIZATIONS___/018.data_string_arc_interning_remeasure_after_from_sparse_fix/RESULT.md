# Re-measure candidate 006 (`Data::String` -> `Arc<str>`) now that candidate 017 moved the sparse+dense floor

`rustc 1.98.0`, `cargo 1.98.0`.

Candidate 010 re-measured candidate 006's `Data::String(String)` -> `Data::String(Arc<str>)`
interning patch on a real-shaped XLSB file and found **no** peak-RSS change, because the peak was
set upstream by the still-unfixed `Range::from_sparse()` sparse-then-dense buffer overlap (same
root cause as candidates 001/002/004/007) - candidate 010's own `RESULT.md` explicitly flagged
that this number would look different "once the sparse-buffer floor moved". Candidate 017 landed
during this same session's batch and does move that floor (`from_sparse_dense_hint()`, unifying
candidates 007/011/013, applied to `Xlsb`/`Xlsx::worksheet_range_ref()` and XLS's own dense-hint
path). This candidate layers candidate 006's own patch on top of candidate 017's patch and
re-measures - **and finds a materially different, format-dependent story**, not simply "006's win
got bigger": a real further win on XLSB, but a real regression back to pre-017 levels on XLSX,
because candidate 006 shipped its own `Xlsx::worksheet_range()` override (written before candidate
017 existed) that bypasses `worksheet_range_ref()` entirely and therefore never reaches candidate
017's fix. See "Why the result looks the way it does" below.

Three variants were built and measured, all against `base_commit` `5c02e86` (unpatched calamine,
same commit as every prior candidate this session):

- **baseline**: unpatched `5c02e86`.
- **017-alone**: candidate 017's `patch.diff` applied verbatim, nothing else.
- **017+006**: candidate 017's `patch.diff`, plus candidate 006's own patch **adapted** on top of
  it (see "How the patches were combined" below - candidate 006's patch does not apply verbatim on
  top of 017's rewritten `worksheet_range_ref()`/import lines; the adaptation is mechanical, no new
  logic, and is captured in full in this candidate's own `patch.diff`).

## How the patches were combined

Candidate 006's patch conflicts with candidate 017's in exactly one file, `src/xlsb/mod.rs`,
because 017 fully rewrote `Xlsb::worksheet_range_ref()`'s body (to call
`Range::from_sparse_dense_hint()`) while 006's patch changes that same function's *signature*
(drops the `'a` lifetime, since `DataRef` itself loses its lifetime parameter once
`SharedString` becomes `Arc<str>` instead of `&'a str`). All other 006 hunks in `src/xlsb/mod.rs`
(the `Arc` import, `strings: Vec<String>` -> `Vec<Arc<str>>`, and the `self.strings.push(...)` call
site) apply cleanly and are semantically identical to 006's original intent. Resolution: kept
017's rewritten function body verbatim, changed only its signature line the same way 006's own
patch changes it (`fn worksheet_range_ref(&mut self, name: &str) -> Result<Range<DataRef>, XlsbError>`,
no `'a`). `src/xls.rs` and `src/xlsx/mod.rs` are touched by both patches too, but at
non-overlapping hunks, so `git apply` applied 006's hunks there unaided - except one import in
`src/xlsx/mod.rs`: 017 dropped the (now locally-unused, from *its own* rewrite's point of view)
`Cell` import from `crate::{...}`, but 006's separate, still-present `Xlsx::worksheet_range()`
override (see below) still constructs `Cell` values directly, so the import had to be restored.
Both fixups are purely mechanical (a function signature, an import list) - no behavior was
authored that isn't already in one of the two source patches. The full resulting diff (017 + 006,
adapted) is this candidate's own `patch.diff`; `git apply --check` against pristine `5c02e86`
succeeds.

## What was tried

Fixture (XLSB): `testdata/big_300k_rows.xlsb` (same 44 MB synthetic-but-real-format-shaped fixture
candidate 010 built - 300k rows x 8 cols, 2.4M cells, ~1.2M string cells sharing 10 unique values;
see `testdata/FIXTURE_NOTES.md`, copied verbatim from candidate 010). Fixture (XLSX):
`___OPTIMIZATIONS___/testdata_external/big_300k_rows.xlsx` (candidate 007's own fixture, 20 MB,
2.4M cells, ~75% shared-string), used as a secondary cross-format check since candidate 017 also
touches `Xlsx::worksheet_range_ref()`.

Measured with `harness/rss_probe` (`range` mode), release build, `taskset -c 3` for pinned runs.
`sig`/`cells` identical across every run in every table below (both files) - confirmed bit-exact,
reported once here rather than repeated per row.

**XLSB peak RSS** (15 samples per variant: 5 pinned + 2 unpinned + 8 from the alternating wall-clock
runs below, all `range` mode on `big_300k_rows.xlsb`):

| Variant | peak_rss_kb range | mean |
|---|---|---|
| baseline (no patches) | 171,988-172,584 | 172,419 |
| 017-alone | 115,912-116,216 | 116,109 |
| **017+006** | **78,232-78,592** | **78,437** |

**XLSX peak RSS** (7 samples per variant: 5 pinned + 2 unpinned, `range` mode on `big_300k_rows.xlsx`):

| Variant | peak_rss_kb range | mean |
|---|---|---|
| baseline (no patches) | 172,080-172,768 | 172,373 |
| 017-alone | 134,368-134,844 | 134,610 |
| **017+006** | **172,104-172,516** | **172,314** |

**XLS peak RSS** (3 pinned samples per variant, `range` mode on
`tests/adhocallbabynames1996to2016.xls`, the same real 18MB/1.47M-cell fixture candidates 003/006/011
used - added as a third format check once the XLSX regression below was found, since candidate 017
also restructures XLS via a dense-hint path, same interaction risk):

| Variant | peak_rss_kb range | mean | end_rss_kb mean |
|---|---|---|---|
| baseline (no patches) | 151,448-151,528 | 151,477 | 78,217 |
| 017-alone | 142,540-142,664 | 142,601 | 52,269 |
| **017+006** | **95,204-95,336** | **95,284** | **5,004** |

XLS behaves like XLSB, not like XLSX: a real further win on top of 017 (**-33.2%** peak RSS beyond
the 017-alone floor, **-37.1%** vs. the original baseline; end RSS drops **-90.4%** vs. baseline,
consistent with candidate 006's own original XLS finding of a large end-RSS reduction). `sig`
(9293658855915755249) and `cells` (1,471,681) identical across all three variants.

**XLSB wall-clock**, alternating 017-alone(A)/017+006(B) pairs (same process, same moment, pinned
`taskset -c 5`, 10 rounds, first 2 discarded as warmup per this workspace's usual practice),
`elapsed_ms`, median of remaining 8:

| Variant | median | min-max | notes |
|---|---|---|---|
| 017-alone (A) | 113.5 | 113-119 | |
| 017+006 (B) | 102 | 101-103 | **faster than A in 10/10 raw rounds (8/8 after discarding warmup)** |

Project test suite (`cargo test --release --all-features`), run three times in this candidate's
own worktree - unpatched, 017-alone, and 017+006 - all three identical: **45/45 unit tests, 165/166
integration tests (1 pre-existing `#[ignore]`, unrelated), 67/67 doc tests**. `cargo clippy
--release --all-features --all-targets` clean on the 017+006 tree.

## Result

**Two of three formats (XLSB, XLS) show the further win candidate 010 predicted; XLSX regresses
instead, for a format-specific reason explained below.**

**XLSB: a real, further peak-RSS win on top of candidate 017's own fix** - 116,109 KB (017-alone)
-> 78,437 KB (017+006), **-32.4%** further, **-54.5%** vs. the original unpatched baseline
(172,419 KB). This is the win candidate 010 predicted would appear "once the floor moves" - it
does, and it is large. Wall-clock also improves further: pinned median 113.5 -> 102 ms (**-10.1%**),
decisive (faster in every one of 10 raw alternating rounds). `sig`/`cells` bit-identical across
every run.

**XLSX: a real regression relative to the 017-alone floor**, not a smaller win - 134,610 KB
(017-alone) -> 172,314 KB (017+006), i.e. **017's entire 21.9% XLSX peak-RSS reduction is undone**,
landing back within noise of the original unpatched baseline (172,373 KB). This is not a
measurement artifact: it reproduces across 7 samples per variant with non-overlapping ranges
(134,368-134,844 vs. 172,104-172,516), `sig`/`cells` bit-identical throughout. See "Why the result
looks the way it does" for the mechanism.

**XLS: the same real further win as XLSB** - 142,601 KB (017-alone) -> 95,284 KB (017+006),
**-33.2%** further, **-37.1%** vs. the original unpatched baseline (151,477 KB); end RSS drops
-90.4% (78,217 -> 5,004 KB). `sig`/`cells` bit-identical across all three variants.

Metric measured: **peak RSS** (`VmHWM`, this workspace's metric of record) as primary, wall-clock
as secondary, per `README.md`'s convention - both dimensions stated explicitly per format above.

## Does it change behavior/output?

- **Bit-exact?** Yes on all three fixtures (XLSB/XLSX/XLS), all three variants
  (baseline/017-alone/017+006) - `sig` and `cells` identical in every run shown above.
- **Is this a breaking change?** Yes, the same public API change candidate 006 already documents:
  `Data::String`'s payload becomes `Arc<str>`, `DataRef<'a>` becomes plain `DataRef`. Not modified
  or extended by this candidate - reapplies 006's own design unchanged (only the two mechanical
  merge fixups described above, neither of which touches public API surface).
- Project test suite: **45/45 unit tests, 165/166 integration tests (1 pre-existing `#[ignore]`,
  unrelated), 67/67 doc tests - passing identically on unpatched, 017-alone, and 017+006 trees.**

## Why the result looks the way it does

**XLSB's win is real and explained the same way candidate 010 already reasoned it would be**: XLSB
has no format-specific `worksheet_range()` override - it uses the generic default implementation in
`src/lib.rs`, which is `worksheet_range_ref()` (now candidate 017's direct-to-dense-buffer
`from_sparse_dense_hint()` path) followed by `.into_iter().map(Into::into).collect()`. Once 017
removes the sparse buffer from `worksheet_range_ref()`, the only thing left standing between "fast
dense read" and "zero extra allocation" is the `DataRef::SharedString -> Data::String` conversion
in that `.map(Into::into)` step - which is exactly what 006's `Arc<str>` change makes free (an
`Arc::clone()` instead of a `String` allocation+copy) for the ~1.2M shared-string cells in this
fixture. With both fixes present, XLSB reaches close to the streaming `worksheet_cells_reader()`
floor (candidate 007's `README.md` measured that floor at ~3.6 MB for this cell count on a
different file - `big_300k_rows.xlsb`'s own end-RSS here, ~3.6-4.0 MB across every variant, is
consistent) while still materializing a full dense `Range<Data>`.

**XLSX's regression is a genuine patch-interaction finding, not noise**: candidate 006 shipped its
own hand-written `Xlsx::worksheet_range()` (see the block quoted below, still present verbatim in
the 017+006 tree), authored specifically because *at the time 006 was written, candidate 007/017's
`from_sparse_dense_hint()` didn't exist yet* - 006's own comment reasons about avoiding "a second
dense grid" by building `Cell<Data>` directly during parsing, but the mechanism it uses to do that
is the *pre-007* pattern: collect a sparse `Vec<Cell<Data>>` via `cells.push(...)`, then call the
plain (never-improved) `Range::from_sparse(cells)`. Candidate 017 never touches
`Xlsx::worksheet_range()` (it only rewrote `worksheet_range_ref()`, per its own scope), so once 006
is layered on top, XLSX's `worksheet_range()` **still calls the reintroduced sparse-then-dense
path** and never reaches `worksheet_range_ref()`/`from_sparse_dense_hint()` at all - 017's fix
becomes entirely unreachable for XLSX through this call path. The `Data`/`DataRef` conversion
itself is still Arc-cheap (hence wall-clock landing between the baseline and 017-alone numbers,
612-620ms vs. baseline's 634-662ms and 017-alone's 606-629ms - a real but small win, swamped by the
peak-RSS regression), but the sparse `Vec<Cell<Data>>` buffer is back, at the same size it always
was, overlapping the dense buffer `from_sparse` still builds internally.

```rust
// src/xlsx/mod.rs, current 017+006 tree - candidate 006's own override, unmodified by this
// candidate, now stale relative to candidate 017:
fn worksheet_range(&mut self, name: &str) -> Result<Range<Data>, XlsxError> {
    ...
    let mut cells = Vec::new();          // <- the sparse buffer 017 eliminates in worksheet_range_ref()
    ...
    loop {
        match cell_reader.next_cell() {
            ...
            Ok(Some(cell)) => cells.push(Cell { pos: cell.pos, val: cell.val.into() }),
            ...
        }
    }
    ...
    Ok(Range::from_sparse(cells))        // <- the plain, never-improved from_sparse()
}
```

XLSX's `worksheet_range_ref()` itself (the zero-copy, `DataRef`-returning entry point, not the
`Data`-returning `worksheet_range()` this candidate's fixture exercises) is unaffected by this
interaction - callers using `worksheet_range_ref()` directly (or XLSB, which has no override at
all) still get 017's full benefit with 006 layered on top.

## What's not done / open questions

- **This candidate does not fix XLSX's regression** - fixing it would mean either (a) deleting
  006's custom `Xlsx::worksheet_range()` override entirely and letting XLSX fall back to the
  generic default (`worksheet_range_ref()` + `.map(Into::into).collect()`, the same path XLSB
  uses, which candidate 012 already confirmed doesn't pay for a second dense buffer thanks to
  `Vec::from_iter`'s in-place-collect specialization), or (b) rewriting the override to call
  `Range::from_sparse_dense_hint()` directly the same way `worksheet_range_ref()` now does. Option
  (a) looks like the simpler, higher-confidence fix (removes code instead of adding a second
  hand-rolled copy of the same dense-hint logic) but this candidate's scope was re-measurement, not
  a new patch - flagging as a follow-up idea rather than attempting it here (see final report).
- XLS was checked (3 pinned samples per variant, see table above) once the XLSX regression raised
  the interaction risk generally - it behaves like XLSB (a real further win, no stale-override
  problem), not like XLSX. Only single-core (`taskset -c 3`) samples were taken for XLS, no
  multi-core/unpinned runs and no alternating wall-clock comparison (time budget went to the
  primary XLSB measurement and diagnosing the XLSX regression) - the peak-RSS win is large and
  reproduces cleanly across 3 samples with non-overlapping ranges, but a reader wanting the same
  statistical rigor as XLSB's 15-sample/alternating-pair treatment would need to add that.
- The XLSB fixture is synthetic (same caveat candidate 010 already recorded - no real-world large
  `.xlsb` with heavy shared-string reuse was found).
- Wall-clock comparisons above are all single alternating-pair runs (`RUNS=10`, one system state);
  not repeated across multiple independent sessions.
