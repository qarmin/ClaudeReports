# `Data::String`: `String` -> `Arc<str>`, so shared-string-table references share one allocation

This session's earlier candidates (001-005) all targeted *adversarial* inputs (crash/OOM fixes).
This one targets the thing the user actually asked for first: peak RSS on **typical, legitimate**
documents. It's a real, user-authorized public API change (`Data::String(String)` ->
`Data::String(Arc<str>)`, and `DataRef<'a>` loses its lifetime parameter entirely, becoming plain
`DataRef`), not a drop-in patch.

## What was tried

Candidate 002 (this same session) found that ~1.8M of `big_300k_rows.xlsx`'s 2.4M cells are
shared-string-table references into a table with only ~430 bytes of *unique* text - each
reference was paying for its own independent heap allocation when converted from
`DataRef::SharedString(&str)` to owned `Data::String(String)` (via `.to_string()`), regardless of
how many other cells reference the exact same string. The real fix flagged there but not attempted
(needs an API change): make `Data::String` cheap to share instead of always copying.

Changed:
- `Data::String(String)` -> `Data::String(Arc<str>)`.
- `DataRef::String(String)` unchanged (unique/inline/computed strings, no dedup benefit expected).
  `DataRef::SharedString(&'a str)` -> `DataRef::SharedString(Arc<str>)` - since this was the only
  variant carrying `DataRef`'s lifetime parameter, `DataRef<'a>` becomes plain `DataRef` (no
  lifetime at all) once this variant no longer borrows.
- The shared-strings tables (`Xlsx::strings`, `Xlsb::strings`, and the local `strings` built by
  `Xls::parse_workbook`) all become `Vec<Arc<str>>` instead of `Vec<String>`; each unique string is
  now allocated exactly once, when the table itself is read, and every referencing cell gets an
  `Arc::clone()` (a refcount bump, not a copy).
- `impl From<DataRef> for Data`: `DataRef::SharedString(v) => Data::String(v)` is now a plain move
  (zero-cost) instead of `Data::String(v.into())` (an allocation, every time, for every cell).
- **Also rewrote `Xlsx::worksheet_range()`** to build `Cell<Data>` directly during parsing instead
  of delegating to `worksheet_range_ref()` + `.into_iter().map(Into::into).collect()` - this exact
  restructuring was tried and reverted as a **regression** in candidate 002, because back then it
  moved an *allocating* conversion earlier, overlapping it with a bigger buffer. With `Arc<str>`
  the conversion is no longer allocating, so the same restructuring is now safe and skips building
  a second full dense grid.

| Variant | XLS real file (`adhocallbabynames1996to2016.xls`, 18MB, 1,471,681 cells, heavy SST reuse) | XLSX real file (`big_300k_rows.xlsx`, 20MB, 2.4M cells, ~75% shared-string, only ~430B unique text) |
|---|---|---|
| baseline (`HEAD` 5c02e86), 5 runs each | peak 150,668-151,164 KB, end 78,032-78,244 KB, 153-160 ms | peak 171,816-172,252 KB, end 3,616-3,968 KB, 621-637 ms |
| patched, 5 runs each | peak 119,824-120,660 KB, end 30,812-30,924 KB, 116-129 ms | peak 171,632-172,420 KB, end 3,604-3,884 KB, 597-670 ms |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`range` mode; peak RSS = `/proc/self/status`
`VmHWM`, end RSS = `VmRSS` read at the same moment, immediately before process exit), release
build.

## Result

This session's metric is peak RSS, but **end RSS (retained memory, `VmRSS` at exit) and wall-clock
both moved too, and are reported alongside** since all three are relevant to "typical document"
usage, not just the adversarial-input peak this session mostly measured elsewhere.

- **XLS real file: a clear, consistent win on every dimension.** Peak RSS: ~150.9 MB -> ~120.3 MB
  (**-20.3%**). End RSS (what's actually retained once the workbook is loaded and held, which
  matters more than the transient peak for a program that keeps the parsed data around):
  ~78.1 MB -> ~30.9 MB (**-60.4%**). Wall-clock: ~156 ms -> ~120 ms (**-23%**, avoiding ~1.47M
  individual heap allocations is a real CPU cost too, not just a memory one). This is XLS's
  `parse_label_sst()` doing `Data::String(s.clone())` directly during the single parsing pass (XLS
  has no separate borrowed/owned reading path) - the interning benefit lands immediately, with
  nothing else in the way.
- **XLSX real file: no measurable peak-RSS change, small wall-clock win.** Peak RSS is unchanged
  within normal run-to-run noise (~172.0 MB both ways) - **explained, not just observed as null**:
  `Range::from_sparse()`'s own sparse-buffer-plus-dense-buffer overlap (root-caused in candidates
  001/002/004 this session) already dominates XLSX's peak *before* any string ever gets converted
  to owned form, so removing the string-allocation cost (which was never what set that peak) can't
  lower it further - fixing that structural issue is a separate, larger, not-yet-attempted
  candidate (see `IDEAS.md`). Wall-clock did move: ~625 ms -> ~608 ms median (**~2.7%**), consistent
  with skipping a whole extra pass that built and then fully consumed an intermediate
  `Range<DataRef>` the same size as the final result.

## Does it change behavior/output?

- **Bit-exact?** Yes on every fixture exercised - `sig`/`cells` from `rss_probe` identical to
  baseline on both real files above, on every run, both directions (baseline built via `git
  stash`, patched from the working tree).
- **What changes, for whom:** this is the change explicitly authorized this session -
  `Data::String`'s inner type is now `Arc<str>` (was `String`), and `DataRef<'a>` is now plain
  `DataRef` (the lifetime parameter is gone). Any downstream code that pattern-matches
  `Data::String(s)` expecting `s: String` (not `Arc<str>`), or that names `DataRef<'a>` explicitly
  (e.g. in a function signature or struct field), needs updating. Code using the existing
  `DataType` trait accessors (`get_string() -> Option<&str>`, `as_string() -> Option<String>`,
  etc.) is unaffected - those signatures are unchanged, since `Arc<str>` derefs to `str` the same
  way `String` did.
