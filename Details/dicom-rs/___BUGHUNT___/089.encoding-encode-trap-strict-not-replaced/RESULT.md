# encoding-encode-trap-strict-not-replaced

`encoding/src/text.rs`'s every non-default character-set codec's `encode()` (the `decl_character_set!`
macro instance and `DefaultCharacterSetCodec::encode`) calls the underlying `encoding` crate's
`.encode(text, EncoderTrap::Strict)`, which returns an error whenever the string being written
contains a character the target charset's *encode* table cannot represent - a leftover half of the
same "text codec roundtrip stability" diagnosis whose decode-side half (`decode_text_trap` emitting
a literal backslash-octal escape) was already investigated and fixed as candidate 052 (never merged
to master, per this workspace's revert-after-verify convention). This candidate's own premise
(decode legitimately producing a `U+FFFD`/`?` placeholder that then fails `Strict` re-encoding) does
**not** hold on current, unpatched master: master's actual `decode_text_trap` still emits pure ASCII
(`\`+3 digits), and ASCII is representable in every charset used here, so that specific decode-trap
-> encode-error chain never fires as originally described. Digging further found a real, independently
reproducible instance of the same underlying `Strict`-vs-asymmetric-charset defect: some byte
sequences that a charset's own **decode** table accepts map to characters that the same charset's
**encode** table cannot represent (an inherent asymmetry in several of the `encoding` crate's legacy
codec tables, not related to any trap), and a plain caller-constructed string containing an
arbitrary unrepresentable Unicode character (e.g. an emoji) hits the exact same `Strict` failure
under `DefaultCharacterSetCodec`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code read of `encoding/src/text.rs` on current master, comparing to the reference branch `origin/fix/text-codec-roundtrip-stability` (`d0395eab`) | n/a | Confirmed the encode-side `EncoderTrap::Strict` is still present, unpatched, in both the `decl_character_set!` macro and `DefaultCharacterSetCodec::encode` |
| Manual boundary-value construction: a temporary in-crate test decoding byte `0x88` under ISO_IR 192 (UTF-8) via `decode_text_trap`, then re-encoding the result | `cargo test -p dicom-encoding --lib`, debug build | The trap's own output (`\210`, pure ASCII) re-encoded successfully under UTF-8 - the candidate's literal premise (decode trap output failing re-encode) does not reproduce on current master, since the trap only ever emits ASCII |
| Manual construction of a caller-supplied string containing a genuinely unrepresentable character (`"AB\u{1F600}CD"`, an emoji) encoded via `DefaultCharacterSetCodec` (ISO-8859-1) | Same temporary test | `EncodeError::EncodeCustom { message: "unrepresentable character" }` - confirms `Strict` does produce a real `Err` for ordinary, non-trap-related input, independent of the trap mechanism |
| Brute-force boundary scan: every single byte 0-255 and every valid double-byte GBK/GB18030 lead-byte pair decoded then re-encoded, per charset, checking for any legitimate (non-trap) decode that fails re-encode | Temporary in-crate test, debug build, ~65,000 decode+encode pairs total | Found 10 single bytes (`0xF0`-`0xF9`) that `IsoIr13CharacterSetCodec` (WINDOWS_31J/Shift-JIS-CP932) legitimately decodes into Private-Use-Area Unicode characters that its own encode table cannot represent back - a genuine, real asymmetry unrelated to any trap |
| Real end-to-end reproduction: a hand-built, well-formed explicit-VR-LE data set (`SpecificCharacterSet` = "ISO_IR 13", one `LO` element containing byte `0xF0`) decoded via the real public `InMemDicomObject::read_dataset_with_ts`, then written back via the real public `write_dataset_with_ts` | Standalone `repro/` crate, debug and release builds | Decode succeeds; write fails with `PrintDataSet { source: WriteValue { source: EncodeText { source: EncodeCustom { message: "unrepresentable character" } } } }` - reproduced deterministically 3/3 in-tree runs plus a fresh-clone `just test` run against pristine upstream `df2b9e42` |

## Status

**Confirmed**, deterministic (3/3 in-tree runs, both debug and release builds, plus one fresh-clone
`just test` run against pristine upstream `df2b9e42`).

**Fully reproducible**: Yes. The repro constructs one well-formed DICOM data set (a static external
artifact - the same bytes a real DICOM file or wire transfer would contain) and drives it through
the library's own real public read (`InMemDicomObject::read_dataset_with_ts`) and write
(`write_dataset_with_ts`) entry points; per `CONFIRMATION.md` section 11 this is a genuine single-
participant repro (a malformed/unusual-but-well-formed file handed to the parser's real public entry
point *is* the external input, with no second live participant to fake).

## Class

`logic` (a well-formed input that decodes successfully cannot be written back out - a spurious
`Err` returned by an otherwise-successful round trip, not a crash or memory-safety issue; the
underlying `EncodeTextError`/`WriteError` types are already proper `Result`s all the way up through
`InMemDicomObject::write_dataset_with_ts`, so no panic/unwrap was found turning this into a crash).

## Root cause

Every character-set codec in `encoding/src/text.rs` (`decl_character_set!`'s generated `encode()`,
and `DefaultCharacterSetCodec::encode`) calls the `encoding` crate's `Encoding::encode` with
`EncoderTrap::Strict`, which returns `Err` for any character the target charset's encode table
cannot represent. Two independent ways a legitimately-produced `String` value can contain such a
character:

1. A caller (or any code path) can put an arbitrary Rust `String` into a text element under a given
   `SpecificCharacterSet` that simply doesn't cover the characters in that string (e.g. an emoji
   under ISO-8859-1) - `Strict` correctly detects this and returns `Err`, but this is a plain "the
   data doesn't fit the declared charset" situation with no crash, just an `Err` a caller has to
   handle.
2. More subtly, some of the `encoding` crate's legacy multi-purpose codec tables are **not perfectly
   symmetric** between decode and encode - `WINDOWS_31J` (used here for "ISO_IR 13") legitimately
   decodes bytes `0xF0`-`0xF9` (Shift-JIS/CP932's "NEC selection of IBM extended characters" row)
   into Private-Use-Area Unicode code points that its own encode table has no reverse mapping for.
   A value that arrived as ordinary, well-formed bytes under a declared non-default character set,
   decoded without any error or trap involvement, still cannot be written back out unmodified -
   turning an otherwise lossless read into a lossy round trip that additionally surfaces as a hard
   `Err` instead of silently degrading.

Both cases share the same fix: `Strict` is the wrong trap for this crate's use case, where the
decoded `String` is the semantic value of an already-successfully-parsed DICOM element and a write
failure over an encode-table gap is a worse outcome than a lossy-but-successful write.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed every `EncoderTrap::Strict` in `encoding/src/text.rs` (the `decl_character_set!` macro's
`encode()` and `DefaultCharacterSetCodec::encode`) to `EncoderTrap::Replace`, matching the reference
branch `origin/fix/text-codec-roundtrip-stability` (`d0395eab`) - the underlying `encoding` crate's
own `Replace` trap substitutes an implementation-defined placeholder byte for any character it can't
encode instead of returning `Err`, so a write can always complete for a value that was itself
successfully decoded (see `patch.diff`).

Verified against more than the one triggering byte (per `CONFIRMATION.md` section 7): re-ran the
same check for all 10 asymmetric `WINDOWS_31J` bytes (`0xF0`-`0xF9`) and for the unrelated
caller-constructed-emoji case under `DefaultCharacterSetCodec` - all now encode successfully instead
of erroring (see `output.txt`).

- Project test suite, before and after the fix (identical in both cases, zero regressions):
  - `cargo test -p dicom-encoding --lib`: 40 passed, 0 failed
  - `cargo test -p dicom-parser --lib`: 49 passed, 0 failed
  - `cargo test -p dicom-object --lib`: 75 passed, 0 failed
  - `cargo test --workspace --lib`: 351 passed, 0 failed, 2 ignored (after fix; same total established
    by prior candidates' own before/after runs on this same commit)

## What's not done / open questions

- This candidate's originally-suspected trigger mechanism (the decode-side trap producing a
  placeholder that then fails re-encode) does **not** actually occur on current master, because
  master's `decode_text_trap` still emits the pure-ASCII backslash-octal escape that candidate 052
  separately confirmed and proposed fixing - ASCII is always encodable in every charset used here.
  If candidate 052's own fix (changing the trap's output to `?` or `U+FFFD`) is ever applied without
  this candidate's fix, the interaction between the two should be re-checked (052's own fix note
  already chose `?` specifically because it round-trips without needing this fix at all, but `?`
  being ASCII means it doesn't independently exercise this candidate's `Strict` code path either).
- Only `IsoIr13CharacterSetCodec` (WINDOWS_31J) was found to have decode/encode table asymmetry via
  the brute-force scan performed (single bytes for all single-byte codecs, double-byte lead/trail
  pairs for GBK/GB18030). `ISO_IR 87` (`ISO_2022_JP`, stateful/escape-based) and `WINDOWS_949` were
  not exhaustively brute-forced beyond the single/double-byte sweep already covered - a longer
  fuzzing campaign (via `fuzz-bug-repro`, run separately) targeting encode/decode round trips across
  all supported charsets would be a reasonable way to find any other such asymmetric spots.
- The plain "caller puts an unrepresentable character into a text element under an unrelated
  declared charset" case (item 1 in Root cause) is arguably not a bug at all - returning `Err` there
  is defensible input validation - but it shares the exact same code path and fix as the genuine
  table-asymmetry case, and the reference branch's own fix treats both uniformly, so this session
  did the same rather than trying to special-case one but not the other.
