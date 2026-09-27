# CP-246 pseudo-sequence item content unconditionally reinterpreted as a nested element header

Follow-up to `Enet4/dicom-rs#169` (2021-09, still open): "Could not read data set token" reported
on a real CT file (Implicit VR Little Endian) with several `(2001,xxxx)`/`(2005,xxxx)` private,
`UN`-VR, undefined-length elements - dcmtk reads the file fine per CP-246. A prior session (see
`IDEAS.md`'s claim entry for this candidate) already confirmed `parser/src/dataset/read.rs`'s
"treat any other undefined-length element as a sequence, discarding the VR" CP-246 mechanism
existed a full year before the issue was filed (commit `182563a3`, 2020-09-13), so that mechanism
alone could not be the missing piece. The original attached file is unavailable (no network
access to GitHub attachments this session), so this candidate independently constructed a
synthetic file with the same byte shape from first principles.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read `parser/src/dataset/read.rs`'s full undefined-length/CP-246 handling path end to end (the `Ok(header) if header.len.is_undefined()` branch, the `in_sequence`/`seq_delimiters` state machine, `update_seq_delimiters`), plus `encoding/src/decode/implicit_le.rs`'s VR resolution (unknown/private tags -> `VR::UN`), plus `object/src/mem.rs`'s `build_object`/`build_sequence` (which recurse into an item's content by calling `dataset.next()` again, i.e. re-invoking `decode_header()` on whatever bytes are there). | Confirmed the CP-246 conversion (undefined-length element -> pseudo-sequence with one item) is generic and unconditional: once an item is entered, its content is *always* re-parsed as a further sequence of ordinary element headers - there is no code path that treats an item's content as an opaque byte blob unless it's specifically the encapsulated-pixel-data case (`is_encapsulated_pixeldata()`, a different, hardcoded tag/VR check). Nothing distinguishes "this private element's undefined-length item genuinely contains a nested data set" (the crate's own passing test `read_dataset_in_dataset` - a real Philips-style private *sequence*) from "this private element's undefined-length item is just an opaque binary blob with an explicit item length" (real-world CP-246 usage for genuinely non-structured private data, per the standard's own CP-246 wording and matching dcmtk's/GDCM's need for a raw-bytes-vs-nested-dataset guessing heuristic on this exact shape). |
| Standalone repro construction (release build, `dicom_object::InMemDicomObject::read_dataset_with_ts` against the Implicit VR LE registry entry, raw hand-built bytes, no file meta needed) | `___BUGHUNT___/123.../repro`, 4 variations: (A) one private UN undefined-length element `(2005,1080)` whose CP-246 item (explicit length 16) contains opaque binary data (two copies of the IEEE-754 bytes of pi) that happens to decode as a plausible-looking tag+length pair; (C) two private UN undefined-length elements adjacent in a row, matching the issue's literal description; (D) same as A, preceded by an ordinary top-level element; a control case with an all-zero opaque payload (which happens to decode as a chain of valid-looking zero-length `(0000,0000)` elements that exactly fill the item, so it parses "successfully" into nonsense - included to show the bug is payload-content-dependent, not a blanket "CP-246 items never parse" failure) | Variations A, C, and D all failed with **`Error::ReadToken` displaying exactly "Could not read data set token"** - the identical message from the original GitHub issue - confirmed via `{:?}` Debug output to be `ReadValue { len: <huge bogus value>, tag: <garbage tag>, source: ReadValueData { ... UnexpectedEof ... } }`: the item's first 8 opaque bytes were read as a bogus nested element header (tag `(0x2D18, 0x5444)`, declared length `1074340347`), and the reader then attempted to read over 1 GB of "value" from a 40-byte input, hitting `UnexpectedEof` almost immediately. The control (all-zero payload) parsed without error, confirming the effect is a direct function of what bytes happen to be in the private payload, not a structural rejection of CP-246 items in general. |
| Repeat-run check | `cargo run --release` re-run multiple times, same build/input | Fully deterministic every time (no randomness/threading in this code path) - reproduces 100% on demand. |

## Status

**Confirmed** - deterministic, reproduces on every run, release build, no fixed seed needed. This
is judged to be (at minimum, a strong contributing instance of) the real root cause behind issue
`#169`: it reproduces the *exact* reported error message ("Could not read data set token") from
first principles, on a plausible real-world byte shape (private UN, undefined length, CP-246
pseudo-sequence, opaque payload) that the issue's own description matches, using an input that
did not exist as a passing test anywhere in the current suite.

## Class

`logic` (a spurious parse failure on well-formed-per-CP-246 input that other DICOM toolkits, per
the reporter, accept) with a `security`-adjacent side effect (an oversized untrusted length drives
a large pre-EOF allocation attempt - already covered generically by already-Confirmed candidate
004, `untrusted-length-preallocation`, not re-litigated here).

## Root cause

`parser/src/dataset/read.rs`'s CP-246 handling (`Ok(header) if header.len.is_undefined()` branch,
~line 458) converts *any* undefined-length element - regardless of VR - into a pseudo-sequence:
`in_sequence = true`, emit `SequenceStart`. The very next tokens are then read exactly as if this
were a real DICOM sequence: `decode_item_header()` for an `Item`, then (once inside the item)
`decode_header()` again to read the item's *content* as more ordinary element headers
(`object/src/mem.rs`'s `build_object`/`build_sequence` recurse the same way a real nested data set
would be built). This is exactly correct when the private element's content really is a nested
data set (the crate's own `read_dataset_in_dataset` test covers this legitimate case, and it still
passes after this candidate's patch).

But per CP-246 and real-world encoders (this is the exact ambiguity DICOM toolkits like dcmtk and
GDCM have long-standing heuristics for), the identical wire shape is *also* used for private
elements whose content is simply **opaque, unstructured binary data** - with no sub-element
structure at all. dicom-rs has no such heuristic: it unconditionally treats the item's content as
more element headers. When the opaque bytes happen to look like a plausible `(tag, length)` pair
(not unlikely for real pixel-derived or floating-point private data - this candidate's repro shows
it happening with ordinary IEEE-754 float bytes), the derived "length" is nonsense, and the
resulting attempt to read that many value bytes runs past the item's own boundary and typically
past EOF - surfacing only as a generic `UnexpectedEof`/`ReadValue` error, displayed at the
`dicom-object` level as the exact, uninformative "Could not read data set token" from the original
report.

## Introduced by

Not cleanly attributable to a single commit - the CP-246 mechanism itself (the underlying
capability being misused here) was added in commit `182563a3` (2020-09-13, per the prior
session's finding), a full year before the issue was filed, and the mechanism has not
fundamentally changed shape since (the `Ok(header) if header.len.is_undefined()` branch and the
item-recursion in `object/src/mem.rs`'s `build_object`/`build_sequence` were not materially
touched by any single later commit found via a light `git log` skim of `read.rs`/`mem.rs`). This
is best described as a **missing feature** (no opaque-payload heuristic was ever implemented) not
a regression introduced at one point in time.

## The fix (bounded, does not achieve full interoperability)

A genuine fix that makes files like the original report's parse *successfully* would require
implementing a heuristic to distinguish "opaque binary payload" from "real nested data set" inside
a CP-246 item - which is exactly what dcmtk/GDCM do for this same shape, via a non-trivial guess
(read tentatively, validate the resulting tag/VR/length against sane bounds, and fall back to
treating the whole item as a raw `OB` value if the guess looks wrong). That is a substantial,
behavior-changing addition, judged out of scope for a bounded patch in this session.

Instead, this candidate's `patch.diff` adds a narrower, purely defensive improvement:
`parser/src/dataset/read.rs`'s `read_value` now checks, before attempting to read a value, whether
the element's declared length would exceed the number of bytes remaining in its immediately
enclosing item/sequence (only meaningful when that container itself has a defined/explicit
length - real per-file-position accounting already tracked by `seq_delimiters`/`base_offset`, not
new state). If it would, a new, specific `Error::ElementLengthExceedsContainer { tag, len,
remaining }` is returned immediately, instead of letting the read proceed to (a) allocate a buffer
sized from the untrusted bogus length and (b) fail with an opaque, generic I/O error once it hits
EOF.

**This does not make any of the reproduced variations parse successfully** - the file remains
unparseable either way. What changes is purely the *diagnosability* and *boundedness* of the
failure: a caller (or a human debugging a support ticket) now gets `Element (2D18,5444) declares a
value length of 1074340347 bytes, which exceeds the 8 bytes remaining in its enclosing
item/sequence` instead of an opaque `UnexpectedEof` several stack frames deep, and the read no
longer attempts to consume/allocate arbitrarily far past the actual container boundary. Given
candidate 004 already covers the generic "untrusted length -> unbounded allocation" resource-
exhaustion angle for every value reader in the crate, this fix's primary value is the clearer
error, not resource safety.

Verified: the repro's own control case (all-zero opaque payload, which legitimately parses without
error) still parses identically before and after the patch - the new check only fires when a
length genuinely would overrun its container, never for well-formed input.

- Project test suite: `cargo test --workspace --exclude dcms` - **410 passed / 0 failed / 2
  ignored**, identical before and after the patch (byte-for-byte identical per-crate breakdown).
  Zero regressions, including the pre-existing `read_dataset_in_dataset` CP-246 test (a real
  nested private sequence, unaffected since its elements' lengths never exceed their enclosing
  item's bounds).

## What's not done / open questions

- **The original issue is not actually fixed** - files shaped like the reproduced variations still
  fail to parse; only the failure mode improved (clear specific error instead of an opaque one).
  Achieving dcmtk-level interoperability would need the opaque-payload-vs-nested-dataset guessing
  heuristic described above; not attempted here given the scope of a single bounded patch. This
  should be treated as "root cause found and defensively hardened," not "issue resolved."
- The original attached file from `#169` was not available (no network access to GitHub
  attachments), so this candidate's repro is an independent construction that reproduces the exact
  reported error message from first principles, not a byte-for-byte replay of the reporter's own
  file. Given how directly the mechanism (CP-246 pseudo-sequence + opaque payload + coincidental
  garbage tag/length) explains the symptom, this is judged sufficiently convincing without the
  original file, but it's worth flagging that the *real* file's actual failure point was not
  independently verified against this exact code path.
- Not fuzzed this session (a short burst against `object/fuzz/fuzz_targets/open_file.rs` seeded
  near this shape - private undefined-length UN elements with plausible-looking opaque payloads -
  would likely turn up further instances of the same underlying gap; left as a follow-up rather
  than done superficially here, given the root cause is already well understood analytically).
