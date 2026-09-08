# Unify the `Level(level)` path's per-row filter-byte write with its row write

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

Leftover from candidate 004: the `DeflateCompression::Level(level)` arm of `write_image_data`
(`src/encoder.rs`) did `zlib.write_all(&[filter_type as u8])?; zlib.write_all(&current)?;` - two
separate `write_all` calls per row - where the `FdeflateUltraFast` arm already writes the filter
byte into `current[0]` and does one `compressor.write_data(current)` call per row. This patch sizes
`current` to `in_len + 1` (matching the `FdeflateUltraFast` arm's shape) and does the same
single-call pattern for the `Level` path.

## What was tried

Measured with `harness/encoder_bench --mode compress --input tests/benches/kodim02.png --only
Level6` (the `Level(6)` path is the crate's own default compression setting), `taskset -c 0`,
paired via `tools/paired.sh`.

| Variant | single-core p50 (paired, RUNS=15) | allocs / reallocs | notes |
|---|---|---|---|
| baseline (two `write_all` calls/row) | 69.745ms | 11 / 10 | |
| patched (one `write_all` call/row) | 69.476ms | 11 / 10 | |

Paired: median 69.745 -> 69.476ms (-0.4%), B (patched) faster in 8 of 15 pairs - **NOT consistent**,
within noise.

## Result

**Time: no change**, within this machine's noise floor (8/15 is not a consistent direction).
**Allocation count/bytes: unchanged** (11 allocations / 10 reallocations, identical before and
after) - expected, since this patch only changes how many `write_all` calls are made per row, not
how anything is allocated.

## Does it change behavior/output?

- **Bit-exact?** yes - `verify=OK`, identical output bytes (631220B) and ratio before and after.
- **Is this a breaking change?** No - internal buffer shape only.
- Project test suite (`cargo test --release`): **107 passed, 1 ignored, 0 failed**, identical
  before and after.

## Why the result looks the way it does

`flate2`'s `ZlibEncoder::write_all` does its own internal buffering before ever touching the
underlying `Vec` sink, so splitting one row's write into two `write_all` calls versus one was never
going to show up against ~70ms of actual zlib compression work for this row count (512 rows). The
backlog entry itself predicted this ("likely a very small win") - this candidate confirms "small"
rounds down to "not measurable here," which is itself useful to have on record so nobody re-tries
it expecting a win.

## What's not done / open questions

- None - this fully closes out the IDEAS.md entry it came from. No further follow-up planned.
