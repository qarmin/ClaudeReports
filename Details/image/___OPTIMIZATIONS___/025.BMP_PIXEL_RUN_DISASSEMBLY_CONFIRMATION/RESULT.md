# BMP palette-lookup fixed-array fix: disassembly confirmation of the 8/4/2/1-bit lookups-per-iteration theory

Answers `IDEAS.md`'s top-of-backlog entry: candidate 008 (`set_8bit_pixel_run`, ~3-5% win) and
candidate 011 (`set_4bit_pixel_run`/`set_2bit_pixel_run`/`set_1bit_pixel_run`, ~7.5-10% win)
applied the identical fix (bind the BMP palette as `&[[u8; 3]; 256]` instead of `&[[u8; 3]]` so
the compiler can prove the pixel index is always in bounds and elide the per-pixel bounds check),
but neither candidate had `objdump`/`perf` available to confirm *why* the sub-8-bit functions won
by more. The working theory (unverified until now): `set_1bit_pixel_run`/`set_2bit_pixel_run`/
`set_4bit_pixel_run` perform up to 8/4/2 palette lookups per outer-loop iteration (one per
unpacked sub-index) versus exactly 1 for `set_8bit_pixel_run`, so eliding one bounds check per
lookup compounds more per iteration. `objdump`/`nm` (binutils) are installed on this machine even
though `perf` is not (see `README.md`), and were used directly on the release `decode` bench
binary's disassembly - the same technique candidate 003 used for its own inlining investigation.

This candidate re-applies candidates 008+011's fix as a single merged patch (structurally
identical to concatenating both diffs; `patch.diff` in this folder is the combined change) so all
four functions can be disassembled from one binary pair, and adds the disassembly-diff evidence
neither candidate had. No new source-level idea is introduced - this is a confirmation/
measurement candidate over an already-known fix.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Built two `--release` `decode` bench
binaries (baseline: unpatched `src/codecs/bmp/decoder.rs`; patched: candidates 008+011's fix
merged into one patch, see `patch.diff`), saved as `tmp/decode_baseline`/`tmp/decode_patched`
(not part of this folder - reproducible via `just test`).

**Disassembly** (`objdump -d -C -M intel --no-show-raw-insn`, `nm --demangle` to locate symbols):

- `set_8bit_pixel_run` and `set_4bit_pixel_run` are standalone symbols in both binaries (real
  `call` sites from `read_image_data`, confirmed via `nm`/`objdump` - not inlined).
- `set_2bit_pixel_run` and `set_1bit_pixel_run` are **fully inlined** into
  `<BmpDecoder<...>>::read_image_data` in *both* binaries (no standalone symbol in either `nm`
  listing) - so their loop bodies were located and compared inside that function's disassembly
  instead of as their own symbols.
- For each bit depth, located the outer-loop's dominant unrolled body (the code path that
  actually runs once per pixel/byte, as opposed to the one-time head/tail remainder handling) and
  counted how many `cmp <index>,<palette-len-derived-reg> / jbe <panic_bounds_check>` sequences
  immediately precede a palette-entry load (`lea r,[r+r*2]` then `movzx`/`mov` from the palette
  base pointer) in the **baseline** build, and how many of those survive in the **patched** build.

| Bit depth | Lookups/iteration (loop shape) | Palette bounds checks in baseline's hot-loop body | Palette bounds checks in patched's hot-loop body | Bounds checks eliminated |
|---|---|---|---|---|
| 8-bit (`set_8bit_pixel_run`, standalone fn) | 1 | 1 (at `+0xd0` in the function, `cmp rdx,r10`/`jbe`) | 0 | 1 |
| 4-bit (`set_4bit_pixel_run`, standalone fn) | 2 | 2 (nibble 1 at `+0x114`, nibble 2 at `+0x161`, both `cmp rdx,r*`/`jbe`) | 0 | 2 |
| 2-bit (`set_2bit_pixel_run`, inlined into `read_image_data`) | 4 | 4 (one per 2-bit sub-index: `>>6`, `>>4&3`, `>>2&3`, `&3`, each its own `cmp rsi,rdi`/`jbe`) | 0 | 4 |
| 1-bit (`set_1bit_pixel_run`, inlined into `read_image_data`) | 8 | 8 (one per bit position `0x80`..`0x01`, each its own `cmp rsi,r10`/`jbe`) | 0 | 8 |

