# 39 - an opaque alpha channel costs up to 80% of the image draw

Ported from `perf_test/___OPTIMIZATIONS___/39_rgba_vs_rgb/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

`make_assets` writes `<name>.png` and `<name>_opaque.png` from the **same pixels**: one RGB8, one
RGBA8 whose alpha channel is 255 everywhere. They render byte-identically - confirmed, **0 differing
pixels of 1 100 800**. They do not cost the same.

## The gap

`--repaint-buffer new` so every frame is a full repaint, `RUNS=7`, 30 items of 200px:

| source | RGB8 | RGBA8 (alpha all 255) | |
| --- | --- | --- | --- |
| 64x64 (12 / 16 KB) | 1.43 ms | 2.66 ms | **+86%** |
| 512x512 (786 KB / 1.0 MB) | 1.23 ms | 2.21 ms | **+80%** |
| 1920x1080 (5.9 / 7.9 MB) | 1.30 ms | 2.27 ms | **+75%** |

An RGBA8 image with a *varying* alpha channel costs the same as one that is fully opaque (2.19 vs
2.21 ms) - there is no shortcut for the opaque case anywhere.

**It is not memory traffic.** A 64x64 source is 12 KB against 16 KB; both fit in L1, and the gap is
the largest of the three. The extra byte per pixel is not what is being paid for.

**It is per destination pixel**, not per item. One image at 850px (722 000 px drawn) and 200 images
at 60px (720 000 px drawn) give gaps of 0.39 ms and 0.46 ms - **0.54 and 0.64 ns per pixel** across a
200x difference in item count.

## Actionable now

Strip the alpha channel from assets that do not need one. A photograph exported as RGBA costs up to
80% more to draw than the same photograph as RGB, for a byte-identical result. Nothing in Slint warns
about this and the file size barely differs, so it is easy to ship by accident.

## Mechanism: partly explained, and three doors closed

I could not fully account for the gap, and the failed attempts are the useful part of this write-up.

**Refuted - a per-pixel opaque fast path.** The `Rgb` arm of `fetch_blend_pixel`
(`software/draw_functions.rs:196`) stores straight through when `alpha == 0xff`; the `Rgba` arm never
does, and always runs a multiply, a divide by 255, a premultiply and a read-modify-write blend.
Adding the same shortcut to the `Rgba` arm - correct, and pixel-identical - was worth **-8% at best
and nothing on three of four configurations**.

**Sized - the arithmetic is only a third of the gap.** Removing the blend from the `Rgba` arm
*entirely* (unsound; store and never blend) on a single 850px image: 0.99 -> 0.89 ms, against 0.72 ms
for RGB8. So the whole per-pixel arithmetic is **0.10 ms of a 0.27 ms gap**; 0.17 ms is elsewhere in
the RGBA path and is still unexplained. This also sizes plan item **A2** (premultiply once at decode
instead of per frame): it can be worth at most that 0.10 ms, so it is a small fix, not a large one.

**Refuted - occlusion.** `is_guaranteed_opaque` (`software/scene.rs:235-253`) accepts only
`TexturePixelFormat::Rgb` and `SharedImageBuffer::RGB8`, so an opaque RGBA image can never occlude
what is behind it. Relaxing it for the `Texture` arm changed nothing; relaxing it for the
`SharedBuffer` arm changed nothing **and did not change the pixels of a translucent image either**,
which means that path is not reached by this scene at all. Both doors closed.

**Refuted - a cheaper path above `fetch_blend_pixel`.** There is none. `draw_texture_line`
(`draw_functions.rs:21-100`) computes the same fixed-point walk for both formats and the format is
only branched on inside `fetch_blend_pixel`; the position closure differs solely in the `bpp` it is
handed.

**Refuted - bounds checks in the inner loop.** The `Rgb` arm reads through one slice
(`&data[pos..pos + 3]`), the `Rgba` arm indexes `data` four separate times. On a loop this tight the
difference between one bounds check and four is a plausible 20%. Rewriting the `Rgba` arm to take a
slice *and* take the opaque shortcut - pixel-identical on five scenes including translucent images,
nine-slice, `scroll` and `text` - measured **+0.0%, winning 5 of 14 paired runs**. Reverted.

That last one nearly went in as a win. An unpaired comparison had it at 1.11 -> 0.90 ms, **-19%**;
the paired run shows the baseline itself swinging between 0.96 and 1.22 ms in two clusters. That is
the same per-process bimodality candidate 035 ran into, and it caught me a second time despite having
written it up. **Unpaired medians are not evidence on this machine.**

So four mechanisms are now excluded - the per-pixel shortcut, the blend arithmetic (worth a third of
the gap at most), occlusion, and inner-loop bounds checks - and the gap stands unexplained. The next
step needs a profiler this environment does not have (`perf_event_open` is blocked); a counter on
each arm plus a cycle count would be the cheap substitute.

## Two process notes

- One patch in this sequence **did not apply** (the anchor matched twice) and I measured anyway,
  because the shell chain used `;` rather than `&&`. The result was two identical binaries reported
  as "-0.5%, no effect". Caught only by re-reading the assertion output. `README.md` already says
  to verify the two binaries differ before believing a comparison; the mechanical version of that is
  `md5sum` on the pair, which is now in the reproduction steps below.
- A `printf` with backticks in it executed `make_assets` and overwrote `perf_test/.gitignore`.
  Restored.

## Reproduce

```sh
cargo run --profile rdebug --bin make_assets

RUNS=7 tools/matrix.sh \
  "rgb |--mode images --count 30 --image-file medium.png        --image-cell 200 --repaint-buffer new" \
  "rgba|--mode images --count 30 --image-file medium_opaque.png --image-cell 200 --repaint-buffer new"

# before believing any before/after here:
md5sum /tmp/pt_before /tmp/pt_after | awk '{print $1}' | sort -u | wc -l   # must print 2
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. Every
attempted variant during investigation (opaque fast path in `fetch_blend_pixel`, relaxed
`is_guaranteed_opaque`, slice-based `Rgba` arm) was checked for pixel-identical output where noted
above, but none was kept - all were reverted after measurement showed the gain was null or
unattributable.
