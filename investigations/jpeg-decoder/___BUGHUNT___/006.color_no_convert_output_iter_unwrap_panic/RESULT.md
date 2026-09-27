# color_no_convert_output_iter_unwrap_panic

`color_no_convert` in `src/decoder.rs` (used for `ColorTransform::None`, the "return raw
per-component samples, don't colour-convert" caller-opt-in mode) writes every element of each
component's upsampled line-scratch buffer into the output slice via `output_iter.next().unwrap()`,
with no bound tying the write count to the output slice's actual length. Flagged by GitHub issue
#289 (filed after 0.3.2, still open); the exact line/code (`src/decoder.rs:1481`) is unchanged at
current HEAD (eb2d7c0). Investigated per the issue's own suggestion to check whether the sibling
`color_convert_line_*` functions share the same unchecked-iterator pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real public API | Release build, real `Decoder::decode()` on a standard 4:2:0 baseline JPEG (`tests/reftest/images/extraneous-data.jpg`, 16x16, 3 components, sampling 2x2/1x1/1x1) with `set_color_transform(ColorTransform::None)` | Panicked at `src/decoder.rs:1481:34`, `Option::unwrap()` on `None`, reproduced 3/3 runs (default `rayon` feature) |
| Same, `--no-default-features` (single-threaded "immediate" worker, no rayon) | Release build | Panicked at the same line, 3/3 runs - rules out a rayon-specific/thread-timing artifact; the bug is in `color_no_convert` itself |
| Code reading of the 4 sibling `color_convert_line_{rgb,ycbcr,ycck,cmyk}` functions | n/a | All 4 use `output.chunks_exact_mut(N).zip(component.iter())...`, which is implicitly bounded to `min(output.len()/N, component.len())` - i.e. bounded by `output`'s own length. Only `color_no_convert` walks each component's buffer to its full, unbounded length. No analogous unchecked-iterator bug found in the sibling functions. |

Did not need a 3rd tool/approach - the second run (no rayon) already gave a clean, deterministic
signal ruling out a flaky/thread-dependent cause, and the code-reading pass across the sibling
functions was cheap and conclusive.

## Status

**Confirmed.** Reproduces 3/3 runs under the default (`rayon`) feature set, and 3/3 runs under
`--no-default-features` (single-threaded), release build, no fixed seed needed - deterministic
given the same input file and API call.

**Fully reproducible**: **Yes**. The repro drives the crate's real public API
(`jpeg_decoder::Decoder::new` / `set_color_transform` / `decode()`) end-to-end on a real on-disk
JPEG file. Per `CONFIRMATION.md` section 11, a malformed/adversarial file handed to a decoder's
public entry point is itself the genuine external input - there is no second participant to fake
here (single-process/single-participant shape is the correct one for this class of finding).

## Class

crash

## Root cause

`Upsampler::new` (`src/upsampler.rs`) allocates each component's line-scratch buffer with a shared
capacity, `line_buffer_size = max(component.size.width) * h_max` - deliberately oversized so it's
a safe upper bound for every `Upsample` impl (in particular `UpsamplerGeneric`'s horizontal
replication), not the number of *valid* (actually-upsampled) samples per row. Every
`color_convert_line_*` function correctly treats only the first `output_width` samples of each
component buffer as meaningful, because they zip against `output.chunks_exact_mut(component_count)`,
which has exactly `output_width` chunks - the zip silently stops there regardless of how much
larger the per-component buffer is.

`color_no_convert` (`src/decoder.rs:1476-1483`) does not do this. It loops
`for pixel in data { for d in pixel { *(output_iter.next().unwrap()) = *d; } }`, walking each
component's *entire* buffer (`line_buffer_size` elements, not `output_width`), and unwrapping
`output_iter.next()` unconditionally. Once any component's sampling factor is below `h_max`/`v_max`
(i.e. any chroma subsampling is present - not a special/malformed case, this is standard 4:2:0/4:2:2
JPEG), `line_buffer_size` exceeds `output_width`, so the total number of writes
(`component_count * line_buffer_size`) exceeds `output.len()` (`component_count * output_width`),
and `output_iter` is exhausted mid-loop, panicking instead of returning a decode error.

Reaching `color_no_convert` at all requires the caller to opt in via
`Decoder::set_color_transform(ColorTransform::None)` - `determine_color_transform()`'s own
marker-driven logic never selects `ColorTransform::None` on its own from file contents. This is a
legitimate, documented, supported caller configuration (`ColorTransform::None`'s doc comment: "No
transform should be applied and the data is returned as-is") for callers who want raw
per-component planes; it is not a fuzzer-only or purely-adversarial code path. Once configured,
*any* baseline JPEG with ordinary chroma subsampling triggers the panic - the file used in the
repro is an unmodified copy of this project's own `extraneous-data.jpg` test fixture, not a
specially malformed input.

No analogous unchecked-iterator pattern was found in `color_convert_line_rgb`,
`color_convert_line_ycbcr`, `color_convert_line_ycck`, or `color_convert_line_cmyk` - all four are
correctly bounded via `zip`.

## Introduced by

21a7a3b "Now performing no colour conversion when no `color_transform` provided" (2022-07-07) -
this commit added `color_no_convert` (and the `ColorTransform::None` dispatch arms that call it)
verbatim with the unbounded `output_iter.next().unwrap()` loop. The oversized
`line_buffer_size = max(component.size.width) * h_max` scratch-buffer allocation it silently
depends on predates it (introduced by a8f7a12 "Adds UpsamplerGeneric") and was harmless until
21a7a3b added the one consumer that doesn't bound its own reads to `output_width`.

## The fix

Bound `color_no_convert`'s per-component write count to `output.len() / data.len()` (the same
"samples per component actually represented in `output`" quantity every sibling function derives
implicitly from `output.chunks_exact_mut(component_count)`), via `.iter().take(width)`, and replace
the unconditional `.unwrap()` with a `match` that returns early instead of panicking if `output_iter`
is ever still exhausted early for some other reason (defense in depth; not expected to trigger given
the `width` bound, but keeps this function panic-free the same way the other four already are).

```rust
fn color_no_convert(data: &[Vec<u8>], output: &mut [u8]) {
    if data.is_empty() {
        return;
    }

    let width = output.len() / data.len();
    let mut output_iter = output.iter_mut();

    for component in data {
        for d in component.iter().take(width) {
            match output_iter.next() {
                Some(o) => *o = *d,
                None => return,
            }
        }
    }
}
```

Verified against `repro/`:
- Before the patch: panics 3/3 runs (both with and without the `rayon` feature) - see
  `output.txt`.
- After the patch: `decode()` completes normally 3/3 runs, returning exactly 768 bytes
  (`16 * 16 * 3` = `output_width * output_height * component_count`) for the same input - i.e. the
  fix writes the full, correctly-sized output rather than silently truncating it.
- Also spot-checked with `--no-default-features` before/after, matching the with-`rayon` result.

- Project test suite (`cargo test --release`, default features): 23 passed, 0 failed both before
  and after the patch (5 unit tests in `src/`, 12 in `tests/lib.rs`, 1 each in `tests/rayon.rs` /
  `rayon-0.rs` / `rayon-1.rs` / `rayon-2.rs`, 2 doc-tests). No new failures, no regressions.

## What's not done / open questions

- The fix was checked against one input (16x16, 4:2:0, 3-component) plus the existing full test
  suite, not against a broader sweep of sampling-factor combinations (4:2:2, 4:4:0, non-integer
  ratios via `UpsamplerGeneric`, 4-component CMYK/`ColorTransform::None`). The root-cause reasoning
  (bounding to `output.len() / data.len()`, matching what the sibling functions already do via
  `zip`) generalizes to all of these, but only the one 4:2:0/3-component case was actually run.
- Not investigated: whether `color_no_convert`'s output layout itself (component-major - i.e. all
  of component 0's row, then all of component 1's row, etc. - rather than per-pixel interleaved
  like the other four `color_convert_line_*` functions produce) is the intended/documented
  contract for `ColorTransform::None`, or a separate latent surprise for callers expecting
  interleaved output. Out of scope for this crash-focused candidate; noted here only in case a
  maintainer wants to look at it separately.
- A short one-off fuzz harness targeting `Decoder::decode` with `ColorTransform::None` forced was
  not built this session (the manual boundary-value construction already gave a clean, immediate,
  deterministic repro on the first standard subsampled test fixture tried, so escalating further
  wasn't needed per the soft 3-approach cap).
