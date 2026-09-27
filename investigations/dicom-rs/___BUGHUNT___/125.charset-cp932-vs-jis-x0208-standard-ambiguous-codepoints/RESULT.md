# charset-cp932-vs-jis-x0208-standard-ambiguous-codepoints

Follow-up to `IDEAS.md`'s open backlog item left by candidate 105: a pure round-trip
(decode-succeeds/encode-fails) sweep can't catch a charset table that is internally
self-consistent but simply *maps to the wrong character* compared to an independent reference
implementation. This candidate builds that differential test against Python's stdlib codecs for
all three charsets flagged as unchecked (`WINDOWS_31J`/candidate 089, `WINDOWS_949`/candidate 104,
and `ISO_2022_JP`'s JIS X 0208 half/candidate 105).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Rust dump tool (`dump.rs`, `encoding` crate v0.2.33, the exact version pinned in this workspace's `Cargo.lock`) brute-forcing every 2-byte lead/trail pair (65536 combos) through `encoding::all::WINDOWS_31J`/`WINDOWS_949` `DecoderTrap::Strict`, plus every JIS X 0208 94x94 (8836) code pair through `encoding::all::ISO_2022_JP` prefixed with the `ESC $ B` designator escape | release build, single run, ~87k total rows written to `charset_dump.tsv` | Produced a full decode table for all three charsets |
| Python differential comparison (`compare2.py`) against `shift_jis` (for WINDOWS_31J), `cp949` (for WINDOWS_949), and `iso2022_jp` (for the JIS X 0208 escape sequences), restricted to genuine double-byte lead-byte ranges (lead >= 0x81) to exclude ASCII/undefined-single-byte noise | 46,084 genuine double-byte rows checked | 3,257 raw mismatches, but only 12 where BOTH sides decoded successfully to a *different* codepoint (the interesting class - a table that's internally consistent but wrong); the remaining 3,245 are Python raising `UnicodeDecodeError` on codepoints Rust's `encoding` crate happens to support (or vice versa) - a repertoire-coverage difference, not a wrong-mapping bug, and out of this candidate's scope |
| Manual cross-check of the 12 mismatches against Python's own `cp932` codec (Microsoft's actual Shift-JIS-derived code page, as opposed to `shift_jis`'s stricter JIS-standard mapping) | 6 distinct byte pairs, each appearing twice (once via WINDOWS_31J's raw Shift-JIS bytes, once via the equivalent ISO_2022_JP/JIS-X0208 escape-sequence encoding of the same 6 codepoints) | All 6 exactly match Python's `cp932` decode, not `shift_jis` - see `output.txt` |

## Status

**Not reproducible** (as a distinct, in-scope bug). The differential test is real and the 12
mismatches are genuine and 100% deterministic (pure function, no I/O/randomness) - but every one
of them turns out to be the single most well-known Shift-JIS interoperability ambiguity (WAVE
DASH/FULLWIDTH TILDE, DOUBLE VERTICAL LINE/PARALLEL TO, MINUS SIGN/FULLWIDTH HYPHEN-MINUS, and
three currency/negation symbols mapped to their fullwidth Windows-only code points instead of
Latin-1 equivalents), not a transcription error in the crate's own tables. Re-run 2x with the same
deterministic result both times (pure lookup table, no seed/timing dependency to vary).

**Fully reproducible**: N/A (Status is Not reproducible, not Confirmed - the "Fully reproducible"
column only applies to Confirmed findings per `CONFIRMATION.md`).

## Root cause (why the suspect turned out to be safe)

The `encoding` crate's `WINDOWS_31J` table (and, by extension, `ISO_2022_JP`'s JIS X 0208 half,
which reuses the same underlying `encoding-index-japanese` mapping data for the double-byte
region) deliberately implements **Microsoft's CP932 code page convention** for 6 historically
ambiguous JIS X 0208 code points, not the "pure" ISO/JIS X 0208:1990 standard mapping that
Python's `shift_jis` codec (and, presumably, a strict non-Windows JIS implementation) uses. This
is not a bug in the crate: `WINDOWS_31J` is literally named after the Windows/Microsoft code page,
and its behavior exactly matches Python's own `cp932` codec byte-for-byte for all 6 disputed
points (`output.txt`) - the crate is correctly implementing what its name promises. Confirmed via
independent cross-check: comparing against the *wrong* reference (`shift_jis`, a different,
stricter standard) is what manufactured the appearance of a mismatch, not a defect in
`dicom-rs` or the `encoding` crate itself.

Whether real-world Japanese DICOM devices encoding under DICOM's `ISO_IR 87`/`ISO_IR 13` (which
PS3.5 defines as "JIS X 0208-1990"/"JIS X 0201-1976", the ISO-registered standards, not explicitly
Microsoft's code page) would ever emit the Microsoft-convention bytes for these 6 code points is a
real-world interoperability question this session has no way to answer without a corpus of real
scanner-produced files exercising exactly these 6 rare symbols - but this is an inherent property
of *any* Shift-JIS-family implementation choosing one of two well-known, both-legitimate
conventions (confirmed by the fact that Python's own standard library ships both `shift_jis` and
`cp932` as separate codecs specifically because of this same ambiguity), not a defect introduced
by `dicom-rs` or a transcription error to fix. This closes out the specific "internally
self-consistent but wrong" class candidate 105 flagged as unchecked for all three charsets: no
such transcription-error-class bug was found in any of them.

## Introduced by

N/A - not a bug, no fix needed.

## The fix

None - not applicable, this is not a defect.

- Project test suite: not run (no code change made).

## What's not done / open questions

- The 3,245 mismatches where one side errors and the other doesn't (repertoire-coverage gaps
  between the `encoding` crate's tables and Python's stdlib tables) were deliberately out of scope
  for this candidate (it targeted the "wrong mapping" class specifically, not "one side supports
  more code points than the other") - not investigated further, no evidence they're relevant to
  DICOM interoperability specifically.
- `IsoIr138CharacterSetCodec` (ISO_8859_8, Hebrew, single-byte) and `IsoIr149`'s already-covered
  Korean table were not independently re-verified beyond what's already in this candidate
  (`WINDOWS_949`/`cp949`, 0 interesting mismatches in the differential run) - the single-byte
  `ISO_8859_*` family maps 1:1 to well-standardized code pages with no known Microsoft-vs-ISO
  ambiguity of this kind, so not considered a live lead.
- If a future session ever gets access to a corpus of real Japanese-scanner-produced DICOM files
  using `ISO_IR 87`/`ISO_IR 13`, checking whether any of them actually contain these 6 specific
  disputed byte sequences (and, if so, which convention the originating device intended) would
  settle the interoperability question definitively - not attempted this session, no such corpus
  available offline.
