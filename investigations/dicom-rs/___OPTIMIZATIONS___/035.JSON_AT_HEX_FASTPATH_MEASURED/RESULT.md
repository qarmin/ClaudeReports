# Candidate 027's `AsAttributeTags` hex-table fix, finally measured (was applied blind, 0 benchmarks)

Candidate 027's `patch.diff` (`json/src/ser/mod.rs`+`json/src/ser/value.rs`) has two parts sharing
one `tag_hex_bytes()` helper: (1) `DicomJson<Tag>`'s `Serialize` impl (element keys - measured,
strong win, `-25% to -28%`) and (2) `AsAttributeTags`'s `Serialize` impl (AT-VR element *values* -
a tag list, e.g. `AttributeIdentifierList`, `DimensionIndexPointer`) - applied "for consistency
with the same mechanism/correctness argument" but **never benchmarked**, since no
`dicom-test-files` fixture in this workspace's `testdata/` exercises the AT VR at all. `IDEAS.md`
flagged this explicitly as an open gap.

## What was tried

Added a new harness mode, `gen-at <out> <n-tags>` (harness-only, not part of `patch.diff`):
builds a synthetic object with one `ATTRIBUTE_IDENTIFIER_LIST` (AT VR) element containing
`n-tags` distinct `Tag` values. Uses Implicit VR Little Endian (not the harness's usual Explicit
VR LE default) because AT is a "short" VR in explicit encoding (2-byte length field, 65,535-byte
max) - too small to hold more than ~16,000 tags in one element; implicit VR's 4-byte length field
has no such limit. Two fixtures: `at_50k.dcm` (50,000 tags, stresses the mechanism at scale) and
`at_10.dcm` (10 tags, closer to a realistic `AttributeIdentifierList`/`DimensionIndexPointer`
element size). Re-applied candidate 027's `patch.diff` unchanged; two binaries (baseline, patched)
compared via `tools/paired.sh` (`RUNS=9`) on `json-encode`.

| Variant | 50,000 tags, round 1 | 50,000 tags, round 2 (re-check) | 10 tags |
|---|---|---|---|
| baseline | 2.046 ms p50 | 2.034 ms p50 | 1.53 us p50 |
| patched | 0.560 ms p50 | 0.572 ms p50 | 0.87 us p50 |

## Result

**A real, strong, fully consistent win at every scale tried** - both the large-scale mechanism
check and a realistic-sized fixture agree:

- 50,000 tags: **-72.6%** (round 1, 9/9), **-71.9%** (round 2 re-check, 9/9) - the largest
  relative win recorded anywhere in this workspace's `SUMMARY.md` to date.
- 10 tags (realistic `AttributeIdentifierList`-sized element): **-43.5%**, 9/9 - confirms the win
  isn't an artifact of the extreme 50,000-tag scale; it's real and large even for the object sizes
  this VR actually appears at in practice.

Time only was measured (this fix's mechanism - avoiding `format!`'s `Formatter`/`pad_integral`
machinery per tag - has no plausible memory-volume effect distinct from the already-tiny output
size; no peak-RSS follow-up taken).

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0x62f52819aeb4534` (FNV-1a of the JSON output) identical baseline
  vs. patched on `at_50k.dcm`, `json length: 550372 bytes` identical too.
- **Is this a breaking change?** No - `AsAttributeTags`'s `Serialize` impl is a private
  serialization detail of the `json` crate; the *output* format (uppercase hex tag strings) is
  unchanged from before candidate 027, only how it's produced.
- Project test suite: `cargo test --release -p dicom-json -p dicom-dump`: **12+5+4+1+3 = 25
  passed / 0 failed** (patch applied), exactly matching candidate 027's own originally-recorded
  count. Reverted-tree (baseline) count is identical.

`just test` was run for real from a fresh clone and reproduced the -72%-class win end-to-end,
checksums bit-exact.

## Why the result looks the way it does

The mechanism is identical to the already-confirmed `DicomJson<Tag>` win (candidate 027's own
callgrind profiling found `format!("{g:04X}{e:04X}")`'s `core::fmt::Formatter`/`pad_integral`
machinery was ~7.8% of total instructions for a 10k-*element* fixture) - `AsAttributeTags` calls
the equivalent `format!("{:04X}{:04X}", ...)` once **per tag inside a single AT value**, not once
per element, so a tag-list-heavy element amplifies the same per-call `Formatter` overhead even
further relative to the rest of that one element's (otherwise cheap) serialization work - explaining
why the relative win here (-72.6%) is larger than the already-strong per-*element* win candidate
027 measured for `seq_10k.dcm`/`rtplan.dcm` (-25% to -28%).

## What's not done / open questions

- No peak-RSS/allocation-count follow-up (see "Result" above for why this wasn't expected to
  matter for a fix this local).
- Only tested via a synthetic `gen-at` fixture - no real-world DICOM file in this workspace's
  `testdata/` happens to carry a populated AT-VR element to cross-check against; the synthetic
  fixture's tag values (`Tag(i/65536, i%65536)`) are not semantically realistic tag values, though
  this shouldn't matter for a pure hex-formatting benchmark.
- This closes the `IDEAS.md` backlog item asking for this measurement - no further action
  recommended, the result is a confirmed, robust, strong win.
