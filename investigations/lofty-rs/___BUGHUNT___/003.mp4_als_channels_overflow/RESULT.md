# mp4_als_channels_overflow

`lofty/src/mp4/properties.rs:912`, in the ALS (`object_type == 36`) branch of MP4 `mp4a`/AAC
properties parsing: `properties.channels = Some(stsd.read_u16()? as u8 + 1);` truncates an
attacker-controlled `u16` "channel count minus 1" field to `u8` via `as u8` before adding 1 with
no widening/checked arithmetic - a candidate carried over from `IDEAS.md`, originally flagged from
static reading plus a standalone rustc program confirming the arithmetic pattern in isolation.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build + a hand-crafted, real MP4/M4A file on disk read through `lofty::read_from_path` | `cargo run` (dev profile, default `overflow-checks = true`); a 208-byte file with `ftyp`/`moov`/`trak`/`mdia`/`mdhd`/`hdlr`/`minf`/`stbl`/`stsd`/`mp4a`/`esds` atoms, `esds`'s `AudioSpecificConfig` encoded to select `object_type == 36` via the extended-object-type path, ALS ident `"\0ALS\0"`, trailing channel-count-minus-1 `u16` = `0xFFFF` | Panicked immediately: `attempt to add with overflow` at `properties.rs:912:52`, reproduced 3/3 runs (see `output.txt`) |

Only one approach was needed - the panic pattern was already independently verified in isolation
(per the candidate's own note), and reproducing it through the real end-to-end parser on the first
attempt was a clean positive with no ambiguity, so the soft cap of escalating tools was not needed.

## Status

**Confirmed**. Reproduces 3/3 runs, debug build (default `cargo` dev profile, `overflow-checks =
true`), deterministic (no RNG/seed involved - the crafted file bytes are fully hand-constructed and
fixed).

**Fully reproducible**: **Yes**. The repro builds a real 208-byte MP4/M4A file on disk and reads
it back with `lofty::read_from_path` - lofty's real public entry point, exercising the actual
`Mp4File::read_from` -> `AtomReader` -> `read_stsd` -> `mp4a_properties` parsing path any caller
opening an untrusted `.m4a`/`.mp4` file would go through. Per `CONFIRMATION.md` section 11, this is
the "malformed file handed to a parser's real public entry point" case: the crafted bytes on disk
are the genuine external input, there is no second participant to fake here (unlike, say, a
client/server protocol finding).

## Class

crash

## Root cause

`mp4a_properties` (in `lofty/src/mp4/properties.rs`) parses an MP4 `esds`/`AudioSpecificConfig`
descriptor chain. When `object_type == 36` (ALS, Audio Lossless Coding), it reads a trailing `u16`
"channel count minus 1" field straight off the untrusted stream and computes:

```rust
properties.channels = Some(stsd.read_u16()? as u8 + 1);
```

The `as u8` cast truncates the `u16` to its low byte *before* the `+ 1` executes. Any file where
that `u16`'s low byte is `0xFF` (e.g. on-disk value `0x00FF` or `0xFFFF`) truncates to `255u8`,
and `255u8 + 1` overflows a `u8`. Under this project's default dev/test profile (and cargo-fuzz
builds), `overflow-checks = true` turns this into a panic; in a release build it would silently
wrap instead (e.g. reporting 0 channels for a file that actually declared 256).

The surrounding sibling field, `properties.channels` set a few lines above from `channel_conf`
(itself already a `u8`, never overflowing since it's built from 4-bit fields), is unrelated and
not affected - only this second, ALS-specific assignment at line 912 has the truncate-then-add
bug.

## Introduced by

d530fc31 "MP4: Fix reading of sample rates for object types > 31" (2022-03-09) - `git log -S"object_type == 36"` finds this as the commit that introduced the ALS (`object_type == 36`) branch, with the `read_u16()? as u8 + 1` truncating cast already present.

## The fix

Widen to `u32` before adding, then saturate down to `u8` (the field is `Option<u8>`, so any real
channel count above `u8::MAX` can't be represented exactly regardless - saturating instead of
wrapping/panicking is the correct degrade):

```rust
// This is a channel count minus 1, widen before adding to avoid
// overflowing when the on-disk value is >= 255 (the field can only
// represent up to `u8::MAX` channels, so saturate rather than wrap)
let channels = u32::from(stsd.read_u16()?) + 1;
properties.channels = Some(channels.min(u32::from(u8::MAX)) as u8);
```

Verified against the same crafted file plus a boundary case reasoning check: `0xFFFF` (the value
used in the repro) now saturates to `255` instead of panicking or wrapping to `0`; values below
`0xFF` low byte (e.g. `0x0000` -> 1 channel) are unaffected and still compute correctly, since
`u32` arithmetic can't overflow for any `u16` input (max `65535 + 1 = 65536`, far under `u32::MAX`).

- Project test suite (`cargo test -p lofty`, includes unit tests + doctests):
  before (bug present) = 176 passed; 0 failed; 23 ignored; 0 measured; 0 filtered out.
  after (patch applied) = 176 passed; 0 failed; 23 ignored; 0 measured; 0 filtered out.
  No new failures, no regressions.

## What's not done / open questions

- The sibling `properties.channels = Some(reader.read_u16()? as u8)` at line 600 (top-level `stsd`
  sample entry channel count, before any codec-specific branch) does the same `u16 -> u8` cast but
  *without* a `+ 1`, so it can't overflow - not a bug, just a similarly-shaped cast worth noting so
  a future session doesn't re-flag it without checking.
- `channel_conf` (built from 4-bit sub-fields throughout `mp4a_properties`) is bounded to `0..=15`
  by construction and was read but not independently boundary-tested; it doesn't share this bug's
  shape (no addition after truncation).
- Only the `object_type == 36` (ALS) branch was checked; the surrounding AAC-specific parsing
  (`frequency_index == 0x0F` explicit-sample-rate path, extended object type computation) reads
  several other untrusted bytes but was reviewed, not independently fuzzed this session - a short
  `cargo fuzz run mp4file_read_from` burst against this file area could still be worth a follow-up
  if a longer campaign is ever run via `fuzz-bug-repro`.
