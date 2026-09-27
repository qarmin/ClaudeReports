# PoC: XLS's `Range::from_sparse()` real-cell-position path IS independently exploitable

Closes the open question left explicit in `___OPTIMIZATIONS___/003.xls_dimension_reserve_cap/RESULT.md`
("What's not done / open questions"): whether XLS's `Range::from_sparse(cells)` /
`Range::from_sparse(formulas)` calls (`src/xls.rs`, building the final range from whatever *real*
BIFF8 cell records were actually parsed) are independently reachable/exploitable, as opposed to
candidate 003's `Dimensions`-record `Vec::reserve()` bug, which needed no real cell data at all.
Candidate 003 hardened this path defensively (`from_sparse_bounded`) but never built a dedicated
PoC targeting real cell records specifically to confirm the mechanism.

**While investigating, found that this exact bug (and a verified fix) was already independently
confirmed in a prior session and is fully documented at
`___BUGHUNT___/004.xls_from_sparse_real_cell_oom/`** - same root cause, same mechanism, same fix
shape as candidate 003's (a fresh, standalone `from_sparse_bounded` addition, not depending on
candidate 001/003's own patches). This candidate reuses that PoC (`poc.xls`) and patch verbatim
and re-confirms the finding independently in this workspace's own `rss_probe` peak-RSS harness, in
a fresh git worktree, to give this workspace's own measured record of it (matching the shape of
candidates 001/003) rather than leaving it only in `___BUGHUNT___/`.

## What was tried

BIFF8 cell records (`Number`, `Label`, `Rk`, `MulRk`, `BoolErr`, `LabelSst`, formula results, ...)
all encode row/column as raw, unvalidated 16-bit fields (`read_u16` in `parse_number`/`parse_rk`/
`parse_label`/etc., `src/xls.rs`). After the per-sheet BIFF record loop, `Range::from_sparse(cells)`
/ `Range::from_sparse(formulas)` compute the bounding box of the *actual* min/max row/column found
across the real cells collected and unconditionally build a dense `vec![Data::default(); rows *
cols]` for it (`src/lib.rs`), with no upper bound - the same unbounded-allocation shape as
candidate 001 confirmed for XLSX/XLSB and candidate 003 confirmed for XLS's `Dimensions` record,
but reached here through genuine cell data rather than a metadata record's declared span.

`testdata/poc.xls` (5632 bytes) is `tests/date.xls` with one real `Rk` record's row/col fields
byte-patched: row `2 -> 0xFFFF` (65535), col `1 -> 0xFFFF` (65535). The file's other real cell (a
`Number` record at row=2, col=0) and its `Dimensions` record are both left completely untouched -
this isolates the bug from candidate 003's separate `Dimensions`-record path. Two genuine,
validly-parsed cell records - one at (2, 0), one at (65535, 65535) - are all that's needed:
`(65535-2+1) * (65535-0+1) = 4,294,835,264` requested cells (the harness's own instrumented error
path below reports the exact runtime value, `4,294,967,296`, i.e. `65536 * 65536` - the two differ
because the real second cell sits at row 65535/col 65535 giving a bounding box computed from
`row_start=2`; either way, multiple billions of cells either way).

| Variant | `poc.xls` peak RSS / outcome | `adhocallbabynames1996to2016.xls` (18MB real file, 1,471,681 real cells) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | process aborts attempting a **137.4 GB** allocation (`memory allocation of 137438953472 bytes failed`, `SIGABRT`, exit 134 under `ulimit -v 4000000`) | 150,784-151,172 KB (3 runs) | `worksheet_range`, reproduced 1/1 in this session (deterministic, matches `___BUGHUNT___/004`'s own reported 137.4 GB exactly) |
| patched | returns `Err(XlsError::CellLimitExceeded { requested: 4294967296, max: 100000000 })` cleanly, ~3.6-3.75 MB peak RSS at the point of failure (3 runs: 3604/3624/3752 KB, no `ulimit` needed) | unaffected (same magnitude, see below) | `worksheet_range` |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode, peak RSS = `/proc/self/status`
`VmHWM`), release build, in a dedicated git worktree at the pinned `base_commit`. The harness's
"range" mode calls `open_workbook_auto(path).expect(...)`, which panics (controlled, exit 101) on
the patched build's `Err` before reaching the harness's own peak-RSS print statement; a small,
worktree-local, non-committed instrumentation change (printing `VmHWM` from the error branch
instead of only the success branch) was used to capture the patched-build peak-RSS numbers above -
this is a harness-only change, discarded with the rest of the scratch worktree, not part of
`patch.diff`.

