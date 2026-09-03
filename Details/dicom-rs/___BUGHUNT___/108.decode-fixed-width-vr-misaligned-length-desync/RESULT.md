# decode-fixed-width-vr-misaligned-length-desync

Follow-up on `IDEAS.md`'s backlog entry for the unmerged `origin/fix/charset-roundtrip` (`43b795c8`)
branch: "diff this branch's decode.rs/tokens.rs hunks specifically against current master since
078 only touched encode.rs". Doing that diff found the branch's `decode.rs` hunk bundles several
unrelated changes beyond the charset-VR-gating already independently confirmed by candidate 002
(decode's `read_value_strs`/`read_value_str` VR-exclusion list) and candidate 052
(`decode_text_trap`'s separator-injection bug in `encoding/src/text.rs`). One of those bundled,
non-charset changes - a `require_known_length_multiple_of` helper the branch introduces and wires
into `read_value_tag`/`read_value_ss`/`read_value_fl`/`read_value_ul`/`read_value_us`/
`read_value_uv`/`read_value_sl`/`read_value_sv`/`read_value_od` - describes a real, distinct,
still-unfixed defect on current master: every one of those functions computes its element count
via a bit shift (`len >> k`) that silently floors when the header's declared byte length isn't a
multiple of the VR's fixed element size, without adjusting how many bytes are actually consumed
from the underlying stream. This is the exact same bug class as already-confirmed candidate 068
(`parser-offset-table-misaligned-length-desync`), but 068's own fix was scoped only to
`read_u32_to_vec` (the encapsulated PixelData Basic Offset Table reader) - it does not cover any
of these 9 general fixed-width-VR value readers, which are reachable for any ordinary element of
VR `AT`/`SS`/`FL`/`UL`/`US`/`UV`/`SL`/`SV`/`OD` in any data set, not just the offset table.

(The branch's other two changes - decode.rs's mid-stream `(0008,0005)` VR-loosening for
`read_value`/`read_value_preserved`, and tokens.rs's `IntoTokens` reordering to emit
`(0008,0005)` first - were checked separately and are **not** adopted here: the decode.rs one
would reintroduce the exact charset encode/decode asymmetry candidate 078 already fixed the other
way, since master's encoder now strictly requires `VR::CS` to switch charsets while this branch's
decode change would make the decoder switch for any string VR; see "What's not done" below.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Diff `43b795c8`'s `decode.rs`/`tokens.rs` hunks against master (`df2b9e42`) | `git diff 401be47d 43b795c8 -- parser/src/stateful/decode.rs object/src/tokens.rs` | Found 3 independent changes bundled together: (a) the length-alignment helper described above, (b) a decode-side VR-loosening for the `(0008,0005)` charset switch, (c) `tokens.rs`'s `IntoTokens` reordering. Cross-checked (a) against candidates 002/052/068's own `RESULT.md`s - confirmed none of them cover the general fixed-width VR readers |
| Code reading (confirm the mechanism against master's actual `read_value_us`/`read_value_ul`/etc.) | Read `parser/src/stateful/decode.rs`'s 9 affected functions directly | Confirmed each does `n = len >> k; ...decode_into(&mut self.from, ...)` (reads exactly `n * elem_size` bytes from the stream) followed by `self.position += len as u64` (advances bookkeeping by the *declared*, possibly-larger `len`) - identical mechanism to 068's `read_u32_to_vec`, just duplicated across 9 more functions |
| Standalone diagnostic (dedicated `dicom-object`/`dicom-core`/`dicom-transfer-syntax-registry` path-dep binary, public API only) | Hand-built explicit-VR-LE 2-element wire: `(0028,0106) US`, declared length 3 (not a multiple of 2), followed by a completely well-formed `(0010,0010) PN "AB"`. Parsed via `InMemDicomObject::read_dataset_with_ts` | Reproduced: `read_dataset_with_ts` fails with `DecodeElementHeader { position: 11, source: ReadLength { ..., source: UnexpectedEof } }` - the 3rd, never-consumed byte of elem1's value gets misread as the start of elem2's header, an opaque failure with no indication that elem1's odd length was the actual cause. See `repro/`, `output.txt` |
| Repeat-run check | 3 separate `cargo run --release` invocations against the pristine tree (fixed input, no randomness in this code path) | 3/3 runs reproduced identically (same wire bytes, same error shape, same exit code) |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

**Fully reproducible**: **Yes** - the repro drives the real, unmodified public API
(`InMemDicomObject::read_dataset_with_ts`) through the real parser code path end-to-end. As with
candidates 002/068/078 in this same cluster, the "second participant" here is simply
well-formed-but-malformed wire bytes handed to the library's own decoder - there is no separate
peer process/binary involved in this specific finding (a single malformed length field in an
otherwise ordinary data set, not a multi-party protocol interaction).

## Class

`logic` (silent parser/stream desynchronization - not a crash by itself in this specific repro,
but corrupts the parse of any element following the malformed one, with a misleading error message
that hides the real cause; depending on the leftover bytes' content this can also surface as a
successful-but-wrong parse of the following element rather than an error, since nothing detects the
desync itself).

## Root cause

`read_value_tag`, `read_value_ss`, `read_value_fl`, `read_value_ul`, `read_value_us`,
`read_value_uv`, `read_value_sl`, `read_value_sv`, and `read_value_od` in
`parser/src/stateful/decode.rs` each compute their element count as `len >> k` (equivalently
`len / elem_size`, rounding toward zero) from the element header's own declared byte length -
fully file/attacker-controlled. When `len` isn't an exact multiple of `elem_size` (2, 4, or 8
depending on the VR), the subsequent `decode_*_into(&mut self.from, ...)` call reads only
`n * elem_size` bytes from the underlying stream, silently leaving `len % elem_size` bytes
unconsumed - while `self.position += len as u64` (used purely for diagnostics) advances by the
full, un-truncated `len`. Since `self.from` is a sequential, non-seekable `Read` stream, those
unconsumed bytes are never skipped or accounted for: they become the leading bytes of whatever the
decoder reads next, i.e. the following element's header, corrupting all subsequent parsing with no
detection of the desync itself. This is exactly the same mechanism candidate 068 already confirmed
and fixed for `read_u32_to_vec` (the Basic Offset Table reader) - 068's own fix, however, is scoped
to that one function only and does not touch any of these 9 general value readers, which are
reachable for any ordinary data-set element of the affected VRs, not just an encapsulated
PixelData's offset table.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added `require_known_length_multiple_of(header, elem_size)` (mirroring `require_known_length`,
next to it in the same `impl` block) that returns the element count and rejects a length that
isn't an exact multiple of `elem_size` with a new, dedicated `InvalidValueLength` error (tag,
position, declared length, and required multiple - the same shape/style as this file's existing
error variants), instead of silently truncating and leaving the stream desynchronized. Wired this
helper into all 9 affected functions (`read_value_tag`/`_ss`/`_fl`/`_ul`/`_us`/`_uv`/`_sl`/`_sv`/
`_od`), replacing each one's own `len >> k` computation - `read_u32`/`read_u32_to_vec` (068's own
already-fixed-separately target) were intentionally left untouched here to keep this candidate's
diff scoped to the 9 functions this candidate is actually about. See `patch.diff`
(`parser/src/stateful/decode.rs` only).

