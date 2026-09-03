# Wiring the vello `take_unpremultiplied` LUT fix into hayro

Full detail on the fix itself (benchmark, exhaustive correctness check, visual diff proof) is in
[`../../vello/___OPTIMIZATIONS___/001.take_unpremultiplied_lut/RESULT.md`](../../vello/___OPTIMIZATIONS___/001.take_unpremultiplied_lut/RESULT.md)
- this file covers only what's specific to wiring it into *hayro*: isolating the fix's effect from
an unrelated confound, and the end-to-end impact.

## What was tried

`patch.diff` - the `[patch]` + `exclude` change to `hayro/Cargo.toml`, pointing `vello_cpu`/
`vello_common` at a local vello checkout with the LUT fix applied. `vello_pixmap_fix.diff` - a
copy of the fix itself (same as in the vello folder), kept here so this folder is self-contained.

### The confound: local vello clone is 2 commits ahead of the pinned rev

No `git-lfs` installed and no root access to install it, so the exact pinned commit
(`dc23e93841d20060743e490ef49c1e0d0d22cf40`) couldn't be checked out standalone in the local
clone (its HEAD, `af32d531`, is 2 commits ahead). To avoid attributing a version-bump side effect
to the fix, tested in 3 steps against the same fixed set of 268 hayro render tests that reliably
pass in this checkout (many others fail for reasons unrelated to this change - missing font/asset
files in the checkout, present regardless of vello version):

| Step | vello state | Passing tests (of 268) | Regressions vs. previous step |
|---|---|---|---|
| 1 (baseline) | original pinned rev, no fix | 268 | - |
| 2 | local HEAD (2 commits ahead), no fix | 268 | 0 |
| 3 | local HEAD + LUT fix | 268 | 0 |

All 4 exact-pixel-diff JPEG2000 render tests (`stream_jpx_1/3/5/6`) pass at every step.

## Result

`hayro-bench render_bench`, `taskset -c 2`, 3 repeated trials each, 3 representative PDFs
(`image_cmyk_icc_jpg.pdf`, `issue391.pdf`, `stream_jpx_1.pdf`, `--iter 30`):

| Run | Original vello (ms) | HEAD + LUT fix (ms) |
|---|---|---|
| 1 | 62.30 | 60.63 |
| 2 | 59.10 | 59.14 |
| 3 | 58.51 | 58.10 |

Within noise (~1-2 ms out of ~59 ms) for these 3 files. Expected - see `../README.md`'s pipeline
breakdown: unpremultiply is only 0.5-5% of total time for these files, so a 25% improvement to it
alone barely moves the total. The effect should be more visible on very large/high-DPI renders or
pages with a lot of alpha/transparency content, where unpremultiply's absolute cost (proportional
to total pixel count) is larger relative to everything else.

## Does it change behavior/output?

- **Bit-exact?** no - inherits the LUT fix's characterized 1-LSB-on-~0.5%-of-(component,alpha)-
  pairs difference, see the vello-side RESULT.md for the exact numbers.
- **Why hayro's own 268-test corpus shows 0 regressions while vello's own tests catch a real
  difference:** the affected (component, alpha) pairs are ~0.5% of the space, and hayro's current
  test corpus uses alpha/opacity sparsely (a handful of discrete levels per test) rather than
  sweeping the full 0-255 range the way vello's own test does. This is a property of the current
  test corpus, not a guarantee for all hayro-rendered PDFs.
- **Is this a breaking change?** no public API change; a rendered-pixel difference of the
  magnitude described above wherever `take_unpremultiplied` is on the code path.
- Project test suite: 268/268 passing before and after (see confound table above); vello's own
  suite has 2 new failures, documented on the vello side.

## Why the result looks the way it does

Unpremultiply is a small slice of total render time for the 3 files tested here (0.5-5%), so even
a real ~25% improvement to that one operation doesn't move the end-to-end number outside the
run-to-run noise floor on this workload.

## What's not done / open questions

- vello's own 2 affected snapshot tests were not regenerated (that project's accepted golden
  files - a call for whoever owns that decision).
- The `[patch]` currently points at an uncommitted local clone. Long-term this needs either:
  committing the fix in the vello clone and pinning `[patch]` to that commit, or upstreaming as a
  PR to linebender/vello and bumping the pinned `rev` in `Cargo.toml` once merged.
