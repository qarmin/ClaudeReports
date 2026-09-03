# ID3v2 CHAP/CTOC unbounded recursion (stack overflow)

`symphonia-metadata`'s ID3v2 tag reader has no fuzz target in `symphonia/fuzz/fuzz_targets/`
(the README's own gap analysis flags standalone `symphonia-metadata` parsers as uncovered).
Reading the frame readers by hand, `read_chap_frame` and `read_ctoc_frame`
(`symphonia-metadata/src/id3v2/frames/readers.rs`) both recursively call back into
`read_id3v2p{2,3,4}_frame` to read a `CHAP`/`CTOC` frame's own sub-frames, with no limit on how
many levels deep this can go, and each level only costs a small, fixed number of bytes (a 10-byte
ID3v2.3+ frame header plus 17 bytes of mandatory `CHAP` fields = 27 bytes/level).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read code against recursion/unbounded-growth heuristics | manual read of `symphonia-metadata/src/id3v2/{mod,frames,frames/readers}.rs` | Found `read_chap_frame`/`read_ctoc_frame` recursing into the generic frame reader with no depth counter anywhere in the crate (`grep -rn "depth\|recursion"` found nothing relevant) |
| Standalone repro calling the real public `Id3v2Reader` API directly | release build, 60,000 nested `CHAP` frames (1,620,010-byte crafted tag), 3 runs | Reliably aborts the process every run: `thread 'main' has overflowed its stack` / `fatal runtime error: stack overflow, aborting` |

## Status

**Confirmed**, 3/3 runs, release build, deterministic (no randomness involved - same crafted input
every time).

**Fully reproducible**: **Yes**. `repro/main.rs` builds the malicious bytes in-process and feeds
them to `symphonia_metadata::id3v2::Id3v2Reader::try_new(...).read_all()` - the same public
`MetadataReader` entry point a real container format reader (or an application embedding
symphonia) uses to parse an ID3v2 tag it read from an untrusted file. No demuxer currently in this
repo drives ID3v2 parsing any differently, so this is the genuine trigger path, not a stand-in for
it.

## Class

crash (stack overflow / process abort on malformed/adversarial input - a DoS, not memory-unsafe
UB, since Rust's stack-overflow guard page catches it and aborts cleanly rather than corrupting
memory).

## Root cause

`read_chap_frame` and `read_ctoc_frame` (`symphonia-metadata/src/id3v2/frames/readers.rs`) each
read a bounded sub-buffer (`BufReader` scoped to the frame's declared body size) and then loop
`while reader.bytes_available() >= min_frame_size(...)`, calling
`read_id3v2p{2,3,4}_frame` on that sub-buffer to read any further sub-frames it may contain. Since
`CHAP`/`CTOC` are themselves entries in `FRAME_READERS`, a `CHAP` (or `CTOC`) frame can contain
another `CHAP`/`CTOC` frame as one of these sub-frames, and so on, with no code anywhere tracking
or limiting how many levels deep this recursion goes. Because each level only costs 27 bytes and
the ID3v2 tag size field is a 28-bit syncsafe integer (up to ~256 MiB), a single tag can encode
many tens of thousands of nesting levels well within realistic file-size limits, which is enough
to exhaust the default 8 MiB thread stack.

## Introduced by

46cd6fe "meta (id3v2): Complete ID3v2 implementation." (2024-11-03) - the recursive `read_chap_frame`/`read_ctoc_frame` design (calling back into the generic frame reader with no depth counter) was present from this commit, which introduced the whole ID3v2 implementation; `git log -S"fn read_chap_frame"` finds no earlier revision.

## The fix

Added a small `ChapterNestingGuard` RAII counter (`thread_local!` `Cell<u32>`, max depth 32) in
`symphonia-metadata/src/id3v2/frames/readers.rs`, entered at the top of both `read_chap_frame` and
`read_ctoc_frame` and released on drop (so it unwinds correctly through the existing `?`-based
early returns). Exceeding the limit returns a `decode_error` instead of recursing further; the
existing caller in `read_id3v2p{2,3,4}_frame` already treats a frame-reader `Err` as "skip this
frame, keep the rest of the tag" (see the `Err(err) => { warn!(...); Ok(FrameResult::Skipped) }`
arm), so a tag that hits the limit still parses successfully overall, just without that
pathologically-nested chapter subtree - consistent with how every other malformed-frame case in
this reader already degrades.

- Re-running `repro/` against the patched tree: `parsed OK (no crash)`, 3/3 runs.
- Project test suite (`cargo test -p symphonia-metadata`): 11 passed, 0 failed, both before and
  after the fix (the fix doesn't touch any tested code path; it only adds a limit to a previously
  unbounded loop).

## What's not done / open questions

- No other frame type in this crate recurses into the generic frame reader (`grep` for calls to
  `read_id3v2p{2,3,4}_frame` outside of `mod.rs` and these two readers found nothing else), so this
  appears to be the only recursive spot in ID3v2 parsing.
- The same "no depth limit on nested structures" class of issue is worth checking in other
  hierarchical formats in this workspace (e.g. Matroska/EBML's nested master elements in
  `symphonia-format-mkv`, which does have a `demux_mkv` fuzz target and so wasn't re-checked here)
  - not investigated this session.
- The chosen limit (32) is somewhat arbitrary; no real-world ID3v2 chapter hierarchy should
  realistically need anywhere near that depth, but a maintainer may want a different number.
