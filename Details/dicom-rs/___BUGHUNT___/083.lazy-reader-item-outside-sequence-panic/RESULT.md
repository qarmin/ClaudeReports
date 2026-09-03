# lazy-reader-item-outside-sequence-panic

`parser/src/dataset/lazy_read.rs`'s `LazyDataSetReader::advance()` panics via an `.expect()` when
a malformed stream contains a top-level Item Delimitation Item `(FFFE,E00D)` (no sequence or item
actually open) immediately followed by an Item marker `(FFFE,E000)`. Flagged from an unmerged
branch (`origin/fix/lazy-reader-item-outside-sequence`, commit `eab55c6d`) with the caveat to check
it against candidate `051` (a related, already-recorded but *not-applied-to-master* fix for a
different symptom - bogus tokens, not a panic - in the same function) before concluding it's
distinct.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code audit of `LazyDataSetReader::advance()`'s `Tag(0xFFFE, 0xE00D)` arm and the `.expect()` at the item-header-push site, cross-checked against candidate 051's `RESULT.md`/`patch.diff` and against `read.rs`'s already-merged equivalent fix (candidates 050) | Read `parser/src/dataset/lazy_read.rs` at current master (`df2b9e42`) | Confirmed the exact two-part gap described in the branch: the `(FFFE,E00D)` arm unconditionally sets `in_sequence = true` and pops the (possibly empty) `seq_delimiters` stack; the subsequent item-header branch does `self.seq_delimiters.last().expect(...)`, panicking once `in_sequence` is true but the stack is empty - exactly the state the first arm leaves behind when hit at the top level. Also confirmed candidate 051's diagnosis/fix (`051.../patch.diff`) was never applied to master (`UnexpectedItemTag` doesn't exist in the current tree) - so this candidate is examined against genuinely pristine code, not an already-patched one. |
| Minimal hand-built byte-sequence reproduction directly against `LazyDataSetReader::new_with_ts` + `.advance()` (explicit VR LE): a bare `(FFFE,E00D)` len 0 at the top level, then a bare `(FFFE,E000)` item header | Constructed directly via the public API, both a zero-length and a non-zero (defined length 4, with matching payload bytes) item-header variant | Both variants panic identically: `thread 'main' panicked at .../lazy_read.rs:377:38: item header should be read only inside an existing sequence`. Reproduced deterministically: 3/3 runs directly in the main checkout (debug build) + 1/1 in `--release` + re-confirmed via `just test`'s `reproduce-before` step against a fresh clone of pristine upstream `df2b9e42` (see `output.txt`) |

## Status

**Confirmed**. Deterministic - reproduces on every run (3/3 direct + release build + fresh-clone
run via `just test`), for any caller of `LazyDataSetReader` directly or via `DicomCollector`, given
this exact two-tag input shape.

Relationship to candidate 051: distinct in symptom (panic here vs. a bogus reserved-tag token
there), but **the same root cause** - both stem from the `(FFFE,E00D)` arm at the top level of
`advance()`'s "expect an ordinary element header" match not checking `self.seq_delimiters.is_empty()`
before mutating state. Candidate 051's recorded fix (never applied to master) guards that exact arm
with `if self.seq_delimiters.is_empty()` and, in that guarded branch, simply warns and ignores the
stray tag instead of setting `in_sequence = true` - which would have *also* closed this candidate's
panic as a side effect, since `in_sequence` would never flip to `true` with an empty stack in the
first place. This candidate's own fix (below) instead converts both connected spots (the `(FFFE,E00D)`
arm and the `.expect()` site) into explicit errors, following the unmerged branch's own diagnosis
rather than 051's warn-and-ignore approach - functionally overlapping with 051 at the first spot,
but additionally closing the second spot (the `.expect()`) directly and defensively, in case any
other code path (not found in this session) can also reach it with an empty stack.

**Fully reproducible**: **Yes**. The malformed byte sequence is exactly the kind of external input
a real DICOM file's data set could contain (a corrupt/truncated file, or a deliberately malformed
one) and is fed straight into `LazyDataSetReader`'s real public API (`new_with_ts` + `advance()`),
the same entry point `DicomCollector` and any other lazy-reading caller uses - no second participant
is being faked here (this is the "malformed file handed to a parser" case `CONFIRMATION.md` section
11 calls out as not needing two communicating participants).

