# A synthetic JPXDecode PDF that actually reaches `hayro-syntax::filter::jpx::scale()`/`round_f32`

Candidate `015` measured a regression from gating `hayro-syntax::math::round_f32`'s `std` fast
path on `sse4.1`/`neon` (the same fix candidate `010` won with on a different call site), but
could only confirm it via an isolated microbenchmark - the real call site
(`hayro-syntax/src/filter/jpx.rs::scale()`, used to rescale JPXDecode image data when the bit
depth isn't 8) is never reached by any of `hayro-tests`' 360 corpus PDFs (confirmed there via
instrumentation, 0 hits). This candidate builds that missing PDF: `testdata/jpx_scale_gray4.pdf`,
a hand-built minimal PDF with a `/Filter /JPXDecode` image XObject wrapping a **real, correctly
encoded 16x16 grayscale JP2 codestream whose own SIZ marker declares 4-bit intrinsic precision**
(built with ImageMagick/OpenJPEG: `magick -size 16x16 gradient:black-white -colorspace Gray -type
Grayscale -depth 4 grad4g2.jp2` - see `testdata/make_pdf.py`). The PDF image dict deliberately
omits both `/BitsPerComponent` and `/ColorSpace` (both are optional for JPXDecode-filtered images
per spec and fall back to the JPX codestream's own header), so `hayro-syntax`'s
`bpc = params.bpc.unwrap_or(image.original_bit_depth())` resolves to `4`, which is `!= 8` and
drives `jpx::decode()` into `scale()` - confirmed directly with a temporary `eprintln!` at
`scale()`'s entry (reverted before finishing, not part of `patch.diff`):
`HIT scale(): bpc=4 num_components=1 width=16 height=16 data_len=256`, hit exactly once per
render of this PDF through `hayro`'s real `render()`/`hayro-bench render_bench` pipeline.

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## What was tried

The change under test is the exact same patch as candidate `015` (`hayro-syntax/src/math.rs`'s
`round_f32`, gate the `std` `x.round()` fast path on `sse4.1`/`neon` actually being present,
`patch.diff` here is a byte-identical copy of `015`'s). What's new in this candidate is the bench:
the real project `hayro-bench render_bench` binary (`--backend hayro`, reused as-is, no new
harness code) driven against `testdata/` (containing only `jpx_scale_gray4.pdf`), and the real
`hayro`-crate `examples/render.rs` for a full end-to-end pixel/PNG comparison.

| Variant | single-core p50 (2 rounds) | multi-core p50 (2 rounds) | notes |
|---|---|---|---|
| baseline (`x.round()` unconditional) | 0.098ms, 0.081ms | 0.083ms, 0.119ms | `render_bench --iter 50000`, `taskset -c 2` for single-core |
| patched (manual fallback when no `sse4.1`/`neon`) | 0.087ms, 0.092ms | 0.085ms, 0.083ms | same |

`uptime` load 15.0/12.4/9.2 throughout (below the 24-core count).

## Result

**No measurable difference at the full-page-render scale** - both variants' 4 rounds (2
single-core, 2 multi-core) overlap in the 0.081-0.119ms band with no consistent ordering; the
spread between rounds of the *same* variant (baseline: 0.081-0.119ms, a ~47% range) is larger
than the gap between baseline's and patched's medians. This is expected, not surprising: this
synthetic PDF's whole page-render time (~85-120us, parse + JP2 decode + `scale()` + rasterize +
PNG encode) is dominated by fixed per-page overhead (PDF object parsing, JP2 header/tile-part
parsing, `vello_cpu` scene setup) - `scale()`'s own working set here is only 256 bytes (16x16x1),
nowhere near enough to surface the ~18% per-element regression candidate `015`'s isolated
microbenchmark measured (which used a much larger 1241x1752 buffer specifically to make the
per-element cost visible above noise). This candidate's contribution isn't a new timing number -
it's that a real, end-to-end wall-clock/correctness comparison through the actual PDF rendering
pipeline is now possible at all for this call site, where before only the isolated microbenchmark
existed.

## Does it change behavior/output?

- **Bit-exact?** yes. `sha256sum` of the rendered PNG (`hayro/examples/render.rs` against
  `testdata/jpx_scale_gray4.pdf`) is identical between baseline and patched release builds:
  `4ad21bb337ba6ece260357b24bdd7da9b7ed3ddba019830e2eba547d20add481` in both cases (also checked
  into `testdata/rendered_expected.png` for future reference). The rendered image is a clean
  16-level gray gradient (`magick ... -define histogram:unique-colors=true` on the output PNG
  shows exactly 16 unique gray values, each an exact multiple of 17 = 255/15, i.e. `scale()`
  round-trips the original 4-bit nibbles exactly through 8-bit decode -> `round_f32`-based
  rescale-down -> `BitWriter` packing -> the interpreter's `default_decode_arr`-based rescale
  back up to 8-bit for display) - this by itself is a meaningful correctness confirmation of
  `scale()`/`round_f32` that no existing corpus PDF could previously provide.
- **Is this a breaking change?** no - `round_f32` is `pub(crate)`, not part of any public API
  (same as `015`'s finding).
- Project test suite:
  - `cargo test -p hayro-syntax -p hayro-tests --release --lib` (hayro-syntax's own unit tests):
    **199 passed, 2 failed, identical before and after** - the 2 failures
    (`pdf::tests::pdf_version_header`, `pdf::tests::pdf_version_catalog`) are pre-existing and
    unrelated: they `unwrap()` a `std::fs::read` of `hayro-tests/downloads/...`, a directory that
    doesn't exist in this checkout (confirmed absent in the main project tree too, not something
    this worktree broke).
  - `cargo test -p hayro-tests --release --test tests` (full snapshot/pixel-diff suite): **430
    passed, 1277 failed, identical before and after** (confirmed with both default parallelism
    and `--test-threads=1`; an initial single default-parallelism run showed a spurious 117
    passed/1590 failed that did not reproduce on rerun in either variant - flaky under the
    default parallel test runner in this sandbox, not caused by this patch). The 1277 failures
    are pre-existing and environment-caused (not new, not JPX-related - present identically on
    both baseline and patched, and this large a failure count exists on a totally unrelated
    single-line change to a `pub(crate)` rounding helper, which has no plausible causal path to
    e.g. `write::*` snapshot tests).
  - `cargo test -p hayro-tests --release --test tests -- jpx` (the 4 JPX-specific snapshot
    tests): **4 passed, 0 failed, identical before and after**, both variants. Note: candidate
    `015` recorded these same 4 tests as `1 passed, 3 failed` at its own `base_commit`; at this
    candidate's `base_commit` (`5a5f0e24`, later than `015`'s) all 4 now pass on **both**
    baseline and patched - independent of this candidate's patch (not investigated further here,
    noted as a possible `015` `CORRECTION.md` candidate for a future session to confirm which
    upstream commit fixed it).

