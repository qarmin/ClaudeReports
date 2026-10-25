# `Compression::decompress_dynamic()` spins forever on a gzip/deflate stream that has bytes after its end

Found because the GDScript mutational fuzz loop (`PackedByteArray.decompress_dynamic()` on mutated gzip data)
hung at seed 3. The hanging 37-byte input was a complete, valid gzip stream followed by 5 trailing bytes
(checked with Python's `zlib.decompressobj(31)`: 500 bytes out, `eof=True`, `unused_data` = 5 bytes).

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of gzip/zstd/deflate/fastlz data (`decompress`/`decompress_dynamic`) | ASan editor build `b8420aa3a6` | zstd/deflate/fastlz clean (seeds 1-2, 1500 it.); gzip `decompress_dynamic` hangs (seed 3) |
| Truncated gzip streams (every cut point 1..34 of a 35-byte stream) | `repro`-style script, 8 s timeout | all return 0 bytes cleanly (Z_BUF_ERROR path), no hang - truncation is **not** the trigger |
| Valid gzip + 1 trailing byte (`repro/project/repro.gd`) | unfixed build, 20 s timeout | hangs 10/10 (100% CPU, no memory growth) |
| Fix verification | `patch.diff`, rebuilt | repro returns 1000 bytes, exit 0, 10/10; gzip and raw-deflate, sizes 1000/16384/40000, 0/1/5/100 trailing bytes: 24/24 round-trip equal |

## Status

**Confirmed.** Deterministic: 10/10 hangs before, 10/10 clean after.

**Fully reproducible**: Yes - real `PackedByteArray.decompress_dynamic()` entry point on gzip data produced by the
engine's own `compress()` plus one appended byte. Other callers found: the C# glue and the objectdb profiler snapshot loader.

## Class

`logic` (infinite loop / denial of service: 100% CPU, never returns)

## Root cause

`core/io/compression.cpp`, `decompress_dynamic()` (gzip/deflate branch), inner loop:

```cpp
do {
    ret = inflate(&strm, Z_SYNC_FLUSH);
    switch (ret) { ... error cases return ... }          // Z_STREAM_END is not handled
} while (strm.avail_out > 0 && strm.avail_in > 0);
```

When the stream ends (`ret == Z_STREAM_END`) while input bytes remain (`avail_in > 0`) and the output chunk
is not full, the loop condition stays true. zlib's `inflate()` then keeps returning `Z_STREAM_END` without
consuming input or producing output, so the loop never terminates and the outer `while (ret != Z_STREAM_END)` is
never evaluated. Any trailing byte after a complete stream (padding, a second gzip member, junk) triggers it.

## Introduced by

6584db1538 HTTPRequest now accepts gzip Added request_raw to HttpRequest Added decompress_dynamic to Compression class Added decompress_dynamic to BytePoolArray (the original implementation)

## The fix

`patch.diff` (+1/-1): add `ret != Z_STREAM_END &&` to the inner loop condition. Trailing bytes are then ignored
(the previous, intended behaviour for single-member streams); concatenated gzip members are not decoded
(that was never supported - only the first member is). Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`, needs
network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone (full ASan scons build per variant); `repro/run.sh` run against the
unfixed and fixed binaries built from the main tree. Brotli branch of the same function was not fuzzed.
`Compression::decompress()` (fixed-size) was only fuzzed briefly (deflate/fastlz, no findings).
