# Move (instead of clone) `raw_bytes` in `parse_captured` for chunk types with no specific parser

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

Candidate 002 fixed PLTE/sBIT/tRNS/eXIf/bKGD's own `.clone()` calls but deliberately left
`parse_captured` (`src/decoder/stream.rs:~1815`, feeds `Info::captured_chunks` for any chunk type
the caller opted into via `DecodeOptions::set_captured_chunks`) cloning `raw_bytes`, because it
runs *before* the type-specific parser in `parse_chunk`'s dispatch - for a chunk type that is both
"captured" and one of the ~23 recognized types with their own parser (IHDR, PLTE, sBIT, tRNS,
pHYs, oFFs, gAMA, acTL, fcTL, cHRM, sRGB, cICP, mDCV, cLLI, bKGD, eXIf, iCCP, tEXt, zTXt, iTXt,
IEND, IDAT, fdAT), that parser still needs `raw_bytes` intact afterward. But for a genuinely
*unrecognized* chunk type that is also captured (the common real-world case - `uNKn`-style custom
ancillary chunks, which is what `set_captured_chunks` exists for), nothing reads `raw_bytes` after
`parse_captured` returns - `parse_chunk`'s dispatch falls through to `_ if is_captured => Ok(())`.
This candidate adds a `has_specific_parser(type_str)` check and moves (via `mem::replace`,
following candidate 002's established non-zero-capacity-floor lesson - never `mem::take`) instead
of cloning when that check is false.

## What was tried

Standalone bench (`bench/` - no existing harness reaches `set_captured_chunks`): builds a real 1x1
grayscale PNG via the crate's own encoder, splices in a hand-built, deliberately unrecognized
`zzZz` ancillary chunk (varying-size, non-uniform byte pattern so a move-vs-clone bug handing back
stale/garbage bytes would be caught) before IDAT, decodes repeatedly with
`set_captured_chunks(&[zzZz])`, verifying the captured bytes match exactly every iteration.

| chunk size | baseline p50 (`taskset -c 0`, n=270) | patched p50 | allocs (avg) | reallocs (avg) | bytes/decode (avg) |
|---|---|---|---|---|---|
| 0 B | 0.0005-0.0006ms | 0.0005-0.0007ms | 7.0 -> 8.0 | 0.0 -> 0.0 | 19.53 -> 19.65 KiB |
| 1024 B | 0.0007-0.0008ms | 0.0007-0.0008ms | 9.0 -> 9.0 | 3.0 -> 3.0 | 22.40 -> 21.53 KiB |
| 65536 B | 0.0043-0.0054ms | 0.0034-0.0036ms | 9.0 -> 9.0 | 9.0 -> 9.0 | 211.40 -> 147.53 KiB |
| 1,048,576 B | 0.0601-0.0729ms | 0.0437-0.0453ms | 9.0 -> 9.0 | 13.0 -> 13.0 | 3091.40 -> 2067.53 KiB |

(Two independent baseline runs and two independent patched runs shown as ranges - both `taskset
-c 0` pinned and unpinned gave the same numbers within noise, confirming this single-threaded
workload isn't memory-bandwidth/lock-bound in a way that would change with core placement.)

Paired/alternating comparison (`tools/paired.sh`, `RUNS=15`, `taskset -c 0`, `--iters 300`; the
tool reads the *last* printed `RESULT` line, the 1,048,576 B config), run both directly and via
this candidate's own `just test` from a fresh clone (identical result both times):

```
median 0.060ms -> 0.044ms  (-25.8%)   B faster in 15 of 15 pairs
verdict: consistent
```

## Result

**Time: real, consistently-reproduced win for captured chunks in the hundreds-of-KB-to-MB range**
(the realistic size range for a custom/unrecognized ancillary chunk large enough for the removed
`memcpy` to matter). -25.8% median latency for a 1 MiB captured unrecognized chunk, 15/15 paired
runs consistent, reproduced independently via the standalone `taskset` runs above and via `just
test`'s own clone-and-build cycle. For small chunks (0-1024 B) the difference is at or below this
machine's measurement floor - not a meaningful win or loss either way.

**Allocated bytes:** roughly halved for chunks large enough for "clone the existing buffer" vs.
"128 fresh bytes plus a move" to matter (65536 B: 211 -> 147 KiB; 1 MiB: 3091 -> 2067 KiB - close
to the removed clone's own worth, same shape as candidate 002's finding for the type-specific
parsers).

**Allocation count:** unchanged for chunks >= 1024 B (a small `Vec::with_capacity(128)` still has
to be allocated to leave a valid buffer behind for the next chunk, same as candidate 002). For a
0 B chunk it's one *more* allocation (7 -> 8) than baseline, since a 128-byte buffer is now
allocated even though there was nothing to save a copy of - negligible in absolute terms, and this
extra small allocation is a real (if tiny) cost of this optimization, not a wash.

## Does it change behavior/output?

- **Bit-exact?** Yes - the standalone bench asserts the decoded captured-chunk bytes match
  exactly, every iteration, for every chunk size tested, both before and after. Also verified: a
  captured chunk that also has a specific parser (e.g. `tEXt` per the existing
  `test_captured_chunks_known` unit test) still gets both its `captured_chunks` entry *and* its
  normal parsed representation, since `has_specific_parser` routes it through the unchanged
  clone path.
- **Is this a breaking change?** No - `Info`'s public fields and the decoder's public API are
  unchanged; only how an internal scratch buffer is recycled for the "no further parser reads it"
  case changed.
- Project test suite (`cargo test --release`, run through the shared build lock): **107 passed, 1
  ignored, 0 failed**, identical before and after the patch.

## Why the result looks the way it does

Same mechanism as candidate 002: `Vec::clone()` for N bytes is one allocation plus one `memcpy` of
N bytes; `mem::replace` with a small fresh buffer is one allocation of 128 bytes (not N) plus a
pointer move - the removed `memcpy` is exactly where the time goes for large chunks, and exactly
why nothing moves for tiny ones (a 128-byte alloc either way, memcpy of <1KB is already below the
noise floor here). The extra branch (`has_specific_parser`, a `matches!` over ~23 fixed chunk
types) is a compile-time-friendly linear/jump-table check on a 4-byte `ChunkType` and is not
visible in the measurements above at any chunk size.

## What's not done / open questions

- `has_specific_parser`'s chunk-type list must be kept in sync with `parse_chunk`'s own `match`
  arms by hand - if a future PR adds a new specific parser for another chunk type without also
  adding it here, `parse_captured` would incorrectly move `raw_bytes` out from under that new
  parser, corrupting or emptying its input. This is a real maintenance coupling the original
  candidate-002 write-up already flagged as the reason this case was left unpatched; a longer-term
  fix would restructure `parse_chunk` so the "does this chunk type have a further reader" fact is
  derived in one place rather than duplicated between the dispatch `match` and this new helper
  (e.g. by moving `parse_captured` to run *after* the specific parser instead, if reordering is
  safe - not investigated in this bounded candidate).
- Not measured: whether a decoder that captures a *very* large number of small unrecognized
  chunks in one file sees the +1-allocation-per-chunk cost from the 0 B row add up to something
  visible in aggregate. Judged out of scope - `Limits::bytes`/chunk-count limits already bound this
  in practice, and the per-chunk cost is a single small, already-cache-resident allocation.
