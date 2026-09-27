# jpeg_marker_section_len_underflow

`lofty/src/picture/mod.rs`, `PictureInformation::from_jpeg` (~line 430-483). Picked up from
`IDEAS.md`'s backlog note ("`from_png`/`from_jpeg` together... might be worth a longer run") left
by candidate `008`, which had only run a short 200k-iteration ad hoc fuzz loop against `from_png`.
`from_jpeg` already had one confirmed arithmetic bug (candidate `002`, a `u8` multiply overflow at
what is now line 474) - the question for this candidate was whether a real, longer fuzzing
campaign (using the project's own existing `pictureinformation_from_jpeg`/`pictureinformation_from_png`
cargo-fuzz targets, not a hand-rolled loop) would turn up anything *else* in either function.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo +nightly fuzz run --sanitizer none pictureinformation_from_jpeg` (project's own existing fuzz target, calling the public `PictureInformation::from_jpeg` directly) | release profile with cargo-fuzz's default debug-assertions/overflow-checks on, `-max_total_time=120`, empty seed corpus | Crashed within seconds (~4700 executions): `thread '<unnamed>' panicked at lofty/src/picture/mod.rs:479:53: attempt to subtract with overflow` on input `ff d8 ff 0d 00 02 ff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 40` - a **different** bug from candidate `002`'s multiply overflow (different line, different operation: subtraction, not multiplication). |
| Manual minimization + boundary-value construction, same debug-assertions build | 10-byte hand-crafted JPEG (`ff d8 ff e0 00 02 ff db 00 00`), driven through the real end-to-end trigger path (`Tag::save_to_path` -> real on-disk ID3v2 APIC frame -> `lofty::read_from_path` -> `From<Tag> for VorbisComments`) | Minimized fuzzer finding to a 10-byte input and confirmed the same panic reproduces through the real public API path, not just the fuzz harness's direct call - 5/5 repeat runs, deterministic. |
| `cargo +nightly fuzz run --sanitizer none pictureinformation_from_png` (project's own existing fuzz target, longer campaign than candidate `008`'s 200k-iteration ad hoc loop) | release profile, debug-assertions/overflow-checks on, `-max_total_time=300`, empty seed corpus | `Done 126828747 runs in 301 second(s)` - 0 crashes, 0 new coverage after early saturation (`cov: 52 ft: 102` unchanged from run #255339 onward). Reinforces candidate `008`'s `from_png` "Not reproducible" conclusion with a genuinely long campaign (126.8M execs vs. the prior 200k). |

## Status

**Confirmed** (for `from_jpeg`'s marker-segment-length subtraction). 5/5 repeat runs, dev-profile
build (default `overflow-checks = true`), deterministic - not order/thread/seed-dependent, same
class as candidate `002`/`003`/`011`'s already-confirmed subtract/multiply-before-check bugs in
this project.

`from_png` remains **Not reproducible** (no new finding this session; see candidate `008` for its
own record) - a 126.8M-execution cargo-fuzz campaign (630x more executions than `008`'s ad hoc
sweep) found nothing.

**Fully reproducible**: **Yes**. `repro/` writes a real MP3 file to disk with a real ID3v2 `APIC`
frame (via `Tag::save_to_path`, lofty's own public writer) carrying the malicious JPEG bytes, reads
it back from disk through `lofty::read_from_path` (the real public reader, exercising the real
`AttachedPictureFrame::parse` path), and only then performs the real `From<Tag> for VorbisComments`
conversion that calls `PictureInformation::from_picture` -> `from_jpeg` on those bytes - the same
real end-to-end shape already established for this file's bugs in candidate `002`. This is a
single-participant finding (a malformed picture is the genuine external artifact a real embedder
would produce; there's no second peer being faked), so no live second participant was needed per
`CONFIRMATION.md` section 11.

## Class

crash

## Root cause

`from_jpeg` walks a JPEG's marker segments looking for a `SOFn` frame. For the *first* marker
segment (right after the `SOI`+first-marker header), the 2-byte declared length is properly
validated with `checked_sub(2)` (line 445) before being used, since the length field always
includes itself and anything `< 2` is invalid. But the loop that walks every *subsequent* marker
segment (line 451 onward) re-reads a fresh `section_len: u16` each iteration (line 453) and, for
any marker that isn't `SOS`/`SOF0`/`SOF2`, skips over that segment's body with
`reader.seek(SeekFrom::Current(i64::from(section_len - 2)))` (the old line 479) - the exact same
"subtract 2 for the length field itself" computation as the first segment, but without the
`checked_sub` guard. Any real JPEG marker segment with an on-disk declared length of `0` or `1`
(both attacker-controlled, one byte read straight off the wire) underflows this `u16` subtraction:
a hard panic under `overflow-checks` (dev builds, or `cargo fuzz`'s default profile), and a silent
wrap to `65534`/`65535` in a plain release build (which then seeks far past EOF; the next
`reader.read_u8()` fails and the function returns `Err(UnknownImageFormatError)` rather than
misparsing further - so release builds fail closed, just via a wrapped intermediate value, not a
crash).

## Introduced by

f99a5873 "PictureInformation: Fix potential overflowing subtraction in `from_jpeg` (2022-07-11)" - best-effort: this commit added the `checked_sub(2)` guard to the *first* marker segment only (fixing that one spot), which is what created the asymmetry this candidate found - the loop handling every *subsequent* marker segment was left with the identical, still-unguarded `section_len - 2` subtraction. The subsequent-marker loop itself predates this commit (present since `9878e89b` "Add PictureInformation tests", 2021-11-22, per `git log -S`), so the bug existed before f99a5873 too - that commit is flagged here because it's the point the code became visibly inconsistent (one guarded, one not) rather than uniformly unguarded.

## The fix

Applied the identical `checked_sub(2)` pattern already used two lines above the vulnerable seek
(candidate `002`/`011` share this same "there's already a validated version of this exact check a
few lines away, it just wasn't copied to the second call site" shape):

```rust
// Same as above: the length contains itself, so anything < 2 is invalid
let Some(remaining_len) = section_len.checked_sub(2) else {
    return Err(NotEnoughDataError::new(Some(2)).into());
};

