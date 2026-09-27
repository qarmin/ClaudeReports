# parser-encode-reserved-tag-group-write-corruption

`parser/src/stateful/encode.rs`'s `StatefulEncoder::encode_element_header` - the single shared
entry point used by every transfer-syntax encoder (explicit VR LE/BE, implicit VR LE, deflated,
etc.) to write an ordinary data element's header - has no guard against being called with a tag
in the reserved group `0xFFFE` (Item `(FFFE,E000)`, Item Delimitation `(FFFE,E00D)`, Sequence
Delimitation `(FFFE,E0DD)`). Those three tags are reserved, by the DICOM standard, exclusively for
structural markers, and are **always** encoded as a bare 4-byte tag + 4-byte length (8 bytes
total, no VR field) in every transfer syntax - even ones that otherwise always encode VR
explicitly. If `encode_element_header` is asked to write an ordinary data element whose tag
happens to fall in that group, it writes the normal explicit-VR shape instead (tag + 2-byte VR +
2 reserved bytes + 4-byte length), producing bytes that are - byte-for-byte - indistinguishable
from a delimiter marker with a garbage length. The crate's own reader, encountering a group-0xFFFE
tag, always takes the "this is a bare delimiter" code path and misreads the VR text and reserved
bytes as the start of the length field, producing nonsense and then failing with `UnexpectedEof`/
"Could not read data set token" once it tries to skip the (nonexistent) garbage-length payload.
The net effect: **the crate can write files that its own reader cannot parse back**, with no error
raised at write time.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing technique: apply already-confirmed-and-patched fixes for candidates 001 (`___BUGHUNT___/001.../patch.diff`) and 048 (`___BUGHUNT___/048.../patch.diff`) temporarily to the real tree, then re-run the existing, official `object/fuzz/fuzz_targets/open_file.rs` harness - the idea being that shallower, easier-to-hit known bugs mask deeper ones from the fuzzer's coverage-guided search | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file -- -max_total_time=150` (150s) with both patches applied | A genuinely new crash, distinct from 001/004/048, found within the 150s budget |
| `cargo +nightly fuzz tmin` to minimize the crashing input | Default tmin settings | 324-byte minimized input, could not be reduced further by tmin |
| Standalone diagnostic (`/tmp/diag3`, `dicom-core`/`dicom-object` path deps) parsing the minimized input and printing its `DataElement`s | debug build | Isolated the failure to a `DataElement { header: DataElementHeader { tag: Tag(0xFFFE, 0xE0DD), vr: UN, len: Length(0) }, value: Primitive(Empty) }` that round-trips incorrectly |
| Minimal, fuzzer-independent reproduction: write a `VR::UN`/`PrimitiveValue::Empty` element via `InMemDicomObject::write_dataset_with_ts` (explicit VR LE) for an ordinary tag and for each of the three group-0xFFFE tags, then attempt `InMemDicomObject::read_dataset_with_ts` on the resulting bytes | Constructed directly via the public API, no adversarial input needed | Ordinary tag round-trips correctly (12 bytes, `09 00 01 00 55 4e 00 00 00 00 00 00`). All three reserved tags "write successfully" (12 bytes each, e.g. `fe ff 00 e0 55 4e 00 00 00 00 00 00` for `(FFFE,E000)`) but fail to read back with "Could not read data set token". Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` (see `backtrace.txt`) |
| Manual byte-level root-cause analysis of the written bytes | N/A | Confirmed the reader misreads the VR text bytes `55 4e` ("UN") plus the 2 reserved zero bytes as a little-endian length field: `0x00004e55` = 20053 - exactly matching the original fuzz crash's observed garbage length, proving the same mechanism in both cases |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any attempt to write an ordinary (non-delimiter) data element whose tag happens to fall in
group `0xFFFE`, regardless of transfer syntax (checked with explicit VR LE; the same
`encode_element_header` entry point is shared by every other transfer-syntax encoder in this
crate, all of which are equally unguarded).

## Class

`logic` (a write/read round-trip asymmetry: the writer silently produces output that violates the
DICOM wire format's own reserved-tag convention, and that same crate's reader cannot parse the
result back). It surfaces to a caller as a downstream `UnexpectedEof`/parse error when the
corrupted bytes are later read (by this crate or any other DICOM implementation), but the actual
defect - a validation gap at write time - is a correctness/logic issue, not a memory-safety
crash. In real-world terms this requires either a program that lets attacker/user-controlled
tag values reach the encoder unchecked, or a private/non-standard tag that happens to collide with
`0xFFFE`'s reserved group (0xFFFE is entirely reserved by the standard for this purpose, so any
tag in that group reaching the encoder as an ordinary element is already a sign something upstream
went wrong - but the encoder should fail loudly rather than silently emit unparseable bytes).

## Root cause

DICOM reserves group `0xFFFE` exclusively for three structural markers - Item, Item Delimitation
Item, and Sequence Delimitation Item - and mandates that these are always encoded as a bare
tag + 4-byte length, with **no VR field**, in every transfer syntax (this is why a decoder can
always tell a delimiter apart from a normal element: it checks the tag's group first, before
deciding whether to expect a VR field). `parser/src/stateful/encode.rs`'s
`encode_element_header` is the generic, shared function that every transfer-syntax encoder in this
crate calls to write a normal data element's header (VR field included, per that transfer
syntax's rules). It had no check preventing a caller from passing it a `DataElementHeader` whose
tag is in group `0xFFFE` - which is possible today since `DataElementHeader`/`DataElement` place
no restriction on `tag` at construction time, and callers (such as `object`'s `InMemDicomObject`,
reachable via ordinary safe public API like `DataElement::new` + `.put()` + `write_dataset_with_ts`)
have no way to know this restriction exists. The dedicated methods this crate uses internally for
writing real delimiters (which do produce the correct bare tag+length shape) are separate
functions entirely, unaffected by and untouched by this bug or its fix.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a `ReservedTag` error variant and a guard at the very top of `encode_element_header`,
before any bytes are written, rejecting any header whose tag falls in group `0xFFFE`:

```rust
use snafu::{Backtrace, OptionExt, ResultExt, Snafu, ensure};
...
    /// Tags in group 0xFFFE (Item, Item Delimitation, Sequence Delimitation)
    /// are reserved for those structural markers and are always encoded
    /// without a VR field, even in transfer syntaxes that otherwise encode
    /// VR explicitly - so a data element header with a tag in this group can
    /// never be represented on the wire without becoming indistinguishable
    /// from (or colliding with) one of those markers when read back.
    #[snafu(display(
        "Cannot encode a data element header with reserved tag {} (group 0xFFFE is reserved for item/delimiter markers)",
        tag
    ))]
    ReservedTag { tag: Tag, backtrace: Backtrace },
