# imageops::resize Rgba<u8> fast path - fused alloc+premultiply

Candidate 014's `Rgba<u8>` pic-scale-safe delegation fast path (`try_pic_scale_safe_rgba8_resize`
in `src/imageops/sample.rs`) premultiplies a non-constant-alpha source via `samples_u8.to_vec()`
(a full copy) followed by a separate in-place `pic_scale_safe::premultiply_rgba8` pass. This
candidate swaps that for `pic_scale_safe::premultiplied_rgba8`, which allocates and premultiplies
in a single fused pass, as suggested in candidate 014's "What's not done" section. This is the
"smaller independent win" option from that backlog item, not the output-size-crossover/alpha-gating
heuristic (not attempted this session - still open, see `IDEAS.md`).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (unchanged since candidates 013/014).
Since 013/014's own patches were reverted at the end of their sessions (per this skill's "the one
rule that matters most"), this candidate's `patch.diff` re-includes both of theirs plus this
session's one-line change to `try_pic_scale_safe_rgba8_resize`, so it applies standalone against
the pinned base commit. "Baseline" below means 013+014 applied, unmodified; "patched" means
013+014+this session's fused-alloc change.

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize`, already extended by
candidate 014 with `src_rgba` (non-constant alpha, 1920x1080) and `src_rgba_opaque` (constant
alpha) sources at 400x300 and (non-opaque only) 2000x2000 - reused as-is, no further bench changes
needed since this candidate doesn't add a new code path, just changes what an existing one does
internally.

All numbers are alternating A/B runs of two prebuilt `--release` criterion binaries (baseline:
013+014 only; patched: 013+014+ this change), `--warm-up-time 1 --measurement-time 2`
(`--measurement-time 3` for the 2000x2000 case), seconds apart on the same machine. `uptime` load
was 1.6-2.8 (well under the 24-core count) throughout this session.

**`resize rgba 400x300 Triangle`, non-constant alpha, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline (013+014) | 9.8803 ms | 9.9997 ms | 9.6264 ms | 9.8071 ms | 9.5232 ms |
| patched (+ fused alloc) | 8.2037 ms | 7.5601 ms | 8.2016 ms | 8.0539 ms | 8.2381 ms |

Patched faster in 5/5 pairs (median 9.8071 -> 8.2016 ms, ~-16 to -24% per pair).

**`resize rgba 400x300 Lanczos3`, non-constant alpha, single-core:**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 14.818 ms | 14.472 ms | 14.835 ms |
| patched | 13.302 ms | 13.062 ms | 13.420 ms |

Patched faster in 3/3 pairs, ~9-11%.

**`resize rgba 400x300 Triangle`, non-constant alpha, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 5.0082 ms | 5.2002 ms | 5.0681 ms | 4.9415 ms | 4.9347 ms |
| patched | 4.1839 ms | 4.0757 ms | 3.9980 ms | 4.0775 ms | 4.1172 ms |

Patched faster in 5/5 pairs, ~17-21%.

**`resize rgba opaque 400x300 Triangle`, constant alpha, single-core (sanity check - this path
doesn't call `needs_premultiply` at all, so no change expected):**

| Variant | run 1 |
|---|---|
| baseline | 4.3968 ms |
| patched | 4.3590 ms |

No meaningful difference (~2%, within this machine's usual single-pair noise) - as expected, since
the opaque/constant-alpha branch never executes the changed code.

**`large/resize rgba 2000x2000 Triangle`, non-constant alpha, single-core
(`--measurement-time 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 28.392 ms | 28.635 ms | 29.016 ms | 30.280 ms | 30.270 ms |
| patched | 26.155 ms | 26.973 ms | 34.144 ms | 25.553 ms | 29.375 ms |

Patched faster in 4/5 pairs; pair 3 is an outlier (patched 34.144 ms vs. baseline 29.016 ms,
inconsistent with every neighboring pair) consistent with this machine's known noise ("mode
lottery", see `IDEAS.md`/`projects/image.md`). Excluding that pair, patched is consistently
~4-13% faster - a real but much smaller relative win than at 400x300, as expected (the fixed
premultiply-copy saving is a smaller fraction of a much larger total resize cost here).

## Result

Consistently faster for every non-constant-alpha case tried (5/5, 5/5, 3/3, and 4/5 paired runs),
by roughly 15-24% at the small-downscale (400x300) size that candidate 014 flagged as a
regression, ~9-11% for `Lanczos3` at the same size, and a smaller but still real ~4-13% at the
2000x2000 upscale size. No change for the constant-alpha path, as expected since it doesn't
execute the changed code.

