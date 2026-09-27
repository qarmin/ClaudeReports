# Auditing the `round_f32`/`floor_f32`-style call sites beyond candidates 010/015/030: `hayro-jpeg2000::math::floor_f32`

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`, `x86_64-unknown-linux-gnu`
(confirmed via `rustc --print cfg`: only `sse`/`sse2`/`fxsr` target features, no `sse4.1`/`avx`).

This candidate is an audit of the whole "`std` `f32`/`f64` method vs. a manual libm-free
fallback, gated (or not) on `feature = "std"`" family in both crates' `math.rs` files, per
candidate 015's own "what's not done" note. There are only two `math.rs` files in the repo
(`hayro-jpeg2000/src/math.rs`, `hayro-syntax/src/math.rs`); every function in that shape in both
was enumerated and checked:

| Function | File | `std` path | Gated on `sse4.1`/`neon` already? | Direct call sites (excluding internal use by `round_f32`) |
|---|---|---|---|---|
| `round_f32` | `hayro-jpeg2000/src/math.rs` | `x.round()` | No (candidate 010's finding/fix, not yet applied to the tree) | `lib.rs`'s `interleave_and_convert` (candidate 010's own target) |
| `floor_f32` | `hayro-jpeg2000/src/math.rs` | `x.floor()` | No - **new, this candidate's target** | `j2c/idwt.rs`: `reversible_filter_53r` (horizontal, scalar) and `reversible_filter_53r_simd` (vertical, SIMD-with-scalar-remainder) |
| `min_f32`/`max_f32` | `hayro-jpeg2000/src/math.rs` | `a.min(b)`/`a.max(b)` | No, but **already optimal** - see "Why the result looks the way it does" | none outside `math.rs` itself |
| `mul_add` | `hayro-jpeg2000/src/math.rs` | `f32::mul_add` | Yes, already gated on `fma`/`neon` (the pattern candidates 010/015 both copied) | many, in `idwt.rs` |
| `round_f32` | `hayro-syntax/src/math.rs` | `x.round()` | No (candidate 015's finding, regression; candidate 030 confirmed unreached by the corpus) | `filter/jpx.rs::scale()` (candidate 015's own target) |
| `floor_f32` | `hayro-syntax/src/math.rs` | n/a - only exists under `not(feature = "std")` | n/a, no `std`-path call site exists | none (compiled out entirely when `std` is on) |
| `trunc_f64` | `hayro-syntax/src/math.rs` | `x.trunc()` | No - **new lead, not chased this candidate** | `object/number.rs:49`, one call per parsed PDF number literal (not a bulk per-element loop) |
| `powi_f64` | `hayro-syntax/src/math.rs` | `x.powi(n)` | No - **not chased**, `powi`'s std path isn't a single-hardware-instruction-vs-libm-call story the way round/floor/trunc are (LLVM expands small integer powers via repeated squaring either way) | `object/number.rs:221`, same call frequency as `trunc_f64` above |

Only `floor_f32` (`hayro-jpeg2000`) turned out to be a real, previously-untested, positive-result
candidate; the reasoning for skipping the others is in "Why the result looks the way it does"
below. The tested fix is the exact same `sse4.1`/`neon` gate pattern candidates 010/015 used,
applied to `floor_f32` instead:

```rust
#[cfg(all(feature = "std", any(all(any(x86, x86_64), target_feature = "sse4.1"), all(aarch64, neon))))]
{ x.floor() }
#[cfg(not(...))]
{ let xi = x as i32; let xf = xi as f32; if x < xf { xf - 1.0 } else { xf } }
```

## What was tried

No existing project benchmark/example reaches `floor_f32`'s two direct call sites:
`reversible_filter_53r`/`reversible_filter_53r_simd` only run for `WaveletTransform::Reversible53`
(the lossless 5-3 wavelet), and **every PDF in `hayro-tests`' corpus that carries a JPX stream
uses `Irreversible97` instead** - confirmed by instrumenting both `reversible_filter_53r` and
`irreversible_filter_97i` with a temporary `eprintln!` and running `hayro-bench`'s
`jpx_decode_bench` over every PDF under `hayro-tests/pdfs` (`find ... | while read f; do
./jpx_decode_bench --pdf "$f" ...; done`): 0 hits for `reversible_filter_53r` across the whole
corpus, only `irreversible_filter_97i` hits (`issue391.pdf`: 3504+1752+... hits at widths up to
1024). This is the same "unreached call site" situation candidate 015 found for
`hayro-syntax::round_f32`'s `scale()`.

Built a temporary `#[cfg(test)] #[ignore]` microbenchmark (`inject_bench.py`, same convention as
candidate 015's own script) appended to `hayro-jpeg2000/src/j2c/idwt.rs` that calls the real,
private `filter_horizontal`/`filter_vertical` functions directly with
`WaveletTransform::Reversible53` on a synthetic 1024x64 `f32` buffer (1024 matches the largest
scanline width seen for `issue391.pdf`'s real `Irreversible97` images - no real-world
`Reversible53` image was available to size this against, since none exists in the corpus), 2000
iterations, checksumming every output element's bit pattern for a correctness check.
`taskset -c 2` for single-core, unpinned for multi-core, each baseline/patched pair measured
twice, per MEASUREMENT.md - all four measurement invocations run through the shared build lock.

