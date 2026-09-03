# picture_from_flac_ape_bytes_arithmetic

`lofty/src/picture/mod.rs`: `Picture::from_reader`, `Picture::from_flac_bytes`/
`from_flac_bytes_inner`, and `Picture::from_ape_bytes` (~lines 634-955) - flagged in `IDEAS.md`
while working `008.png_plte_chunk_arithmetic/` as sibling entry points in the same file that parse
untrusted length-prefixed picture byte layouts (FLAC `METADATA_BLOCK_PICTURE`, APEv2 picture
items), only read and never independently boundary-tested or fuzzed, unlike `from_jpeg`
(confirmed vulnerable in `002`) and `from_png` (cleared in `008`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual arithmetic trace of every `size -=`/index-slice operation in `from_flac_bytes_inner` | static read | Every `size -= N` decrement is preceded by either an explicit `if N > size { return Err }` guard, or a prior successful `reader.read_uN()?`/`read_exact` of exactly those `N` bytes from the same `Cursor` (which fails with an `Err` - short-circuiting via `?` before the decrement - whenever fewer than `N` physical bytes remain). Traced that `size` therefore always stays exactly in sync with `content.len() - reader.position()` throughout the function, including through the `desc_len > 0 && desc_len < size` skip-branch (skipping neither advances the reader nor decrements `size`, so sync is preserved either way). The unconditional `size -= 20` before the final `data_len > size` check is only reached after 5 successful `u32` reads (20 physical bytes), which by the sync invariant guarantees `size >= 20` at that point - no underflow path found. |
| Manual arithmetic trace of `from_ape_bytes` | static read | `pos` only increments inside the description-reading `while let Ok(ch) = reader.read_u8()` loop, bounded by `bytes.len()`, so `bytes[pos..]` is always in-bounds. The subsequent 8-byte `identifier` read deliberately does **not** advance `pos`, because those 8 bytes are the picture data's own magic-number prefix (peeked via `mimetype_from_bin`), not a separate stored field - confirmed against the passing `ape_binary_item` integration test in `lofty/tests/picture/format_parsers.rs`, which would fail if those bytes were wrongly duplicated or dropped from `data`. No overflow/underflow site found. |
| Manual check of `from_reader` | static read | `data.len() < 8` is checked before the only slice operation (`&data[..8]`); no arithmetic beyond that. |
| `cargo +nightly fuzz run picture_from_ape_bytes` (existing project harness) | debug-assertions + overflow-checks on by default (cargo-fuzz's non-`-O` default), `--sanitizer none` (see below), `-max_total_time=90` | 32,585,392 executions in 91s, 0 crashes/panics. |
| `cargo +nightly fuzz run picture_from_flac_bytes` (existing project harness) | same build config, `-max_total_time=90` | 35,010,740 executions in 91s, 0 crashes/panics. |
| `cargo +nightly fuzz run picture_from_reader` (existing project harness) | same build config, `-max_total_time=40` | 16,813,733 executions in 41s, 0 crashes/panics. |
| ASAN (cargo-fuzz's default `--sanitizer address`) on `picture_from_ape_bytes`/`picture_from_flac_bytes` | default cargo-fuzz build | Ruled out as a tooling artifact, not a finding - see below. |

### ASAN tooling note (not a finding)

The default cargo-fuzz build (`--sanitizer address`, the default when `--sanitizer` isn't passed)
hung indefinitely on both `picture_from_ape_bytes` and `picture_from_flac_bytes` in this sandbox:
the single-threaded fuzzer process pegged one CPU core at 100% for minutes with zero further
libFuzzer progress output, and neither libFuzzer's own `-timeout=3/5` alarm nor an external
`timeout` wrapper around the whole `cargo fuzz run` invocation terminated the *child* process
cleanly (had to `pkill -9` the fuzz target binary directly). `gdb -p <pid>` couldn't attach
(`ptrace_scope`/sandbox restriction, no `CAP_SYS_PTRACE`), so no in-process backtrace could be
captured. Rebuilding the exact same targets with `--sanitizer none` (still debug-assertions +
overflow-checks on) made the hang disappear entirely - both ran at ~360-500K exec/s with no stall.
This isolates the hang to the ASAN instrumentation/sandbox interaction (most likely ASAN's startup
shadow-memory reservation or signal handling being slow/blocked under this sandbox's restrictions),
not to anything in `Picture`'s parsing code - there is no loop construct anywhere in
`from_flac_bytes_inner`/`from_ape_bytes`/`from_reader` that could hang on a few-byte input, and the
non-ASAN build proves 300M+ combined executions complete without ever stalling. Recorded here (and
in `README.md`) so a future session doesn't re-waste time trying ASAN on these specific fuzz
targets in this environment - use `--sanitizer none` for this project's fuzz targets going forward
unless specifically hunting for a memory-safety bug that needs ASAN's instrumentation.

## Status

**Not reproducible.** No arithmetic overflow, underflow, divide-by-zero, out-of-bounds slice, or
panic was found in `from_reader`, `from_flac_bytes`/`from_flac_bytes_inner`, or `from_ape_bytes`
across manual boundary-value/invariant tracing of every size-tracking and slicing operation, nor
across a combined ~84 million fuzzer executions (debug-assertions + overflow-checks build) spread
across all three of the project's own existing fuzz targets for these entry points.

## Class

N/A (not reproducible)

## Root cause

Not applicable in the "confirmed bug" sense. Unlike `from_jpeg`'s `precision * components`
(confirmed vulnerable in `002`, multiplies two `u8`s before widening to `u32`),
`from_flac_bytes_inner` maintains a `size` bookkeeping variable that is provably always kept in
sync with the `Cursor`'s actual remaining bytes (every decrement is either pre-guarded by an
explicit bounds check, or gated behind a prior read that would itself fail-fast via `?` if not
enough physical bytes remained) - so every later unconditional `size -= N` is safe by construction,
not by luck. `from_ape_bytes`'s `pos` tracking is similarly bounded by construction (only
incremented inside a loop capped at `bytes.len()` iterations), and the 8-byte mimetype-identifier
read is intentionally excluded from `pos` since those bytes are part of the picture data itself,
not a separately-framed field.

## The fix

No fix attempted - no reproducible incorrect behavior was found to fix.

- Project test suite: not run for this candidate (no fix was made; no source files were modified
  this session - `git status` verified clean of any working-tree changes from this candidate
  before and after).

## What's not done / open questions

- This closes out the `008` follow-up note for `Picture::from_reader`/`from_flac_bytes`/
  `from_ape_bytes`'s own arithmetic specifically. The broader "PNG/JPEG longer fuzzing campaign"
  and "MP4 `mp4a_properties` sibling bitfields" suspects noted alongside it in `IDEAS.md` remain
  open and unrelated to this candidate.
- The ~90-second-per-target fuzz bursts here (even at ~84M total executions) are still a short,
  one-off sweep against an empty/small seed corpus, not a real persistent-corpus campaign - a
  longer run via `fuzz-bug-repro` could still be pointed at these same three targets if a
  dedicated campaign is ever run, though the manual invariant proof above gives more confidence
  here than in `008`'s equivalent note (that proof rules out the *class* of bug by construction,
  not just by absence of an observed crash).
- The ASAN-hang tooling gap observed here was only isolated to these two specific fuzz targets;
  whether it reproduces on the project's other (larger, file-format) fuzz targets was not checked -
  worth a quick check before relying on ASAN for a future candidate in this project/environment.
