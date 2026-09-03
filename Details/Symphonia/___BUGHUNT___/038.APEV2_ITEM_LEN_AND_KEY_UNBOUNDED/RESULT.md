# APEv1/APEv2 item length, key length and num_items taken unbounded from the tag

`symphonia-metadata/src/ape.rs` reads three counts straight out of an untrusted tag and uses them
without any upper bound: the per-item value length (`let len = reader.read_u32()? as usize;`,
`ape.rs:253`, consumed by `reader.read_boxed_slice_exact(len)` / `read_utf8_value(reader, len)` at
`:271-275`), `ApeHeader::num_items` (a raw `u32` driving the item loop at `:386`), and `read_key`'s
NUL-terminated key loop (`:459-479`), whose own `TODO` at `:471` admits the documented
255-character limit is not enforced. Nothing cross-checks either count against `header.size`, and
the item reads go against the raw `MediaSourceStream` rather than a `ScopedStream` bounded by the
tag size. This is the "untrusted 32-bit count trusted for allocation" shape that
`symphonia-format-isomp4` already defends against with `MAX_TABLE_INITIAL_CAPACITY`
(`atoms/stsc.rs:64`, `stsz.rs:38`, `stco.rs:27`), and `ApeReader` is a public standalone
`ProbeableMetadata` reader with no `cargo-fuzz` target in `symphonia/fuzz/`.

The suspicion tested here: a crafted tag forces a multi-GB allocation (or an OOM/panic) out of a
small file, and/or `read_key` grows a `String` until memory runs out.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the whole read path, including what the item reads actually bottom out in | manual read of `ape.rs`, `symphonia-core/src/io/mod.rs`, `symphonia-core/src/formats/probe.rs` | Both item-value reads go through `ReadBytes::read_boxed_slice_exact`, whose default implementation is `safe_read_into_boxed_slice` (`symphonia-core/src/io/mod.rs:533-590`) - explicitly hardened: it never preallocates `len`, it grows 4 MiB then doubling (1 GiB cap) and uses `try_reserve_exact`, returning `io::ErrorKind::OutOfMemory` instead of panicking. `MediaSourceStream` does not override it (only `buf_reader`/`scoped_stream`/`monitor_stream` do). So the declared length only bounds the read from above; the real bound is how many bytes the stream has |
| Debug-assertions/overflow-checks build + manual boundary-value construction, 40 crafted tags in one pass | `release` with `debug-assertions = true` + `overflow-checks = true`, inputs: item `len` in {0, 1, u32::MAX-1, u32::MAX} x all 4 value types, `num_items` in {0, 1, 0xFFFF, u32::MAX}, `size` in {0, 1, 31, 32, 33, u32::MAX} with and without the `has_header` `size + 32` arithmetic, unterminated keys of 255 B / 4 KiB / 1 MiB / 8 MiB, APEv1 and APEv2, footer-anchored and header-first | No panic, no overflow, no hang anywhere. Every hostile count produced a clean `Err` (`unexpected end of file`, `ape: header and footer mismatch`, `ape: invalid preamble`) or an `Ok` with the honestly-declared number of tags |
| Peak-RSS measurement against real files (`VmHWM`, one child process per case so the high-water mark is not polluted), 16 MiB and 64 MiB crafted files, plus an ID3v2 control of the same size | `release` + `overflow-checks`, `debug-assertions`, files read through a real `std::fs::File`-backed `MediaSourceStream`; 4 independent runs (3 by hand + 1 via `just test` against a fresh clone) | `item-len=u32::MAX` (4 GiB claimed) peaked at 2.8-2.9x the file size, `read_key` with no NUL for the whole file at 2.0x - i.e. bounded by the stream, with the multiple being the doubling-realloc transient, not the 4 GiB claim. `num_items=u32::MAX` peaked at 17.9x the file size - but an *honestly* declared tag with the same item stream peaks at exactly the same 17.9x, and an equally sized ID3v2.3 tag of minimal frames through `Id3v2Reader` peaks at 13.8x |
| One-off `cargo-fuzz`/libFuzzer target for `ApeReader::try_new` + `read_all`, driven at offset 0 and at the two real probe anchors (-32 and -160) | `cargo-fuzz 0.12.0`, nightly, ASAN + `debug-assertions` + `overflow-checks`, `-max_len=65536`, `-rss_limit_mb=2048`, `-timeout=20`, seeded with 7 hand-built valid tags (APEv1 text-only, APEv2 footer-only, APEv2 header-first, APEv2 header+footer, APEv2 before an ID3v1 trailer, item-less APEv2; the APEv2 seeds carry text/list/locator items, a `replaygain_track_gain` item that reaches a `RawTagParser`, and PNG + JPEG `Cover Art` binary items that reach `try_parse_image_data`) - seeds verified to parse to 6 tags + 2 visuals before fuzzing. Two runs: 15,096,191 execs / 331 s and 18,699,810 execs / 401 s (each exec drives 3 parses, so ~101M parses total) | No crash, no OOM, no leak, no hang (`slowest_unit_time_sec: 0`), no artifacts written. Coverage plateaued at `cov: 132 ft: 328-341` within the first seconds and never moved |
| Reachability check through the real end-to-end public API | `symphonia::default::get_probe().probe(...)` on (a) a valid WAV with a hostile APEv2 footer appended, (b) a valid WAV with a hostile APEv2 header leading it | Both genuinely reach `ApeReader::read_all` (confirmed by the RSS/time signature matching the direct-reader cases). (a) still returns a working `FormatReader` - `Probe::probe_trailing` deliberately swallows trailing-metadata errors (`probe.rs:520-530`) and repositions the stream; (b) propagates the error and the probe fails |

