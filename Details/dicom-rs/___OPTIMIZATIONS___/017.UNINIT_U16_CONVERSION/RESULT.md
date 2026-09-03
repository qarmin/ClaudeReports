# bytes_to_vec_u16 / signed_buffer: skip zero-init before NativeEndian::read_*_into (3 sites)

Another instance of the same anti-pattern as candidates 009/014/015, found by grepping this
session's discovery target list (`vec![0; n]` / `resize_with(..., Default::default)`) across the
whole workspace for any remaining unaddressed sites. `bytes_to_vec_u16` (`pixeldata/src/lib.rs`,
used at 6 call sites for converting raw bytes into 16-bit pixel samples) and two separate
`signed_buffer` sites (16-bit signed pixel conversion in `frame_data`/`convert_pixel_slice`) all
did `vec![0; n]` immediately followed by `NativeEndian::read_{u16,i16}_into(...)`, which writes
every element - the zero-fill is redundant.

## What was tried

Added a generic `uninit_vec<T: Copy>(len: usize) -> Vec<T>` helper (`Vec::with_capacity` +
`set_len` without writing) and used it at all 3 sites (`bytes_to_vec_u16`, both `signed_buffer`
sites). Same soundness argument as candidates 009/014/015: `T: Copy` (`u16`/`i16` here) has no
`Drop` glue and every bit pattern is valid, and every call site immediately fully overwrites the
buffer via a `NativeEndian::read_*_into` call that writes every element in order.

**Miri verification was attempted but inconclusive for this candidate specifically** (unlike
009/014/015) - `cargo +nightly miri test -p dicom-pixeldata --lib` hit unrelated failures: with
default features, one test spawns real OS threads via rayon, which Miri's isolation doesn't support
well; with `--no-default-features --features image,native`, a large fraction of tests failed for
reasons unconnected to this patch (most look like network-fetch or other environment issues under
Miri's isolation, not memory-safety violations - the failure list included many tests this patch's
code doesn't even touch, e.g. JPEG transcoding tests). Did not spend further bounded time
disentangling Miri-environment issues from this crate's own test suite from what a clean Miri run
would show for this specific unsafe code. The standard (non-Miri) `cargo test --workspace` run
(537/0/6, matching baseline) is the correctness evidence for this candidate.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `tovec`/`colorspace`/`decode-raw` modes
against `WG04/RLE/CT1_RLE` (512x512, 16-bit monochrome, RLE Lossless - `bytes_to_vec_u16` is called
in the monochrome-image-building path after RLE decode).

## Result

**No reliable change measured at the full-pipeline level.** `decode-raw` (doesn't call this code):
no consistent change, as expected (-0.3%, 4/9). `colorspace`: -1.7%, 5/9 - not consistent. `tovec`:
-1.3% (6/9, not consistent), then -2.3% (8/9, consistent) on a second round - ambiguous enough to
re-check at `RUNS=15` per this workspace's own established discipline for borderline results,
which came back **-1.6%, 9/15 - not consistent**, resolving the ambiguity as noise rather than a
real effect. Bit-exact in the one configuration checked (`dump-image` on `CT1_RLE.dcm`).

## Does it change behavior/output?

- **Bit-exact?** Yes (checked on `CT1_RLE.dcm`'s `dump-image` output).
- **Is this a breaking change?** No public API/type changes - `uninit_vec` is a private function.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. Miri verification was
  attempted but inconclusive for this crate/candidate (see "What was tried").

## Why the result looks the way it does

Same underlying mechanism as candidates 009/014 (removing a redundant memset before a full
overwrite), but the removed work here (a 512KB `u16` buffer, once per `to_vec_frame`/monochrome-
image-build call) is small relative to everything else `tovec`/`colorspace` do on this fixture
(RLE decode, LUT construction with its own rayon dispatch, the final numeric conversion pass) -
consistent with candidate 015's finding that this general technique's measured benefit scales with
both the buffer size *and* how much of the total call's cost the removed memset represents, not
just the buffer size alone.

## What's not done / open questions

- Not tried: an isolated microbenchmark calling `bytes_to_vec_u16` directly and repeatedly (bypassing
  the full RLE-decode-then-LUT pipeline), which would likely show a clear win matching candidates
  009/014's isolated-microbenchmark pattern, the same way candidate 005's isolated text-codec
  microbenchmark showed a clear win that didn't always surface at the full-pipeline level. Not
  built this candidate, given the full-pipeline result was the more directly useful question and
  time was better spent moving to the next lead.
- Miri verification remains genuinely incomplete for this specific patch (see "What was tried") -
  a future session with more patience for disentangling this crate's Miri-environment issues could
  attempt a narrower Miri run (e.g. `--test <specific single test>` for a test that exercises
  `bytes_to_vec_u16` without also spawning rayon threads or touching the network) for a cleaner
  signal than either the full-suite Miri run or none at all.
- Kept in the patch (not reverted) despite the null result, per this workspace's own "record null
  results with the same care as wins" convention - see candidate 015's identical reasoning.