This is an exact 1:2:4:8 ratio, matching the loop shape (1/2/4/8 palette lookups per outer-loop
iteration for 8/4/2/1-bit respectively) precisely - not just "more checks removed for the smaller
bit depths" but the *exact* count predicted by counting sub-index unpacks in each
`macro_rules! set_pixel!`/loop body.

Byte-size deltas for the two standalone functions (whole function, including one-time
head/tail/error paths, not just the hot loop):

| Function | baseline size | patched size | delta |
|---|---|---|---|
| `set_8bit_pixel_run` | 368 bytes | 320 bytes | -48 bytes (-13.0%) |
| `set_4bit_pixel_run` | 544 bytes | 448 bytes | -96 bytes (-17.6%) |

`set_4bit_pixel_run` (2 lookups/iteration) shrinks by exactly 2x as many bytes as
`set_8bit_pixel_run` (1 lookup/iteration) - consistent with 2 bounds checks removed vs. 1.
`read_image_data` (which contains the inlined 2-bit/1-bit loops, plus a large amount of unrelated
header-dispatch code) shrinks from 5056 to 4837 bytes (-219 bytes, -4.3% of the *whole* function,
diluted by all the unrelated code sharing that symbol).

**Timing** (same `benches/decode.rs` criterion harness as candidates 008/011/020, filtered to
`Core_8_Bit.bmp`/`Core_4_Bit.bmp`/`Core_1_Bit.bmp`, the largest samples of each depth already in
the repo's BMP corpus/`BENCH_DEFS`), measured fresh this session with the merged patch to confirm
the combined build reproduces both original candidates' numbers together, 5 alternating pairs
each, `--measurement-time 0.4-0.5 --warm-up-time 0.15-0.3 --sample-size 15-20`:

| Variant | single-core median (5 pairs) | multi-core median (5 pairs) | notes |
|---|---|---|---|
| `Core_8_Bit.bmp` baseline | 202.0 µs | 103.0 µs | |
| `Core_8_Bit.bmp` patched | 196.3 µs (-2.8%) | 97.8 µs (-5.1%) | multi-core 5/5 wins; single-core 2/5 clear wins, noisy (this machine's known noise floor - see `README.md`/`IDEAS.md`) |
| `Core_4_Bit.bmp` baseline | not cleanly measured (see below) | 115.6 µs | |
| `Core_4_Bit.bmp` patched | not cleanly measured (see below) | 106.0 µs (-8.3%) | multi-core 5/5 wins; single-core hit a severe machine-noise episode mid-run (one baseline sample read 105.9 µs, another 237 µs, with no code difference) and was abandoned rather than reported as a number - candidate 011 already documents this exact size/depth needing an idle core; not re-chased here since the disassembly evidence above is unambiguous regardless |
| `Core_1_Bit.bmp` baseline | 147.1 µs | 74.4 µs | |
| `Core_1_Bit.bmp` patched | 139.5 µs (-5.2%) | 68.7 µs (-7.7%) | multi-core 5/5 wins; single-core 5/5 wins |

All three multi-core deltas land within the same range candidates 008/011/020 originally reported
separately (8-bit ~3-5%, 4-bit ~8-10%, 1-bit ~7.5-8%), confirming the combined single-session
build behaves the same as the two original patches applied separately.

## Result

The disassembly directly confirms `IDEAS.md`'s working theory, with an exact count rather than an
inference from timing alone: the patched build eliminates precisely 1/2/4/8 palette-lookup bounds
checks per outer-loop iteration for 8/4/2/1-bit respectively - exactly matching each function's
number of palette lookups per iteration (1/2/4/8). This is not a case of "more lookups happen to
correlate with a bigger win" - it is the literal mechanism: each additional sub-index unpacked
per iteration (via `macro_rules! set_pixel!` for 4-bit/2-bit, or the bit-test loop for 1-bit) had
its own independent bounds check in the baseline build, and the patch removes all of them
simultaneously since they all index the same newly-fixed-size palette array. The timing
re-measurement this session (multi-core, 5/5 paired wins each depth) reproduces candidates
008/011's original percentages closely enough to confirm the combined build isn't behaving
differently from the two separate patches.

## Does it change behavior/output?

- **Bit-exact?** Yes. Decoded and hashed (temporary `examples/_correctness_check_bmp.rs`, not
  saved - reproducible from the patch: `image::open` + `to_rgba8` + `DefaultHasher` over the raw
  byte buffer) all 66 `.bmp` files under `tests/images/bmp/` with baseline and patched binaries:
  every hash and decoded width/height matched exactly (`diff` of the two hash listings was empty).
- **Is this a breaking change?** No. Same as candidates 008/011: internal-only signature change to
  four private (non-`pub`) functions and two local variable bindings inside private methods - no
  public API surface touched. This candidate's patch is a straightforward merge of 008's and 011's
  already-characterized changes, not a new behavior change.
- Project test suite (`cargo test --release`): 550 passed / 1 failed before, 550 passed / 1 failed
  after - the same single pre-existing failure both times (`resizing_with_catmul` in
  `tests/regression.rs`, matching every prior BMP-decoder candidate in this workspace: 008, 011,
  016, 019, 020). No new failures, no `^error` lines from a build failure in either run.

## Why the result looks the way it does

This candidate *is* the "why" for candidates 008/011's original open question - see "Result"
above. The mechanism is now confirmed at the instruction level rather than inferred from timing:
LLVM cannot prove `palette[idx as usize]` in bounds when `palette: &[[u8; 3]]` (an unsized slice
whose length isn't known at compile time), so every one of the loop's palette lookups compiles to
an explicit `cmp` + conditional branch to a shared `panic_bounds_check` call. Binding the same
data as `&[[u8; 3]; 256]` gives the length to the type system, and LLVM removes exactly the
lookups it can now prove are always in range - which for `set_4bit_pixel_run`/`set_2bit_pixel_run`
is a masked value (`idx & 0xf`, `(idx >> 6) & 0x3`, etc.) already guaranteed less than 16/4, and
for `set_1bit_pixel_run` is a boolean-derived `0`/`1`, all trivially provable once the array length
is a compile-time constant. Since each function's macro-expanded or manually-unrolled loop body
repeats this same lookup pattern once per sub-index, the total number of eliminated branches
scales linearly with lookups-per-iteration - 1, 2, 4, 8 - which is exactly what was measured.

## What's not done / open questions

- The 2-bit/1-bit loops were compared as inlined code inside `read_image_data` rather than as
  standalone function disassembly (no `#[inline(never)]` was added to force them into their own
  symbols) - the loop-body boundaries used for the comparison table above were identified by
  reading the surrounding code (mask constants `0x3`/bit positions `0x80`>>N, and the palette
  3-byte-stride address arithmetic), not by a clean function-level `nm` lookup the way 8-bit/4-bit
  were. A future session wanting a fully mechanical (scriptable) version of this comparison could
  add a temporary `#[inline(never)]` to `set_2bit_pixel_run`/`set_1bit_pixel_run` to force them
  into standalone symbols, re-disassemble, and confirm the counts match this candidate's manual
  reading (expected to, since the source-level masking logic is unchanged either way).
- `Core_4_Bit.bmp`'s single-core timing wasn't cleanly re-measured this session (a severe
  mid-run machine-noise episode made the 5-pair sample untrustworthy, see the timing table above)
  - not chased further since the disassembly evidence is unambiguous and multi-core timing
  reproduced candidate 011's original number closely. A future session with more time budget
  could re-run single-core for this one case specifically on a confirmed-idle core.
- RLE4's absolute/encoded-run call site and RLE8's absolute-mode call site (both patched, both
  covered by the bit-exact hash check via the RLE test files under `tests/images/bmp/`) were not
  separately disassembled - same "not benchmarked in isolation" gap candidates 008/011 already
  left open for those call sites; this candidate's disassembly focused on the plain
  (non-RLE) `read_palettized_pixel_data` path shared by all four bit depths, which is where the
  original theory's loop-shape claim actually lives.
