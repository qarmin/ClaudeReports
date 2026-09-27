# ID3v2 CTOC dfs unbounded recursion (stack overflow)

`ChapterGroupBuilder::build()`'s private `dfs` helper (`symphonia-metadata/src/id3v2/mod.rs`)
walks the post-parse graph of ID3v2 CTOC (table-of-contents) element-ID references with no depth
limit. This is distinct from candidate 001 (`___BUGHUNT___/001.ID3V2_CHAP_RECURSION`): 001's fix
(a `ChapterNestingGuard`) only bounds *physical* frame-nesting depth inside
`read_chap_frame`/`read_ctoc_frame` (`symphonia-metadata/src/id3v2/frames/readers.rs`), i.e. CTOC
frames literally byte-nested inside one another. `dfs` runs afterward, over a flat list of
top-level CTOC frames that reference each other by element ID (`toc_0 -> toc_1 -> ... -> toc_N`,
each with a single child ID, none of them physically nested), and evades that guard entirely.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read code against recursion/unbounded-growth heuristics | manual read of `ChapterGroupBuilder::build`/`dfs` and `read_ctoc_frame` in `symphonia-metadata/src/id3v2/{mod.rs,frames/readers.rs}` | Confirmed `dfs` recurses once per chain link with no depth counter, and that this graph is built from top-level (non-nested) `CTOC` frames, so candidate 001's `ChapterNestingGuard` does not cover it |
| Standalone repro calling the real public `Id3v2Reader` API directly | release build, 500,000 chained `CTOC` frames (12,777,788-byte crafted ID3v2.3 tag), 3 runs | Reliably aborts the process every run: `thread 'main' has overflowed its stack` / `fatal runtime error: stack overflow, aborting` |

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

After all frames in an ID3v2 tag are read (a flat, non-recursive loop in `read_id3v2_body`), any
`CTOC` frames encountered are collected into a `HashMap<String, Id3v2TableOfContents>` keyed by
their element ID, each holding a list of child element IDs (`toc.items`) referencing other
TOC/chapter frames by ID rather than by physical byte position. `ChapterGroupBuilder::build()`
then calls a private `dfs(id, tocs, chaps)` helper that recursively follows these ID references,
once per level, with no depth counter anywhere. A `CTOC` frame is cheap to encode (ID3v2.3 frame
header 10 bytes + element-ID string + 1 flags byte + 1 entry-count byte + one child element-ID
string, roughly 26 bytes/level for short numeric IDs), and the ID3v2 tag size field is a 28-bit
syncsafe integer (up to ~256 MiB), so a single tag can encode hundreds of thousands of `dfs`
levels well within realistic file-size limits - here, 500,000 levels in a ~12.8 MB tag, enough to
exhaust the default 8 MiB thread stack. Crucially, none of these `CTOC` frames are nested inside
one another in the byte stream (they're all read directly by the top-level frame loop), so
candidate 001's `ChapterNestingGuard` - which only counts recursive calls made *from inside*
`read_chap_frame`/`read_ctoc_frame` while reading a frame's own sub-frames - never triggers; the
recursion this candidate found happens entirely in `dfs`, called once, after all frames are
already read.

## Introduced by

dc287cc "core (formats,meta): Replace and rewrite cues with chapters." (2024-10-22) - `git log -S"fn dfs"` on `symphonia-metadata/src/id3v2/mod.rs` finds this as the only commit introducing the `dfs` helper; it added the whole ID-reference chapter graph traversal with no depth counter from the start.

## The fix

Added a `depth: u32` parameter to `dfs`, incremented on each recursive call, and a `const
MAX_TOC_DEPTH: u32 = 32` cap (same value candidate 001 chose, for consistency) checked at the top
of `dfs`; hitting the cap returns `None` without removing the TOC from the map, so it's treated
exactly like any other broken/unreachable TOC reference - it surfaces via the existing "id3v2:
missing chapter or toc element" debug log and gets swept up by the existing "unreferenced toc
and/or chapter elements" cleanup logic already in `build()`. No new error path or behavior change
for any tag with a totally ordinary (shallow) chapter hierarchy.

- Re-running `repro/` against the patched tree: `parsed OK (no crash)`, 3/3 runs.
- Project test suite (`cargo test -p symphonia-metadata --release`): 11 passed, 0 failed, both
  before and after the fix (the fix doesn't touch any tested code path; it only adds a limit to a
  previously unbounded recursive helper local to `ChapterGroupBuilder::build`).

## What's not done / open questions

- No other recursive traversal over an ID-reference graph (as opposed to byte-stream nesting) was
  found elsewhere in this crate (`grep` for other private `dfs`/recursive helpers in
  `symphonia-metadata` found none), so this appears to be the only such spot.
- The chosen limit (32) is somewhat arbitrary, matching candidate 001's own choice for the
  physical-nesting guard; no real-world ID3v2 chapter hierarchy should realistically need anywhere
  near that depth, but a maintainer may want a different number, or to unify both guards behind one
  shared constant since they're conceptually the same "how deep can chapters legitimately nest"
  policy applied to two different recursion sites.
- Same "no depth limit on a graph built from an untrusted file" class of issue may be worth
  checking in other hierarchical formats in this workspace (e.g. Matroska/EBML nested master
  elements) - not investigated this session (candidate 001 made the same note).
