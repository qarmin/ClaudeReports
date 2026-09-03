# encoding-invalid-byte-trap-introduces-separator

`encoding/src/text.rs`'s `decode_text_trap` - the fallback invoked by every non-default character
set codec (`ISO_IR 192`/UTF-8 and every `ISO-8859-*` variant, via the shared `decl_character_set!`
macro) whenever a raw byte can't be decoded under that character set - represented the invalid
byte as a **literal, 4-character C/Python-style octal escape sequence**, e.g. byte `0x88` became
the 4 ASCII characters `\`, `2`, `1`, `0` (`\210`). Since `\` (backslash) is DICOM's own
value-multiplicity separator for multi-valued text elements, this trap can silently **inject a
spurious separator into decoded text that was never part of the original semantic content**.
Nothing else in the codebase ever interprets a `\NNN` sequence as an escape and reverses it, so
this is a purely one-way, lossy transformation with a side effect: any later step that treats the
decoded text as potentially multi-valued (re-splitting on `\`, e.g. after a write-then-read-again
cycle) can misinterpret the injected separator as if it were a real one, silently turning one
stored value into two.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Stacked-fuzzing technique (as used for candidates 049-051): apply candidates 001, 048, 049, 050, 051's already-confirmed patches to the real tree, then re-run the existing `object/fuzz/fuzz_targets/open_file.rs` | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run open_file -- -max_total_time=150` (150s) | A new crash: a parse/write/re-parse/compare round trip reported `NOT EQUAL` on a `(0008,0016) SOPClassUID` (VR UI) element |
| `cargo +nightly fuzz tmin` to minimize the crashing input | Default tmin settings | 366-byte minimized input, could not be reduced further by tmin |
| Standalone diagnostic (dedicated `dicom-core`/`dicom-object` path-dep binary) parsing, writing, and re-parsing the minimized input, comparing byte-for-byte and printing the full object at each step | debug build, with 001+008+048+049+050+051's patches applied | Isolated the discrepancy to `(0008,0016)`'s raw byte `0x88` (under declared `SpecificCharacterSet` "ISO_IR 192"/UTF-8) decoding to a single `Strs` item containing a literal `\210` on first parse, then splitting into two items (`"1.2.840.1"` and `"2100008.5.1..."`) after being written back out and re-parsed |
| Checking whether this was actually already-documented candidate 002 (VR::UI should always use the default character repertoire, not the declared one) - stacking 002's patch as well | Same diagnostic, with 002's patch additionally applied | The specific fuzz-found crash disappeared entirely (`EQUAL`) - `0x88` decodes without error under the default ISO-8859-1 codec once VR::UI correctly stops using the declared UTF-8 codec, since ISO-8859-1 maps every byte 0-255 and never invokes the trap. This confirmed the *fuzz input's own proximate trigger* was candidate 002, already documented separately |
| Independent, minimal, hand-built reproduction: a plain `LO` (`InstitutionName`) element - which, unlike `UI`, legitimately uses the declared non-default character set per the standard - containing one raw byte (`0x88`) that isn't valid standalone UTF-8, decoded directly via `InMemDicomObject::read_dataset_with_ts` | Constructed directly via the public API, no fuzzer, file meta table, or dependency on candidate 002 involved | Confirmed the same defect independently of candidate 002: the decoded value was `Strs(["AB\210CD "])` - one item containing a literal backslash. A full decode -> encode -> decode round trip (see `backtrace.txt`) showed this then splits into `Strs(["AB", "210CD  "])` - two items - on the second decode. Reproduced deterministically: in-tree checks during investigation + 2 independent `just test` runs against a fresh clone of pristine upstream `df2b9e42` |

## Status

**Confirmed**, with a bounded fix applied and verified. Deterministic - reproduces on every run,
for any text value legitimately decoded under a declared non-default character set
(`SpecificCharacterSet` other than the ISO_IR 6 default) that contains a byte the declared
character set cannot represent.

## Class