| Variant | single-core p50 (2 rounds) | multi-core p50 (2 rounds) | notes |
|---|---|---|---|
| baseline (`x.floor()` unconditional) | 0.110499, 0.110225 ms/iter | 0.110723, 0.110558 ms/iter | |
| patched (manual fallback when no `sse4.1`/`neon`) | 0.080830, 0.080887 ms/iter | 0.080656, 0.084019 ms/iter | |

All four baseline rounds and all four patched rounds landed in tight, non-overlapping bands
(baseline: 0.1102-0.1107ms; patched: 0.0807-0.0840ms) - the direction is consistent, not
noise-overlapping, even under this session's noisy host (`uptime` load ~16-20 on a 24-core
machine throughout, below the "re-baseline" threshold but not quiet).

## Result

**Strong, consistent win: ~-27% single-core p50, ~-25% multi-core p50** for the real
`filter_horizontal`+`filter_vertical` pair on a synthetic `Reversible53` buffer sized to match
this repo's own largest real JPX scanline width. Unlike candidate 015's regression on the
sibling `hayro-syntax::round_f32` fix, this one wins for the reason spelled out in candidate
015's own "why" section: whether the manual fallback auto-vectorizes depends on whether the
surrounding loop has a per-element stateful side effect (015's `BitWriter::write()` blocks it;
this loop has none - see below).

## Does it change behavior/output?

- **Bit-exact?** yes. The buffer checksum (`wrapping_add` of every output `f32`'s `to_bits()`
  over all 2000 iterations x 65536 elements) is identical between baseline and patched builds in
  every run this session: `346744999608642000`.
- **Is this a breaking change?** no - `floor_f32` is `pub(crate)`, not part of any public API.
- Project test suite:
  - `cargo test -p hayro-jpeg2000 --release --lib`: **1 passed, 0 failed, 1 ignored**
    (`test_jpeg2000_standard_example_b4` passes; the ignored test is this candidate's own
    temporary bench), identical before and after.
  - `cargo test -p hayro-jpeg2000 -p hayro-tests --release --test tests -- jpx`: **4 passed, 0
    failed**, identical before and after (matches candidate 030's own result at this same
    `base_commit`). One transient first-run failure ("new reference image was created" panic
    for 2 of the 4 tests) occurred once before the patch was applied and did not reproduce on
    immediate rerun - this matches candidate 030's own documented flakiness under this sandbox's
    test runner, unrelated to this patch (not investigated further, same as 030 left it).

## Why the result looks the way it does

`objdump -d --demangle` on `filter_horizontal`'s compiled code confirms the mechanism directly:

