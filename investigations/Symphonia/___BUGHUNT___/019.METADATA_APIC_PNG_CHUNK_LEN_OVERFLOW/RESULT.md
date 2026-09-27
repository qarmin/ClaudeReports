# APIC/PNG chunk-length arithmetic overflow in symphonia-metadata's embedded-image parser

This project's own knowledge file (`~/.claude/skills/bug-hunt/projects/symphonia.md`) flags
`symphonia-metadata`'s standalone metadata parsers (ID3v2/APEv2/etc., used outside a demuxer
context) as lacking any `cargo-fuzz` target in `symphonia/fuzz/`. Manually reading these parsers
for the same OOB-index/unwrap-panic/overflow classes that produced confirmed bugs #001/#004 in
this crate turned up an arithmetic overflow in `symphonia-metadata/src/utils/images.rs`'s PNG
parser, which `id3v2/frames/readers.rs::read_apic_frame` calls on the raw image bytes embedded in
an `APIC`/`PIC` frame (and `ape.rs`'s "Cover Art" binary items call the same function).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read code against overflow/untrusted-arithmetic heuristics across `symphonia-metadata/src/**` (`grep` for `unwrap`/`expect`/`[0]`/`panic!`/`unreachable!`, then manual read of the standalone parsers: `ape.rs`, `id3v1.rs`, `id3v2/mod.rs`, `id3v2/frames.rs`, `id3v2/frames/readers.rs`, `utils/images.rs`) | manual read, no build | Found `parse_png`'s indexed-color-type chunk-skip loop (`utils/images.rs:156`) computing `chunk_len + 4` in `u32` arithmetic (chunk_len is an attacker-controlled `u32` read straight from a PNG chunk length field) before widening to `u64` for `ignore_bytes` - overflows for `chunk_len` in `[u32::MAX - 3, u32::MAX]` |
| Debug-assertions build + boundary-value construction: a standalone example (`cargo run -p symphonia-metadata --example ...`) calling `symphonia_metadata::utils::images::try_get_image_info` directly on a hand-built minimal indexed-PNG buffer with a bogus ancillary chunk length of `u32::MAX - 1`, then `u32::MAX` | `dev` profile (overflow-checks on by default), 2 runs each value | Both values panic: `attempt to add with overflow` at `utils/images.rs:156:47` |
| Escalated to the real end-to-end entry point: built a full ID3v2.3 tag (via `symphonia_metadata::id3v2::Id3v2Reader::try_new(...).read_all()`, the real public `MetadataReader` API) containing one `APIC` frame whose image data is the malicious PNG bytes | `dev`, then `release` with `overflow-checks = true` explicitly set, 3 runs | Same panic, now reached through the real standalone ID3v2 tag reader's full frame-dispatch path (`read_id3v2` -> `read_id3v2_body` -> `read_id3v2p3_frame` -> `read_apic_frame` -> `try_get_image_info` -> `parse_png`), confirmed via full backtrace (`RUST_BACKTRACE=1`) |

## Status

**Confirmed**, 3/3 runs, deterministic (no randomness involved - same crafted input every time),
reproduced both with `cargo run` against the main checkout's working tree and via `just test`
against a genuinely fresh `git clone` of the repo at `base_commit`. The overflow is only visible
under `overflow-checks = true` (the default `dev` profile, and explicitly enabled in `repro/`'s
`release` profile); a plain `--release` build without that flag would silently wrap instead of
panicking - see "Root cause" for why the wrapped value is still a real (non-panicking) parsing bug
in that build too.

**Fully reproducible**: **Yes**. `repro/main.rs` builds a complete ID3v2.3 tag byte stream in
memory and feeds it to `symphonia_metadata::id3v2::Id3v2Reader::try_new(...).read_all()` - the
same public `MetadataReader` entry point a real container format reader (or any application
embedding symphonia to read a standalone/embedded ID3v2 tag from an untrusted file) uses. This is
a single-participant finding (a malformed embedded-image byte sequence inside an otherwise
well-formed ID3v2 tag, handed to the parser's real public API) - there is no second participant to
fake here, per `CONFIRMATION.md` section 11's own carve-out for this shape of finding.

## Class

crash (arithmetic overflow panic under `overflow-checks` on malformed/adversarial attacker-supplied
image metadata; a debug-only manifestation of a genuinely wrong computation, not memory-unsafe UB).

## Root cause

`parse_png` (`symphonia-metadata/src/utils/images.rs`), when handling an Indexed/palette PNG color
type (`color_type == 3`), loops over ancillary chunks looking for a `tRNS` transparency chunk,
skipping any other chunk with:

```rust
reader.ignore_bytes(u64::from(chunk_len + 4))?;
```

`chunk_len` is a `u32` read directly off the wire via `reader.read_be_u32()` with no validation -
entirely attacker-controlled. The addition `chunk_len + 4` is evaluated in `u32` arithmetic first
(both operands are `u32`), and only the *result* is widened to `u64` by `u64::from(...)`. For any
`chunk_len` in `0xFFFF_FFFC..=0xFFFF_FFFF`, this addition overflows `u32::MAX` before the widening
ever happens. Two chunk-skip lines exist in this same file for this exact purpose (JPEG's
`ignore_bytes(u64::from(chunk_len) - 2)` at line 96 widens *before* subtracting) - the PNG one is
the only line in the file that got the widen-then-arithmetic order backwards. Under
`overflow-checks = true` this panics; under a plain (non-checked) release build it would silently
wrap to a small `u64` (e.g. `chunk_len = u32::MAX` wraps `chunk_len + 4` to `3`), causing the
parser to skip far too few bytes and then misinterpret whatever follows as the next chunk header -
a real (if less immediately visible) parsing-correctness bug in that build configuration too, not
just a debug-only artifact.

This function is reachable from every standalone metadata format in this crate that embeds
attacker-controlled binary image data: `id3v2/frames/readers.rs::read_apic_frame` (ID3v2 `APIC`/
`PIC` frames) and `ape.rs::read_all` (APEv2 "Cover Art (...)" binary items) both call
`try_get_image_info` directly on unvalidated bytes from the file.

## Introduced by

937e253 "meta: Detect image format and attributes." (2024-10-27) - this commit added
`symphonia-metadata/src/utils/images.rs` in its entirety, including the `chunk_len + 4` line as
originally written; `git log -S"chunk_len + 4" -- symphonia-metadata/src/utils/images.rs` finds
only this one commit, and the line has not been touched since.

## The fix

Changed the evaluation order to widen `chunk_len` to `u64` before adding, matching the sibling
JPEG chunk-skip line two lines earlier in the same file:

```rust
-reader.ignore_bytes(u64::from(chunk_len + 4))?;
+reader.ignore_bytes(u64::from(chunk_len) + 4)?;
```

Verified against both the originally-triggering value (`chunk_len = u32::MAX - 1`) and a second
boundary value in the same class (`chunk_len = u32::MAX`) per `CONFIRMATION.md` section 7 - both
panic on the unpatched tree and both parse without panicking (`parsed OK (no crash)`) after the
fix.

- Re-running `repro/` against the patched tree (`just apply && just reproduce-after` from a fresh
  clone): `parsed OK (no crash)`, 3/3 runs.
- Project test suite (`cargo test -p symphonia-metadata`): 11 passed, 0 failed, 0 ignored, both
  before and after the fix (the fix doesn't touch any code path currently under test - no existing
  test exercises the indexed-PNG chunk-skip loop at all).

## What's not done / open questions

- The same `try_get_image_info`/`parse_png` code path is also reachable from `ape.rs`'s APEv2
  "Cover Art" binary items and from `id3v2/frames/readers.rs`'s legacy `PIC` frame variant - not
  independently re-verified with a separate repro since they all funnel through the exact same
  `parse_png` function already shown to panic; the fix covers all of them identically.
- `parse_jpeg`, `parse_bitmap`, and `parse_gif` in the same file were read for the same
  widen-before-arithmetic pattern and did not show it (JPEG's own chunk-skip already widens
  first; BMP/GIF don't do chunk-length arithmetic at all) - not exhaustively fuzzed, just manually
  checked.
- No dedicated fuzz target exists for any of `symphonia-metadata`'s standalone parsers
  (`Id3v2Reader`, `ApeReader`, `Id3v1Reader`) in `symphonia/fuzz/` - this candidate found the bug
  by manual code reading plus hand-built boundary-value input, not fuzzing; a short one-off
  harness driving `Id3v2Reader`/`ApeReader::try_new(...).read_all()` from raw bytes would likely
  turn up more of this class quickly (see `IDEAS.md` for this as an open backlog item, distinct
  from the already-tracked "no seed corpus" item).