## Status

**Not reproducible.**

No crash, panic, arithmetic overflow, hang or OOM could be produced from any of the three
unbounded counts, in ~101 million fuzzed parses plus 40 hand-constructed boundary-value tags plus
the peak-RSS measurements above. Everything malformed exits through a clean `Err`.

What *was* measured, repeatably (4/4 runs, linear in file size across 16 MiB and 64 MiB, numbers
within 0.1x of each other run to run):

- A declared item value length of `u32::MAX` allocates **2.8-2.9x the file size**, not 4 GiB. A
  4 GiB claim behind a 47-byte tag fails with `unexpected end of file` after touching 4 MiB
  (`repro/`'s `tiny-stream check`).
- `read_key` with no NUL terminator for the whole file allocates **2.0x the file size** - the
  255-character cap really is unenforced (a 64 MiB key was built), but the growth is 1:1 with
  bytes the attacker had to supply anyway.
- `num_items = u32::MAX` with a `size` field declaring a 32-byte (item-less) tag really does read
  items far past the tag's own declared extent, and peaks at **17.9x the file size**. But the
  identical item stream declared *honestly* (`size` and `num_items` both truthful) peaks at the
  same 17.9x, so a `num_items`-vs-`size` cross-check would not change the resource cost at all;
  and `Id3v2Reader` fed an equally sized tag of minimal frames peaks at 13.8x, so the per-byte
  amplification is a property of storing one `Tag` per parsed item/frame shared with the crate's
  other standalone reader, not something specific to APE.

Coverage of the fuzzing, stated explicitly per `CONFIRMATION.md` section 10: 33.8M libFuzzer execs
(~101M `read_all` calls) over 732 s total, `-max_len=65536`, ASAN + overflow-checks, seeded corpus
of 7 valid tags spanning both versions, all three probe positions, and the text/list/locator/binary
item types. That is enough to say "no cheap crash reachable from a ≤64 KiB APE tag", not "this
parser has no bugs" - the coverage plateau at 132 edges suggests the mutator saturated the reachable
paths quickly, which is consistent with how little branching `ape.rs` has, and a longer campaign via
`fuzz-bug-repro` would add little for this file specifically.

**Fully reproducible**: n/a (nothing to reproduce). `repro/` is the negative-result harness: it
builds the crafted files, drives them through the real public entry points, and prints the numbers
above, so a future session can re-check the conclusion in one command instead of re-deriving it.

## Class

n/a - Not reproducible.

## Root cause

Why the suspect turns out to be safe, so this spot is not re-suspected:

1. **The 32-bit item length never drives an allocation.** `ape.rs` calls
   `ReadBytes::read_boxed_slice_exact(len)`, and that trait method's default body is
   `safe_read_into_boxed_slice` (`symphonia-core/src/io/mod.rs:533`), which was written precisely
   for this threat: `INIT_MAX_READ_LEN = 4 MiB` for the first chunk, doubling up to
   `ABS_MAX_READ_LEN = 1 GiB`, each chunk obtained with `try_reserve_exact` and mapped to
   `io::ErrorKind::OutOfMemory` on failure instead of panicking. `MediaSourceStream` uses that
   default. The consequence is that `len` is an upper bound only - the buffer can never exceed the
   number of bytes the stream actually yields, so `read_utf8_value(reader, u32::MAX as usize)`
   against a short file costs one 4 MiB buffer and returns `Err`. This is the difference from
   isomp4's `stsc`/`stsz`/`stco`, which call `Vec::with_capacity`-style preallocation directly and
   therefore genuinely needed `MAX_TABLE_INITIAL_CAPACITY`.
2. **`read_key` is unbounded but not amplifying.** It grows one `String` by one byte per byte
   consumed and stops at the first NUL or at EOF, so its peak is the file's own size (measured 2.0x
   including the doubling transient). The missing 255-character cap is a spec-conformance gap, not a
   resource bug.
