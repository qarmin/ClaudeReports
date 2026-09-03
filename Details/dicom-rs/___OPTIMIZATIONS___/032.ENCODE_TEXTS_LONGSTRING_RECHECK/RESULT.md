# Candidate 024's single-value fast path, re-checked with much longer per-element strings

`StatefulEncoder::encode_texts_element`'s single-value fast path (candidate 024's patch, applied
unchanged here - `parser/src/stateful/encode.rs`) skips one `self.buffer.clear()` +
`extend_from_slice` memcpy for single-valued text elements (the common case for CS/UI/PN/LO/etc.
in real DICOM data). Candidate 024 found **no consistent effect** on `seq_10k.dcm`/`seq_100k.dcm`,
but those fixtures' text values are only 20-40 bytes (short UID-style strings) - too small for a
per-element memcpy-avoidance effect to plausibly clear this box's noise floor. `IDEAS.md` flagged
this explicitly as unresolved: does a longer per-element string change the answer?

## What was tried

Extended the harness's `gen-seq` mode (harness-only change, not part of `patch.diff`) with an
optional `str-len` argument: each of the 10,000 sequence items also gets a single-valued LO
element (`InstitutionName`) of exactly that many bytes, so the skipped memcpy scales with a
controlled, much larger value size than candidate 024's original fixtures ever exercised.
Generated `seq_10k_long300.dcm` (300-byte strings, 3.88MB) and `seq_10k_long1000.dcm` (1000-byte
strings, 10.88MB) - roughly 8x-25x the original value length.

Two binaries built from the same source tree, one with candidate 024's `patch.diff` applied, one
without; compared via `tools/paired.sh` (alternating-run, `RUNS=9`).

| Variant | write-only, 300B strings (unpinned) | write-only, 1000B strings (unpinned) | write-only, 1000B strings (`taskset -c 2`) | roundtrip, 1000B strings (unpinned, round 1) | roundtrip, 1000B strings (round 2, re-check) |
|---|---|---|---|---|---|
| baseline | 18.34 ms p50 | 33.36 ms p50 | 34.39 ms p50 | 59.48 ms p50 | 60.29 ms p50 |
| patched | 18.42 ms p50 | 33.42 ms p50 | 34.55 ms p50 | 60.14 ms p50 | 60.39 ms p50 |

## Result

**No consistent change in any configuration measured** - same conclusion as candidate 024, now
confirmed at 8x-25x the original string length:

- `write-only`, 300B strings, unpinned: +0.4%, 3/9 - not consistent.
- `write-only`, 1000B strings, unpinned: +0.2%, 3/9 - not consistent.
- `write-only`, 1000B strings, `taskset -c 2` (single-core): +0.5%, 5/9 - not consistent.
- `roundtrip`, 1000B strings, round 1: +1.1%, 1/9 - clears the script's own "consistent" bar (in
  the regression direction).
- `roundtrip`, 1000B strings, round 2 (re-run to check reproducibility, per MEASUREMENT.md's
  paired-run discipline): +0.2%, 4/9 - does NOT clear the bar this time, contradicting round 1.

The one "consistent" result (`roundtrip` round 1) did not reproduce on a second round, so it is
noise, not a real regression - the same "measure twice before trusting a single consistent verdict"
lesson this workspace has hit before (see `projects/dicom-rs.md`'s candidate 009/025 note on
below-noise-floor timescales, though this case is at a much larger, tens-of-ms timescale, not
microseconds). Time only was measured - no allocation-count/peak-RSS follow-up, since the patch's
own mechanism (skip one memcpy of an already-allocated buffer, no allocation added or removed) has
no plausible memory-volume effect to check.

## Does it change behavior/output?

- **Bit-exact?** Yes - `write-only` checksums (`checksum=0x...`, FNV-1a of the written bytes)
  identical between baseline and patched on both `seq_10k_long300.dcm` and
  `seq_10k_long1000.dcm`.
- **Is this a breaking change?** No - `encode_texts_element` is a private method of `parser`'s
  internal `StatefulEncoder`, same as candidate 024.
- Project test suite, patch applied: `cargo test --release -p dicom-parser -p dicom-object`:
  75 + 9 + 22 + 49 + 0(doctests) = **155 passed / 0 failed**, matching the counts candidate 024
  originally recorded for the same crates. Reverted-tree (baseline) counts are identical (same
  numbers, unaffected by this private-method change either way).

This candidate's own `patch.diff` is candidate 024's patch, applied unchanged - the real fix
under test here is unmodified from that candidate. The only new thing in this candidate is the
harness's `gen-seq str-len` extension (harness-only, not part of `patch.diff` - see
`../README.md`'s mode table, updated).

`just test` was run for real from a fresh clone (`repo_url` = the upstream `Enet4/dicom-rs.git`
URL, matching this workspace's standing convention for every other candidate's justfile - `origin`,
the qarmin fork, would fail since its own `master` branch is stale and doesn't reach this
`base_commit`) and reproduced the null result end-to-end, checksums bit-exact both fixture sizes.

## Why the result looks the way it does

Confirms candidate 024's own explanation still holds at a larger scale: the fast path only
removes one `memcpy` of an already-allocated buffer (the codec's own `convert_text_untrailed`
allocation is unchanged either way) - a single `memcpy` of even 1000 bytes is far cheaper than the
rest of a `write_all`/`roundtrip` iteration's work (element header encoding, VR dispatch, syscall
buffering, and - for `roundtrip` - a full re-parse), so its effect stays below this box's
measurement noise floor even at 25x the original value length. Larger still (10-100KB per value)
would be a more decisive test, but that stops looking like realistic DICOM text-VR content (LO/SH/
PN/UI/CS are all capped at 64 bytes by the DICOM standard itself, except UC which is unbounded -
the 300B/1000B fixtures here already exceed every VR this fast path applies to except UC).

## What's not done / open questions

- UC (Unlimited Characters) is the only VR among this fast path's targets with no length cap in
  the standard - a fixture with realistic UC-sized content (tens of KB) was not built; the 1000B
  fixture here already exceeds the 64-byte cap of every *other* affected VR, so it's arguably
  already testing an unrealistic case for those, but a genuinely large single UC value remains
  untested.
- This closes the `IDEAS.md` backlog item asking for a longer-string re-check; no further action
  recommended - the result is a confirmed, robust null, not a hidden win needing yet another
  fixture size.
