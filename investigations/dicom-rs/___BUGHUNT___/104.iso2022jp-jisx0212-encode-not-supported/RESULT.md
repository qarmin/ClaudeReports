# encoding-decode-encode-asymmetry-iso2022jp-windows949-sweep

Extension of candidate 089's decode-then-re-encode brute-force scan to the two charsets that
scan explicitly skipped: `ISO_IR 87` (`ISO_2022_JP`, a stateful escape-sequence codec) and
`WINDOWS_949` (double-byte Korean), both declared via `encoding/src/text.rs`'s
`decl_character_set!` macro, whose generated `encode()` calls the `encoding` crate's
`Encoding::encode(text, EncoderTrap::Strict)` - the same trap 089 already showed can turn a
legitimately-decoded value into a spurious write-back `Err` when a charset's decode and encode
tables aren't symmetric.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct source read of `encoding` crate v0.2.33's `src/codec/japanese.rs` (the `ISO_2022_JP` codec backing `IsoIr87CharacterSetCodec`) | n/a | Found the asymmetry is explicitly documented in the crate's own doc comment: `"JIS X 0212-1990 (\`ESC $ ( D\`, XXX asymmetric support)"`. `ISO2022JPDecoder` has a full state machine for the `ESC $ ( D` escape (switches to JIS X 0212 mode, decodes two-byte codes via `index::jis0212::forward`), but `ISO2022JPEncoder` has no code path for JIS X 0212 at all - only ASCII, JIS X 0201 Katakana, and JIS X 0208 via `index::jis0208::backward` |
| Direct source read of `encoding` crate's `src/codec/korean.rs` (`WINDOWS_949`) | n/a | `Windows949Decoder`/`Windows949Encoder` both go through the same `index_korean::euc_kr::forward`/`backward` table pair over the same lead/trail byte ranges (`0x81..=0xfe` / `0x41..=0xfe`); no asymmetry documented or apparent from reading the code alone |
| Temporary in-crate brute-force test (`encoding/src/text.rs`, `#[cfg(test)]`, removed before this session ended): every JIS X 0212 lead/trail pair (`0x21..=0x7e` x `0x21..=0x7e`, 8,836 pairs) prefixed with the `ESC $ ( D` escape, decoded via `IsoIr87CharacterSetCodec::decode`, then re-encoded via `::encode` | `cargo test -p dicom-encoding --lib -- --nocapture`, debug build | 5,786 of 8,836 pairs (65%) decode successfully but fail re-encode with `EncodeCustom { message: "unrepresentable character" }` - confirms the documented asymmetry is real and large, not a corner case |
| Same harness, `WINDOWS_949` double-byte lead/trail pairs (`0x81..=0xfe` x `0x41..=0xfe`, 12,144 pairs) via `IsoIr149CharacterSetCodec` | Same run | 0 asymmetric pairs found - no bug found in `WINDOWS_949` via this sweep (see "What's not done" below for what this does and doesn't rule out) |
| Real end-to-end reproduction: a hand-built, well-formed explicit-VR-LE data set (`SpecificCharacterSet` = "ISO_IR 87", one `LO` element containing the `ESC $ ( D` escape sequence followed by JIS X 0212 lead/trail bytes `0x22 0x2f`, decoding to U+02D8 BREVE) decoded via the real public `InMemDicomObject::read_dataset_with_ts`, then written back via the real public `write_dataset_with_ts` | Standalone `repro/` crate, debug and release builds, in-tree and via a fresh `just clone` from `https://github.com/qarmin/dicom-rs.git` @ `df2b9e42` | Decode succeeds; write fails with `PrintDataSet { source: WriteValue { source: EncodeText { source: EncodeCustom { message: "unrepresentable character" } } } }` - reproduced deterministically 3/3 in-tree release runs, 1/1 in-tree debug run, and 1/1 fresh-clone `just test` run |

## Status

**Confirmed**, deterministic (3/3 in-tree release runs, 1/1 in-tree debug run, 1/1 fresh-clone
`just test` run against pristine `df2b9e42` - see `output.txt`). `WINDOWS_949`'s own sweep found
no asymmetric pairs within the byte ranges checked (see "What's not done").

**Fully reproducible**: Yes. The repro constructs one well-formed DICOM data set (a static
external artifact - the same bytes a real DICOM file or wire transfer would contain, using a
documented DICOM code-extension escape sequence that the codec's own decoder explicitly supports)
and drives it through the library's own real public read (`InMemDicomObject::read_dataset_with_ts`)
and write (`write_dataset_with_ts`) entry points. Per `CONFIRMATION.md` section 11 this is a
genuine single-participant repro (a well-formed-but-unusual file handed to the parser's real
public entry point *is* the external input, with no second live participant to fake).

## Class

`logic` (a well-formed input that decodes successfully cannot be written back out - a spurious
`Err` from an otherwise-successful round trip, not a crash; `EncodeTextError`/`WriteError` are
proper `Result`s all the way up through `InMemDicomObject::write_dataset_with_ts`, no panic found).

## Root cause

`IsoIr87CharacterSetCodec` (declared via `encoding/src/text.rs`'s `decl_character_set!` macro over
the `encoding` crate's `ISO_2022_JP`) inherits an asymmetry the underlying crate documents but does
not fix: its decoder (`ISO2022JPDecoder` in `encoding-0.2.33/src/codec/japanese.rs`) implements the
`ESC $ ( D` escape to switch into JIS X 0212-1990 mode and decode two-byte codes through
`index::jis0212::forward`, but its encoder (`ISO2022JPEncoder`) has no matching code path - it only
ever emits ASCII, JIS X 0201 Katakana (`ESC ( I`), or JIS X 0208 (`ESC $ B`, via
`index::jis0208::backward`). Any character reachable only through JIS X 0212 (5,786 of the 8,836
two-byte codes checked, i.e. most of the JIS X 0212 repertoire, since JIS X 0208 and JIS X 0212 are
largely disjoint supplementary character sets by design) round-trips through `decode` successfully
but then hits `decl_character_set!`'s `EncoderTrap::Strict`, which returns `Err` for any character
the encode table can't represent - turning a lossless read into a hard write failure, the exact
same `Strict`-vs-asymmetric-table defect class candidate 089 already found in `WINDOWS_31J`
(`IsoIr13CharacterSetCodec`), just far larger in scope here (thousands of characters rather than 10
bytes) because it's an entire supplementary character-set plane the encoder never learned to write
instead of 10 legacy IBM-extension code points.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Same fix as candidate 089 (never merged to master, so this candidate re-applied and re-verified it
independently against this specific finding): changed `decl_character_set!`'s generated `encode()`
in `encoding/src/text.rs` from `EncoderTrap::Strict` to `EncoderTrap::Replace` - the underlying
`encoding` crate's `Replace` trap substitutes an implementation-defined placeholder byte (`?`, byte
`0x3f`) for any character the target charset's encode table can't represent, instead of returning
`Err`, so a write can always complete for a value that itself decoded successfully. This candidate
scoped the fix to only the `decl_character_set!` macro (the code path `IsoIr87CharacterSetCodec`
actually goes through); `DefaultCharacterSetCodec::encode` (also `Strict` in 089's own patch) was
left untouched here since this candidate's finding doesn't involve it.

Verified against more than the one triggering pair (per `CONFIRMATION.md` section 7): re-ran the
fixed codec against two more JIS X 0212 pairs from the 5,786-pair asymmetric set (`0x50 0x50` ->
"祆", `0x60 0x60` -> "軱") - both now encode successfully (`Ok([63])`, i.e. `?`) instead of erroring
(see `output.txt`).

- Project test suite, before and after the fix (identical in both cases, zero regressions):
  - `cargo test -p dicom-encoding --lib`: 40 passed, 0 failed (both before and after)
  - `cargo test -p dicom-parser --lib`: 49 passed, 0 failed (both before and after)
  - `cargo test -p dicom-object --lib`: 75 passed, 0 failed (both before and after)
  - `cargo test --workspace --lib` (after fix only): 351 passed, 0 failed, 2 ignored

## What's not done / open questions

- `WINDOWS_949`'s sweep covered exactly the lead/trail byte ranges both the decoder and encoder
  advertise (`0x81..=0xfe` x `0x41..=0xfe`, 12,144 pairs) and found zero asymmetric pairs - this
  rules out that specific asymmetry class for `WINDOWS_949` within that range, but does not
  constitute a full audit of the `encoding-index-korean` table data itself (e.g. whether the
  `euc_kr::forward`/`backward` tables have any internal duplicate-mapping quirks not exercised by
  a pure lead/trail sweep); a longer fuzzing campaign (via `fuzz-bug-repro`, run separately) would
  be a more exhaustive way to confirm this charset clean.
- `ISO_IR 87`'s own sweep only covered the JIS X 0212 escape path (`ESC $ ( D`); the JIS X 0208
  path (`ESC $ B`/`ESC $ @`) and Katakana path (`ESC ( I`) were not separately brute-forced this
  session, since the encoder does have matching code for both of those (unlike JIS X 0212) - a
  full sweep of those two paths would still be worth doing to rule out any narrower table gaps
  within JIS X 0208/Katakana specifically (JIS X 0208's own `index_japanese::jis0208::forward`/
  `backward` pair, analogous to what was checked for `WINDOWS_949` here).
- Same open point candidate 089 already noted: the reference fix branch
  `origin/fix/text-codec-roundtrip-stability` (`d0395eab`) applies this same `Strict` -> `Replace`
  change to `DefaultCharacterSetCodec::encode` as well, for an unrelated reason (a caller-supplied
  string with an arbitrary unrepresentable character under ISO-8859-1); this candidate's own fix
  intentionally left that path untouched since it isn't part of what this candidate demonstrated -
  see 089's own `RESULT.md`/`patch.diff` for that broader fix if applying both together.
- `just`'s clone recipe needed an explicit `git fetch origin <sha>` before `git checkout <sha>`
  (added to this candidate's own `justfile`) because `base_commit` (`df2b9e42`, the local
  checkout's actual `HEAD`) is not reachable from `qarmin/dicom-rs`'s advertised `master` ref on
  GitHub (`git ls-remote` shows `master` at the much older `6dc8782e`) - a plain `git clone` +
  `git checkout <sha>` fails with `fatal: unable to read tree`. GitHub's public smart-HTTP server
  does allow fetching an arbitrary reachable SHA directly even when it isn't on an advertised ref,
  which is what the added `git fetch origin {{base_commit}}` step uses. This same gap (no explicit
  SHA fetch, and a `repo_url` of `https://github.com/Enet4/dicom-rs.git` where `df2b9e42` isn't
  reachable at all) is present in every other `NNN.*/justfile` in this workspace that shares this
  `base_commit` (checked: 089's own `justfile` has the identical unfixed `clone` recipe) - `just
  test` in any of those folders would fail at the `clone`/`checkout` step exactly as this one did
  before this fix, meaning those candidates' own "actually run `just test` before shipping" step
  was apparently never verified against a truly fresh network clone. Filed as a new `IDEAS.md`
  backlog item rather than fixed retroactively across all of them in this session (out of this
  candidate's own scope).
