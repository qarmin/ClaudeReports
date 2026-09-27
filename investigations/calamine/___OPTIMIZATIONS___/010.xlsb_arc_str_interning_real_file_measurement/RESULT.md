# Re-measure candidate 006 (`Data::String` -> `Arc<str>`) on a real-shaped, large XLSB file

Candidate 006's `RESULT.md` flagged one unverified gap: its `Data::String(String)` ->
`Data::String(Arc<str>)` interning change was reasoned to behave the same on XLSB as on XLSX (same
`Range::from_sparse()` mechanism, same `DataRef::SharedString` -> `Data::String` move) but never
actually measured on XLSB, because no large real-world-shaped `.xlsb` fixture with heavy
shared-string reuse was available at the time. This candidate builds one and re-runs candidate
006's own peak-RSS/end-RSS/wall-clock measurement against it. No new patch was authored - this
candidate re-applies candidate 006's own `patch.diff` verbatim (copied unmodified into this
folder) to `calamine`'s worktree and measures it against the new fixture.

`rustc 1.98.0`, `cargo 1.98.0` (same toolchain as candidate 006).

## What was tried

Fixture: `testdata/big_300k_rows.xlsb` (44 MB, synthetic but real-format-shaped - see
`testdata/FIXTURE_NOTES.md` for exactly how and why it had to be built rather than downloaded, and
for a real correctness bug in calamine's XLSB "short-form record" handling found and worked around
while building it). 300,000 rows x 8 columns, 2,400,000 cells: even columns numeric, odd columns a
shared string from a 10-entry pool (~1.2M string cells sharing 10 unique values) - heavier reuse
than candidate 006's XLSX fixture (`big_300k_rows.xlsx`, ~430 unique bytes across its shared
strings) but the same "many refs into a tiny unique-text table" shape.

Measured with `harness/rss_probe` (`range` mode - the same `Range<Data>`-materializing path
candidate 006 measured), release build, base commit `5c02e86` (same as candidate 006).
`sig`/`cells` identical across every run below, both variants, confirming bit-exact output
throughout - reported once here rather than repeated per row.

**Peak/end RSS** (5 runs each, `taskset -c 3`, non-alternating - collected first, before the
alternating wall-clock runs below):

