# FileMetaTable UTF-8-byte-count vs char-count padding/length mismatch

`object/src/meta.rs`'s `padded()`/`ui_padded()`/`txt_padded()` helpers (used by every
`FileMetaTableBuilder` string setter) and `dicom_len()` (used by
`FileMetaTable::calculate_information_group_length()`) both decided string parity/length using
Rust's `.len()` (UTF-8 byte count) instead of char count. Flagged as an unconfirmed suspect in an
earlier session's project memory (a prior fuzz-bug-repro campaign had already found and fixed the
`padded()` half of this on a since-discarded local branch, never merged upstream, and explicitly
left `dicom_len()` unchecked - see `IDEAS.md`'s seeded entry for this candidate). Re-verified as
still present in current upstream `master` (`df2b9e42`) by direct read, then confirmed live.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (trace the write path) | Read `object/src/meta.rs` (`padded`, `dicom_len`, `read_str_body`, `FileMetaTableBuilder` setters, `calculate_information_group_length`) and `parser/src/stateful/encode.rs` (`encode_text_element`, `convert_text_untrailed`, `DefaultCharacterSetCodec`) to determine whether these helpers' output actually reaches the wire, or is just internal bookkeeping. | Confirmed `encode_text_element` always recomputes the true post-transcode byte length for the *element* header (so `FileMetaAttribute::HasLength` in the same file, the originally-seeded lead, is a dead end - it's only used by the generic `DicomAttribute` accessor API, never by `write()`). But `dicom_len()` (used for the *group length* header, tag 0002,0000) and `padded()` (used to decide the *stored string's* own padding) are never independently re-verified against the real encoder's output - a real mismatch. |
| Standalone repro (release build, `object`'s public API only: `FileMetaTableBuilder`, `FileMetaTable::write`/`from_reader`) | `___BUGHUNT___/001.../repro`, `cargo run --release`, two scenarios: (a) 4-generation parse/write/reparse loop with SOP Class UID `"1Ö"`; (b) single write->reparse round trip with SOP Class UID `"ÖÖ"` | Both reproduced deterministically. (a): wire byte length grows by 2 bytes every generation (98 -> 100 -> 102 -> 104), unbounded. (b): `FileMetaTable::write()` followed immediately by `FileMetaTable::from_reader()` on the very same bytes fails outright (`DecodeElement { ReadHeaderTag, UnexpectedEof }`) because the computed `information_group_length` header (80, from byte-count-based `dicom_len`) undercounts the real written group content (84 bytes), so the reader stops 2 bytes short and misparses whatever follows as a bogus element header. |
| Repeat-run check | `for i in 1..5: ./target/release/repro` | 5/5 runs reproduced both scenarios identically - fully deterministic, no randomness/layout dependence in this code path. |

## Status

**Confirmed** - deterministic, 5/5 runs, release build, no fixed seed needed (the bug depends only
on the input string content, not on memory layout or timing).

## Class

`logic` (unbounded output growth across parse/write generations) and `crash`-adjacent
(`from_reader` returns a hard `Err` on a file this same library just wrote, for a second,
narrower manifestation of the same root cause - not a panic/abort, but a self-inflicted round-trip
failure with no malicious input involved at all).

## Root cause

Both `padded()` (decides whether to append a padding character to a `FileMetaTableBuilder` string
field so its *stored* length is "even") and `dicom_len()` (predicts a field's *encoded* byte count
for the `information_group_length` header sum) used `s.len()` - Rust's UTF-8 byte count of the
`String` - as a proxy for "bytes on the DICOM wire". That proxy is wrong whenever the string
contains a character outside ASCII: these fields are always transcoded through
`DefaultCharacterSetCodec` (`parser/src/stateful/encode.rs`, `encoding/src/text.rs`), which is
ISO-8859-1 under the hood - exactly **one wire byte per `char`**, not per UTF-8 byte. A Latin-1
character like 'Ö' (U+00D6) is 2 bytes in a Rust UTF-8 `String` but exactly 1 byte on the wire.

This produces two independent, compounding failures:
1. `padded()` over-pads (adds a spurious character) whenever a field's *char* count is already
   even but its *UTF-8 byte* count is odd (or vice versa) - and because `read_str_body` (used when
   re-parsing) never trims trailing padding before feeding the string straight back into the same
   builder setter, each parse/write generation can add another spurious pad character on top of
   the previous one's, growing the value (and the whole file) without bound.
2. `dicom_len()` under- or over-counts a field's real encoded byte contribution to
   `information_group_length` whenever UTF-8 byte count and char count diverge in the *other*
   direction (no ASCII padding "absorbing" the difference) - producing a group-length header value
   that doesn't match the actual number of bytes written, which desyncs `from_reader`'s
   `while total_bytes_read < group_length` loop and causes it to either read past the real group
   content (misparsing the start of the data set as a bogus file-meta element) or, as reproduced
   here, stop short and hit `UnexpectedEof` trying to read what it still believes is one more
   file-meta element header.

Neither failure requires adversarial/malformed input - only a `FileMetaTableBuilder` call with a
non-ASCII Latin-1-range string (something the public API accepts and silently misencodes; also
reachable by parsing an existing DICOM file that legitimately has a non-ASCII byte in a file-meta
UI/SH/AE field, since decode already round-trips such bytes into the equivalent Rust `char`).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed both `padded()` and `dicom_len()` (`object/src/meta.rs`) to use `s.chars().count()`
instead of `s.len()` - this matches what the real encoder actually produces (1 wire byte per
Latin-1-range `char`) for both size-prediction sites. See `patch.diff` (7-line diff, two functions,
same file). Verified against more than the one triggering input: the growth scenario (`"1Ö"`,
4 generations) and the group-length-mismatch scenario (`"ÖÖ"`, single round trip) both now pass -
`repro/` re-run against the patched tree produces stable, identical output across all 4
generations and both scenarios reparse successfully.

- Project test suite: `cargo test -p dicom-object --lib` - 75 passed / 0 failed both before and
  after the patch. `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored both before
  and after (no regressions, no newly-passing tests either - this bug had no existing test
  coverage).

## What's not done / open questions

- Only the file-meta group's string helpers were checked. The main data set's own string handling
  (`parser/src/stateful/encode.rs`'s `encode_text_element`/`encode_texts_element`) independently
  recomputes byte length from the real transcoded bytes every time (verified while tracing this
  bug) and does not share this specific `dicom_len`/`padded`-style precomputed-length pattern, so
  it is not believed to share this exact bug - not exhaustively re-verified with its own dedicated
  candidate, though.
- The originally-seeded lead this candidate grew out of (`FileMetaAttribute::HasLength` in the
  same file using raw `.len()`) turned out to be a dead end for wire correctness (see "What was
  tried") - it's only consulted by the generic `DicomObject`/`DicomAttribute` accessor API
  (`obj.attr_opt(tag)`), never by `write()`. It's still technically inconsistent (an API consumer
  asking a `FileMetaAttribute` for `.length()` on a non-ASCII field gets the wrong byte count) but
  this is a much narrower, cosmetic-only mismatch with no data-corruption or crash consequence -
  not fixed in this candidate, left as a separate, much lower-value lead if anyone wants it (not
  re-added to `IDEAS.md`, since it's already fully understood and not worth a dedicated candidate).
- Not fuzzed - found by direct reasoning from a previously-seeded lead plus manual construction,
  per `HEURISTICS.md`'s guidance for arithmetic/length-prediction bugs (boundary-value construction
  over a fuzz harness). A short fuzz campaign against `object/fuzz/fuzz_targets/open_file.rs`
  seeded with non-ASCII file-meta content might find further instances of the same class in real
  data-set (non-file-meta) code paths, if any exist.

## Addendum (2026-08-25 session): independently reconfirmed live via fresh fuzzing

Since this skill's own fixes are never actually applied to the real tree (every candidate's
`patch.diff` is reverted before the session ends, by design - see `SKILL.md` "the one rule that
matters most"), this bug remains fully live in the current checkout. A 150-second
`ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file` burst against the project's own
existing `object/fuzz/fuzz_targets/open_file.rs` (previously not run this session) found a crashing
input within seconds - `obj.write_all()` followed immediately by `dicom_object::from_reader()` on
the exact bytes just written failed with the identical `ParseMetaDataSet { .. ReadHeaderTag ..
UnexpectedEof .. }` signature this candidate already diagnosed. Root-caused via a standalone
diagnostic (not kept, not part of this workspace) to the crash input's `Implementation Class UID`
(0002,0012) containing high-byte (Latin-1-range, >0x7F) bytes that decode into a Rust `String`
whose UTF-8 byte count differs from its char count - precisely the mechanism already described
above. **`patch.diff` in this folder still applies cleanly to the current tree** (verified via
`git apply --check`) and is still the right fix. No new candidate folder was created for this
re-discovery, per `SKILL.md`'s one-bug-per-candidate rule - this addendum exists so a future
session doesn't mistake the fuzzer's finding for something new. **Practical takeaway for
prioritization**: this bug is trivially, repeatedly rediscoverable by a few minutes of fuzzing the
project's own pre-existing harness, which is a strong signal it's genuinely easy for a real
attacker/malformed file to trigger too - worth prioritizing if/when any of this workspace's patches
are actually applied upstream.
