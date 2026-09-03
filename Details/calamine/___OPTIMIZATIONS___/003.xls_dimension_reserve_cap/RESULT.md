# Cap the XLS `Dimensions`-record-driven `Vec::reserve()` and dense-grid build

**Depends on candidate 001** - this patch calls `Range::from_sparse_bounded()`, which candidate
001 added to `src/lib.rs`. `patch.diff` here is captured as a diff of `src/xls.rs` only, applied
*on top of* candidate 001's patch (verified with `git apply --check` against 001-then-003, not
against pristine `HEAD` alone - see this candidate's `justfile`).

While verifying an open question from candidate 001/`___BUGHUNT___/001.xlsx_xlsb_dense_range_oom` ("was XLS independently
checked for the same class of bug?"), found something worse and more directly triggerable: XLS's
`src/xls.rs`, in its BIFF `Dimensions`-record handler, does
`cells.reserve(rows.saturating_mul(cols))` where `rows`/`cols` come straight from the record's own
declared row/column span - no real cell data required at all. The extended (14-byte) form of this
record encodes the end row as a raw `u32` and end column as a raw `u16`, both fully
attacker-controlled. A single malformed 14-byte record in an otherwise-untouched, valid 5632-byte
`.xls` file makes calamine attempt to `reserve()` **~11.2 PB** and abort the process. Full writeup
and standalone repro (confirmed against the published `calamine = "0.36"` crate):
`___BUGHUNT___/002.xls_dimension_reserve_oom/`.

## What was tried

1. `cells.reserve(rows.saturating_mul(cols))` -> `cells.reserve(rows.saturating_mul(cols).min(MAX_DENSE_RANGE_CELLS))`
   (new `MAX_DENSE_RANGE_CELLS = 100_000_000` constant in `src/xls.rs`, matching the value already
   used in `ods.rs`/`xlsx::mod.rs`/`xlsb::mod.rs`).
2. `rows`/`cols` computation switched from `(end.0 - start.0 + 1) as usize` (an unchecked `u32`
   subtraction that wraps, not panics, in a release build if a crafted file sets `start > end`) to
   `end.0.saturating_sub(start.0) as usize` + `saturating_add(1)` - closes a second, smaller
   integer-underflow angle on the same record noticed while fixing the first issue, not
   independently PoC'd.
3. The two `Range::from_sparse(cells)` / `Range::from_sparse(formulas)` calls later in the same
   function (building the final range/formula grids from whatever cells were *actually* collected)
   switched to `Range::from_sparse_bounded(_, MAX_DENSE_RANGE_CELLS)` - defense in depth against the
   same real-cell-position vector candidate 001 fixed for XLSX/XLSB, for XLS too (not independently
   proven exploitable for XLS - BIFF8 cell records use 16-bit row/column fields, a structurally
   smaller worst case than XLSX/XLSB's text-based references - but reuses already-built,
   already-tested infrastructure at near-zero marginal cost, so hardening it anyway was cheap).

| Variant | `poc.xls` (crafted Dimensions record, real data untouched) peak RSS | `adhocallbabynames1996to2016.xls` (18MB real file, 1,471,681 real cells) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | process aborts attempting to `reserve()` ~11.2 PB (`ulimit -v 4000000`, exit 134) | 150,148-150,880 KB (3 runs) | `worksheet_range` |
| patched (001+003) | 3,476-3,652 KB (3 runs, no `ulimit` needed), returns real data cleanly (`cells=6`, matches the file's actual small dataset - the crafted dimension is capped, not treated as real) | unaffected (same magnitude, see below) | `worksheet_range` |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode, peak RSS = `/proc/self/status`
`VmHWM`), release build.

## Result

- **Adversarial input (`poc.xls`):** peak RSS drops from "process aborts attempting to reserve
  ~11.2 PB" to a bounded ~3.5 MB, and the file's *actual* small dataset (6 real cells, from the
  untouched original `date.xls` cell records) is still read out correctly - the capped `reserve()`
  doesn't lose or corrupt real data, it just stops trusting the declared span for sizing.
- **Legitimate large input (`adhocallbabynames1996to2016.xls`):** peak RSS unaffected - the
  100,000,000-cell cap sits far above this file's real declared dimensions (1,471,681 actual
  cells), so the `reserve()` call requests the same amount as before, uncapped.

This session's metric is peak RSS; wall-clock is not the point of this candidate.

## Does it change behavior/output?

- **Bit-exact for every input this cap doesn't reject?** yes.
- **What changes, for whom:** a file whose `Dimensions` record declares (or whose real cells'
  bounding box would require) more than 100,000,000 cells now returns
  `Err(XlsError::CellLimitExceeded { requested, max })` from `worksheet_range()` instead of the
  process aborting - for `poc.xls` specifically, the *reserve* is now capped but the file's real
  (small) data still parses successfully, since the actual cell count never approaches the cap; a
  file whose *real* cell positions genuinely exceed the cap would now get the new `Err` instead of
  attempting the dense-grid build (this path was not independently reproduced as exploitable for
  XLS - see "What's not done" below).
- **Is this a breaking change?** Same shape as candidate 001: new non-`#[non_exhaustive]` enum
  variant (`XlsError::CellLimitExceeded`) - a caller exhaustively matching `XlsError` without a
  wildcard arm needs to add one. No `Ok` input changes.
- Project test suite: **165/166 integration tests passing both before and after (1 pre-existing
  `#[ignore]`, unrelated), 45/45 unit tests both before and after, 67/67 doc tests both before and
  after** (`cargo test --all-features`, patched tree = candidate 001 + this candidate applied
  together, since this candidate depends on 001's `from_sparse_bounded`). `cargo clippy
  --all-features --all-targets` clean.

## Why the result looks the way it does

Same root pattern as candidate 001 (an untrusted, unbounded value from the file directly driving
an allocation size), but reached through a different, more direct path: XLS's `Dimensions` record
handler never even collects real cells before requesting the reservation - it trusts the record's
own declared span outright. The existing `if 0xFF < cf || cl < cf { cf = 0; }` guard just above
this code (comment cites `tests/OOM_alloc2.xls`) only clamps the *start* column, not the end
row/column values that actually drive the `rows`/`cols` computation - it was addressing a different,
narrower problem (an inconsistent start column) and was never a general bound on the allocation
size.

## What's not done / open questions

- **The real-cell-position vector for XLS (mirroring candidate 001's XLSX/XLSB finding) was
  hardened via `from_sparse_bounded` but not independently confirmed exploitable** - BIFF8 cell
  records (Number/Label/etc.) encode row/column as 16-bit fields, giving a structurally smaller
  worst case (`u16::MAX * u16::MAX ≈ 4.29 billion` cells, ~172 GB at `size_of::<Cell<Data>>()`)
  than XLSX/XLSB's text-based references, but a PoC for this specific angle was not built in this
  session - the `Dimensions`-record angle above was confirmed instead, and was the more severe,
  more directly triggerable one found.
- **`MAX_DENSE_RANGE_CELLS = 100_000_000` is the same judgment call as candidate 001** (matches
  existing precedent, not independently derived for XLS's own realistic worksheet-size
  distribution).
- This candidate's `patch.diff` only applies cleanly on top of candidate 001's - see the header
  note and `justfile`. If candidate 001 is ever revised, this one needs re-checking against the
  new `from_sparse_bounded` signature.