- **Is this a breaking change?** Yes, explicitly - a public field type change (`Data::String`'s
  payload) and a public generic-parameter removal (`DataRef<'a>` -> `DataRef`). Both are the kind
  of change semver would require a major/minor bump for, not a patch release. Authorized by the
  project owner for this session specifically ("możesz modyfikować API - nie ma z tym problemu").
- Project test suite: **45/45 unit tests, 165/166 integration tests (1 pre-existing `#[ignore]`,
  unrelated), 67/67 doc tests - all passing both before and after** (`cargo test --all-features`,
  baseline via `git stash`, patched from the working tree; the project's own test/example/doc-test
  source needed mechanical `.to_string()` -> `.into()`/`Arc::from()` adaptations to compile against
  the new type - included in `patch.diff`, not a functional change). `cargo clippy --all-features
  --all-targets` clean (including fixing the now-genuinely-unused-lifetime warnings this change
  surfaced in `worksheet_range_ref()` signatures and one `impl CellType for DataRef` - all cleaned
  up via `cargo clippy --fix` plus one manual fix, included in `patch.diff`).

## Why the result looks the way it does

XLS and XLSX hit this fix through structurally different paths, which is why one shows a dramatic
win and the other shows none on peak RSS specifically:

- **XLS** never had a separate "borrowed" reading stage - `parse_label_sst()` builds `Data::String`
  directly, in the one and only parsing pass, so the interning benefit is exactly where the
  allocations used to happen. Nothing else in the pipeline was already setting a higher peak that
  this change couldn't get under.
- **XLSX** (and XLSB) go through a `DataRef`-based "borrowed" stage first
  (`worksheet_range_ref()`), where `Range::from_sparse()` already builds a dense grid from a sparse
  `Vec<Cell<DataRef>>` - and that sparse-plus-dense overlap (root-caused this session in candidates
  001/002/004, see `IDEAS.md`) is bigger than anything the string-allocation savings could remove,
  for a file shaped like `big_300k_rows.xlsx` (2.4M real, densely-packed cells). The interning
  change is real and correct - it removes real allocations, real CPU time, and (per XLS's numbers)
  real retained memory - it just isn't the thing currently setting XLSX's *peak*.

## What's not done / open questions

- **Not applied for real** - like every candidate in this workspace, this patch is measured,
  verified, and reverted; applying it for real (bumping the crate's version appropriately given
  the breaking API change, updating the Changelog) is a separate, conscious decision for the
  project owner to make outside this workspace.
- **The still-unaddressed structural issue for XLSX/XLSB peak RSS** (`Range::from_sparse()`'s
  sparse+dense buffer overlap) is the natural next target now that this candidate has shown the
  string-allocation angle alone won't move XLSX's peak - see `IDEAS.md`'s entry on streaming
  directly into a pre-sized dense buffer during parsing, which would need to reuse the
  `MAX_DENSE_RANGE_CELLS`-style cap from candidate 001 and carefully preserve `HeaderRow` handling
  (flagged in candidate 004's own "what's not done" as needing real test-coverage investment, given
  this project's own suite has little coverage for wrong/mismatched `<dimension>` headers - not
  attempted this session for that reason).
- **XLSB's own interning benefit was not independently measured** on a real large file - no large
  real-world `.xlsb` fixture with heavy shared-string reuse was available this session (see
  `___OPTIMIZATIONS___/README.md`'s note on this gap). The code path is structurally identical to
  XLSX's (same `Range::from_sparse` mechanism, same `DataRef::SharedString` -> `Data::String` move),
  so the same "no peak-RSS change, small wall-clock win" result is expected but not confirmed with
  a real fixture.
- **`DataRef::String` (inline/computed, non-shared strings) was deliberately left as owned
  `String`**, not converted to `Arc<str>` - there's no dedup benefit expected for values that are
  actually unique per cell, and converting them too would add an `Arc` allocation where a plain
  `String` allocation already sufficed, for no benefit. Not measured independently to confirm this
  assumption holds (i.e. that inline strings are in fact rarely repeated in real files).
- **`Data::DateTimeIso`/`Data::DurationIso` were left as owned `String`** (not `Arc<str>`) -
  flagged as a possible follow-up in this session's `IDEAS.md` (enum-size angle) but not attempted
  here since these aren't shared-string-table-sourced and the dedup argument doesn't apply the same
  way.
