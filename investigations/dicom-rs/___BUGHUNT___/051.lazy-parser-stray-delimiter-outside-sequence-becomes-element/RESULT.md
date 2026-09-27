# lazy-parser-stray-delimiter-outside-sequence-becomes-element

`parser/src/dataset/lazy_read.rs`'s `LazyDataSetReader` - a separate, independent, hand-written
token-generation implementation from `parser/src/dataset/read.rs`'s `DataSetReader` (used by
`DicomCollector` and any other caller that wants to skip or lazily read values instead of eagerly
building an `InMemDicomObject`) - independently duplicates the exact same gap that candidate `050`
found and fixed in `read.rs`: no exception for a stray Item `(FFFE,E000)` or stray Sequence
Delimitation `(FFFE,E0DD)` tag reaching the "expect an ordinary element header" state
(`decode_header()`, as opposed to `decode_item_header()` - only called while genuinely iterating a
sequence's items). This is the same pattern previously seen with candidates `006`/`007` (a
character-set scoping bug independently present in both readers, since they are two entirely
separate implementations of the same token-generation contract) - worth checking as a matter of
course whenever a bug is found in one of `read.rs`/`lazy_read.rs`.

Additionally, `lazy_read.rs`'s existing Item Delimitation `(FFFE,E00D)` handling (the one case it
did already special-case) was *unconditional* - unlike `read.rs`'s equivalent, which only pops its
delimiter stack and continues normally when `self.seq_delimiters.is_empty()` (nothing open) is
false. `lazy_read.rs`'s version pops regardless, silently getting `None` back from an empty stack,
and unconditionally flips `in_sequence = true` and emits an `ItemEnd` token even when nothing was
actually open. This is a real (if more subtle) behavioral gap, but this session did not observe
it causing wrong end-user output beyond a spurious extra token, and it was left alongside the
other two fixes only implicitly - see "What's not done" for details.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code audit, following candidates 006/007's precedent ("a bug found in `read.rs` is worth checking for independently in `lazy_read.rs`, since the two are separate implementations") | Read `parser/src/dataset/lazy_read.rs`'s equivalent `advance()` match statement | Confirmed the exact same two missing-arm gaps candidate 050 found in `read.rs`: no exception for `(FFFE,E000)` (falls into the "undefined length -> treat as SQ" branch) and no exception for `(FFFE,E0DD)` (falls into the generic element-header catch-all) |
| Minimal, hand-built byte-sequence reproduction directly against `LazyDataSetReader::new_with_ts` + `.advance()` (explicit VR LE), reusing the same two byte sequences from candidate 050's repro | Constructed directly via the public API | (a) "stray Item followed by delimiters" (`Item(undef) + SeqDelim(0) + ItemDelim(0) + SeqDelim(0)`) produced a bogus `SequenceStart { tag: Tag(0xFFFE, 0xE000), .. }` token; (b) "stray SequenceDelimiter" alone produced a bogus `ElementHeader(DataElementHeader { tag: Tag(0xFFFE, 0xE0DD), .. })` token. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
under the same conditions as candidate 050 (a stray Item or Sequence Delimitation tag reaching
`LazyDataSetReader`'s "expect an ordinary element header" state with no sequence currently open),
for any caller of `LazyDataSetReader` directly or via `DicomCollector`.

## Class

`logic` (same class as candidate 050: parses malformed input into tokens that violate the crate's
own invariants without raising an error). Independent of, but structurally identical to, candidate
050 - found by directly checking the sibling implementation once 050 was already fixed, rather
than via a new fuzz crash.

## Root cause

Same underlying cause as candidate 050, independently duplicated: `lazy_read.rs`'s `advance()`
match on `self.parser.decode_header()`'s result has no dedicated arm for a bare Item marker
`(FFFE,E000)` reaching this state (it falls into the generic "undefined length -> treat as a
nested sequence, discard the VR" branch) nor for a bare Sequence Delimitation Item `(FFFE,E0DD)`
(it has a *defined* length, so it doesn't match that branch either, and falls through to the fully
generic `Ok(mut header) => { ...; LazyDataToken::ElementHeader(header) }` catch-all). Unlike
`read.rs` prior to candidate 050's fix, `lazy_read.rs` did not even have the Item Delimitation
`(FFFE,E00D)` exception guarded by `self.seq_delimiters.is_empty()` - its one existing special
case for that tag runs unconditionally.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Mirrors candidate 050's fix exactly, adapted to this module's types (`LazyDataToken` instead of
`DataToken`, and a new `UnexpectedItemTag` error variant added to `lazy_read::Error` since no
equivalent already existed here - `read.rs` already had one, reused for its own fix). Two new
match arms were added, and the existing (previously unconditional) Item Delimitation arm was made
conditional on `self.seq_delimiters.is_empty()`, exactly matching `read.rs`'s already-established
precedent:

```rust
Ok(DataElementHeader {
    tag: Tag(0xFFFE, 0xE00D),
    ..
}) if self.seq_delimiters.is_empty() => {
    tracing::warn!(...);
    continue;
}
Ok(DataElementHeader {
    tag: Tag(0xFFFE, 0xE0DD),
    ..
}) if self.seq_delimiters.is_empty() => {
    tracing::warn!(...);
    continue;
}
Ok(DataElementHeader {
    tag: tag @ Tag(0xFFFE, 0xE000),
    ..
}) => {
    self.hard_break = true;
    Some(UnexpectedItemTagSnafu { tag }.fail())
}
```

Since `advance()` (unlike `read.rs`'s `Iterator::next`) was not already structured as a `loop`
around this match, the whole match was wrapped in one (`loop { return match ... { ... } }`) so
that `continue` could be used for the two "ignore and retry" cases without recursing - a plain
recursive call (`return self.advance()`) was considered and rejected during this investigation,
since a long run of consecutive stray delimiters in adversarial input would otherwise grow the
call stack once per delimiter, reintroducing exactly the kind of unbounded-resource-use defect
this bug-hunt session has been finding and fixing elsewhere (e.g. candidates 004, 013, 015).

See `patch.diff` for the exact diff (fix only; the two accompanying unit tests added during
investigation - `lazy_read_stray_item_outside_sequence` and
`lazy_read_stray_sequence_delimiter_outside_sequence` - are not included, matching this session's
convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-parser --lib`: baseline (both tests present, fix removed) fails to even
  **compile**, since the tests reference `super::Error::UnexpectedItemTag`, a variant that doesn't
  exist without the fix (`error[E0599]: no variant named 'UnexpectedItemTag' found for enum
  'lazy_read::Error'`) - a stronger baseline signal than a mere test failure. With the fix restored
  - **51 passed, 0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **353 passed / 0
  failed / 2 ignored** with this fix + its 2 tests applied - zero regressions anywhere in the
  existing suite.

## What's not done / open questions

- The Item Delimitation `(FFFE,E00D)`-with-non-empty-`seq_delimiters` case, and the equivalent
  question for Sequence Delimitation, carry over unresolved from candidate 050 (see that
  candidate's own "What's not done" section) - not re-litigated here since the reasoning is
  identical.
- This fix only restructures `lazy_read.rs`'s existing Item Delimitation handling into the
  conditional form to close the "unconditional pop on an empty stack" gap noted above as a direct
  side effect of mirroring `read.rs`'s already-established pattern; it was not separately
  fuzzed or exercised with a dedicated test beyond what the two new tests already cover
  incidentally (both hit the `is_empty()` branch, not a case where `seq_delimiters` is non-empty
  at this call site).
- As with candidate 050, the underlying reason a stray delimiter would ever reach this state in
  a real file (a file meta group whose declared length doesn't land on an element boundary) was
  root-caused there and not re-investigated here; this candidate isolates the parser-level defect
  directly, independent of file meta parsing.
