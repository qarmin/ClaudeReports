# PR #1353 - honor `ImageDecodeParams::target_dimension` for DCTDecode and other filters

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

Tests [PR #1353](https://github.com/LaurenzV/hayro/pull/1353) verbatim (`patch.diff` is the PR's
own diff, unmodified, `git apply --check`-clean against `base_commit`). The PR has two parts: (1)
`hayro-syntax`'s DCTDecode filter gets a true IDCT-scaled decode via `jpeg-decoder`'s
`Decoder::scale`, gated behind a new `jpeg-decoder` Cargo feature (default-on in
`hayro-syntax`'s own `[features] default = [...]`); (2) `hayro-interpret` gets an unconditional,
not-feature-gated post-decode integer box-filter downsample applied to every filter's decoded
output when `target_dimension` implies >=2x oversampling.

**Central finding: part (1) - the PR's own headline mechanism - is dead code in the real `hayro`
build graph as submitted.** See "Why the result looks the way it does" below - this is why the
measured timing result and the "what if wired correctly" secondary check diverge so sharply.

## What was tried

Synthetic PDF (`testdata/dct_downscale.pdf`, built by `testdata/make_dct_pdf.py`): a real,
baseline (non-progressive) 3000x2000 RGB JPEG (`ImageMagick`-generated plasma-noise photo, 830KB,
4:2:0 subsampling), embedded via `/Filter /DCTDecode`, drawn on a 300x200-pt page at exactly
300x200 device pixels - a 10x reduction on both axes, matching the PR's own motivating scenario
("PDFs commonly embed images 2-10x larger than their on-page footprint"). `hayro-bench
render_bench --backend hayro` (real project binary, parse+interpret+decode+rasterize, no
PNG/JPEG encode), `--iter 300`, 3 repeated rounds per variant; all heavy commands run serialized
under this workspace's shared `build.lock` (sibling candidates were building/benching
concurrently in this session).

| Variant | single-core p50 (`taskset -c 2`, 3 rounds) | multi-core p50 (unpinned, 3 rounds) |
|---|---|---|
| baseline (HEAD, `5a5f0e24`) | 24.749 / 21.775 / 22.371 ms | 19.278 / 18.923 / 22.171 ms |
| **PR #1353 as submitted** (`patch.diff` applied, real `hayro`/`hayro-bench` build) | 20.955 / 21.455 / 24.977 ms | 22.950 / 21.520 / 19.404 ms |
| *secondary, not part of `patch.diff`* - PR #1353 + one-line Cargo.toml feature-wiring fix (see below) | 12.337 / 12.126 / 12.286 ms | 10.533 / 10.507 / 10.489 ms |

Additional baseline noise-floor check at `--iter 100` (5 rounds, single-core): 32.225 / 26.271 /
22.272 / 24.870 / 24.011 ms - confirms the baseline/as-submitted rows above (which overlap
21-25ms) are within this workload's own noise band, not a real separation.

## Result

**"PR #1353 as submitted": no measurable timing change.** Baseline and patched round-robin
between ~19-25ms on both single- and multi-core, with patched rounds landing inside the same
spread as baseline rounds (no consistent ordering across 3+3 rounds either direction) - this is a
noise-floor tie, not a small win or regression.