reader.seek(SeekFrom::Current(i64::from(remaining_len)))?;
```

Verified against more than just the one fuzzer-found input: the minimized 10-byte repro (marker
`0xDB`, `section_len = 0`) was also re-checked with `section_len = 1` by hand (same underflow,
same fix path) before considering this sufficient, per `CONFIRMATION.md` section 7.

Re-running `repro/`'s `just test` (full clone -> before -> apply patch -> after cycle, captured in
`output.txt`): before the patch, the real end-to-end path panics
(`attempt to subtract with overflow` at `picture/mod.rs:479:53`, process exit 101); after applying
`patch.diff` to the same clone, the identical repro completes cleanly (`RESULT: no panic, 1
picture(s) survived conversion`, exit 0).

- Project test suite (`cargo test -p lofty`, main working tree): **822 passed, 0 failed, 96
  ignored** before the fix, and **822 passed, 0 failed, 96 ignored** after - identical counts, no
  regressions, and confirming none of the existing tests already exercised this code path (the bug
  had gone undetected until this session's fuzz campaign).

## What's not done / open questions

- `from_png` was independently re-checked this session with a much longer (126.8M-execution)
  cargo-fuzz campaign than candidate `008`'s ad hoc sweep, still with no finding - a persistent,
  multi-hour/multi-day corpus-based campaign (the kind `fuzz-bug-repro` is built for, run as its
  own separate effort, not inside this skill) was still not attempted and could in principle still
  turn up something `from_png` alone doesn't currently expose to a 5-minute burst.
- `from_jpeg`'s loop has one other read site with the exact same "length field includes itself"
  shape as the fixed line and the already-fixed line 445 - the `SOFn` branch itself (lines 466-469)
  reads `precision`/`height`/`width`/`components` with no further subtraction, so it isn't the same
  bug class; not flagged further here.
- Not re-checked whether other JPEG-adjacent parsers elsewhere in the codebase (if any exist
  outside `lofty::picture`) share this same "second occurrence of a checked computation is missing
  the check" pattern - only `from_jpeg` itself was in scope for this candidate.
