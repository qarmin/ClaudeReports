# Re-running candidate 007's `Tf`-resolution-cache fix against `fonts_type1_latex.pdf`

Backlog entry from `../IDEAS.md`: candidate 007 measured a -16% to -18% `render()` win from
caching the resolved `Tf` (text font selection) font by resource name, but only against `img.pdf`.
This candidate re-runs the exact same patch (`patch.diff` is byte-for-byte identical to candidate
007's) against `fonts_type1_latex.pdf` - another text-heavy PDF already in this workspace's
testdata (`hayro-tests/pdfs/custom/fonts_type1_latex.pdf`, a 1-page pdfTeX-produced document) - to
see whether the win generalizes or was specific to `img.pdf`'s font-reuse rate.

## What was tried

No new code was written; only the bench's input file changed (`bench/src/main.rs` embeds
`fonts_type1_latex.pdf` instead of `img.pdf`, otherwise identical to candidate 007's bench: 300
iterations, 10% warmup discarded, min/p50/mean/p95/max of `render()` wall time, FNV-1a hash of the
premultiplied RGBA8 pixmap bytes checked every iteration for bit-exactness).

`taskset -c 2` (single-core) and unpinned (multi-core/realistic), each run twice, both baseline
and patched, measured against the real hayro working tree at commit `5a5f0e24` (this candidate's
`base_commit`):

| Variant | single-core p50 | multi-core p50 | pixmap hash |
|---|---|---|---|
| baseline (run 1) | 2.0482ms | 2.0361ms | `0x3a8ea147d77eb8e0` |
| baseline (run 2) | 2.0334ms | 2.0366ms | `0x3a8ea147d77eb8e0` |
| patched (run 1) | 2.0357ms | 2.0405ms | `0x3a8ea147d77eb8e0` |
| patched (run 2) | 2.0297ms | 2.0290ms | `0x3a8ea147d77eb8e0` |
| patched (run 3, via `just test` from a fresh clone) | - | 2.0842ms | `0x3a8ea147d77eb8e0` |

All 300 iterations of every run (baseline and patched alike) produced the identical pixmap hash.

## Result

**No measurable change.** Baseline p50 clusters at ~2.03-2.05ms, patched p50 at ~2.03-2.08ms -
the spread between repeated runs of the *same* variant (about 1.5%) is as large as the spread
between baseline and patched, so the difference is within this bench's own noise floor. This is a
sharp contrast with candidate 007's -16% to -18% win on `img.pdf`.

Supporting data for *why*: `fonts_type1_latex.pdf`'s single page contains only 26 raw `Tf` operator
occurrences (counted directly in the uncompressed content stream - this PDF's streams use
`ASCIIHexDecode`, not `FlateDecode`, so a plain byte search is reliable), versus 326 `Tf` calls on
the `img.pdf` page candidate 007 profiled. Candidate 007 estimated each `Tf` call cost the
unpatched path roughly 860ns on average; at that rate 326 calls account for ~280us of `img.pdf`'s
~2.95ms baseline render time (about 9.5%, consistent with the profiled 6.4pp `text_font_op`
bucket), while 26 calls account for only ~22us of `fonts_type1_latex.pdf`'s ~2.03ms baseline
render time (about 1.1%) - too small a share of total render time for this bench's noise floor
(~1.5%) to resolve as a real signal, even if the per-call fix is doing exactly the same thing it
did for `img.pdf`.

## Does it change behavior/output?

- **Bit-exact?** Yes. All 300 iterations of every run (both variants) produced identical pixmap
  hashes (`0x3a8ea147d77eb8e0`) for `fonts_type1_latex.pdf`, matching candidate 007's finding that
  this patch does not alter rendering.
- Project test suite: ran `cargo test --release -p hayro-tests --test tests -- render` before and
  after the patch (same command candidate 007 used). Both runs: 267 passed, 1259 failed, 181
  filtered out. Diffed the two runs' failing-test-name lists: **identical set** (1259/1259 lines
  match). As in candidate 007, every failure is a missing local test-asset file
  (`Os { code: 2, kind: NotFound }`), a pre-existing environmental limitation of this sandbox (no
  full multi-thousand-PDF corpus/git-lfs assets checked out), not a render regression.
- **Is this a breaking change?** No - same patch as candidate 007, internal to
  `hayro-interpret::interpret()`, no public API change.

## Why the result looks the way it does

The fix's saving is proportional to `Tf`-call count times per-call overhead removed; it is not a
fixed cost independent of the input. `fonts_type1_latex.pdf` calls `Tf` roughly 12.5x less often
per page than `img.pdf` did (26 vs. 326), so even though the underlying mechanism (name-keyed
resolved-font cache instead of re-hashing the font dict on every call) works identically here, the
absolute time it saves for this particular file is far below this bench's measurement noise floor.
The backlog entry's own framing ("the win should scale with `Tf`-call frequency and font-reuse
rate") is confirmed directionally, though this file happens to sit at the low end of that range
rather than a case that actively defeats the cache (e.g. a PDF that changes fonts on every single
`Tf` call, which was not tested here either).

## What's not done / open questions

- No per-operator instrumentation (candidate 007's `_reference/interpret_dispatch_instrumentation.diff`)
  was re-run against this PDF to get a real `text_font_op` percentage breakdown - the 26-vs-326
  `Tf`-count comparison above is a reasonable proxy (same operator, same fix, same cost-per-call
  assumption) but not a re-measurement of the bucket itself. A future session could re-run that
  instrumentation against this file for a directly comparable percentage if a third data point
  ever becomes useful.
- Only two PDFs have now been checked (`img.pdf`: strong win; `fonts_type1_latex.pdf`: no change).
  The other representative PDFs mentioned in candidate 007's open questions (`issue391.pdf`,
  `image_cmyk_icc_jpg.pdf`) are still untested.
- The full `hayro-tests` corpus could not be run in this sandbox for the same reason candidate 007
  hit (missing local assets) - the 267 tests that could run all passed identically before/after.
