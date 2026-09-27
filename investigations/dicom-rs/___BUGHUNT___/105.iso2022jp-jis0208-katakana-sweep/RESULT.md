# iso2022jp-jis0208-katakana-sweep

Follow-up to candidate 104 (`ISO_IR 87`/`ISO_2022_JP` decode-then-re-encode asymmetry), which only
brute-forced the `ESC $ ( D` (JIS X 0212) escape path and left the sibling `ESC $ B`/`ESC $ @`
(JIS X 0208, via `encoding` crate's `index_japanese::jis0208::forward`/`backward`) and `ESC ( I`
(half-width Katakana) escape paths unswept - unlike JIS X 0212, `encoding-0.2.33`'s
`ISO2022JPEncoder` (`encoding/src/codec/japanese.rs`) does have matching code for both of these, so
the open question was whether the underlying `jis0208` forward/backward table pair, or the
Katakana byte range, has any narrower internal asymmetry the encoder's presence doesn't rule out.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct source read of `encoding-0.2.33/src/codec/japanese.rs`'s `ISO2022JPDecoder`/`ISO2022JPEncoder` state machines (`Trail0208`/`Lead0208` decode states, `ensure_Lead`/`ensure_Katakana` encode macros) | n/a | Both decode and encode go through the exact same `index_japanese::jis0208::forward`/`backward` pair for the JIS X 0208 path (decoder: `map_two_0208_bytes` calling `forward`; encoder: `index::jis0208::backward` directly) over the identical valid domain (`lead`/`trail` in `0x21..=0x7e`, 8,836 pairs) - and the same single-byte Katakana range (`0x21..=0x5f`, 63 bytes) on both sides, unlike the JIS X 0212 path which has no encoder-side table at all |
| Temporary in-crate brute-force test (`encoding/src/text.rs`, `#[cfg(test)]`, added and reverted this session, no bug found so nothing was kept as a patch - see `sweep_harness.rs` in this folder for the exact code that was run): every JIS X 0208 lead/trail pair (`0x21..=0x7e` x `0x21..=0x7e`, 8,836 pairs) prefixed with `ESC $ B`, decoded via `IsoIr87CharacterSetCodec::decode`, then re-encoded via `::encode`; separately, every Katakana byte (`0x21..=0x5f`, 63 bytes) prefixed with `ESC ( I` | `cargo test -p dicom-encoding --lib iso_ir_87_jis0208_and_katakana_sweep -- --nocapture`, debug build | 0 of 8,836 JIS X 0208 pairs and 0 of 63 Katakana bytes decode-but-fail-to-re-encode - the full valid domain of both escape paths round-trips cleanly, no asymmetry found |
| Full existing `dicom-encoding` test suite, with the temporary sweep test present | `cargo test -p dicom-encoding --lib` | 41 passed, 0 failed (40 pre-existing + the new sweep test) - no regression, no other signal |

## Status

**Not reproducible.** The brute-force sweep is an exhaustive enumeration of the entire valid input
domain both the decoder and encoder accept for these two escape paths (8,836 JIS X 0208 lead/trail
pairs = the full 94x94 JIS X 0208 code plane; 63 Katakana bytes = the full `0x21..=0x5f` range the
decoder's `Katakana` state accepts), not a sample - so this is a complete negative for this
specific bug class (decode-succeeds-but-encode-fails) within these two paths, not just an absence
of evidence from limited sampling. No further tool was tried past this one approach: per
`SKILL.md`'s soft cap, an exhaustive full-domain enumeration that comes back clean is a stronger
negative than three partial/probabilistic approaches would be, so escalating to a second/third
tool (e.g. a fuzz harness sampling the same already-fully-covered domain) would add no information.

## Class

n/a (no bug found).

## Root cause

Why this candidate's suspected class doesn't apply here, unlike JIS X 0212: `encoding-0.2.33`'s
`ISO2022JPEncoder` and `ISO2022JPDecoder` both route the JIS X 0208 path through the exact same
`index_japanese::jis0208::forward`/`backward` function pair over the exact same domain (`0x21..=
0x7e` square), so any character the decoder can produce from a valid two-byte code, the encoder can
by construction map back to *some* valid two-byte code (not necessarily the same bytes, since the
table can have many-to-one collisions collapsed by `backward`, but always *a* valid encoding - this
sweep only checks "does encoding succeed at all", which is exactly what candidate 104's JIS X 0212
finding hinges on, and here it always does). The Katakana path is a direct 1:1 offset mapping
(`ch as usize - 0xff61 + 0x21` in the encoder, the exact inverse of `0xff61 + b as u32 - 0x21` in
the decoder) with no table lookup at all, so it cannot be asymmetric by construction. JIS X 0212
was different specifically because its encoder-side code path does not exist at all (see candidate
104) - that is a structural gap this sweep confirms is not mirrored in the JIS X 0208/Katakana
paths, which do have a complete encoder.

## The fix

N/A - no bug found, so there is nothing to fix. The temporary sweep test was reverted (`git
checkout -- encoding/src/text.rs`) before this candidate folder was written; the main project's
working tree is clean.

- Project test suite: N/A (no fix applied); for reference, `cargo test -p dicom-encoding --lib`
  with the temporary sweep test present: 41 passed, 0 failed.

## What's not done / open questions

- This closes the specific `IDEAS.md` backlog item that flagged the `ESC $ B`/`ESC $ @` and
  `ESC ( I` paths as unswept siblings of candidate 104's JIS X 0212 finding - both are now
  confirmed clean within the byte ranges their own decoder/encoder accept.
- As with candidate 104's own `WINDOWS_949` sweep, this rules out the specific
  decode-succeeds/encode-fails asymmetry class within the exact domain checked; it is not a full
  audit of the `encoding-index-japanese` crate's `jis0208.rs` table data for other kinds of defects
  (e.g. an incorrect but self-consistent mapping, which a round-trip sweep cannot detect since it
  never compares against an external reference like the actual JIS X 0208 standard table). A
  differential test against a reference JIS X 0208 table (e.g. via Python's `codecs`/`iconv` with
  `iso2022_jp`) would be a more exhaustive way to catch that different class of bug, if ever
  pursued separately.
- Candidate 104's own JIS X 0212 finding (`ISO_IR 87` decode succeeds, re-encode fails,
  `EncoderTrap::Strict`) remains open/unfixed on `master` as of this session - this candidate does
  not change that finding's status, it only rules out the same bug class in the two sibling escape
  paths.