## Class

`crash` (a reachable `.expect()` panic on malformed but externally-suppliable input, not a memory-
safety issue).

## Root cause

`LazyDataSetReader::advance()`'s "expect a data element header or item delimiter" match arm for
`Tag(0xFFFE, 0xE00D)` (item delimitation item) runs unconditionally:

```rust
Ok(DataElementHeader { tag: Tag(0xFFFE, 0xE00D), .. }) => {
    self.in_sequence = true;
    // pop item delimiter
    self.seq_delimiters.pop();
    self.delimiter_check_pending = true;
    Some(Ok(LazyDataToken::ItemEnd))
}
```

When this tag is seen with nothing actually open (`seq_delimiters` empty - e.g. right at the start
of the data set, or after everything legitimately open has already been closed), `.pop()` is a
silent no-op on the empty `Vec`, but `in_sequence` is still flipped to `true`. On the next call to
`advance()`, `self.in_sequence` being `true` routes the reader into the "expecting item header"
branch, which calls `self.parser.decode_item_header()`. If the next bytes decode as a plain Item
marker `(FFFE,E000)` (`SequenceItemHeader::Item { len }`, not a delimiter), the code then does:

```rust
self.push_sequence_token(
    SeqTokenType::Item,
    len,
    self.seq_delimiters.last()
        .expect("item header should be read only inside an existing sequence")
        .pixel_data,
);
```

`self.seq_delimiters.last()` is `None` (the stack is still empty - nothing ever pushed anything
onto it), so `.expect()` panics.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Applied the unmerged branch's own fix (`eab55c6d`, `parser/src/dataset/lazy_read.rs`) directly,
unmodified in shape: converts both connected spots into proper `snafu` errors instead of
panicking/silently corrupting state.

1. The `(FFFE,E00D)` arm now checks the result of `self.seq_delimiters.pop()`; if it returns
   `None` (nothing was open), it sets `hard_break = true` and returns the already-existing
   `UnexpectedItemDelimiter` error instead of setting `in_sequence = true`.
2. The item-header branch now matches on `self.seq_delimiters.last()` instead of `.expect()`-ing
   it; if `None`, it sets `hard_break = true` and returns a new `UnexpectedItem` error instead of
   panicking.

Verified against both repro variants (zero-length and non-zero-length item after the stray
delimiter): both now print `rejected with a clean parse error: Unexpected item delimiter at 8
bytes` and exit cleanly, no panic, confirmed via a direct re-run in the main checkout and via
`just test`'s `reproduce-after` step against a fresh patched clone (see `output.txt` for the
before side; the after side was observed live during verification and is not separately saved per
this session's convention of keeping `patch.diff` and `output.txt` focused on the failing case).

- `cargo test -p dicom-parser --lib`: 49 passed / 0 failed, unchanged before and after (no existing
  test exercised this path either way, so no test failed before, and none regressed after).
- `cargo test --workspace --lib` (all 12 crates with a `#[lib]` target): **351 passed / 0 failed /
  2 ignored**, identical before and after the fix - zero regressions anywhere in the existing
  suite.

## What's not done / open questions

- No unit test was added to the source tree for this fix (per this session's convention of keeping
  `patch.diff` minimal and reproductions in `repro/`), unlike the unmerged branch's own commit,
  which didn't include tests either.
- Candidate 051's separate symptom (a stray `(FFFE,E00D)`/`(FFFE,E0DD)` with a *non-empty*
  `seq_delimiters` stack behaving oddly, and the missing dedicated arm for a stray `(FFFE,E000)`/
  `(FFFE,E0DD)` reaching the generic element-header catch-all) is **not** fixed by this patch - it
  remains exactly as recorded in candidate 051, still not applied to master. This candidate only
  closes the two spots the unmerged branch's diagnosis targeted (the empty-stack case for
  `(FFFE,E00D)`, and the `.expect()` panic it can trigger).
- Whether any other code path in `advance()` can reach the `.expect()` site (now a checked
  `match`) with an empty stack besides the one fixed here was not exhaustively searched; the fix
  is defensive at that exact site regardless of how it's reached.
- As with candidates 050/051, the underlying reason a stray delimiter would ever reach this state
  in a real file (a file meta group or preceding element whose declared length doesn't land on an
  element boundary) was not re-investigated here.
