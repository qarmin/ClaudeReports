# zune-inflate `StreamingDecoder::decode_chunk` never resets the bitstream's chunk-relative read position, corrupting any multi-chunk decode

Found incidentally while working `___OPTIMIZATIONS___/005.PREMUL_ALPHA_U8_U16_SIMD` (running the
full workspace test suite as before/after validation for an unrelated `zune-imageprocs`
performance patch) - no connection to that candidate's own fix. `cargo test --release -p
zune-inflate` was seen to fail 30 of its 59 lib tests on a clean, unmodified checkout of
`base_commit` below, so this session investigated further.

`crates/zune-inflate/src/tests.rs`'s own `run_streaming_test` harness follows the exact usage
pattern documented in `StreamingDecoder::decode_chunk`'s own doc comment (`streaming_decoder.rs`,
"# Usage" section): feed the compressed stream to `decode_chunk` in caller-chosen chunk sizes,
advancing the input offset on `NeedsMoreInput` and retrying the same chunk on `NeedsMoreOutput`.
Every test in the suite that exercises more than a single `decode_chunk` call (small chunk sizes,
large inputs that need multiple 64KB+ reads, etc.) fails.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo test --release -p zune-inflate --lib`, in the main checkout | rustc 1.90.0, release profile | Confirmed: 29 passed / 30 failed, every failure a panic in `run_streaming_test`'s own `DecodeStatus::Error`/length-mismatch assertions, various underlying error messages ("corrupt block", "Len and nlen do not match", "Cannot work with empty pre-code table", "asked for more input but fed the final chunk") |
| Standalone scratch binary (`/tmp/inflate_repro`, path-dependency on the real crate) calling `StreamingDecoder::decode_chunk` directly, bisecting to the smallest failing case | release build | A single 1-byte payload (`[0u8]`, 3-byte compressed form), fed to `decode_chunk` one compressed byte per call, already fails - not something that requires a large/edge-case input |
| Comparing a single `decode_chunk(&compressed, true, ...)` call (whole stream at once) against the same stream split into three 1-byte calls | release build | Whole-stream call: `Finished`, correct output. Split into 1-byte calls: 3rd call returns `Error(CorruptData)` |
| Read `StreamingBitStreamReader::refill`/`refill_slow`/`update_chunk` (`streaming_decoder/streaming_bitstream.rs`) | read-only | `position` is used as an index into whatever `chunk` slice the *current* call received, but nothing resets it to 0 between calls - it only ever grows via `refill`'s own bookkeeping |
| Grepped for `StreamingDecoder` usage elsewhere in the workspace | read-only | `crates/zune-png/src/streaming.rs`'s `process_zlib_stream` is the only other caller, and it manually calls `decoder.reset_position()` before every genuinely new `byte_buf` read (line 128) - a pre-existing (but undocumented, not part of `decode_chunk`'s own contract) workaround for exactly this bug, which is why PNG decoding through the crate's actual production path is unaffected even though the crate's own test suite, following the *documented* public API usage, is not |
| Decoded `test-images/png/benchmarks/speed_bench.png` (6.3MB, IDAT well over the crate's 64KB internal read-chunk size) via the real `zune_png::PngDecoder` public API, unmodified `base_commit` | release build, standalone scratch binary | Succeeded - confirms zune-png's own `reset_position()` workaround fully protects its own decode path from this bug |
| Patch A (first attempt): reset `position = 0` unconditionally inside `StreamingBitStreamReader::update_chunk` | applied to a scratch copy (`/tmp/zune-inflate-patched`), `cargo test --release --lib` | Fixed 24 of the 30 originally-failing tests, but **introduced 3 new regressions** (`test_output_grows_by_one`, `test_match_straddles_needs_more_output` hung in an infinite loop, `test_store_block_tiny_output`) - all involving the documented `NeedsMoreOutput`-retry-with-the-same-chunk contract, which this naive fix broke by also resetting position on a same-chunk retry |
| Patch B (final): track whether the previous `decode_chunk` call returned `NeedsMoreOutput` (`awaiting_output_retry` field on `StreamingDecoder`); only call `stream.update_chunk` (which resets `position`) when NOT retrying the same chunk | same scratch copy, `cargo test --release --lib`, 3 consecutive runs | 59 passed / 0 failed, all 3 runs |
| Applied Patch B to the actual working checkout (temporarily, reverted after) and ran `cargo test --release -p zune-png` (lib + all `tests/*.rs` integration tests) | release build | All passed, no regressions - confirms the fix is compatible with zune-png's own existing `reset_position()` workaround |
| `just test` against a fresh `git clone` of the public repo at `base_commit` (HTTPS; SSH unavailable in this sandbox) | real clone, real standalone `repro/` binary using the documented public API | Confirmed end-to-end: reproduces before the patch (exit 1, "REPRODUCED - ... Error: Corrupt data"), fixed after (exit 0) |

## Status

**Confirmed**.

Reproduces deterministically - every run, both via the crate's own 30-test lib suite and the
minimal 1-byte standalone repro, on a clean unmodified `base_commit` checkout.

**Fully reproducible**: **Yes**. `repro/` is a standalone binary crate depending on `{ path =
"../clone/crates/zune-inflate" }`, calling the real, public `StreamingDecoder::decode_chunk` API
exactly per its own doc comment's usage example (feed chunks, advance on `NeedsMoreInput`, retry
on `NeedsMoreOutput`) - not a fabricated or internal-only entry point. No second participant is
involved (single-crate API misuse-of-its-own-contract, not an interaction between two components).