## Result

- **Adversarial input (`poc.xls`):** peak RSS/outcome goes from "process aborts attempting to
  allocate 137.4 GB" to a bounded ~3.6-3.75 MB with a catchable `Err`. Confirms this candidate's
  central question: the real-cell-position path IS independently exploitable, using only
  genuinely-parsed BIFF8 cell records (no `Dimensions` record involved at all) - a structurally
  smaller worst case than XLSX/XLSB's ~549 GB (candidate 001) but still far beyond anything any
  allocator will grant.
- **Legitimate large input (`adhocallbabynames1996to2016.xls`):** peak RSS unaffected within
  normal run-to-run noise (150.78-151.17 MB baseline vs. 150.78-150.94 MB patched across 3 runs
  each) and `cells`/`sig` are bit-identical (`cells=1471681`, `sig=9293658855915755249`, every
  run, both baseline and patched) - the 100,000,000-cell cap sits far above this file's real
  bounding box.

This session's metric is peak RSS; wall-clock is not the point of this candidate.

## Does it change behavior/output?

- **Bit-exact for every input this cap doesn't reject?** yes - `cells`/`sig` identical to baseline
  on `adhocallbabynames1996to2016.xls` and on the full project test suite (below); the new
  `Err` path is only reachable when the requested dense grid exceeds 100,000,000 cells.
- **What changes, for whom:** a file whose *real* cell records' bounding box would require more
  than 100,000,000 cells now returns `Err(XlsError::CellLimitExceeded { requested, max })` from
  `worksheet_range()` instead of the process aborting. No known legitimate `.xls` worksheet
  approaches this (BIFF8's own structural max is `u16::MAX * u16::MAX ≈ 4.29 billion` cells - the
  100M cap sits comfortably below that ceiling and far above any realistic real-world sheet).
- **Is this a breaking change?** Same shape as candidates 001/003: a new, non-`#[non_exhaustive]`
  `XlsError::CellLimitExceeded` variant - a caller exhaustively matching `XlsError` without a
  wildcard arm needs to add one. No previously-`Ok` input becomes `Err`.
- Project test suite (this worktree, `cargo test --release --all-features`): **165/166 integration
  tests passing both before and after patch (1 pre-existing `#[ignore]`, unrelated), 45/45 unit
  tests both before and after, 67/67 doc tests both before and after.** `cargo clippy
  --all-features --all-targets` clean on the patched tree.

## Why the result looks the way it does

Same root pattern as candidates 001/003 (an untrusted, unbounded value from the file directly
driving a dense-grid allocation size), reached through a third, independent path this time: real
BIFF8 cell records' own row/column fields, rather than a metadata record's declared span
(candidate 003) or XLSX/XLSB's text-based cell references (candidate 001). All three share the
same underlying `Range::from_sparse()` (`src/lib.rs`) with no cap - each call site needed its own
independent confirmation because each is reached by a structurally different kind of
attacker-controlled input (a declared span vs. two genuinely-parsed data records vs. a text
reference), even though the fix (`from_sparse_bounded`) is identical machinery once written.

## What's not done / open questions

- **This candidate's `patch.diff` is identical to `___BUGHUNT___/004.xls_from_sparse_real_cell_oom/patch.diff`**
  (byte-for-byte, diffed and confirmed in this session) - reused rather than re-derived, since
  that session had already produced a verified, self-contained fix for exactly this mechanism (adds
  `from_sparse_bounded` to `src/lib.rs` fresh, does not depend on candidate 001's or 003's own
  patches being applied first).
- **`MAX_DENSE_RANGE_CELLS = 100_000_000` is the same judgment call as candidates 001/003/004**
  (matches existing precedent, not independently derived for XLS's own realistic worksheet-size
  distribution).
- Only a single `Rk` record was byte-patched for this PoC (matching `___BUGHUNT___/004`'s own
  approach); `Number`/`Label`/`MulRk`/`LabelSst`/formula-result records share the identical
  unvalidated `read_u16` row/column read pattern, so there's no reason to expect a different
  result from those record types, but each wasn't independently re-verified in this session.
- If candidate 003's own patch (`Dimensions`-record `reserve()` cap) is ever applied for real
  independently of this one, the two patches touch overlapping regions of `src/xls.rs` (003's
  `Dimensions`-record fix sits a few lines above this candidate's `Range::from_sparse` call sites
  in the same function) and would need merging, not applying blind in sequence.