**Secondary check (not the tested patch - see below): wiring the `jpeg-decoder` feature through
correctly reproduces the PR's intended win cleanly.** Single-core p50 drops from a baseline median
of ~22.4ms to ~12.25ms (**-45%**), multi-core from ~19.4ms to ~10.5ms (**-46%**), and - notably -
run-to-run variance collapses too (12.126-12.337ms vs. baseline's 21.8-24.7ms band): the scaled
IDCT decode does real, substantially cheaper, more consistent work than the current full-resolution
`zune-jpeg` decode.

## Does it change behavior/output?

Not bit-exact in either variant, for a different reason each:

- **"PR #1353 as submitted"**: the syntax-level scaled decode never runs (see below), so the
  output difference is caused entirely by `hayro-interpret`'s new unconditional post-decode box
  filter. `sha256` of the rendered PNG: baseline `8a320327cb...`, patched `9e6b470dfe...` (differ).
  Characterized with `magick compare`: at 0% fuzz, 47882/60000 pixels (79.8%) differ (expected -
  two different resampling paths essentially never agree to the last bit); at 2% fuzz
  (~5/255) only 174/60000 (0.29%) differ; at 10% fuzz only 35/60000 (0.058%); at 20% fuzz only
  4/60000 (0.0067%). Max single-channel deviation 76/255 (G channel), PSNR 46.96dB. Amplified
  diff image: `testdata/diff_amplified_as_submitted.png` (difference channel x8).
- **Secondary "feature wired" variant**: `sha256` `ca8bffab64...` (differs from both baseline and
  as-submitted). More pixels differ at 0% fuzz (59669/60000, 99.4% - the scaled decode changes
  nearly every pixel by 1 unit, as expected) but the *magnitude* is tighter: 0 pixels differ past
  5% fuzz (baseline's as-submitted variant still had 35 past 10% fuzz), max single-channel
  deviation only 14/255 (~5.5%), PSNR 41.33dB. Amplified diff image:
  `testdata/diff_amplified_feature_wired.png`. Consistent with the PR author's own description
  ("visually indistinguishable, no artifacts").
- **Is this a breaking change?** No public API signature changes; this is a rendered-pixel-output
  change for any oversampled embedded image, which is exactly the tradeoff the PR's own
  description flags and asks a maintainer to weigh in on (whether to gate the interpret-level
  filter behind an `InterpreterSettings` opt-out).
- **Project test suite** (`cargo test --release`, both variants):
  - `-p hayro-syntax --lib`: 199 passed, 2 failed, identical baseline vs. patched (pre-existing,
    unrelated - `pdf::tests::pdf_version_header`/`pdf_version_catalog` `unwrap()` a
    `hayro-tests/downloads/...` path absent in this checkout, matching candidate 030's own note).
  - `-p hayro-interpret --lib`: 48 passed, 0 failed, identical baseline vs. patched.
  - `-p hayro-tests --test tests -- image --test-threads=4`: this checkout has no git-lfs, so
    most snapshot references don't exist yet; running once against baseline auto-creates them
    (hayro-tests' own harness behavior: first run creates the reference and fails-by-design,
    second run compares against it) - giving a real, self-consistent "baseline vs. itself" oracle
    for a before/after diff without needing the actual upstream references. Baseline (reproduced
    twice, stable): **43 passed, 5 failed** (`image_jbig2_2`/`_3`/`_4`, `pdfjs_images_1bit_grayscale`,
    `svg::image_jbig2_4` - pre-existing, same failures both reruns, unrelated to this patch).
    Patched, same references: **38 passed, 10 failed** - the same 5 pre-existing failures plus 5
    new ones: `render::image_ccit_3`, `render::image_ccit_4`, `render::image_inline_2`,
    `render::image_inline_3`, `render::image_jbig2_crash` (all genuine pixel-diff failures, not
    crashes - e.g. `image_ccit_4`: "pixel diff was 12417"). This matches 5 of the 6 snapshot tests
    the PR's own description names as changing.
  - `filter_flate_1bit` (not caught by the `image` keyword filter above, run separately the same
    way): baseline passes, patched fails ("pixel diff was 4419") - the 6th test the PR names.
  - `svg::image_ccit_4` (the 7th test the PR names as changing): stayed passing in this checkout
    on both baseline and patched - did not reproduce here, not investigated further (see "What's
    not done" below).

## Why the result looks the way it does

**The `jpeg-decoder` feature that gates `hayro-syntax`'s new scaled-decode path is never enabled
in the actual `hayro`/`hayro-bench`/`hayro-tests` build graph, even though `patch.diff` sets it
`default-on` in `hayro-syntax`'s own `Cargo.toml`.** The root-level `[workspace.dependencies]`
pins `hayro-syntax = { path = "hayro-syntax", ..., default-features = false }`
(`Cargo.toml:34`) - this suppresses hayro-syntax's own `default = [...]` feature list for every
consumer that references it via `workspace = true`. Every actual consumer
(`hayro-interpret/Cargo.toml:14`: `features = ["std", "images", "unsafe"]`,
`hayro-bench/Cargo.toml`, `hayro-tests/Cargo.toml`) lists its own explicit feature set, and none
of them lists `"jpeg-decoder"`. Confirmed directly: `cargo tree -p hayro-bench` shows no
`jpeg-decoder` node in the dependency tree at all (only `zune-jpeg`); `cargo tree -p hayro-syntax`
(the crate built *standalone*, where its own `[features] default = [...]` does apply) does show
it. `hayro-syntax/src/filter/dct.rs`'s `try_scaled_decode` is therefore compiled out of every real
binary this PR is meant to speed up - confirmed empirically with a temporary `eprintln!` at that
call site (reverted, not part of `patch.diff`): it never printed, even on the exact synthetic PDF
this candidate built specifically to trigger it (`target_dimension = Some((300, 200))`,
`width=3000 height=2000`, a 10x reduction - comfortably past the >=2x gate). This is why "PR #1353
as submitted" shows no timing change: the only code that actually runs is `hayro-interpret`'s
unconditional post-decode box filter, which does real (but comparatively small, since the
full-resolution JPEG decode - the dominant cost for this workload - still happens unchanged)
extra work, roughly canceling out in the noise. The secondary check (one-line fix:
`hayro-interpret/Cargo.toml`'s `hayro-syntax` feature list gets `"jpeg-decoder"` added) shows what
the PR's own description is actually measuring against - and that number (-45%/-46%, tighter
variance too) is real and reproduces cleanly.

## What's not done / open questions

- The Cargo feature-wiring fix (`hayro-interpret/Cargo.toml`'s `hayro-syntax` features list
  gaining `"jpeg-decoder"`) is not part of `patch.diff` and was not submitted anywhere - it's a
  one-line fix a maintainer would need to make (and likely also add to `hayro-bench`/`hayro-tests`
  if the scaled-decode path is meant to be exercised by the project's own benches/snapshot tests)
  for the PR's stated performance goal to actually take effect. Recorded as an incidental finding
  for `___BUGHUNT___` (see this candidate's own agent report) rather than folded into this
  `patch.diff`, per this skill's "one fix per candidate" rule.
- `svg::image_ccit_4`, one of the 7 tests the PR's own description names as changing output, did
  not reproduce as newly-failing in this checkout (stayed passing both before and after) - not
  investigated further; could be a difference in which corpus PDF revision this checkout has, or
  the SVG-export path not being sensitive to this particular box-filter change the way the raster
  `render::` tests are.
- Peak memory/RSS (the PR description's own primary motivating number - "73 MB -> 50 MB
  worst-page peak RSS" on an 11MB/291-page real-world document) was not measured here - this
  candidate focused on wall-clock/CPU per this workspace's usual metric; the PR's memory claim is
  plausible given the mechanism (bounding decoded buffer size to ~2x drawn size) but wasn't
  independently confirmed, and would need a similarly large real multi-page test document to
  observe (this candidate's single-page synthetic PDF is too small for a meaningful peak-RSS
  comparison).
- No attempt was made to determine whether the feature-wiring gap was intentional (e.g. the author
  meant to keep it opt-in pending review, per the PR body's own "Happy to gate ... behind an
  `InterpreterSettings` flag" offer) vs. an oversight - the PR body describes the mechanism as
  active by default with no mention of it needing manual feature activation, which reads as
  unintentional, but this wasn't confirmed with the author.
