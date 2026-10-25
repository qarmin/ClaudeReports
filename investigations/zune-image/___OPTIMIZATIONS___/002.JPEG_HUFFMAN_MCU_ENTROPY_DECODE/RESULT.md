# JPEG Huffman/MCU entropy decode: defer unconditional AC-lookup-table read to the slow path

`crates/zune-jpeg/src/bitstream.rs`, `BitStreamHuffman::decode_mcu_block` (line ~730-745),
`discard_mcu_block` (line ~794-807) and `decode_mcu_ac_first` (line ~883-898). This area was
flagged by `perf record`/`perf report` (`--call-graph dwarf`, decoding
`test-images/jpeg/benchmarks/speed_bench.jpg` via `zune_jpeg::JpegDecoder::decode()` in a loop) as
the largest remaining JPEG hot spot: `BitStreamHuffman::decode_mcu_block` at 15.05% of total decode
cycles, plus `decode_mcu_ycbcr_baseline_inner` (7.53%), `decode_dc` (1.72%), `refill` (1.39%) and
`drop_bits` (0.84%) in the same bitstream-decode path (see
`___OPTIMIZATIONS___/IDEAS.md`/this task's own framing, ~24-26% of total decode cycles combined).

Reading the AC-coefficient inner loop in all three functions, every iteration unconditionally reads
*two* separate lookup tables at the same `symbol` index derived from `peek_bits::<HUFF_LOOKAHEAD>()`
(`HUFF_LOOKAHEAD = 9`, so both tables have 512 entries): `ac_lookup[symbol]` (an `Option<[i16; 512]>`,
the fast-AC combined run/value/length table) and `ac_table.lookup[symbol]` (an `[i32; 512]` table,
only actually needed on the `decode_huff!` fallback path when `fast_ac == 0`). The second read's
result (`symbol = ac_table.lookup[symbol as usize]`) is entirely discarded whenever the fast-AC path
is taken (`if fast_ac != 0 { ... }` branch), which - per the surrounding code's own comments
("Accelerated AC table decoding borrowed from stb_image.h ... improved by me to handle more cases")
- is the intended common case for well-formed JPEGs. This looked like a plausible, low-risk
micro-optimization: defer the `ac_table.lookup[symbol]` read into the `else` branch so it's only
paid on the (less common) slow path, removing one unconditional 2KB-table-adjacent load from the hot
loop.

rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05).

## What was tried

Moved `symbol = ac_table.lookup[symbol as usize];` from immediately after the `peek_bits` call (run
unconditionally, before the `if fast_ac != 0` check) to the start of the `else` branch (run only
when the fast-AC lookup misses), in all three call sites that share this shape:
`decode_mcu_block`, `discard_mcu_block`, `decode_mcu_ac_first`. No other logic changed - the value
computed and consumed on each path is identical to before, just computed later on the path that
needs it.