3. **`num_items` is unbounded, and items are genuinely read past the declared tag extent, but
   neither costs anything an honest tag could not.** Each item needs at least 9 physical bytes in
   the file (4-byte length + 4-byte flags + a bare NUL key terminator), so `num_items` cannot
   outrun the file; the loop simply fails at EOF. The 17.9x figure is `sizeof(Tag)` plus the
   `Arc<String>` behind its value per 9 input bytes, and it is the same for a truthfully declared
   tag and 1.3x the equivalent `Id3v2Reader` figure.
4. **Neither the leading nor the trailing probe path turns a bogus tag into anything worse than an
   error.** `Probe::probe_trailing` explicitly ignores a failing trailing metadata reader and
   repositions the stream (`probe.rs:520-530`), and the leading path propagates the `Err` so the
   probe just fails. The only arithmetic on the untrusted size (`size + if has_header { 32 }`, and
   `-(header.size as i64)` for the seek-back) is done in `u64`/`i64` on a value that came from a
   `u32`, so it cannot overflow.

## Introduced by

n/a - no bug to attribute.

## The fix

None. No patch was produced, because no defect was reproduced.

- Project test suite: `cargo test --workspace` in the worktree at `ee35874` - **106 passed, 0
  failed** (0 ignored). No source change was made, so there is no "after" run that differs; the
  baseline is recorded to show the tree was healthy while this candidate was investigated.

## What's not done / open questions

- **`read_all` never checks that the item loop consumed exactly `header.size` bytes.** The comment
  at `ape.rs:435-438` claims the final footer read "checks the entire APE tag was read", but the
  only check is field equality between the header and the footer - not a byte-count/position check.
  A leading APEv2 tag can therefore swallow a chosen number of bytes of the *following* stream as
  items (with the probe then resuming after them) provided the attacker also plants a matching
  32-byte footer there. That is a mis-parse of a file the attacker authored anyway, so it was not
  pursued as a candidate here, but it is the one behavioural (not resource) consequence of the
  missing `ScopedStream`/size cross-check and would be a reasonable separate candidate.
- `read_key` silently *drops* bytes outside `0x20..=0x7e` instead of rejecting the key
  (`ape.rs:474`), so two distinct keys in a file can collapse to the same parsed key. Not
  investigated.
- The 17.9x (APE) / 13.8x (ID3v2) tag-storage amplification is a crate-wide property of
  `MetadataBuilder`, not an APE issue; if it is ever considered worth bounding, the right place is
  a shared cap in `symphonia-core`'s `MetadataBuilder`/`MetadataOptions`, not `ape.rs`. Deliberately
  not written up as a bug here since APE is not an outlier among the crate's own readers.
- The fuzz target built for this candidate (`ApeReader` at offset 0 / -32 / -160, seeded corpus) is
  worth upstreaming into `symphonia/fuzz/` - there is still no fuzz target for any standalone
  metadata reader (`ape`, `id3v1`, `id3v2`) in this repo. Its full source is inlined in the section
  at the end of this file rather than shipped as a second crate in this folder.
- Only `x86_64-unknown-linux-gnu` was exercised. On a 32-bit target `len as usize` still fits
  (`u32::MAX` == `usize::MAX`), and `safe_read_into_boxed_slice`'s behaviour is unchanged, so no
  target-specific difference is expected - but it was not measured.

### The fuzz target used (for re-use / upstreaming)

```rust
#![no_main]
use std::io::{Cursor, Seek, SeekFrom};

use libfuzzer_sys::fuzz_target;
use symphonia_core::io::{MediaSource, MediaSourceStream};
use symphonia_core::meta::{MetadataOptions, MetadataReader};
use symphonia_metadata::ape::ApeReader;

fn drive(data: Vec<u8>, from_end: Option<i64>) {
    let mut mss = MediaSourceStream::new(Box::new(Cursor::new(data)), Default::default());
    if let Some(off) = from_end {
        // Emulate exactly what the probe does for a footer-anchored APE tag: seek to
        // `byte_len - off` before constructing the reader.
        let Some(len) = mss.byte_len() else { return };
        if len < off as u64 {
            return;
        }
        if mss.seek(SeekFrom::Start(len - off as u64)).is_err() {
            return;
        }
    }
    if let Ok(mut r) = ApeReader::try_new(mss, MetadataOptions::default()) {
        let _ = r.read_all();
    }
}

fuzz_target!(|data: Vec<u8>| {
    drive(data.clone(), None);
    drive(data.clone(), Some(32));
    drive(data, Some(160));
});
```

Run it with symbolization disabled - `ASAN_OPTIONS=symbolize=0` - otherwise libFuzzer's first
`NEW_FUNC` line forks a symbolizer that never returns in this environment and the run stalls at
~180 execs with a child process spinning at 100% CPU. With it disabled the target sustains ~47k
exec/s.