## Class

logic (incorrect internal state carried across `decode_chunk` calls; not memory-unsafe - `position`
staying stale just causes `refill`/`refill_slow` to silently stop reading further input bytes,
which the decoder then reports as spurious `Error`s rather than out-of-bounds access).

## Root cause

`StreamingBitStreamReader::position` is a byte index into the `chunk: &[u8]` slice passed to the
*current* `decode_chunk` call. Per `decode_chunk`'s own documented usage, every call except a
`NeedsMoreOutput` retry receives a brand-new slice that logically starts back at relative index 0
(a fresh window into the compressed stream, e.g. `&compressed[input_pos..end]`) - but nothing in
`decode_chunk`/`update_chunk` ever resets `position` back to 0 for that new slice. Once `position`
advances past the length of whatever chunk happens to be shorter than the previous one (trivially
true for a 1-byte chunk size, but equally true for any workload where the compressed stream needs
more than one internal 64KB-ish read), every subsequent `refill`/`refill_slow` call's
`src.get(self.position..)` finds nothing to read (the slice is exhausted relative to the stale
`position`), so the reader silently stops accumulating any further input bits even though fresh
bytes were genuinely supplied. Depending on which decode state this starvation hits, it surfaces as
one of several different error messages (`CorruptData`, "Len and nlen do not match", "Cannot work
with empty pre-code table", etc.) or, for calls made right at the true end of input, an incorrect
`NeedsMoreInput` on what should have been the final chunk.

A `reset_position()` method already existed on `StreamingDecoder`/`StreamingBitStreamReader`
specifically for this purpose, but `decode_chunk` never calls it itself - it's left for the caller
to discover and call manually, which is undocumented and not shown anywhere in `decode_chunk`'s own
"# Usage" doc example. `zune-png`'s own `process_zlib_stream` independently discovered/implemented
this workaround (calling `decoder.reset_position()` before every new IDAT byte-buffer read), which
is why the crate's actual production PNG-decoding path is unaffected - but the crate's own test
suite (following the documented public contract literally, with no knowledge of this workaround)
and any other direct consumer of `zune_inflate::StreamingDecoder` following the published docs
would hit this bug.

The fix is not as simple as resetting `position` on every call, though: `decode_chunk` also uses
`position` to resume mid-chunk on a `NeedsMoreOutput` retry (the caller is contractually required
to pass the *same* chunk again, and the decoder has its own internal snapshot/rollback logic,
e.g. around the `snapshot_pos` used in the fast Huffman decode loop, that depends on `position`
carrying over correctly across that retry). An unconditional reset breaks that second contract
instead (confirmed above - Patch A's 3 new regressions, all `NeedsMoreOutput`-retry scenarios).

## Introduced by

2901c37b "png: Initial streaming version" - identified via `git log -S "fn decode_chunk" --
crates/zune-inflate/src/streaming_decoder.rs`, which shows this as the commit that introduced
`decode_chunk` (and the `StreamingBitStreamReader`/`position` design) in the first place; none of
the later commits touching this file (`6b3291bd`, `410ba6b6`, `ae97369a`, `0ed11509`, `961f9324`)
added or removed a `position` reset.

## The fix

Two coordinated changes, both required together (see "Root cause" above for why a single-sided fix
regresses the other contract):

1. `StreamingBitStreamReader::update_chunk` now resets `self.position = 0` (it previously only
   updated `is_final` and masked the bit buffer) - this is the natural, minimal place for it: every
   call announces "here is a genuinely new/updated chunk slice", so of its two existing callers
   only one wants a reset.
2. `StreamingDecoder::decode_chunk` becomes a thin wrapper around the old function body (renamed
   `decode_chunk_inner`): it now tracks an `awaiting_output_retry: bool` field, set whenever the
   previous call returned `NeedsMoreOutput`, and only calls `stream.update_chunk(..)` (and thus
   only resets `position`) when NOT continuing such a retry - preserving `position` exactly when
   the documented "retry with the SAME chunk" contract requires it.

Verified against the checkout this bug was found in (not just the standalone `repro/` clone):

- `cargo test --release -p zune-inflate --lib`: 29 passed/30 failed -> 59 passed/0 failed, 3
  consecutive runs.
- `cargo test --release -p zune-png` (lib + all `tests/*.rs`): all passed both before and after (no
  regression - confirms compatibility with zune-png's own pre-existing `reset_position()`
  workaround).
- `cargo test --release -p zune-image --lib`: 36 passed/0 failed, unaffected.
- `patch.diff` diffstat: `+32/-2` across `crates/zune-inflate/src/streaming_decoder.rs` and
  `crates/zune-inflate/src/streaming_decoder/streaming_bitstream.rs` (`git diff --numstat` at
  capture time).

Not fixed by this patch, left as a separate, smaller, pre-existing issue noticed along the way: the
rustdoc example inside `decode_chunk`'s own "# Usage" doc comment does not compile
(`cargo test --release -p zune-inflate --doc` fails with `E0433`/`E0425` - it references
`DecodeStatus`/`DeflateOptions` without importing them, and the snippet is not written as a
complete, runnable example). Not investigated further or fixed here since it's a documentation
build issue, unrelated to this bug's runtime behavior - worth a follow-up candidate or a quick
maintainer fix on its own.

## What's not done / open questions

- Not checked: whether `zune-png`'s `process_zlib_stream` also relies on `position` in some other
  way that this patch's `awaiting_output_retry` gating could subtly interact with beyond what the
  `cargo test -p zune-png` run already exercises (e.g. the `slide_window`/`current_dest_offset`
  interplay on very large, multi-frame APNG streams) - the existing zune-png test suite doesn't
  include a multi-64KB-chunk stress test of its own, so this patch's compatibility rests on the
  general-purpose zune-inflate test suite plus zune-png's existing (smaller) fixtures passing, not
  a dedicated large-PNG-through-zune-png regression check.
- Not investigated: whether a longer, dedicated fuzzing campaign (via `fuzz-bug-repro`) against
  `StreamingDecoder::decode_chunk` with randomized chunk sizes/output-growth patterns would turn up
  a related bug in the `NeedsMoreOutput` snapshot/rollback path itself (only the specific
  position-reset bug reported here was chased down; the broader chunked-decode state machine was
  not otherwise audited).
- The broken `decode_chunk` doctest (noted above, under "The fix") is a distinct, smaller
  documentation-only bug, not fixed by `patch.diff` - flagged here rather than filed as its own
  candidate since it doesn't affect runtime behavior.