| Variant | peak_rss_kb range | end_rss_kb range |
|---|---|---|
| baseline (`git stash`-equivalent, `5c02e86` unpatched) | 172,136-172,428 | 3,616-3,968 |
| patched (candidate 006's patch applied) | 171,824-172,560 | 3,616-3,968 |

**Wall-clock**, alternating baseline(B)/patched(P) pairs (same process, same moment, to cancel out
system drift - system load rose to ~11 mid-session from sibling work in this batch, so alternating
rather than block-sequential runs was used to keep the comparison paired) - first pair of each
block discarded as warmup, elapsed_ms, median of remaining 9:

| Variant | pinned (`taskset -c 5`) median | unpinned median | notes |
|---|---|---|---|
| baseline | 182 ms | 173 ms | min 176/169, max 191/198 |
| patched | 169 ms | 167 ms | min 167/164, max 177/180; **lower than baseline in 8-9 of 9 paired rounds in both modes** |

## Result

**Peak RSS: no measurable change**, exactly as candidate 006 predicted for the XLSX case and now
confirmed for XLSB too - patched range (171,824-172,560 KB) fully overlaps baseline's
(172,136-172,428 KB); the difference is within run-to-run noise, not a directional effect. End RSS
likewise unchanged (3,616-3,968 KB both ways).

**Wall-clock: a small, consistent win**, larger than XLSX's ~2.7% but the same direction and same
mechanism - pinned median 182 -> 169 ms (**-7.1%**), unpinned median 173 -> 167 ms (**-3.5%**). The
paired/alternating design makes this more trustworthy than the spread alone suggests: patched was
faster than baseline in 8 of 9 (unpinned) and 9 of 9 (pinned) same-moment paired rounds, despite
noticeably higher system load during this run (`uptime` load average reached ~11 during the
alternating runs, from sibling candidates in this session's batch) than candidate 006's original
session saw.

This closes candidate 006's flagged gap: **the "no peak-RSS change, small wall-clock win" result
for XLSX does hold for XLSB**, confirmed rather than assumed by analogy.

## Does it change behavior/output?

Same change as candidate 006 - see that candidate's own `RESULT.md` for the full behavior-change
writeup (this candidate applied that same patch verbatim, no new source change). Summary:

- **Bit-exact?** Yes on this fixture - `sig`/`cells` identical across every baseline and patched
  run above (2,400,000 cells, same signature value every time).
- **Breaking change?** Yes, the same one candidate 006 already documents (`Data::String`'s payload
  becomes `Arc<str>`, `DataRef<'a>` becomes plain `DataRef`) - not reintroduced or modified here.
- Project test suite: **45/45 unit tests, 165/166 integration tests (1 pre-existing `#[ignore]`,
  unrelated), 67/67 doc tests - all passing both before and after** (`cargo test --all-features`,
  run in this candidate's own worktree, both on the unpatched tree and after applying candidate
  006's patch.diff). Same counts as candidate 006's own original run - re-confirms the patch still
  applies clean and behaves identically against the current `base_commit`.

## Why the result looks the way it does

Identical mechanism to candidate 006's XLSX finding, now confirmed rather than assumed: XLSB goes
through the same `Range::from_sparse()` sparse-then-dense-grid path as XLSX (both go through
`worksheet_range_ref()` first). That sparse+dense buffer overlap (root-caused in candidates
001/002/004, still not fixed) already sets XLSB's peak RSS before any string ever gets converted to
owned form, so removing the string-allocation cost can't lower a peak it was never responsible for
- exactly the same explanation candidate 006 gave for XLSX, now verified on a second format instead
of inferred by code-path analogy. The wall-clock win is real for the same reason it was on XLSX:
`Arc::clone()` (refcount bump) replacing `.to_string()` (allocation + copy) for ~1.2M shared-string
cell conversions, plus skipping the second full `Range<DataRef>` -> `Range<Data>` pass (candidate
006's `worksheet_range()` rewrite applies to XLSX only per that patch - XLSB's own
`worksheet_range()` in `src/lib.rs`'s generic default impl still benefits from the now-zero-cost
`DataRef::SharedString` -> `Data::String` move even without an XLSB-specific restructuring).

## What's not done / open questions

- This candidate is a **pure re-measurement**, not a new patch - `patch.diff` in this folder is
  candidate 006's own patch, copied verbatim (confirmed identical: applied byte-for-byte from
  `006.data_string_arc_interning/patch.diff`, `git apply --check` against this candidate's own
  pinned `base_commit` succeeds). Nothing here should be treated as a second, independent
  implementation to compare against candidate 006's.
- The fixture is synthetic (no real-world large `.xlsb` with heavy shared-string reuse was found -
  same gap noted in `___OPTIMIZATIONS___/README.md`), though shape-matched to the existing XLSX
  fixture and built via a real third-party writer (SheetJS), not hand-crafted byte-for-byte.
- **A real, separate correctness bug in calamine was found while building this fixture** (silent
  cell loss on MS-XLSB "short-form" cell records - `BrtShortRk`/`BrtShortIsst`/`BrtShortSt`/etc.,
  `0x000C`-`0x0012`, not handled by `src/xlsb/cells_reader.rs`'s `next_cell()`/`next_formula()`).
  This is unrelated to candidate 006's change (present on `base_commit` before candidate 006's
  patch, and not touched by it) and is **not fixed by this candidate** - see
  `testdata/FIXTURE_NOTES.md` for the full root-cause writeup. Flagged for this session's batch-level
  `___BUGHUNT___/` transcription rather than fixed here, per this candidate's own task scope (a
  pure measurement re-check, not a new fix).