`logic` (silent, one-way corruption of a value's structure - a single stored value can be turned
into multiple values without any error being raised, purely as a side effect of how an
unrepresentable byte is displayed). Distinct from, but discovered alongside, already-documented
candidate 002 (which explains why *this specific fuzz input* reached the trap for a `UI` element
in the first place) and candidate 008 (`to_multi_str`'s separate re-split defect, operating on an
already-correctly-parsed multi-item value rather than on an initial decode). This candidate's root
cause and fix are independent of both.

## Root cause

`encoding/src/text.rs` implements each supported non-default character set (UTF-8/"ISO_IR 192",
every `ISO-8859-*` variant, Shift-JIS, etc.) via the `decl_character_set!` macro, which wires up
the underlying `encoding` crate's `Encoding::decode` with `DecoderTrap::Call(decode_text_trap)` -
a callback invoked whenever the underlying codec encounters a byte sequence it cannot decode.
`decode_text_trap` represented that byte as a literal octal escape (`\NNN`, matching a C/Python
string-literal convention for showing raw bytes) written directly into the output string via
`output.write_char('\\')` followed by three digit characters. This is fine as a purely cosmetic,
throwaway debug representation, but this crate uses the *actual decoded string* as the real,
semantic value of the DICOM element - and DICOM already assigns meaning to `\` as the
value-multiplicity separator for every multi-valued text VR. No corresponding decode-side
awareness of "this is an escape, not a real separator" exists, and no encode-side unescaping
exists either - so the injected backslash is indistinguishable from a real one to anything that
processes the value afterwards.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed `decode_text_trap` to represent an unrepresentable byte with the Unicode replacement
character (`U+FFFD`), matching the same convention already used by `String::from_utf8_lossy` and
other standard lossy-decoding APIs - a single, well-known placeholder character that cannot be
confused with any DICOM-meaningful text (unlike a backslash):

```rust
fn decode_text_trap(
    _decoder: &mut dyn RawDecoder,
    _input: &[u8],
    output: &mut dyn StringWriter,
) -> bool {
    output.write_char('\u{FFFD}');
    true
}
```

See `patch.diff` for the exact diff (fix only; the accompanying unit test added during
investigation - `iso_ir_192_invalid_byte_does_not_introduce_separator` - is not included, matching
this session's convention of keeping `patch.diff` minimal).

- `cargo test -p dicom-encoding --lib`: baseline (test present, fix removed) - the new test
  **FAILED** with `decoding an invalid byte must not introduce a literal backslash: "1.2.840.1\
  \2100008.5.1"` (exactly the predicted output) -> with the fix restored - **41 passed, 0
  failed**.
- Full workspace (`cargo test --workspace --lib`, all 12 crates with a `#[lib]` target):
  **351 passed / 0 failed / 2 ignored** (pristine, no candidate changes) -> **352 passed / 0
  failed / 2 ignored** with this fix + its 1 test applied - zero regressions anywhere in the
  existing suite.

## What's not done / open questions

- This fix accepts that decoding a genuinely invalid byte is inherently lossy - the exact original
  byte value cannot be recovered from `U+FFFD`, and a decode -> encode -> decode round trip will
  necessarily grow the value's byte length (`U+FFFD` is 3 bytes in UTF-8, vs. 1 byte for the
  original invalid byte). This is expected and considered correct: the goal is only to prevent the
  *structural* corruption (spurious value splitting), not to achieve a lossless round trip of
  input that was never valid under its own declared character set in the first place.
- No attempt was made to make the trap character configurable or to surface a warning/error when
  it fires; this mirrors the existing, unchanged behavior of silently tolerating invalid bytes
  (the `decode()` call still returns `Ok`, not `Err`) - only the specific *character* used to
  represent the invalid byte was changed, keeping the fix minimal and behavior otherwise
  unchanged for any input that doesn't hit this trap.
- Not investigated: whether any caller relies on the specific historical `\NNN` format being
  present in decoded text (e.g. for display or logging purposes) - a workspace-wide grep found no
  code parsing or depending on that specific format (see "Root cause"), but this was not verified
  against external, out-of-tree consumers of this crate.

