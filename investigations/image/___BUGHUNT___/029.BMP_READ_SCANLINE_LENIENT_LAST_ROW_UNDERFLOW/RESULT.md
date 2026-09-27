# BMP `read_scanline` last-row underflow when `data_offset` points past EOF

Investigated per an `IDEAS.md` backlog entry asking whether the 2bpp palettized BMP decode path
(`set_2bit_pixel_run`, `src/codecs/bmp/decoder.rs:871`) had been independently boundary-tested, and
whether a "2bpp RLE" variant even exists. Code reading first established that no such RLE variant
exists (BMP's only RLE compression tags are `BI_RLE4`/`BI_RLE8`, each tied to a matching
`bit_count`; `set_2bit_pixel_run` is only ever called from the ordinary, fully-buffered
`read_palettized_pixel_data` path, not any RLE path) and that `set_2bit_pixel_run` itself is safe
by construction (its `n_pixels` counter is always satisfiable given how `row_byte_length` is
derived from `width`). Manual boundary-value testing (widths 0/1/2/3/15/16/17/31/32/33/100,
heights 0/1/-1/-2, truncated pixel data at many offsets) found nothing. A short coverage-guided
libFuzzer burst seeded with those same crafted 2bpp BMPs, however, found a real, unrelated bug one
call frame up, in `read_scanline` - the row-reading helper shared by *every* BMP pixel-format
decode path (palettized 1/2/4/8bpp, 16-bit, 24-bit, 32-bit), not specific to 2bpp.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced every caller of `set_2bit_pixel_run` and every BMP compression-tag dispatch (`BI_RLE4`/`BI_RLE8` vs. bit_count validation) | N/A | Confirmed no "2bpp RLE" format exists in this codec; `set_2bit_pixel_run` is only reachable via the non-RLE, fully-buffered `read_palettized_pixel_data` path, where `indices` is always a complete, `read_exact`-populated row buffer sized to guarantee `n_pixels` is always satisfiable - ruled out as a distinct suspect. |
| Manual boundary-value construction: hand-crafted 2bpp `BI_RGB` BMPs via a standalone probe crate (`___BUGHUNT___/_reference/2bit_probe/`), fed through the public `image::load_from_memory` entry point (debug/overflow-checked build) | widths {0,1,2,3,15,16,17,31,32,33,100} x height=1; width=17 x heights {0,1,-1,-2}; width=100/height=4 truncated at 8 different byte offsets; a near-`i32::MAX` width truncated to a few bytes | No panics - every case either decoded correctly or returned a clean `ImageError`. |
| Coverage-guided fuzzing: `cargo fuzz run fuzzer_script_bmp` (libFuzzer, `fuzz/fuzzers/fuzzer_script_bmp.rs`, calls `image::load_from_memory_with_format(data, ImageFormat::Bmp)`), seeded with the 27 crafted 2bpp BMPs from the boundary-value step above, `fuzz/Cargo.toml`'s `image` dependency temporarily restricted to `default-features = false, features = ["bmp"]` | First attempt: default cargo-fuzz build (ASAN + `-Cdebug-assertions`, nightly toolchain) - **hung indefinitely** (still running, 99.9% CPU, past `-max_total_time`/`-timeout` budgets by 2-4x, with `-fork=1` mode's own child-watchdog also failing to kill it). Diagnosed via a from-scratch mutation-search script (`mutate_search.py`, 799 random mutations of the same seeds, each wrapped in an external `timeout -s KILL 1`, direct binary invocation, no libFuzzer/ASAN in the loop) that found **zero** hangs, and via an ASAN-disabled (`-s none`) rebuild that ran 200 real fuzzing iterations instantly with no hang - this is an ASAN-runtime/sandbox interaction in this specific environment (this environment also blocks `ptrace` for `gdb -p`; see `projects/image.md`), not a real algorithmic hang in the image crate. | Re-ran ASAN-disabled, debug-assertions build, `-max_total_time=110 -timeout=5`: **found a genuine panic** in `read_scanline` (`decoder.rs:730`, `attempt to subtract with overflow`) within ~230 fuzzer iterations, on a mutated 2bpp BMP whose `data_offset` field points past the actual end of the file. Minimized with `-minimize_crash=1` to a 70-byte input. |

Two escalating approaches were used (manual boundary-value construction, then a fuzz harness) per
`SKILL.md`'s soft cap of 3 - the fuzz harness produced a clear signal on its second (ASAN-disabled)
attempt, so no third approach was needed.

## Status

**Confirmed.** Reproduces 5/5 runs via the standalone `repro/` (deterministic single-input replay,
no randomness/layout dependence - a plain unsigned-integer-underflow arithmetic bug, not a
memory-layout-dependent one).

