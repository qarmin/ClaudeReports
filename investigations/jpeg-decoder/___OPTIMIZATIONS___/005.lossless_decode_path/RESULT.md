# 005.lossless_decode_path

First read-through of `src/decoder/lossless.rs` (`decode_scan_lossless` /
`compute_image_lossless`), the structurally-separate lossless decode path identified but never
opened by candidate 004. Looked for the patterns in HEURISTICS.md (allocations in a per-pixel
loop, redundant bounds checks/indexing, recomputed-per-pixel work that could be hoisted) since
lossless prediction is inherently sequential and unlikely to gain from SIMD/parallelism.

## What was found reading the code

`decode_scan_lossless` has two clearly separate phases, confirmed with temporary
`Instant`-based phase timers (added around the entropy-decode loop, lines ~50-106, and the
prediction loop, lines ~108-177; reverted before the final patch - not present in `patch.diff`):

```
PHASE entropy_decode: ~150-170ms
PHASE predict:        ~30-41ms
(total decode, incl. header/marker parsing not covered by either timer: ~248-256ms)
```

on `tests/reftest/images/lossless/1/jpeg_lossless_sel1-rgb.jpg` (3072x2048, predictor selection
1 = Ra, 3 components), 5 iterations, unpinned multi-core. So entropy decode (sequential Huffman
bit reading, unavoidably serial per HEURISTICS.md's caveat on entropy coders) is the dominant
cost at roughly 60-65% of wall time, but the prediction phase is a real ~13-15% of total time,
not noise - worth a look on its own.

Reading the `Predictor::Ra` fast-path branch (this is the branch taken for `jpeg_lossless_sel1*`,
`MR4.jpg`, `XA1.jpg` - i.e. the primary/designated benchmark image and several others; the
generic `predict()` branch below it handles selections 2-7 and was left untouched, see below):
the final "calculate rows, using left pixel as predictor" loop recomputed the left-neighbour
value on every iteration via `results[i][mcu_y * width + mcu_x - 1] as i32` - a full
`Vec<Vec<u16>>` double-indexed *read* of the value the immediately preceding iteration had just
*written* one line above, instead of carrying it forward in a local (register) variable the way
the "leftmost column" loop just above it already does (`let mut previous = ...; previous =
result;`). Both loops in this branch also re-indexed through the outer `Vec<Vec<_>>` (`results[i]`,
`differences[i]`) on every single element access instead of borrowing `&differences[i]` /
`&mut results[i]` once per component and indexing the resulting flat slices directly.

This matches HEURISTICS.md's "redundant work recomputed per-pixel that could be hoisted" +
"redundant bounds checks"/double-indirection patterns exactly, and is squarely on the ~13-15%-of-
wall-time prediction phase for the primary benchmark image.

## What was tried

Patch (`patch.diff`): in the `Predictor::Ra` branch only, hoist `&differences[i]` /
`&mut results[i]` into local slice bindings once per component, and replace the per-iteration
`results[i][... - 1]` read in the row loop with a carried-forward `previous` local (same pattern
already used one loop above it for the leftmost column). No change to the general/else
`predict()` branch (selections 2-7) - left untouched since it wasn't the phase-timed hot path for
the primary benchmark image and a smaller, single, well-isolated fix was preferred over a bigger
speculative rewrite.

| Variant | multi-core p50 (paired.sh, RUNS=9, `jpeg_lossless_sel1-rgb.jpg`, iters=20) | notes |
|---|---|---|
| baseline | 249.770 ms (batch 1) / 250.394 ms (batch 2) | two independent RUNS=9 batches |
| patched | 230.105 ms (batch 1) / 229.112 ms (batch 2) | |

```
batch 1: median 249.770 -> 230.105  (-7.9%)   B faster in 9 of 9 pairs   verdict: consistent
batch 2: median 250.394 -> 229.112  (-8.5%)   B faster in 9 of 9 pairs   verdict: consistent
```

Both `paired.sh` batches (unpinned multi-core, per `projects/jpeg-decoder.md`'s recommendation for
this machine) are 9/9 decisive and well above the ~2.2% same-binary noise floor previously
measured on this same image (see that file's "Measurement noise" section).

Also spot-checked (single run each, not full `paired.sh` batches, all bit-exact hash matches
baseline vs patched):

| image | predictor | baseline min | patched min | hash match |
|---|---|---|---|---|
| `1/jpeg_lossless_sel1-8bit.jpg` | Ra | 2.908 ms | 2.562 ms | yes |
| `1/jpeg_lossless_sel1.jpg` (grayscale) | Ra | 1.067 ms | 0.990 ms | yes |
| `1/lossless16bit.jpg` | Ra | 11.004 ms | 9.777 ms | yes |
| `2/MR4.jpg` | Ra | 3.418 ms | 3.063 ms | yes |
| `2/XA1.jpg` | Ra | 14.315 ms | 12.936 ms | yes |
| `1/jpeg_lossless_sel2.jpg` .. `sel7.jpg` | generic (untouched branch) | ~1.18-1.22 ms each | within ~1-2% of baseline each | yes |

The `sel2`-`sel7` images (generic `predict()` branch, not touched by this patch) show no
consistent direction beyond noise, as expected, and remain bit-exact - confirms the patch is
scoped to the branch it claims to affect and doesn't regress the untouched one.

## Result

Real, decisive win on the primary benchmark image: -7.9% to -8.5% median wall-clock (two
independent paired batches, 9/9 pairs each), consistent with the ~13-15%-of-total prediction
phase becoming meaningfully cheaper once the redundant memory read and doubly-indexed `Vec<Vec<_>>`
accesses are removed from its single hottest loop. Every other Ra-predictor lossless image tried
also decoded faster and bit-exact; the untouched generic-predictor images (sel2-sel7) were
unaffected either direction.

## Does it change behavior/output?

- **Bit-exact?** yes - every image tried (the primary target, both bit-depth variants, MR4, XA1,
  and the untouched sel2-sel7 images) produced identical output hashes before/after.
- **Is this a breaking change?** no - internal function body only, no signature/API change.
- Project test suite: 5 unit + 12 integration (`tests/lib.rs`, includes the full reftest/crashtest
  suite covering all the lossless selection-value images) + 4 separate rayon integration tests +
  2 doctests, all passing both before and after (`cargo +stable test --release`). No new
  failures.

## Why the result looks the way it does

The row-prediction loop was doing a genuinely wasted memory read every iteration (re-reading a
value the previous iteration had just written, through a doubly-indexed `Vec<Vec<u16>>`) where a
single carried-forward local would do - a textbook case of HEURISTICS.md's "redundant work
recomputed per-pixel that could be hoisted" that had nothing to do with parsing/entropy-decode
speed at all. Because the primary benchmark image's prediction phase is ~30-40ms out of ~250ms
total, a meaningful fraction-of-phase improvement there is visible as a decisive whole-decode
percentage even against this machine's multi-percent noise floor.

## What's not done / open questions

- The generic `predict()` branch (selections 2-7, `Predictor::Rb`/`Rc`/`RaRbRc1-3`/`RaRb`/
  `NoPrediction`) was read but not patched - it has a true per-pixel dependency on the
  same-row left neighbour (`ra`) so the same "carry in a register" trick applies less cleanly
  there (still uses `ra[i]`/`rb[i]`/`rc[i]` scratch arrays sized `MAX_COMPONENTS` rather than a
  local, and still double-indexes `results[i][...]`/`frame.image_size.width as usize` instead of
  the already-computed `width` local) - worth a follow-up if a selection-2-7 image ever becomes a
  primary workload; not attempted here since it wasn't on the phase-timed hot path for this
  session's designated benchmark image. Left as a concrete `IDEAS.md` lead.
- The initial entropy-decode loop (Huffman DC decode + `receive_extend`, ~60-65% of total time)
  was phase-timed but not patched this round - see `IDEAS.md`'s new lead on the
  `differences: Vec<Vec<i32>>` intermediate buffer (a large, avoidable allocation+full
  write/read pass) as the next thing to try there.
- Phase timers were `Instant`-based `eprintln!` calls, added and removed by hand around the two
  loops; not left in the codebase or `patch.diff` in any form.