## Why the result looks the way it does

See "Result" above - `scale()` is real but small work (256 bytes for this test image) inside a
page-render pipeline whose fixed costs (parsing, JP2 header/tile decode, rasterization) are two
to three orders of magnitude larger for a 16x16 test image. A synthetic PDF with a much larger
non-8bpc JPX image (e.g. matching `issue391.pdf`'s ~1241x1752 real image, per `015`'s own
microbenchmark sizing) would let `scale()`'s share of total page time grow enough to potentially
surface above noise in a full end-to-end `render_bench` run - not attempted here since building a
correctly-encoded large non-8bpc JP2 test asset was out of scope for confirming the harness itself
works; the isolated microbenchmark already established the per-element magnitude precisely, and
`015`'s own explanation for *why* (auto-vectorization availability differs by call site) still
applies unchanged.

## What's not done / open questions

- No corresponding **Indexed color space** variant was built (the candidate description's other
  suggested route) - the intrinsically-non-8bpc-JPX route above was simpler to get right (a real
  encoder-produced codestream, not a hand-assembled palette + JPX index stream) and already
  reaches the exact same `scale()` call with the same parameters an Indexed-colorspace PDF would
  (`is_indexed` only affects a separate, unrelated palette-resolution step in `hayro-jpeg2000`,
  not which branch of `jpx::decode()` calls `scale()`) - so it wasn't judged necessary for
  coverage purposes, though it would still be a legitimate PDF-realism improvement for a future
  session (Indexed + non-default BPC is a more common pattern in real-world producer output than
  a raw intrinsically-low-precision JPX image).
- A larger (e.g. `issue391.pdf`-sized) synthetic non-8bpc JPX asset, to let `scale()`'s cost grow
  large enough to surface end-to-end above this harness's own noise floor - not built this
  session, see "Why the result looks the way it does".
- The `015`-vs-this-candidate JPX-test pass-count discrepancy (`1/4` vs `4/4`, see "Does it
  change behavior/output?") wasn't bisected to find which commit between `015`'s `base_commit`
  and this candidate's fixed it.
- `testdata/jpx_scale_gray4.pdf` and its source `grad4g2.jp2` are small (939 bytes / 238 bytes)
  and self-contained - good candidates for adding to `hayro-tests`' own corpus for real (as a
  regression test for this call site) if a maintainer wants permanent coverage; not done here
  since that's a change to the tested project's own test corpus, not a performance patch, and out
  of this skill's scope to commit.
