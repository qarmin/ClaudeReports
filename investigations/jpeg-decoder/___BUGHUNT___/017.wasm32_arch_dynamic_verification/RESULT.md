# wasm32_arch_dynamic_verification

`src/arch/wasm.rs`'s unsafe SIMD blocks were only compile-checked with `target-feature=+simd128`
forced in candidate 013.unsafe_blocks_triage_sweep - no wasm runtime/test runner was wired up that
session, so there was zero dynamic coverage of this code path. This candidate closes that gap by
actually building and running the crate on the real `wasm32-unknown-unknown` target under a real
wasm runtime (`wasm-bindgen-test-runner` + Node, the exact mechanism
`.github/workflows/rust.yml`'s `cross_compile_wasm` CI job already uses, but which had apparently
never been run with `target-feature=+simd128` forced even in CI - see "What's not done" below).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Get a real wasm runtime working locally, matching the project's own CI mechanism (`wasm-bindgen-test-runner` downloaded from the same GitHub release CI uses, `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` env var, Node v22.23.2 as the backend) instead of only compile-checking | `+1.87.0` toolchain (pinned `1.61.0` can't build the current `Cargo.lock`'s `wasm-bindgen`/`wasm-bindgen-test` deps - same finding as candidate 013), `wasm-bindgen`/`wasm-bindgen-test` resolved to latest-compatible (0.2.127/0.3.77) with a matching `wasm-bindgen-cli` release download | Works; confirmed the existing single wasm-bindgen-test (`tests/lib.rs::included_file`) actually executes under this setup, and (via a temporary `panic!()` sanity check, reverted) that `RUSTFLAGS="-C target-feature=+simd128"` genuinely routes through `arch::wasm::dequantize_and_idct_block_8x8`/`color_convert_line_ycbcr`, not the scalar fallback |
| Real dynamic boundary-value test (new, temporary `tests/wasm_simd_boundary.rs`, not carried into `src/`) - 12 real fixtures spanning every residue mod 8 from width 1-33 (`tests/reftest/images/mozilla/jpg-size-{1,2,5,7,8,9,15,16,17,31,32,33}x{same}.jpg`, matching candidate 013's own miri boundary set minus the two the project's own `tests/reftest/images/disabled.list` already excludes as not reftest-comparable at all) plus 3 grayscale fixtures (isolate IDCT-only from color-convert), decoded through the real public `Decoder` API with `simd128` forced, compared against the project's own pre-existing PNG references using the exact tolerance `tests/reftest/mod.rs`'s own `reftest_decoder` applies (`diff <= 3` per channel for non-lossless coding) | `cargo +1.87.0 test --target wasm32-unknown-unknown`, `RUSTFLAGS="-C target-feature=+simd128"`, `wasm-bindgen-test-runner` + Node | 15/15 pass. (First pass, before switching the oracle from "match native ssse3 output" to "match PNG with the project's own tolerance", showed 8 apparent mismatches for widths >=8 - all resolved to ordinary, tolerance-covered fixed-point rounding differences between the wasm and ssse3 backends, the same kind of arch-to-arch variance the project's own reftest already accepts; not a bug, see below) |
| Real dynamic malformed-input test (new, temporary `tests/wasm_crashtest.rs`) - all 111 fixtures from the project's own `tests/crashtest/images/` (the corpus the native `crashtest()` test already runs, just never previously on wasm32), same real-runtime setup, `simd128` forced, each fixture only required to decode without panicking/trapping (`Ok` or `Err` both fine) - the same bar the native test applies | Same build/runtime as above | 110/111 pass; 1 failure (`max_size.jpg`) - see "Status" below, this is the actual finding |

An 8-mismatch false alarm surfaced on the first pass of the boundary-value test (comparing wasm
output directly against native ssse3 output instead of against the project's own PNG ground
truth): all 8 mismatches were off by at most 1 per channel, i.e. within the same +/-3
fixed-point-rounding tolerance `tests/reftest/mod.rs` already applies between different SIMD
backends for non-lossless JPEGs - re-running against the actual PNG references with that same
tolerance (matching the project's own correctness bar, not an ad hoc stricter one) showed no
mismatch at all. This is recorded here so a future session doesn't need to rediscover it: the wasm
IDCT/color-convert kernels' own `assert!`-guarded bounds and their actual pixel output are both
fine; the only real finding is the integer-overflow bug below, which is unrelated to the SIMD
kernels' own correctness.

## Status

**Confirmed.** `tests/crashtest/images/max_size.jpg` (a small, 186-byte, well-formed-per-SOF-syntax
JPEG declaring 65535x65535 dimensions, 1 grayscale component, 1x1 sampling - part of the project's
own existing malformed-input corpus) panics when decoded via the real, public
`jpeg_decoder::Decoder::decode()` API on the real `wasm32-unknown-unknown` target under a real
wasm runtime, 3/3 runs, deterministic (not layout/seed-dependent - this is a pure integer-overflow
condition, not a race or an environment-dependent allocation failure). Two additional,
independently-constructed inputs in the same class (see "vary the input" below) reproduce the same
panic 100% of the time as well. The identical file decodes cleanly (returns `Err`, no panic) on
native x86_64 (both debug and release, `+1.87.0`), because the vulnerable computation is a `usize`
multiplication and `usize` is 64 bits there - this bug is specific to 32-bit targets (wasm32 here;
also true of any other 32-bit Rust target, e.g. `i686`/`armv7`, though only wasm32 was actually
verified this session, since that was this candidate's scope).

**Fully reproducible**: **Yes**. The repro drives the real, unmodified, public `Decoder::decode()`
entry point with a real external JPEG file (byte-for-byte the project's own existing crashtest
fixture, plus two files built by patching only the two big-endian width bytes of that same fixture
- same DHT/SOS/entropy bytes throughout) on the real wasm32 target under a real wasm runtime
(`wasm-bindgen-test-runner` + Node, the same mechanism this project's own CI already uses). There
is no second participant to fake here (CONFIRMATION.md section 11): a JPEG file is a
single external artifact handed to a decoder's public entry point, the same as any real caller
would supply.

## Class

crash (a reliable, deterministic panic/trap on malformed-but-spec-legal input, not memory-unsafety
- see "Root cause" for why the worst case stays a panic rather than an out-of-bounds write even in
a release build with overflow checks disabled).

## Root cause

`src/worker/immediate.rs`'s `ImmediateWorker::start_immediate` (and `src/worker/rayon.rs`'s
equivalent `elements` computation) allocates each component's decode buffer with:

```rust
data.component.block_size.width as usize * data.component.block_size.height as usize
    * data.component.dct_scale * data.component.dct_scale
```

`block_size` is the component's block-grid size in 8x8 blocks (derived from the frame's declared
image dimensions - the JPEG spec allows up to 65535x65535 in the SOF marker, and this crate does
not itself impose a lower maximum); `dct_scale` defaults to `8` (meaning "no downscaling") for a
plain `decoder.decode()` call with no `Decoder::scale()` call. For `max_size.jpg` specifically:
`block_size` resolves to 8192x8192, so the product is `8192 * 8192 * 8 * 8 == 2^32` exactly - on a
64-bit host this is an unremarkable ~4.29 GB `usize` value (the subsequent allocation attempt then
fails for an unrelated reason - this file's actual entropy data runs out almost immediately, so
`decode()` returns a clean `Format`/Huffman-decode `Err` before ever touching most of that buffer);
on a 32-bit target (wasm32) the identical multiplication wraps a 32-bit `usize` to exactly `0`
(debug builds catch this as an "attempt to multiply with overflow" panic instead of wrapping,
since Rust's default debug profile has `overflow-checks = true`).

This computation happens as soon as `decode_scan` begins processing the frame's first component
row - i.e. essentially immediately after a valid `SOF` + `DHT` + `SOS` are parsed, using only the
frame header's declared dimensions, independent of how much real entropy-coded scan data actually
follows. No SIMD code, no `arch/wasm.rs`, and no `unsafe` block is involved anywhere in this bug -
it is a plain, safe-Rust arithmetic bug that happens to only be reachable via a 32-bit target,
which is exactly why this candidate (the first session to actually execute this crate dynamically
on such a target) is what surfaced it, where 13 prior native-only/miri-only bug-hunt sessions in
this workspace could not have.

**Release-mode nuance** (checked as part of this candidate, not a separate approach in the table
above since it's a direct follow-up on the same finding): with overflow checks off (a `--release`
wasm32 build), the multiplication silently wraps to `0` instead of panicking for `max_size.jpg`
specifically, `Vec::resize(0, ..)` succeeds trivially, and this particular fixture's real decode
then fails for the same unrelated reason noted above (no real entropy data) before any row is ever
written into that (empty) buffer - so this exact 186-byte fixture does not panic in `--release`.
This does **not** mean the underlying bug is release-mode-safe in general: `append_row_immediate`'s
own subsequent slice indexing into that buffer is always bounds-checked by Rust regardless of
build profile, so the worst outcome this bug class can produce - even for a hypothetical
larger/different malicious file with a wrapped-but-nonzero buffer size and enough real entropy data
to actually reach a row-write past that undersized buffer - is a bounds-check panic, not an
out-of-bounds write/memory corruption. Constructing such a file (real Huffman-coded MCU data
consistent with a huge declared image size) was not attempted this session; see "What's not done".

**Second overflow surface found while varying the input** (see below): a product that fits inside
a 32-bit `usize` (no wraparound at all) can still exceed `isize::MAX` (`Vec`'s actual allocation
ceiling on any target, 32-bit or 64-bit, since `RawVec` rejects any capacity above `isize::MAX`
bytes) - this manifests as a `Vec::resize`/`with_capacity` "capacity overflow" panic instead, on
the *same* underlying "attacker fully controls a `usize` product used as an allocation length with
no upper bound" flaw. This surface is not wasm32-specific in principle (a 64-bit target could in
theory also hit it with a large enough product - the current maximum possible product from a real
JPEG's dimensions/dct_scale never actually reaches `isize::MAX` on 64-bit, so it is not currently
reachable there in practice, but the missing bound is exactly the same one either way).

## Introduced by

3123fc57 "Add support for IDCT downscaling" (2019-11-06) introduced the `dct_scale * dct_scale`
factor into `src/worker/immediate.rs`'s buffer-size computation (before this commit, the
equivalent line only multiplied `block_size.width * block_size.height`, which cannot reach 2^32
for any legal JPEG dimensions - max ~67 million for 65535x65535 at 1x1 sampling - so the
`dct_scale` factor is what is actually needed to reach the overflow threshold). `src/worker/rayon.rs`
carries an independent copy of the identical unguarded expression, introduced later by afd3f036
"Restructure multithreading" (2022-02-25) when the rayon-based worker was split into its own file
(copied from the same pattern, not a new introduction of the underlying idea).

## The fix

Added a single bounds check in `src/parser.rs`'s `update_component_sizes` (called once per
component, both at initial `SOF` parsing and whenever `Decoder::scale()` recomputes sizes) right
after `block_size` is computed: reject the frame with a `Error::Format` before either worker ever
sees it, using the *same* widened `usize` chain those workers would otherwise compute, but checked
with `.checked_mul(...)` at each step and additionally bounded by `isize::MAX` (not just
`usize::MAX`/`checked_mul`'s own overflow) - covering both overflow surfaces above with one change.
This is the earliest common point both `ImmediateWorker` and the rayon worker's buffer-size
computations are downstream of, so it protects both call sites without touching either worker
directly or changing any public API/error-handling signature.

Verified:
- The original `max_size.jpg`: **before** - panics ("attempt to multiply with overflow",
  `src/worker/immediate.rs:34`); **after** - returns a clean `Err` (`Format("decoded component
  buffer size exceeds addressable memory")`), no panic, no trap.
- **Varied within the same bug class** (CONFIRMATION.md section 7), not just the one triggering
  file: (a) the same file with only its declared width patched from 65535 to 65529 (same
  DHT/SOS/entropy bytes) - same overflow-wraparound mechanism, same before/after result; (b) the
  same file with width patched to 65524 instead - a product that fits in a 32-bit `usize` (no
  wraparound) but exceeds `isize::MAX`, which panicked with a *different* message ("capacity
  overflow" from `Vec::resize`, not "attempt to multiply with overflow") under an earlier,
  narrower version of this fix that only checked `usize::MAX`/`checked_mul` - the `isize::MAX`
  bound in the final fix was added specifically because this second case exposed that gap, and
  both (a) and (b) are clean `Err`s after the final fix.
- Full `tests/wasm_crashtest.rs` (111 original fixtures + these 2 new ones = 113): 113/113 pass
  after the fix (was 110/111 before, only `max_size.jpg` failing among the originals).
- Full `tests/wasm_simd_boundary.rs` (15 fixtures) and the existing `tests/lib.rs::included_file`
  wasm-bindgen-test: still 15/15 and 1/1 passing after the fix (unaffected, as expected - this fix
  only rejects frames that would have overflowed anyway).
- Project test suite, native x86_64 `+1.87.0` (this bug and its fix are both invisible to native
  64-bit `usize`, so this is a regression check, not a repro of the bug itself): identical before
  and after the fix in both feature configurations - `cargo test --tests` (default features,
  rayon): 21 passed, 0 failed both before and after (5 lib unit + 12 `tests/lib.rs` + 4
  `rayon-*`); `cargo test --tests --no-default-features`: 17 passed, 0 failed both before and
  after (5 lib unit + 12 `tests/lib.rs`).

## What's not done / open questions

- The release-mode nuance above (silent wraparound to a small-but-nonzero buffer size, with real
  enough entropy data to reach a row-write past it) was reasoned through but not built as an actual
  repro this session - Rust's mandatory slice-bounds-checking means the worst case is still "just"
  a panic (confirmed by code reading, not by constructing and running such a file), so this was not
  treated as urgent enough to spend further time on within this candidate's scope, but a future
  session wanting to be fully sure could construct one (needs real Huffman-coded MCU data
  consistent with a declared 8192+-block-wide image, i.e. actual encoder work, not just a patched
  header).
- Only `wasm32-unknown-unknown` was actually verified dynamically this session (this candidate's
  scope, per its name). The same class of bug (a `usize`-based, attacker-controlled buffer-size
  multiplication with no upper bound) would apply identically to any other 32-bit Rust target
  (`i686-unknown-linux-gnu`, `armv7-unknown-linux-gnueabihf`, etc.) - not verified here, but the fix
  is target-independent (it uses `usize`/`isize` directly, not `#[cfg(target_arch = "wasm32")]`),
  so it protects those targets too without further work.
- `src/arch/wasm.rs`'s own SIMD kernels (this candidate's originally-stated target) showed no
  issue: real dynamic execution (not just compile-checking) across every residue mod 8 from width
  1-33 plus 3 grayscale fixtures, and across the full 111-fixture malformed-input corpus, found no
  `assert!` violation, no incorrect pixel output beyond the project's own accepted rounding
  tolerance, and no panic attributable to the SIMD code itself (the one panic found is in
  `src/worker/immediate.rs`/`rayon.rs`, upstream of and unrelated to `arch/wasm.rs`). This
  corroborates candidate 013's manual invariant proof for wasm32 with real dynamic evidence, the
  same way candidate 016 did for aarch64/NEON.
- This session's `.github/workflows/rust.yml` reading found that CI's own `cross_compile_wasm` job
  never sets `RUSTFLAGS="-C target-feature=+simd128"` - meaning `cfg(target_feature = "simd128")`
  is false during that CI run, so `arch::get_dequantize_and_idct_block_8x8()`/
  `get_color_convert_line_ycbcr()` return `None` on wasm32 in CI today and the scalar fallback runs
  instead. CI's existing wasm-bindgen-test therefore provides no coverage at all of
  `arch/wasm.rs`, independent of anything found in this candidate - worth flagging to the project
  maintainers as a CI gap, separate from (and not required to fix) the bug this candidate is
  reporting.
- The temporary test files this candidate added (`tests/wasm_simd_boundary.rs`,
  `tests/wasm_crashtest.rs`) were investigation scaffolding for this session, not carried into
  `src/`/committed anywhere - the worktree they lived in was discarded per "the one rule that
  matters most". A future session wiring up permanent wasm dynamic-test coverage in CI could reuse
  the same approach (see `projects/jpeg-decoder.md` for the exact `wasm-bindgen-cli`
  download/`CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER` mechanics that made this work locally).