Bench harness: no existing example in `crates/zune-jpeg` reached `JpegDecoder::decode()` directly
with per-iteration timing (the project's own `benchmarks/benches/decode_jpeg.rs` criterion bench
compares against `mozjpeg` and adds criterion overhead/statistics this workspace's own
paired-comparison discipline doesn't need) - added `bench/bench_decode.rs` in this candidate's own
folder (from `~/.claude/skills/perf-optimize/templates/bench_template.rs`, adapted for
`zune_jpeg::JpegDecoder::decode()`, same shape as candidate 001's PNG bench). This is harness, not
fix, so it's NOT part of `patch.diff` - this candidate's `justfile` copies it into
`clone/crates/zune-jpeg/examples/bench_decode.rs` as part of `clone:`, before either build, so it's
present identically for both the baseline and patched binary.

Input: `test-images/jpeg/benchmarks/speed_bench.jpg` (1.27 MB file, baseline/non-progressive,
decodes to a hash-stable RGB buffer) - same file used for the profiling session that identified
this hot spot. Machine: AMD Ryzen (24 logical cores), `uptime` load average ~0.6-0.7 throughout
(well under core count) at measurement time.

| Variant | single-core p50 (`taskset -c2`) | single-core p50 (`taskset -c2`, repeat) | single-core p50 (`taskset -c3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|---|---|
| baseline | 70.014 ms | 70.726 ms | 71.582 ms | 70.665 ms | unconditional `ac_table.lookup[symbol]` read every iteration |
| patched | 70.225 ms | 70.809 ms | 72.418 ms | 72.431 ms | `ac_table.lookup[symbol]` read deferred to the `else` (slow) branch |

Paired alternating comparison (`___OPTIMIZATIONS___/tools/paired.sh`, `RUNS=9`, `iters=15` per
invocation, baseline vs. patched binaries confirmed non-identical via `md5sum` first, unpinned):

```
A (baseline): 70.401 70.219 70.803 71.047 70.677 70.086 70.511 70.445 70.729
B (patched) : 70.279 70.138 71.429 70.575 71.169 71.214 70.287 71.009 71.488
median 70.511 -> 71.009  (+0.7%)   B faster in 4 of 9 pairs
verdict: NOT consistent - the medians are not evidence
```

## Result

No measurable time change. The four single-run p50s (baseline vs. patched) differ by less than
the baseline's own repeat-run noise floor (baseline p50 ranged 70.0-71.6 ms across its own two
`taskset -c2` repeats and the `-c3` run, i.e. ~2.3% spread run-to-run with *no* source change at
all), and the 9-pair alternating comparison came back inconsistent (patched binary faster in only
4 of 9 pairs, median moved +0.7% - within noise, not a real regression either). This is a genuine
"no change" result, not a regression and not a win: removing the redundant table read did not
measurably change wall-clock decode time for this input on this machine. Time only - no
memory/allocation-count claim made (the change doesn't add or remove allocations, only reorders an
existing array read).

## Does it change behavior/output?

- **Bit-exact?** yes. `fnv1a64` hash of the full decoded output (`79dc3c1ab33b6ab1`) is identical
  between baseline and patched across every iteration of every run above (the bench driver asserts
  this itself and aborts on mismatch; it never did).
- **Is this a breaking change?** no. Internal-only reordering inside a private method of
  `crates/zune-jpeg/src/bitstream.rs`; no public API, file format, or documented behavior change.
- Project test suite (`crates/zune-jpeg` only - the only crate whose source changed), `cargo test
  --release -p zune-jpeg --no-fail-fast`, run through the build/test lock, before and after:
  - **lib tests**: 13 passed / 1 failed, **identical before and after** - the one failure
    (`decoder::planar_layout_helpers::set_options_refreshes_cached_dispatch`) is pre-existing on
    `base_commit` with the patch fully reverted (reproduced with the exact same
    `assert_eq!(left, right)` panic at `decoder.rs:2783`, comparing two independently-obtained
    function-pointer addresses that should be but aren't equal - see "Incidental finding" below).
  - **integration tests** (all binaries under `crates/zune-jpeg/tests/`): 148 passed / 0 failed / 1
    ignored, **identical before and after** (9+0+18+2+23+1+46+7+11+10+8+13 across the 12 test
    binaries, same split both times).
  - Total: **161 passed / 1 failed / 1 ignored, identical set of passing/failing/ignored tests
    before and after** - the patch introduces no new failures and fixes none.

## Why the result looks the way it does

Both tables (`ac_lookup: Option<[i16; 512]>` = 1024 bytes, `lookup: [i32; 512]` = 2048 bytes) are
read from the *same* `symbol` index on every loop iteration in the baseline code, and the loop runs
this many times per MCU block across the whole image - so both tables are hot in L1/L2 cache almost
immediately and stay resident for the rest of the decode (combined ~3KB, far smaller than any
realistic L1). Removing one of the two reads on the fast path saves an L1-hit load, not a cache
miss - on a modern out-of-order core, an extra L1 load that isn't on the dependency chain feeding
the branch (`fast_ac != 0`) is cheap to schedule in parallel with other work and largely hidden by
the surrounding branch/shift/table-lookup chain that dominates this loop's critical path (refill,
peek_bits, the fast_ac branch itself, drop_bits). The profiled 15% cost of `decode_mcu_block`
likely comes from that critical path (the inherently serial bit-buffer state machine: refill depends
on the previous iteration's `bits_left`, peek/drop_bits depend on refill, the branch depends on the
fast_ac lookup) rather than from memory bandwidth/cache pressure on these two small tables - removing
one redundant but cheap, already-cached load doesn't shorten that critical path.

## What's not done / open questions

- This candidate only targeted the redundant-table-read angle; it did not attempt a restructuring
  of the serial bit-buffer state machine itself (refill/peek_bits/drop_bits dependency chain),
  which is the more likely source of the profiled cost per the analysis above - a genuine
  restructuring there (e.g. reducing the refill-to-peek dependency latency, or processing more bits
  per iteration) is a larger, riskier change than this session attempted and would need its own
  candidate.
- Not tried: merging `ac_lookup`/`lookup` into one combined table (one array of packed
  `(i32, i16)`-equivalent entries) so only one load happens regardless of which path is taken -
  this was considered but not implemented/measured, since the measured result here suggests the
  read itself isn't the bottleneck (see "Why the result looks the way it does"), so a combined-table
  variant would likely show the same null result; noted in `IDEAS.md` in case someone wants to
  verify that reasoning independently rather than relying on it.
- The pre-existing `set_options_refreshes_cached_dispatch` test failure (comparing two
  independently-computed function-pointer addresses for equality, which fails nondeterministically
  based on ASLR/codegen - reproduced identically with the patch fully reverted, so unrelated to this
  candidate) is not fixed here; flagged separately for `___BUGHUNT___/` per this session's final
  report rather than folded into this performance record.