**Fully reproducible: Yes.** The repro calls the real public entry point
(`image::load_from_memory_with_format`) on a single hand-crafted/fuzzer-minimized BMP file - an
external artifact handed to a parser's real public entry point, the same bytes a real
attacker/user could supply as a `.bmp` file. There is no second participant/peer/wire exchange
involved (not about how one process reacts to another live process's behavior), so
`CONFIRMATION.md` section 11's "real second participant" requirement does not apply; a
single-process repro against the real decode entry point is the correct and sufficient shape.

## Class

crash

## Root cause

`read_scanline` (`src/codecs/bmp/decoder.rs`) special-cases the very last scanline in the default
`SpecCompliance::Lenient` mode, to tolerate real-world BMPs that are missing their final trailing
byte:

```rust
if is_last_row && spec_strictness == SpecCompliance::Lenient {
    let current_pos = reader.stream_position()?;
    let end_pos = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(current_pos))?;
    ...
    if Ok(head.len()) == usize::try_from(end_pos - current_pos) {
```

`current_pos` is the reader's position when the last row's read begins - derived from the file's
own attacker-controlled `data_offset` field (`BITMAPFILEHEADER.bfOffBits`) plus however many whole
rows have already been consumed. Nothing validates that `data_offset` actually lies within the
file before `read_palettized_pixel_data` (and the sibling 16-bit/24-bit/32-bit readers) seek
there and call `read_scanline`. A `Cursor<&[u8]>`/file `Seek` happily allows seeking past the
real end of the data, so when a malformed file declares a `data_offset` beyond its own length,
`current_pos > end_pos` on the last row, and the unsigned subtraction `end_pos - current_pos`
underflows. In a debug build (or any build with `overflow-checks = true`, which is Cargo's
default for the `dev`/`test` profiles used by `cargo build`/`cargo test`/`cargo run`) this panics
with `attempt to subtract with overflow`. In a plain `--release` build (`overflow-checks = false`
by default) it instead silently wraps to a huge `u64`, the `if` becomes false, and execution falls
through to the ordinary `read_exact` on line 737, which then correctly returns a clean
`UnexpectedEof` `io::Error` - so the panic is specifically a debug/overflow-checked-build hazard,
not a release-mode memory-safety bug.

`read_scanline` is shared by **every** BMP pixel-format decode path (`read_palettized_pixel_data`
for 1/2/4/8bpp, `read_16_bit_pixel_data`, `read_full_byte_pixel_data` for 24/32bpp, and the
bitfields readers) - the crafted repro happens to be a 2bpp image (since that's what the fuzz
corpus was seeded with, matching the original 2bpp candidate), but the bug is bit-depth-agnostic;
the single fix below (inside `read_scanline` itself) covers all call sites at once, so no separate
per-bit-depth check was needed.

The original suspect, `set_2bit_pixel_run` itself, remains ruled out as safe - it is only reached
*after* `read_scanline` has already successfully filled the row buffer, so this bug pre-empts it
entirely for the specific input that triggers it.

## Introduced by

ed6b3fbe "BMP: Implement lenient decoding for missing final byte on last row" (2026-04-28)

## The fix

Replace the unchecked `end_pos - current_pos` with `end_pos.checked_sub(current_pos)`, and treat
`None` (i.e. `current_pos > end_pos`) the same as "not exactly one byte short" - fall through to
the ordinary `read_exact(buf)`, which already correctly produces a clean `UnexpectedEof` error for
this case (matching the release-build behavior described above, now made the behavior in every
build profile):

```rust
if let Some(remaining) = end_pos.checked_sub(current_pos) {
    if Ok(head.len()) == usize::try_from(remaining) {
        reader.read_exact(head)?;
        *last = b'\0';
        return Ok(());
    }
}
```

Verified: `repro/` no longer panics against the patched tree (`did NOT reproduce: decode returned
a clean error: failed to fill whole buffer`, 5/5 runs) - see `output.txt` for the actual
before/after captured output.

- Project test suite (`cargo test --lib`): **331 passed, 0 failed** before the fix, **331 passed, 0
  failed** after the fix (identical - no test in the suite already covered this path, and the fix
  doesn't change behavior for any well-formed or previously-tested malformed input).

## What's not done / open questions

- The fix was verified against the specific `data_offset`-past-EOF shape found by fuzzing; it was
  not separately re-verified with hand-crafted 1bpp/4bpp/8bpp/16bpp/24bpp/32bpp variants of the
  same `data_offset`-past-EOF condition, though the fix lives in the single shared `read_scanline`
  function so it structurally covers all of them the same way it covers the 2bpp case tested here.
- The apparent ASAN hang in this environment (see "What was tried" above) is recorded as a tooling
  limitation in `projects/image.md` for future sessions - it is not itself a finding about the
  `image` crate.
- A longer, dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately, ASAN disabled per the
  workaround here) against `fuzzer_script_bmp` and the other format fuzzers would likely be
  worthwhile - this session's burst was only ~110 seconds and found this on essentially the first
  useful mutation.
