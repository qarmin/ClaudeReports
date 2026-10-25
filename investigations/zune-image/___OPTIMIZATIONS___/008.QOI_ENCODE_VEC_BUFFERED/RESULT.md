<!-- rustc 1.98.1 (48a229cea 2026-09-01), cargo 1.98.1 (797e8a9bc 2026-08-05) -->

# QOI encode: buffer opcodes into a local `Vec<u8>` instead of per-byte `ZWriter` writes

`zune_qoi::QoiEncoder::encode` (crates/zune-qoi/src/encoder.rs) writes every QOI opcode byte
individually through the generic `ZWriter<T>`/`ZByteWriterTrait` abstraction
(`stream.write_u8_err(...)`, `stream.write_const_bytes(&[..])`, etc.), even when the sink is a
concrete `&mut Vec<u8>` (the overwhelmingly common case - e.g. `zune-image`'s `QoiEncoder::encode_inner`
always calls it this way). This matches upstream issue
[#173](https://github.com/etemesi254/zune-image/issues/173), where the maintainer attributes the
gap partly to "a single API for writing and reading which means some functions won't be extremely
optimized". A `perf record` of a tight encode loop showed ~12% of self time inside
`__memmove_avx512_unaligned_erms` - a real libc call - attributable to single-byte writes going
through `extend_from_slice(&[u8; N])` across the `zune-qoi` -> `zune-core` crate boundary.

## What was tried

| Variant | mechanism | result |
|---|---|---|
| v1: `stream.reserve(self.max_size())` hint only | pre-reserve the sink's capacity before the write loop | **no measurable effect** - see "Why the result looks the way it does": `zune-core`'s blanket `impl<T: std::io::Write> ZByteWriterTrait for T` (`crates/zune-core/src/bytestream/writer/std_writer.rs`) makes `reserve_capacity` an unconditional no-op for any `T: Write`, including `&mut Vec<u8>`, because the `std` feature is on by default and the `Vec`-specific `reserve_capacity` impl (in `no_std_writer.rs`) only compiles under `#![cfg(not(feature = "std"))]`. Reverted, not part of the final patch. |
| v2 (applied): encode into a local, concretely-typed `Vec<u8>` via plain `.push()`/`.extend_from_slice()`, then flush it to the generic sink with one `ZWriter::write_all(&buf)` call at the end | removes all per-opcode `ZByteWriterTrait` generic dispatch from the hot per-pixel loop; only the final bulk write goes through the trait | real, modest, consistent win - see Result below |

### A confound found and corrected during this candidate

The first measurement pass used the project's own criterion harness
(a new `benchmarks/benches/encode_qoi.rs`, modeled on the existing `decode_qoi.rs` - no QOI
*encode* bench existed in this project before this candidate) and appeared to show a **~16%**
win (baseline median ~13.2ms -> patched median ~11.1ms, `taskset -c2`, two repeat runs each,
p < 0.05, tight confidence intervals). That number turned out to be an artifact of running the
baseline criterion session immediately after a long, from-scratch `cargo build --release` of the
*entire* `zune-benches` workspace (including `mozjpeg`/`libvips`/etc., several minutes of CPU load
right before the ~50s-long criterion run), versus the patched criterion session running after a
much shorter incremental rebuild - on a `powersave`-governor machine, that is enough build-heat
difference to change effective clock speed for the whole following benchmark. A quick, standalone,
non-criterion bench (`bench/src/main.rs` in this folder) run against the **unpatched** source
alone already reported ~11.0ms, i.e. close to what the confounded comparison had attributed to the
patch.

This was caught and corrected before writing up a result by switching to a genuine **alternating
paired comparison** (build both `bin_baseline`/`bin_patched` once, then run them `A,B,A,B,...`
back to back so any drift/throttling affects both sides equally) - see `MEASUREMENT.md`'s
paired-comparison discipline. The numbers below are from that corrected methodology, not the
initial confounded criterion numbers.

## Result

Standalone bench (`bench/src/main.rs`, 40 internal iterations per process invocation, 10% warmup
discarded, `min` reported), 15 alternating `A,B` pairs, `wikipedia_008.qoi` (1152x858 RGB,
1,521,134 bytes compressed / 2,965,248 bytes raw pixels):

- **Single-core** (`taskset -c 2`): patched faster in **15/15** pairs. Baseline median 10.807ms,
  patched median 10.651ms -> **-1.44%**. (First baseline sample was a 11.783ms cold-start outlier;
  excluding it changes the baseline median negligibly, to 10.806ms, delta -1.41%.)
- **Multi-core** (unpinned): patched faster in **14/15** pairs. Baseline median 10.914ms, patched
  median 10.638ms -> **-2.53%**.

This is a real, decisive (15/15 and 14/15 pairs, non-overlapping typical ranges), but *modest*
win - not the ~16% the initial (confounded) criterion comparison suggested. Time only; peak
RSS/allocation *count* were not separately measured (the patch does replace many small
`Vec::reserve` checks inside `extend_from_slice` with one upfront `Vec::with_capacity`, which
plausibly reduces allocation *count* even where the win in wall-clock is small, but this wasn't
instrumented for this candidate).

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified two ways:
  1. The standalone bench hashes (fnv1a64) the encoded output every iteration and asserts it's
     stable across iterations and across the baseline/patched binaries - hash
     `d105b17ecfe92427` for `wikipedia_008.qoi` on both.
  2. A direct byte-for-byte dump-and-compare (`cmp`) of baseline vs patched encoder output for the
     same input - `IDENTICAL`, matching md5sums.
- **Is this a breaking change?** No. Public API (`QoiEncoder::encode`) is unchanged; the only
  change is an internal, now-dead `encode_headers` helper method being removed (it was `fn`,
  private/crate-local, not part of the public API) and its logic inlined.
- Project test suite (`cargo test --release --workspace --no-fail-fast`), baseline vs patched:
  **525 passed / 39 failed in both** - identical failing tests before and after (all in
  `zune-inflate`, `zune-jpeg`, `zune-tests`, and four crates' doctests - none touch `zune-qoi`,
  confirmed pre-existing and unrelated to this patch). `zune-qoi`'s own test binary:
  **2 passed / 0 failed**, identical baseline vs patched.

## Why the result looks the way it does

The `perf record` self-time profile (see `bench/` in this folder, or re-run via `perf record -g`
against the standalone bench binary) attributed ~12% of self time to `memmove` calls from
single-byte `extend_from_slice` writes crossing the `ZWriter`/`ZByteWriterTrait` generic boundary.
Removing that indirection (v2) measures as only a ~1.4-2.5% whole-function win, not a ~12% one -
the memmove cost is real but is a fraction of a *self-time* profile of the *whole* encode
function, most of which (index-hash computation, the DIFF/LUMA/RGB opcode-selection branch chain,
the 4-byte pixel equality checks) is unaffected by this change and dominates the total. `v1`
(the `reserve_capacity` hint alone) had zero effect because it's silently a no-op for the common
`std`+`Vec<u8>` sink combination - see the table above; this is worth flagging as a small,
separate, non-performance finding about `zune-core`'s writer trait design (the hint can never
reach a `Vec`'s real `reserve` once `std` is enabled, regardless of the concrete sink type),
independent of whether v2's Vec-buffering approach is adopted.

## What's not done / open questions

- `QoiDecoder::decode_inner_generic` (the read side) was not touched - this candidate is encode
  only. A symmetrical read-side buffering/indirection-removal pass was not investigated; decode
  was already fairly close to `rapid-qoi` in the project's existing `decode_qoi.rs` bench, per
  prior observation, so it looked like a lower-priority target within this session's scope.
- The `reserve_capacity`-is-a-no-op-for-`std`+`Vec<u8>` design gap (v1's root cause) affects every
  encoder in `zune-core`'s generic-writer family (`zune-png`, `zune-farbfeld`, `zune-ppm`, etc.),
  not just `zune-qoi` - worth a maintainer decision on whether `zune-core`'s trait design should
  special-case this (not attempted here, would need either specialization or a different trait
  shape, out of scope for a single bounded candidate).
- Allocation *count*/peak RSS were not separately instrumented for v2 even though the mechanism
  (replacing many implicit small-growth checks with one upfront `Vec::with_capacity`) plausibly
  reduces allocation count - noted as a possible follow-up, not measured here.
