# dicom-json serialize: hand-written hex encoding for tag keys instead of `format!`

Found while profiling `json-encode` with callgrind after candidate 026's fix (to see what's now
hottest): `<core::fmt::Formatter>::pad_integral` (3.56%), `core::fmt::write` (2.38%), and
`<u16 as core::fmt::UpperHex>::fmt` (1.90%) - roughly 7.8% combined - traced to
`json/src/ser/mod.rs`'s `impl Serialize for DicomJson<Tag>`: `serializer.serialize_str(&format!
("{g:04X}{e:04X}"))`. This runs once per data element being serialized (the JSON key, e.g.
`"00080018"`) - a real hot path for element-heavy documents, going through `format!`'s full
`Arguments`/`Formatter`/width-padding machinery (designed for arbitrary user formatting, not a
fixed 4-hex-digit-per-field case) plus a `String` heap allocation, for what is structurally just
"look up 8 hex nibbles in a 16-entry table."

`json/src/ser/value.rs`'s `AsAttributeTags::serialize` (used for the `AT` VR - a value that is
itself a list of tags) has the exact same `format!("{:04X}{:04X}", ...)` pattern per tag - fixed
in the same patch for consistency, though not itself profiled as hot (no fixture exercised it).

## What was tried

Added `pub(crate) fn tag_hex_bytes(tag: Tag) -> [u8; 8]` (`json/src/ser/mod.rs`) - a 16-entry
`b"0123456789ABCDEF"` lookup table indexed by each of the 8 nibbles (4 from `group`, 4 from
`element`), building a stack-allocated `[u8; 8]` with no heap allocation and no generic formatting
machinery. `DicomJson<Tag>`'s `Serialize` impl now builds this array and calls
`serializer.serialize_str` on it directly (`unsafe { str::from_utf8_unchecked(&buf) }` - sound
since every byte is one of the ASCII hex digit bytes in the table, always valid UTF-8).
`AsAttributeTags::serialize` reuses the same helper (still allocates a `String` per tag via
`buf.to_vec()`, since `collect_seq` needs owned `Serialize` items, but skips the `format!`
machinery for each one).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s existing `json-encode` mode, against 3
fixtures spanning different element-count/size profiles: `seq_10k.dcm` (10,000 SQ items, ~20,000+
tag serializations), `rtplan.dcm` (real fixture, 2.7KB, many small text/numeric elements, few
large values), `liver.dcm` (real fixture, 102KB, dominated by one large native `PixelData`
InlineBinary-base64 value, relatively few top-level tags).

| Fixture | baseline p50 | patched p50 | delta |
|---|---|---|---|
| `seq_10k.dcm` (2 rounds) | 2.206/2.207 ms | 1.646/1.630 ms | -25.4%/-26.1%, 9/9 both |
| `rtplan.dcm` | 0.014 ms | 0.010 ms | -28.2%, 9/9 |
| `liver.dcm` (2 rounds) | 0.132/0.129 ms | 0.139/0.135 ms | +5.4%/+4.8%, 0/9 both (REGRESSION) |

## Result

**Mixed.** Element-heavy files see a large, fully consistent win: `seq_10k.dcm` -25.4%/-26.1%
across two rounds (9/9 both), `rtplan.dcm` -28.2% (9/9). **`liver.dcm` shows a small but fully
consistent REGRESSION** (+4.8%/+5.4% across two rounds, 0/9 both - the patch is slower every
single paired run). The regression is small in absolute terms (~6-8 microseconds on a ~130
microsecond baseline) but real and reproducible, not noise - `uptime` load average was 1.8-1.9
throughout, no unusual system load.

## Does it change behavior/output?

- **Bit-exact?** Yes - checksum (FNV-1a of the encoded JSON bytes) identical between baseline and
  patched builds on all 3 fixtures. The hex digits produced are byte-for-byte identical to
  `format!("{:04X}", ...)`'s output (both uppercase, both zero-padded to 4 digits, no separators).
- **Is this a breaking change?** No public API/type changes - `tag_hex_bytes` is `pub(crate)`,
  and both `Serialize` impls it's used from were already producing the same output format.
- Project test suite: `cargo test --release -p dicom-json -p dicom-dump`: **12 + 3 + 5 (json) + 4
  + 1 (dump) = 25 passed / 0 failed**, identical to baseline. `cargo build --release --workspace`:
  clean, no errors, no new warnings.

## Why the result looks the way it does

For files with many small elements (`seq_10k.dcm`'s 20,000+ tag serializations, `rtplan.dcm`'s
denser-than-`liver.dcm` element mix relative to its total size), tag-key serialization is a real,
non-trivial fraction of total encode time, and removing `format!`'s heap allocation plus its
generic `Formatter`/padding dispatch pays off directly and substantially. **The `liver.dcm`
regression is NOT explained by this candidate's own mechanism** - the file has few top-level tags
(the `info` mode shows only 7 attributes plus file-meta elements) and one large `PixelData` value
that goes through the entirely separate, untouched `InlineBinary`/base64 path - a fixed few-tag
improvement should be a tiny, unmeasurable-or-positive effect on a ~130 microsecond total, not a
reproducible 0/9 regression. The leading, **unconfirmed** hypothesis (not verified this session,
consistent with candidate 006's own similar unresolved regression) is a code-layout/inlining
side effect: adding a new function and changing `DicomJson<Tag>`'s codegen shifts instruction
placement/cache behavior for the whole `dicom-json` crate in a way that happens to cost more for
this specific file's particular code-path mix (dominated by the base64 path, not the tag path) -
plausible given `pad_integral`'s removal changes what LLVM inlines where, but not confirmed via
disassembly comparison in this session.

## What's not done / open questions

- **`liver.dcm`'s regression root cause is unconfirmed.** A quick `nm -C -S`/binary-size check was
  done (bounded, ~5 minutes): total binary size is nearly identical between baseline and patched
  (166,326,160 vs 166,320,760 bytes - patched is actually very slightly *smaller*), and the
  `pad_integral` symbol instantiation count is identical (3 in both). **This does NOT support the
  simple "code got bigger, worse icache behavior" version of the code-layout hypothesis** the way
  candidate 006's own 18%-larger-function finding did - whatever's causing this regression (if
  it's real and not some subtler noise this session's paired-run discipline didn't fully rule out)
  is not explained by gross code size. A full `objdump -d -C` line-by-line comparison of the
  actually-executed code path was not attempted (out of this candidate's own bounded time for
  chasing the regression, per SKILL.md's guidance) - left open, now with one hypothesis already
  ruled out for a future session to build on rather than re-check.
- Not measured: whether the regression is specific to `liver.dcm`'s exact structure or a broader
  "large-binary-value-dominated, few-tag" pattern - only one such fixture was tried.
- The `AsAttributeTags` (`AT` VR) fix was applied for consistency (same mechanism, same
  correctness argument) but not independently benchmarked - no available fixture exercises it,
  and it wasn't hot in any profile taken this session.
- Peak RSS not measured - the mechanism removes a `String` heap allocation per `Tag` serialized
  (replaced with a stack array), which is exactly the class of change `projects/dicom-rs.md`
  flags for a peak-RSS check, but for 8-byte strings the effect (if any) is expected to be far
  below `VmHWM`'s useful resolution - not measured, per the same reasoning as candidate 016's own
  "each individual buffer is tiny, peak RSS was never going to move here."