Verified: `repro/` re-run against the patched tree no longer desyncs - the malformed length is
now rejected immediately and specifically at its own element (`InvalidValueLength` at tag
`(0028,0106)`, position 8 - the element's own header, not the following element's), and the
well-formed `(0010,0010) PN` element is never touched or misinterpreted. See `output.txt` for the
full before/after capture.

- Project test suite:
  - Before fix: `cargo test -p dicom-parser -p dicom-object --lib` -> 49 passed + 75 passed, 0
    failed. `cargo test --workspace --lib` -> 351 passed, 0 failed, 2 ignored.
  - After fix: `cargo test -p dicom-parser -p dicom-object --lib` -> 49 passed + 75 passed, 0
    failed (identical). `cargo test --workspace --lib` -> 351 passed, 0 failed, 2 ignored
    (identical - no regressions, no newly-passing tests; no existing test exercised a
    misaligned length on any of these 9 VRs).

## What's not done / open questions

- The branch's other two bundled changes were deliberately **not** adopted:
  - `decode.rs`'s mid-stream `(0008,0005)` VR-loosening (routing any string VR, not just `CS`,
    through `read_value_cs` for that one tag) would, if applied on top of current master, recreate
    exactly the charset encode/decode asymmetry candidate 078 already fixed - post-078, the
    encoder strictly requires `VR::CS` to switch the active codec, so a decoder that switches for
    `VR::SH` (or any other string VR) at `(0008,0005)` would decode using one charset and
    (silently) fail or misencode on write using another. Adopting decode's loosening without a
    matching, deliberate encoder change would be a net regression, not a fix.
  - `tokens.rs`'s `IntoTokens` reordering (emitting `(0008,0005)` first when a lower-tag element
    exists) was reasoned about but not independently verified with its own repro this session -
    `InMemDicomObject`'s elements already iterate in ascending-`Tag` `BTreeMap` order, so encode
    and decode already see any lower-tag (e.g. group-`0x0000`) elements in the same relative
    order on both sides; whether the reordering is solving a real asymmetry (as opposed to just
    changing which self-consistent order is used) was not established, and touching it was out of
    scope for this candidate's focus (the length-alignment class). Left as an open question rather
    than folded in speculatively.
- Only the `US` case (2-byte element, odd declared length) was reproduced end-to-end. The fix is
  structurally identical across all 9 affected functions (same helper, same `elem_size` parameter
  changed per call site), so the other 8 (`AT`/`SS`/`FL`/`UL`/`UV`/`SL`/`SV`/`OD`) are expected to
  share the same fix, but each was not independently re-verified with its own dedicated repro this
  session.
- `read_u32`/`read_u32_to_vec` (candidate 068's own target) were intentionally left as-is here,
  since 068 already has its own confirmed fix for that function specifically - if both this
  candidate's and 068's patches are ever merged together, they touch adjacent but disjoint
  functions in the same file and are expected to apply cleanly together, but this was not
  independently verified this session (see the existing similar follow-up note for candidates
  004/080/081 in `IDEAS.md`).
- Not fuzzed - found by direct code reading against a seeded lead (the unmerged branch's own
  `decode.rs` hunk) plus manual boundary-value construction (a single odd-length `US` element),
  following the same pattern as candidates 068/078. A short fuzz campaign against `parser/fuzz`
  targeting arbitrary-VR/arbitrary-length elements might turn up further variants or confirm the
  other 8 VRs individually.