- **Baseline**: 3 static `call *floorf@got` sites inside `filter_horizontal`, 79 scalar
  (`movss`) float moves, essentially no packed SIMD activity attributable to the `floor_f32`
  calls themselves (the function's own separate `Irreversible97`/`mul_add` code path, which
  never calls `floor_f32`, does show baseline-SSE `addps`/`mulps` - that's LLVM successfully
  auto-vectorizing the *other* transform's arithmetic, unrelated to this candidate).
- **Patched**: 0 `floorf` calls remain anywhere in `filter_horizontal`; 39 packed-SSE
  instructions appear instead (`addps`/`subps`/`mulps`/`cmpps`/`andps`/`andnps`/`blendvps`,
  matching the shape of a vectorized branchless floor: truncate, compare, blend). LLVM
  successfully auto-vectorizes the manual fallback here.

This is the opposite of candidate 015's outcome, and the reason is exactly the auto-vectorization
question this candidate was scoped to check: `filter_step_horizontal`'s loop
(`for i in (middle_start..width-1).step_by(2) { scanline[i] = f(scanline[i], scanline[i-1],
scanline[i+1]) }`) writes only same-parity elements and reads only opposite-parity elements - the
elements it reads are never written by any iteration of the same loop, so there is no loop-carried
dependency for LLVM to worry about, unlike a lifting-scheme loop that reads its own just-written
neighbor. Candidate 015's loop was blocked for a different reason (a per-element
`BitWriter::write()` side effect, inherently serial); this loop has no such side effect, so it's
free to vectorize once the libm call is removed - matching candidate 010's own `interleave_and_convert`
finding, not 015's.

`min_f32`/`max_f32` were checked and found **already optimal without any gate**: a small isolated
compile check (`a.min(b)` / `a.max(b)`, release, same default target) shows both already compile
to a single `minss`/`maxss` instruction plus a NaN-handling `cmpunordss`/blend sequence - `minss`/
`maxss` are baseline SSE1 instructions, not `sse4.1`-gated the way `roundss`/`floor`'s `roundss`
are, so there is no libm call to eliminate and no candidate here. The same check shows `a.trunc()`
does call `truncf` via PLT (same shape as `round`/`floor`) - a real, still-open lead, but its only
call site (`hayro-syntax`'s PDF number-literal parser) is a single call per parsed number token,
not a bulk per-element loop, so it's a much lower-priority target than `floor_f32`'s IDWT hot loop
and wasn't chased further this candidate (see IDEAS.md-worthy note below).

## What's not done / open questions

- **This call site is currently unreached by every PDF in `hayro-tests`' corpus** (confirmed via
  the same instrumentation approach candidate 015/030 used) - exactly like candidate 015's
  `scale()` finding before candidate 030 built a synthetic PDF for it. This candidate's number
  says what happens *if* a `Reversible53`-encoded JPX image is ever decoded through this path; it
  has no measurable effect on any currently-passing benchmark today. A synthetic
  `Reversible53`-encoded `.jp2` (candidate 030's `testdata/make_pdf.py`/ImageMagick approach could
  likely produce one with `-define jp2:rate=` or an explicit lossless flag) would let this be
  re-measured end-to-end through `render_bench`, the way candidate 030 did for `scale()` - not
  built this session.
- `trunc_f64` (`hayro-syntax/src/math.rs`) is a genuine, unaudited libm-call site (confirmed via
  disassembly: `x.trunc()` compiles to a `truncf` PLT call, same shape as `round`/`floor`) but is
  called once per parsed PDF number literal, not in a bulk loop - plausibly not worth a dedicated
  candidate, added to `IDEAS.md` for a future session to size properly (e.g. how many number
  tokens does a typical/pathological content stream parse) before deciding.
- `powi_f64` was not disassembled to confirm one way or the other whether it's currently a
  libm-style call or a fully inlined repeated-squaring loop - lower priority since even if it is
  a call, `powi`'s manual fallback (also repeated squaring) wouldn't obviously vectorize any
  better than what LLVM would inline for the `std` path in the first place.
- Not verified with `sse4.1` actually enabled (`-C target-feature=+sse4.1`/`target-cpu=native`)
  that the gate correctly falls back to the hardware `x.floor()` path and that this remains at
  least as fast there - same open item candidate 010 left for `round_f32`, inherited here.
