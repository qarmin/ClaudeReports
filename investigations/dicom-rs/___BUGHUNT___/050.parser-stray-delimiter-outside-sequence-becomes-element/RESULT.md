# parser-stray-delimiter-outside-sequence-becomes-element

`parser/src/dataset/read.rs`'s `DataSetReader` - the state machine every top-level DICOM data set
reader in this crate is built on - has two related gaps in how it handles group `0xFFFE`
structural markers (Item `(FFFE,E000)`, Item Delimitation `(FFFE,E00D)`, Sequence Delimitation
`(FFFE,E0DD)`) when one of them is encountered via the "expect an ordinary element header" state
(i.e. `decode_header()`, not `decode_item_header()` - which is only called while genuinely
iterating a sequence's items). The crate already has an explicit, deliberate exception for a
stray Item Delimitation tag reaching this state (logged and ignored when no sequence is open) -
but no equivalent exception exists for a stray **Item** tag or a stray **Sequence Delimitation**
tag. As a result:

- A bare **Item** `(FFFE,E000)` tag reaching this state (almost always with undefined length,
  since that's how real Items with implicit length are encoded) falls into the generic "any
  element with undefined length is treated as the start of a nested sequence, discarding its VR"
  branch, manufacturing a bogus `SequenceStart` token for a tag that was never a real sequence
  element.
- A bare **Sequence Delimitation** `(FFFE,E0DD)` tag reaching this state (always length 0, a
  *defined* length) has no special-case arm at all and falls into the fully generic "ordinary
  element header" catch-all, becoming a real `DataToken::ElementHeader` and ultimately a real
  top-level `DataElement` in the parsed `InMemDicomObject` - one that carries a reserved tag this
  crate's own encoder cannot represent on the wire (see candidate `049`).

Both gaps let malformed/corrupted input "successfully" parse into an `InMemDicomObject` that is
itself invalid by this crate's own rules, silently, with no error at read time.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing technique (as used for candidate 049): apply candidates 001's and 048's already-confirmed patches to the real tree, then re-run the existing `object/fuzz/fuzz_targets/open_file.rs` | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file -- -max_total_time=150` (150s) | A new crash, distinct from 001/004/048/049: a panic at the fuzz target's `.expect("writing DICOM file should always be successful")`, wrapping `ReservedTag { tag: Tag(0xFFFE, 0xE0DD), .. }` - i.e. candidate 049's write-side guard correctly refusing to encode an element that should never have existed in the parsed object in the first place |
| `cargo +nightly fuzz tmin` to minimize the crashing input | Default tmin settings | 324-byte minimized input, could not be reduced further by tmin |
| Standalone diagnostic (dedicated `dicom-core`/`dicom-object` path-dep binary) parsing the minimized input and printing the full `FileDicomObject` | debug build, with 001+048+049's patches applied | Confirmed the parsed object contained a stray top-level `DataElement { header: DataElementHeader { tag: Tag(0xFFFE, 0xE0DD), vr: UN, len: Length(0) }, value: Primitive(Empty) } }`, matching the panic exactly |
| Manual byte-offset replay of `object/src/meta.rs`'s file-meta-table loop against the raw 324-byte input (Python script mirroring its exact `while total_bytes_read < group_length` accounting, including its already-correct group-0xFFFE bare-tag-plus-length special case inherited from `ExplicitVRLittleEndianDecoder::decode_header`) | N/A | Traced precisely where meta parsing hands off to the main data set: the file's declared `information_group_length` (168) doesn't land on an element boundary once a garbled off-group tag is encountered mid-meta (itself handled gracefully by `meta.rs`'s existing "unexpected off-group tag" fallback, not a bug), so meta parsing actually consumes 172 bytes and hands off the last 8 bytes - `fe ff dd e0 00 00 00 00`, a bare Sequence Delimitation Item - to the main `DataSetReader` as if it were the start of the real data set |
| Minimal, fuzzer-independent reproduction: fed two small, hand-built byte sequences directly into `InMemDicomObject::read_dataset_with_ts` (explicit VR LE) - (a) a bare Item with undefined length followed by 3 more bare delimiter-shaped byte groups (mirroring the structural shape of the original crash's tail), and (b) a bare Sequence Delimitation Item on its own | Constructed directly via the public API, no fuzzer or file meta table involved | (a) produced **two** bogus top-level elements: an empty `SQ`-tagged sequence for `(FFFE,E000)` and a plain element for the final `(FFFE,E0DD)`; (b) alone produced the single bogus plain element for `(FFFE,E0DD)`. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any input where a bare Item or Sequence Delimitation tag reaches the data set reader's
"expect an ordinary element header" state with no sequence currently open (typically: right at
the very start of the data set, immediately after file meta parsing, if the meta group's declared
length doesn't land exactly where a real implementation would expect, or generally anywhere
between elements at the top level of a malformed file).

## Class

`logic` (parses malformed input into a data structure that violates the crate's own invariants
without raising an error). It is closely coupled to candidate `049`: 049 hardened the *write*
side so that this invalid state can no longer be silently re-serialized into equally-invalid
bytes; this candidate closes the *read* side gap that let the invalid state be constructed in the
first place. Note this candidate's fix is independent of 049's and would still be worth applying
even without 049 in place - a `DataElement` with a reserved tag violates this crate's own
documented conventions regardless of whether writing it back out happens to succeed or fail.

## Root cause

`parser/src/dataset/read.rs`'s `Iterator::next` implementation for `DataSetReader` has a large
match on `self.parser.decode_header()`'s result, used whenever an ordinary element header or a
delimiter is expected (as opposed to `decode_item_header()`, used only while genuinely iterating
a sequence's items). This match already has two dedicated arms carving out an exception for a
stray Item Delimitation tag `(FFFE,E00D)` reaching this state with no sequence open (logs a
warning and ignores it, since decode_header() correctly decodes its bare tag+length shape but the
higher-level state machine has no other slot for it to fit into). No equivalent arms exist for:

- `(FFFE,E000)` (Item): falls through to `Ok(header) if header.len.is_undefined()`, a generic
  branch whose comment literally says "treat other undefined length elements as data set
  sequences, discarding the VR in the process" - true and useful for e.g. old non-standard
  encodings of `SQ` without an explicit `VR::SQ` marker, but wrong for an actual Item marker,
  which should never be interpreted as anything other than what it is.
- `(FFFE,E0DD)` (Sequence Delimitation): has a *defined* length (always 0), so it doesn't match
  the "undefined length" branch either, and falls all the way through to the fully generic
  `Ok(mut header) => { ...; DataToken::ElementHeader(header) }` catch-all - the same code path
  used for any ordinary, non-special tag.

Both are then processed exactly like real data: an `ElementHeader`/`SequenceStart` token feeds
into `object`'s `InMemDicomObject` builder, which has no reason to suspect these tags are
special, and dutifully inserts them.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added two dedicated match arms to `parser/src/dataset/read.rs`'s `DataSetReader::next`, placed
before the generic `is_undefined()`/catch-all branches, mirroring the existing Item Delimitation
exception:

```rust
Ok(DataElementHeader {
    tag: Tag(0xFFFE, 0xE0DD),
    ..
}) if self.seq_delimiters.is_empty() => {
    // ignore delimiter, we are not in a sequence - same treatment as a
    // stray item delimiter above.
    tracing::warn!(
        "Sequence delimitation item outside of a sequence in position {}",
        self.parser.position()
    );
    continue;
}
Ok(DataElementHeader {
    tag: tag @ Tag(0xFFFE, 0xE000),
    ..
}) => {
    // An Item marker is only ever valid where an item header is expected
    // (decoded via `decode_item_header` while iterating a sequence's
    // items), never here.
    self.hard_break = true;
    Some(UnexpectedItemTagSnafu { tag }.fail())
}
```

The Item case is rejected outright (there is no legitimate way for a bare Item marker to reach
this state - it reuses the crate's existing `UnexpectedItemTag` error, already used for the
analogous situation inside encapsulated pixel data parsing). The Sequence Delimitation case
mirrors the existing Item Delimitation exception's leniency (log + ignore) rather than erroring,
matching the crate's own established precedent for this specific tag; see "What's not done" below
for the one case this leniency doesn't cover.

See `patch.diff` for the exact diff (fix only; the two accompanying unit tests added during
investigation - `read_stray_item_outside_sequence` and
`read_stray_sequence_delimiter_outside_sequence` - are not included, matching this session's
convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-parser --lib`: baseline (both tests present, fix removed) - both new tests
  **FAILED** with exactly the predicted wrong tokens (`SequenceStart { tag: Tag(0xFFFE, 0xE000),
  len: Length(Undefined) }` and `ElementHeader(DataElementHeader { tag: Tag(0xFFFE, 0xE0DD), vr:
  UN, len: Length(0) })`) -> with the fix restored - **51 passed, 0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **353 passed / 0
  failed / 2 ignored** with this fix + its 2 tests applied - zero regressions anywhere in the
  existing suite.
- Confirmed against the original fuzz-found 324-byte crash input directly: before the fix, the
  fuzz target's `.expect("writing DICOM file should always be successful")` panics; after the
  fix, a clean, single-shot replay (`cargo +nightly fuzz run open_file <input>`) reports no crash.

## What's not done / open questions

- The Sequence Delimitation exception added here only covers the case matching the existing Item
  Delimitation precedent: `self.seq_delimiters.is_empty()` (nothing open at all). A Sequence
  Delimitation reaching this same "expect element header" state while `self.seq_delimiters` is
  *non-empty* (meaning we're inside some item's content, which hasn't yet been closed by its own
  Item Delimitation) was not observed in this session's fuzzing and has no test covering it; by
  the same reasoning used for the Item Delimitation precedent, it likely also indicates malformed
  input (a missing/skipped Item Delimitation) and could plausibly be handled by additionally
  closing the open item (`SequenceEnd` instead of just ignoring), but this was left unimplemented
  since no concrete failing input demonstrates it's reachable or what the "correct" recovery
  should look like - flagged here for a future session rather than guessed at.
- This fix targets `parser/src/dataset/read.rs`'s `DataSetReader` specifically, since that's what
  `object`'s `InMemDicomObject`/`FileDicomObject::from_reader` builds on and what the fuzz target
  exercises. `parser/src/dataset/lazy_read.rs`'s `LazyDataSetReader` is a separate, independent
  token-generation implementation (as previously found to be the case for the charset-leak bug in
  candidates 006/007) and was not audited for the same gap in this session.
- The underlying reason the fuzzer's input reached this state at all - a file meta group whose
  declared `information_group_length` doesn't land on an element boundary once an off-group tag
  appears mid-meta - is itself handled gracefully by `object/src/meta.rs`'s existing "unexpected
  off-group tag" fallback (see the byte-level trace in `backtrace.txt`); that behavior was
  reviewed and is not itself a bug, just the specific way this session's fuzz input happened to
  arrive at the data set boundary early.