## Addendum (2026-08-29 session): this fix's own `U+FFFD` substitution can make a value permanently unwritable

Found via this session's stacked-fuzzing of the official, tracked `object/fuzz/fuzz_targets/
open_file.rs` target (this candidate's own patch, among several others, applied - see
`projects/dicom-rs.md`'s "stacked-fuzzing" technique). The official fuzz harness asserts that
`write_all` must always succeed for any object that was successfully parsed
(`.expect("writing DICOM file should always be successful")`) - libFuzzer found a crash violating
that invariant within a couple minutes.

**Root cause**: this candidate's own fix changed `decode_text_trap`'s substitution from an
ASCII-only octal escape (`\NNN`, always representable in every single-byte codec) to `U+FFFD`
(the Unicode replacement character) - correctly fixing the original backslash-injection bug, but
`U+FFFD` itself is *outside every DICOM-supported character set's repertoire except UTF-8-based
ones* (ISO-8859-*, WINDOWS_31J/874/949, etc. are all single-byte-or-similar codecs with no code
point for `U+FFFD`). Every character-set codec in this file (`DefaultCharacterSetCodec` and every
`decl_character_set!`-generated one) encodes via `EncoderTrap::Strict`, which fails outright on
any character the target codec can't represent. So a value that had to be substituted during
decode (via this fix's own trap) becomes *permanently unwritable* under the same codec - exactly
the case for `(0008,0016)` (`VR::UI`, an "always default charset" tag per PS3.5 6.1.2.3) in the
fuzzer-found crash: `DefaultCharacterSetCodec::decode` successfully substitutes one invalid byte
with `U+FFFD`, but `DefaultCharacterSetCodec::encode` then fails with `EncodeCustom {message:
"unrepresentable character"}` trying to write that same `U+FFFD` back out.

**Precondition**: this addendum's own bug requires *this candidate's own patch* to be applied
first - it is not reachable on pristine `df2b9e42` alone, since the original (unpatched) trap's
octal-escape substitution is always re-encodable. Confirmed independently: candidate 052's patch
*alone* (no other candidate's patch stacked) is sufficient to reproduce this addendum's crash -
see `repro_addendum_encoder_trap/`'s own `crash-input.bin` (the exact minimized artifact
libFuzzer wrote out) and its `main.rs`.

**The fix**: changed every codec's `encode()` from `EncoderTrap::Strict` to `EncoderTrap::Replace`
(substitutes an unrepresentable character with `?`, universally representable in every one of
this file's codecs) - restoring the same "always lossy-but-successful" symmetry `decode()` already
has via its own trap, instead of erroring. See `patch_addendum_encoder_trap.diff` (applies on top
of `patch.diff`, not on pristine master directly - 2 lines changed, `encoding/src/text.rs`).

Verified: `repro_addendum_encoder_trap/` (own `crash-input.bin`, a real fuzzer-found artifact, not
hand-constructed) reproduces the write failure 3/3 runs against 052's patch alone, and writes
successfully 3/3 runs with this addendum's own fix also applied - see its own `output.txt`. Project
test suite: `cargo test -p dicom-encoding` - 40 passed/0 failed (lib) + 4/0 (doctests), identical
before/after (052 alone vs. 052+addendum), zero regressions. Full workspace `cargo test --workspace
--exclude dcms` (with 052+addendum applied) - 0 failures.

**What's not done for this addendum**: a decode -> encode round trip through this fix remains
inexact for genuinely invalid input (an undecodable byte becomes `U+FFFD` on decode, then `?` on
re-encode, then a literal `?` character on the next decode - never recovering the original byte or
even staying stable across more than one round trip). This is considered acceptable for the same
reason the original fix's own "what's not done" section already accepts byte-length growth: the
input was never valid under its own declared character set to begin with, so exact round-tripping
was never achievable - the goal here is only to keep `write_all` from failing outright (matching
the crate's own official fuzz-target invariant), not to achieve lossless recovery of invalid data.
