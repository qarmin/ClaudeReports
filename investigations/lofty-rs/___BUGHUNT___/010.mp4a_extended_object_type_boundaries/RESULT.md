# mp4a_extended_object_type_boundaries

`lofty/src/mp4/properties.rs`, `mp4a_properties` (~lines 841-895): the `frequency_index == 0x0F`
explicit-sample-rate path, the extended `object_type = 32 + ((byte_a & 7) | (byte_b >> 5))`
computation, and the `channel_conf` construction, in the same function that had the confirmed ALS
channel-count overflow (`003.mp4_als_channels_overflow/`). Carried over from `IDEAS.md` as a
follow-up: these bitfield combinations were only read, not independently boundary-tested or
fuzzed, in that earlier session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build (`overflow-checks = true`) + exhaustive manual boundary-value construction, driven through the real `lofty::mp4::Mp4File::read_from` entry point | A standalone scratch binary (`___BUGHUNT___/_reference/010_scratch/`) that builds a minimal real MP4/M4A `ftyp`/`moov`/.../`esds` structure per combo and parses it back. Exhaustive sweep over all 65536 `(byte_a, byte_b)` combinations (every value the extended-object-type/frequency_index/channel_conf bitfields can take), skipping only `object_type == 36` (the already-confirmed, separate ALS bug from candidate 003), crossed with 5 boundary `explicit_sample_rate` values (`0x000000`, `0x0000FF`, `0x7FFFFF`, `0x800000`, `0xFFFFFF`) and 4 boundary `byte_c` values (`0x00`, `0x7F`, `0x80`, `0xFF`) - 1,308,800 total crafted files, each parsed with a `catch_unwind` around the call | 0 panics out of 1,308,800 combinations (~10.5s runtime) |
| Short one-off fuzz burst on the project's own existing `fuzz/fuzz_targets/mp4file_read_from.rs` harness (real `lofty::mp4::Mp4File::read_from` entry point, unconstrained byte-level mutation of the whole file, not just the two bytes above) | `cargo +nightly fuzz run mp4file_read_from --sanitizer none -- -max_total_time=90` (per this project's `___BUGHUNT___/README.md` note that the default ASAN sanitizer build hangs in this sandbox; `--sanitizer none` still keeps debug-assertions/overflow-checks on) | 30,886,129 executions in 91s, 0 crashes, no artifacts written to `fuzz/artifacts/mp4file_read_from/` |

Two approaches were enough: the first was an exhaustive sweep of every value the suspect bitfields
can take (not a sample), and the second was a 30M+-execution general fuzz burst against the same
real entry point with no constraints at all - both came back clean with no partial signal (no
sanitizer warning, no flaky repro, nothing worth escalating a third tool for), so the soft cap
wasn't needed.

## Status

**Not reproducible**. Covered: an exhaustive sweep of all 65536 `(byte_a, byte_b)` combinations
(the full domain of the extended-object-type computation, `frequency_index`, and the initial
`channel_conf` bitfields) times 5 boundary `explicit_sample_rate` values times 4 boundary `byte_c`
values (1,308,800 total, 0 panics), plus a 30.9M-execution general-purpose fuzz burst on the real
`mp4file_read_from` target (0 crashes). A future session shouldn't need to re-check this same
bitfield space by hand.

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A (not Confirmed).

## Root cause

Not a bug: every operation in the suspect region is bitwise (`&`, `|`, `>>`, `<<`) or a bounded
addition, never a truncate-then-add pattern like candidate 003's confirmed ALS bug.

- `object_type = byte_a >> 3` is a `u8` in `0..=31`. When it's `31`, `object_type = 32 +
  ((byte_a & 7) | (byte_b >> 5))` adds a `u8` value bounded to `0..=7` (the OR of two 3-bit
  fields) onto the constant `32`, so the result is bounded to `32..=39` - nowhere near `u8::MAX`,
  no widening needed and none skipped.
- `frequency_index` is always a 4-bit field (`& 0x0F` or `<< 1 | ...>> 7`, both bounded to
  `0..=15`), used only as a match discriminant and, in the `< SAMPLE_RATES.len()` arm, as an
  already-bounds-checked array index.
- `explicit_sample_rate` comes from `AtomReader::read_u24`, which returns `u32` (confirmed by
  reading `lofty/src/mp4/read/atom_reader.rs:71-73`) holding a genuine 24-bit value
  (`0..=0xFFFFFF`). `sample_rate = explicit_sample_rate >> 1` or `<< 1` both stay far under
  `u32::MAX` (max `0x1FFFFFE`), and every `channel_conf` derived from it uses only `&`/`|` against
  `u8`-sized masks after already being masked down, so the intermediate values can never exceed a
  `u8`.
- `AudioObjectType::try_from(object_type)` just matches on the `u8` and returns `Err(())` for any
  unmapped value (including the whole `32..=39` extended range not exactly listed, e.g. `36` is
  ALS but say `31` itself can't recur since it was already consumed) - a graceful `Result`, not a
  panic path.

This confirms the pattern the candidate description already suspected on inspection ("no
widening-then-add pattern spotted") - the fix in candidate 003 was needed specifically because
`stsd.read_u16()? as u8 + 1` truncated a `u16` to `u8` *before* adding; nothing in this candidate's
suspect region does that.

## The fix

N/A - no bug found, nothing to fix.

- Project test suite: not run (no source change was made; nothing to verify against).

## What's not done / open questions

- Only `mp4a_properties`'s AAC/`esds` parsing was covered here. The sibling `alac_properties` and
  the top-level `stsd` sample-entry parsing (outside this function) weren't swept - they don't
  share this exact bitfield shape, so weren't in scope for this candidate.
- The general fuzz burst (30.9M execs, 90s) is still a short one-off per this skill's own scope
  (see `SKILL.md`'s "When NOT to use" - not a persistent-corpus campaign) - a longer run via
  `fuzz-bug-repro` with a saved corpus could still be attempted later if this area is revisited,
  though the exhaustive bitfield sweep already covers this candidate's exact suspect region more
  thoroughly than a longer random fuzz run would.
- The scratch sweep binary is left at `___BUGHUNT___/_reference/010_scratch/` (session-only
  helper material, not part of this candidate's deliverable) for reuse if a similar bitfield-space
  sweep is needed elsewhere in the MP4 parser.