This alone does **not** eliminate candidate 014's regression at 400x300 (candidate 014 measured
baseline-vs-013/014-fast-path deltas of roughly 8.1974 ms -> 9.8883 ms, i.e. the fast path was
~21% slower than the *original, non-delegating* algorithm at that size for `Triangle`; this
session's patched fast path measured ~8.0-8.2 ms at the same size and filter, i.e. now roughly on
par with the original algorithm rather than clearly slower, but not clearly faster either) - it
narrows the gap this backlog item described rather than closing it, consistent with candidate
014's own diagnosis that `premultiply_rgba8`'s own arithmetic cost (not the extra copy) is the
larger part of the fixed tax.

## Does it change behavior/output?

- **Bit-exact? Yes.** Verified via a temporary standalone example
  (`examples/_resize_rgba_premul_diff_tmp.rs`, written and run this session, removed before
  capturing `patch.diff`) resizing a deterministic 1920x1080 non-constant-alpha `RgbaImage` to
  400x300 across all five `FilterType` variants, hashing the raw output bytes (`DefaultHasher`
  over the `Vec<u8>`) for both the pre-change (013+014) and post-change (+ this patch) source,
  toggling only the one code block in between two runs of the same example binary. All five
  filters produced byte-identical hashes (e.g. Triangle: `4e7b3cfa8e0e7448` both times, Lanczos3:
  `357089dfb02ad5b0` both times) - expected, since `pic_scale_safe::premultiplied_rgba8` computes
  the exact same `div_by_255` formula per channel as `to_vec()` + `premultiply_rgba8`, just into a
  freshly-allocated buffer instead of a copied one.
- **Is this a breaking change?** No new breakage beyond what candidate 014 already documented for
  the `Rgba<u8>` delegation itself (this candidate doesn't change what algorithm runs, only how
  the premultiply buffer is obtained) - see `014.../RESULT.md` for that discussion.
- **Project test suite:** 331 passed, 0 failed, both before and after (`cargo test --release
  --lib`) - same count both times, no new failures. Same result 013/014 already found; the
  existing suite does not pin `imageops::resize`'s exact numeric output for `RgbaImage` inputs.

## Why the result looks the way it does

`samples_u8.to_vec()` followed by `premultiply_rgba8(&mut premultiplied)` is two full passes over
an ~8 MiB (1920x1080x4) buffer: one memcpy-shaped copy, then one read-modify-write arithmetic pass.
`pic_scale_safe::premultiplied_rgba8` fuses these into a single pass that reads the source and
writes directly into a fresh allocation, doing the same arithmetic but touching each byte of
memory roughly once instead of twice (ignoring the target-buffer zero-init `vec![0u8; ...]` that
`premultiplied_rgba8` does internally before overwriting every element, which itself is typically
implemented as a fast `calloc`-style zeroed allocation rather than a byte-by-byte write). Removing
one whole pass over an 8 MiB buffer is a meaningful fraction of the
total 400x300 resize's cost (which candidate 014 measured at only ~8-10 ms total), but a much
smaller fraction of the much larger 2000x2000 resize's cost (~26-30 ms total) - consistent with
the size-dependent relative-win pattern measured above, and with candidate 014's diagnosis that
`premultiply_rgba8`'s own arithmetic (not just the redundant copy) is the dominant fixed cost, so
removing only the copy narrows but doesn't erase the regression this backlog item was opened to
investigate.

## What's not done / open questions

- **The output-size crossover and/or alpha-constancy-based gating heuristic** (this backlog item's
  other suggested approach) was **not** attempted this session - this candidate only tried the
  smaller, independent `premultiplied_rgba8` swap first, as the backlog item allowed. Candidate
  014's regression at 400x300 is narrowed (fast path now roughly on par with the original algorithm
  rather than clearly slower) but not conclusively reversed by this change alone; a real threshold
  or heuristic is still open work if the delegation is pursued for real.
- **`pic_scale_safe::premultiply_rgba8`'s own arithmetic cost** (as opposed to the redundant copy
  this candidate removes) remains uninvestigated with `perf`/disassembly, same open item candidate
  014 left (`perf` unavailable in this environment - see `README.md`).
- Only `Triangle`/`Lanczos3` at 400x300 and `Triangle` at 2000x2000 were measured this session (not
  `CatmullRom`/`Gaussian`/`Nearest`, and not `Lanczos3` at 2000x2000) - candidate 014 already
  measured those combinations for the pre-change fast path, but this session didn't re-measure all
  of them against the fused-alloc change specifically.
