# object-collector-ts-lookup-retry-panic

**This candidate overturns the "Not reproducible" verdict of earlier candidate `030`
(`collector-source-parser-unreachable-safe`).** 030 examined a *different* line in the same
function (`object/src/collector.rs`'s `CollectionSource::set_parser_with_ts`) - the
`let CollectionSource::Parser(parser) = self else { unreachable!() }` re-match immediately after
assignment, which genuinely is safe by Rust's `&mut` exclusivity guarantee, as 030 correctly
established. This candidate is a distinct bug earlier in the same function that 030's session did
not consider. See `030.collector-source-parser-unreachable-safe/CORRECTION.md` for the pointer
from the old record to this one; 030's own original analysis of the `unreachable!()` line is not
rewritten and still stands as correct for the specific line it covered.

`CollectionSource::set_parser_with_ts` (`object/src/collector.rs:440-474`) does, in order:

```rust
let src = src.take().unwrap();          // (1) take ownership of the reader out of `self`

// look up transfer syntax
let ts = ts_index
    .get(ts_uid)
    .context(UnrecognizedTransferSyntaxSnafu { ts_uid: ts_uid.to_string() })?;   // (2) can fail

let mut options = LazyDataSetReaderOptions::default();
// ... construct the parser using `src` and `ts`, assign `*self = CollectionSource::Parser(...)`
```

If step (2)'s lookup fails, the function returns `Err(..)` via `?` - but step (1) already ran,
so `self` is left as `CollectionSource::Raw { reader: None, .. }` (the `Option` was `take()`n and
never put back). `DicomCollector::has_parser()` still reports `false` for this state (it's still
the `Raw` variant, not `Parser`), so any caller that calls `read_dataset_to_end` (or any of the
five other public methods that funnel through `set_parser_with_ts` when `!has_parser()`) again on
the same collector re-enters this function, and step (1)'s `src.take().unwrap()` now unwraps
`None` and panics.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-060): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/collection-source-ts-lookup-panic` (commit `5de82156`) reordering these exact two statements | Confirmed unmerged |
| Cross-checked against already-completed candidate 030 (same function) | Read 030's `RESULT.md` | Confirmed 030 examined a different line/mechanism entirely (the post-assignment re-match, not the pre-lookup `take()`); this is a genuinely separate, previously-unconsidered bug |
| Read every public call site of `set_parser_with_ts` in `DicomCollector` (`read_dataset_to_end`, `read_dataset_up_to`, and 4 others) - all guard with `if !self.source.has_parser()` | Static code reading | Confirmed `has_parser()` only checks the enum variant (`Raw` vs `Parser`), not whether `reader` is `Some`/`None` within `Raw` - so a failed first call leaves the collector in a state that looks retriable but isn't |
| Found `DicomCollector::new_with_ts` - a public constructor that sets the transfer syntax hint explicitly, bypassing file-meta-based guessing entirely | Static code reading | Makes triggering the failing lookup trivial: just pass any UID not in the transfer syntax registry |
| Temporary in-tree `#[test]` in `object/src/collector.rs` (kept as a permanent regression test in `patch.diff`, following this file's existing test-naming conventions): `DicomCollector::new_with_ts(reader, "1.2.3.4.not.a.real.ts")`, call `read_dataset_to_end` twice | `cargo test -p dicom-object ts_lookup_panic_repro` | First call returns `Err`, second call panics: `called Option::unwrap() on a None value` at `object/src/collector.rs:451:38` |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`) | `cargo run` | Reproduces the same panic deterministically on the second call; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run (2/2 calls, no fixed seed needed - the panic
depends only on calling the same public method twice after an ordinary unrecognized-UID error,
no adversarial byte content required). Reproduced both via a temporary/now-permanent in-tree test
against the live tree and independently via `repro/` against a pristine clone of the exact commit
this session audits (`df2b9e42`).

## Class

`crash` (panic on `Option::unwrap()`, reachable through this crate's own public
`DicomCollector` API via an entirely ordinary usage pattern: constructing a collector with an
explicit but invalid/unrecognized transfer syntax UID, then calling any of its read methods -
which any caller handling user-supplied transfer syntax strings, or simply retrying after
catching the first error, would hit).

## Root cause

`set_parser_with_ts` takes ownership of the `Option<S>` reader (`src.take().unwrap()`) *before*
the transfer-syntax lookup that can fail, instead of after. On the error path, this leaves the
`Option` empty with no way to put the reader back, silently corrupting `self`'s invariant (`Raw`
variant should always have `reader: Some(_)`) without changing which enum variant `self` is in -
so `has_parser()`'s variant-only check can't detect the corruption, and a second call re-triggers
the same `unwrap()` on the now-permanently-empty `Option`.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Reorder the two statements: perform the transfer syntax lookup (which can fail and return early
via `?`) *before* taking the reader out of `self`, so a failed lookup leaves `self` completely
unchanged and a retry correctly re-runs (and re-fails on) the same lookup instead of panicking:

```rust
// look up transfer syntax BEFORE taking the reader, so that a
// failed lookup leaves self intact and a retry doesn't unwrap None
let ts = ts_index
    .get(ts_uid)
    .context(UnrecognizedTransferSyntaxSnafu { ts_uid: ts_uid.to_string() })?;

let src = src.take().unwrap();
```

See `patch.diff` for the exact diff (scoped to `object/src/collector.rs`; includes the
regression test kept in the `tests` module).

- `cargo build -p dicom-object`: builds cleanly with the fix applied.
- `cargo test -p dicom-object --lib` (against the patched pristine clone): **76 passed / 0
  failed** (75 pre-existing + this candidate's own new regression test), confirming zero
  regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics on the second call before the patch, both calls return a clean
  `Err` after.