...
    pub fn encode_element_header(&mut self, mut de: DataElementHeader) -> Result<()> {
        ensure!(de.tag.group() != 0xFFFE, ReservedTagSnafu { tag: de.tag });
        if let Some(len) = de.len.get() {
            de.len = Length(even_len(len))
        }
        ...
```

See `patch.diff` for the exact diff (fix only, test excluded - the test lives only in the
in-tree working copy during investigation, matching this session's convention of keeping
`patch.diff` minimal).

Verified with a dedicated unit test (`encode_element_header_with_reserved_tag_fails`, added
in-tree during investigation but not included in `patch.diff`) asserting all three reserved tags
fail to encode while an ordinary tag still succeeds, plus the standalone `repro/` reproduction
(`backtrace.txt`): before the fix, writing any of the three reserved tags "succeeds" but produces
bytes the reader can't parse back; after the fix, the write call itself returns
`Err(Error::ReservedTag { .. })` immediately, and the ordinary tag continues to round-trip
correctly in both cases.

- `cargo test -p dicom-parser --lib`: baseline (error variant + test present, guard removed) -
  **49 passed, 1 failed** (the new test, as expected) -> with the guard restored - **50 passed,
  0 failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **352 passed / 0
  failed / 2 ignored** with this fix + its test applied - zero regressions anywhere in the
  existing suite.

## What's not done / open questions

- The fix rejects the write outright rather than allowing it in some safe alternate encoding -
  there is no valid way to put an ordinary VR-bearing data element on the wire with a group-0xFFFE
  tag in DICOM's wire format at all, so failing loudly is the only correct behavior; this was not
  a judgment call between multiple valid fixes.
- Whether any legitimate, spec-compliant use case exists for a non-delimiter element to carry a
  group-0xFFFE tag was not found during this investigation (the DICOM standard reserves the entire
  group for the three delimiter markers), so no allowlist/exception was added.
- This candidate's guard lives in `encode_element_header` specifically, the one shared entry point
  identified for ordinary element writes; a broader audit of whether any other encoder path in
  this crate (e.g. any transfer-syntax-specific override that might bypass this shared function)
  could also construct a group-0xFFFE header without going through this guard was not performed
  beyond confirming (via the full workspace test suite and this crate's own existing delimiter-
  writing tests) that legitimate Item/Delimiter writes are unaffected.
- The reader side was not changed and does not need to be: its group-0xFFFE-means-bare-delimiter
  assumption is correct per the standard; the bug was entirely on the writer's side allowing that
  assumption to be violated in the first place.
- This was found by temporarily stacking two already-confirmed, already-patched fixes (candidates
  001 and 048) onto the real tree before re-running the project's own existing fuzz target - a
  technique worth reusing in future sessions, now documented in `projects/dicom-rs.md`.
