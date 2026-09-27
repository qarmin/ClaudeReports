# `hayro-syntax::math::round_f32`: reapplying candidate 010's `sse4.1`/`neon` gate to the sibling copy

`hayro-syntax/src/math.rs::round_f32` has the same unconditional-`x.round()`-on-`std` pattern
candidate `010` fixed in `hayro-jpeg2000`'s copy. This candidate applies the identical patch here
(gate the `std` fast path on `sse4.1`/`neon` actually being present, otherwise use the existing
manual branch-based fallback). Unlike `010`, the fix's only real call site
(`hayro-syntax/src/filter/jpx.rs::scale()`, used for bit-depth rescaling of palette/JPX image
data when the decoded image isn't 8bpc) is **never reached by any PDF in `hayro-tests`' 360-file
corpus** - confirmed by adding a temporary `eprintln!` at `scale()`'s call site and running
`hayro-bench`'s `render_bench` over the entire `hayro-tests/pdfs` tree (0 hits) as well as over
the 5 PDFs that do carry `JPXDecode` streams (`issue391.pdf`, `stream_jpx_1/3/5/6.pdf`; also 0
hits - none of these images have bit depth != 8). No real end-to-end bench exists for this call
site in this repo, so measurement here uses a temporary isolated `#[cfg(test)]` microbenchmark
(added to `hayro-syntax/src/math.rs`, reverted before finishing, not part of `patch.diff` - same
convention `010` used) that mirrors `scale()`'s per-byte loop shape: `round_f32((byte as f32 /
255.0) * mul_factor) as u32` over a 1241x1752x1 buffer (matching `issue391.pdf`'s largest embedded
JPX image, for a size that's at least grounded in a real dimension), 4-bit palette depth.

## What was tried

`cargo test -p hayro-syntax --release --features std,images --lib round_f32_scale_bench --
--ignored --nocapture --test-threads=1`, `taskset -c 2` for single-core, unpinned for
multi-core, `uptime` load 2.2-5.0 throughout (below the 24-core count). Each baseline/patched
variant measured twice per mode, per MEASUREMENT.md.

| Variant | single-core p50 (2 rounds) | multi-core p50 (2 rounds) | notes |
|---|---|---|---|
| baseline (`x.round()` unconditional) | 3.923ms, 3.923ms | 3.923ms, 3.987ms | |
| patched (manual fallback when no `sse4.1`/`neon`) | 4.617ms, 4.611ms | 4.593ms, 4.765ms | |

An earlier pair of rounds (higher system load, ~5.0) showed the same direction and similar
magnitude: baseline 3.998ms/3.990ms (pinned, after a noisy first warmup round discarded),
4.030ms/4.051ms (unpinned); patched 4.791ms/4.795ms (pinned), 4.797ms/4.796ms (unpinned) -
consistent across two independent measurement sessions.

## Result

**Regression, not a win**: patched is ~**+18% single-core p50** and ~**+17% multi-core p50**
slower than baseline for this call site's usage shape - the opposite direction from candidate
`010`'s -3.2%/-2.2% win on the `hayro-jpeg2000` copy of the same fix. Every patched round was
slower than every baseline round across both measurement sessions (4 vs. 4, and 4 vs. 4) - the
direction is consistent, not noise-overlapping.

## Does it change behavior/output?

- **Bit-exact?** yes. `ACC` (a `wrapping_add` checksum over all 2,174,232 `round_f32` outputs in
  the microbenchmark) is identical between baseline and patched builds: `16306575` in every run.
- **Is this a breaking change?** no - `round_f32` is `pub(crate)`, not part of any public API.
- Project test suite: `cargo test -p hayro-syntax -p hayro-tests --release --test tests --
  stream_jpx` - **1 passed, 3 failed both before and after this patch, identical pixel-diff
  counts** (`stream_jpx_3`: 65974, `stream_jpx_5`: 91173, `stream_jpx_6`: 29580) - the same
  pre-existing, unrelated failures `010` already documented (none of these 3 PDFs' JPX images
  are non-8bpc, so `scale()`/`round_f32` isn't on their failure path either).
- Additionally noted, not a regression from this patch: applying it makes the pre-existing
  `floor_f32` free function in this file dead code under `--no-default-features --features
  images` (a `no_std` build) - `cargo build -p hayro-syntax --release --no-default-features
  --features images` emits `warning: function 'floor_f32' is never used` with the patch applied
  (no warning without it). Harmless (a warning, not an error; the crate's default features
  include `std`, so this doesn't affect the default build), but worth cleaning up if this patch
  is ever taken for real - either have the new manual-fallback branch call the existing
  `floor_f32` instead of defining its own `floor_manual`, or drop `floor_f32` and adjust its
  callers. `010`'s own patch avoids this because `hayro-jpeg2000`'s `floor_f32` is `pub(crate)`
  and used unconditionally by other functions in that crate, not gated to only the `no_std`
  branch of `round_f32`.

## Why the result looks the way it does

Disassembling the compiled microbenchmark (`objdump -d --demangle`) shows why the direction
flips versus `010`: **neither variant vectorizes** in this loop shape (both process one `u8` at
a time, matching the real `scale()` call site, which also can't vectorize - each element feeds a
stateful `BitWriter::write()` call for bit-packing, an inherently serial operation). Without
vectorization to amortize per-element cost:

- Baseline's `x.round()` compiles to a single indirect `call` per element (confirmed in the
  disassembly: `call *%rbp` right after the `divss`/`mulss` setting up the argument) - this is a
  real function-call (presumably resolving to glibc's `roundf`), but on this specific machine
  (AMD Ryzen 9 7900, has `sse4.1` at actual runtime even though the binary is compiled without
  `target-feature=sse4.1`), glibc's `roundf` itself very plausibly dispatches to a hardware
  `ROUNDSS`-based implementation via its own runtime IFUNC CPU-feature resolution, independent of
  what the *calling* binary was compiled with - candidate `010`'s premise ("no `sse4.1` means
  this is always a slow libm call") describes the compiled code's own capability, not
  necessarily what the linked libm implementation does internally at runtime.
- Patched's manual fallback compiles to fully inlined scalar SSE code (`cvttss2si` truncate +
  compare + `andps`/`andnps` blend for the two-case selection) - no call, but several more
  instructions per element than a single hardware round instruction would need, with no SIMD
  width to spread that cost over.

Candidate `010`'s real win came from a *different* property: in `hayro-jpeg2000`'s bulk
`interleave_and_convert` loop (a plain, densely-packed conversion with no per-element side
effects), the manual fallback is auto-vectorizable, letting LLVM pack multiple elements per
instruction and outrun the serial call-per-element baseline. `scale()`'s loop can't get that
benefit because of the interleaved `BitWriter` call - so the exact same source-level fix produces
opposite results depending on whether the surrounding loop lets the compiler vectorize the
manual fallback. This is a genuinely new finding, not just "unmeasured magnitude" as originally
scoped in `IDEAS.md`.

## What's not done / open questions

- Not confirmed via `nm`/`readelf` which exact libm symbol/implementation the baseline `call`
  resolves to at runtime (would need to step into `roundf`'s IFUNC resolver, e.g. via `gdb` or
  `LD_DEBUG=bindings`) - the explanation above is a plausible, disassembly-grounded hypothesis,
  not a fully proven one.
- Since this call site is unreached by the entire existing PDF corpus, this result only says what
  happens *if* a JPX+non-8bpc PDF is ever exercised through it - it has no measurable effect on
  any currently-passing benchmark or test today either way.
- A PDF that actually exercises `hayro-syntax::filter::jpx::scale()` (a `JPXDecode` image with an
  `Indexed` color space and a non-8-bit-per-component `Indexed` base, or a raw JPX image whose
  intrinsic bit depth isn't 8) would be needed to confirm this holds end-to-end, not just in the
  isolated microbenchmark - not constructed this session (no existing corpus PDF qualifies, and
  building a synthetic JP2 codestream from scratch was out of scope for this candidate).
