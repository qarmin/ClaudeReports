# Cap the XLSX shared-strings `Vec::reserve()` driven by the `uniqueCount` XML attribute

**Depends on candidate 001** - reuses `MAX_DENSE_RANGE_CELLS`, added to `src/xlsx/mod.rs` by
candidate 001's patch. `patch.diff` here is a diff of `src/xlsx/mod.rs` only, applied *on top of*
candidate 001's patch (verified with `git apply --check`, same pattern as candidate 003).

While checking XLSX/XLSB for other instances of the "unbounded allocation from an untrusted
declared count" pattern that produced candidates 001 and 003, found a third, distinct, and more
easily triggered instance: `read_shared_strings()` (`src/xlsx/mod.rs`, called during
`Xlsx::new()`/`open_workbook()` itself) does `self.strings.reserve(n)` where `n` comes straight
from `sharedStrings.xml`'s `<sst uniqueCount="...">` attribute - no real string content required.
A 2253-byte crafted file with `uniqueCount="4000000000"` makes calamine attempt to reserve **~96
GB** and abort **during `open_workbook()` alone** (no `worksheet_range()` call needed - the
earliest/easiest trigger point of the three findings this session). Full writeup and standalone
repro (confirmed against published `calamine = "0.36"`):
`___BUGHUNT___/003.xlsx_shared_strings_uniquecount_oom/`.

## What was tried

`self.strings.reserve(n)` -> `self.strings.reserve(n.min(MAX_DENSE_RANGE_CELLS))` - one line,
reusing the constant candidate 001 already added.

| Variant | `poc.xlsx` (`uniqueCount="4000000000"`, 1 real string) peak RSS | `big_300k_rows.xlsx` (2.4M real cells, real shared strings) peak RSS | notes |
|---|---|---|---|
| baseline (`HEAD` 5c02e86) | process aborts attempting to reserve ~96 GB (`ulimit -v 4000000`, exit 134) - triggers on `open_workbook()`/`open`-mode alone | 171,568-172,160 KB (3 runs) | `worksheet_range` |
| patched (001+005) | 3,632-3,704 KB, returns cleanly; the file's one real string still reads correctly (`range` mode: `cells=1`) | unaffected, same 3 runs, same numbers | `worksheet_range` |

Measured with `___OPTIMIZATIONS___/harness/rss_probe` (`open`/`range` modes, peak RSS =
`/proc/self/status` `VmHWM`), release build.

## Result

- **Adversarial input (`poc.xlsx`):** peak RSS drops from "process aborts attempting to reserve ~96
  GB, during file open alone" to a bounded ~3.7 MB, and the file's real (single) string still reads
  correctly.
- **Legitimate large input (`big_300k_rows.xlsx`, real shared strings, ~1.8M shared-string cell
  references):** peak RSS unaffected - the 100,000,000-string cap sits far above any realistic
  shared-strings table, so the `reserve()` call requests the same amount as before, uncapped.

This session's metric is peak RSS; wall-clock is not the point of this candidate.

## Does it change behavior/output?

- **Bit-exact for every input this cap doesn't reject?** yes.
- **What changes, for whom:** a file whose `sharedStrings.xml` declares a `uniqueCount` above
  100,000,000 now has that reservation capped (the actual `Vec<String>` still grows normally via
  `push` if the real string count somehow exceeds the cap - correctness is unaffected, only the
  upfront reservation size is bounded) instead of the process attempting a single arbitrarily large
  allocation and aborting.
- **Is this a breaking change?** No - this doesn't touch any public error type or add a new
  variant; `reserve()`'s cap is purely an internal sizing-hint change, invisible to callers. Unlike
  candidates 001/003 (which needed a new `CellLimitExceeded` error variant since they turn a crash
  into a returned `Err`), this fix keeps the function infallible - the workbook still opens
  successfully even for a file with a wildly-inflated `uniqueCount`, it just doesn't trust that
  number for the reservation size.
- Project test suite: **165/166 integration tests passing both before and after (1 pre-existing
  `#[ignore]`, unrelated), 45/45 unit tests both before and after, 67/67 doc tests both before and
  after** (`cargo test --all-features`, patched tree = candidate 001 + this candidate applied
  together). `cargo clippy --all-features --all-targets` clean.

## Why the result looks the way it does

Same root pattern as candidates 001/003 (an untrusted, unbounded value from the file directly
driving an allocation size) found by deliberately re-applying that same investigative lens to every
other place in the codebase that reads a "count"/"size" hint from file metadata before the real
content backing it has been parsed - `uniqueCount` is exactly that kind of hint (meant purely as a
reader optimization), and like the `Dimensions` record in candidate 003, was trusted outright with
no upper bound.

## What's not done / open questions

- **XLSB's own shared-strings reader was checked and found not to share this specific shape** (see
  `___BUGHUNT___/003.xlsx_shared_strings_uniquecount_oom`'s "Scope" section) - it loops per-declared-count rather than reserving upfront,
  so an inflated count fails fast on the first missing record instead of attempting one huge
  allocation. Not touched by this candidate.
- **`MAX_DENSE_RANGE_CELLS = 100_000_000` reused as-is** for a string count, not a cell count - the
  name is now slightly overloaded (candidate 001 introduced it specifically for the dense-grid cell
  cap). A maintainer taking this patch might prefer a differently-named constant (e.g.
  `MAX_RESERVE_HINT` or similar) shared across both use sites, or a dedicated, independently-tuned
  cap for shared strings specifically - this candidate reused the existing one for minimalism, not
  because 100M is necessarily the "correct" value for a string-count cap specifically.
- This candidate's `patch.diff` only applies cleanly on top of candidate 001's - same caveat as
  candidate 003.
