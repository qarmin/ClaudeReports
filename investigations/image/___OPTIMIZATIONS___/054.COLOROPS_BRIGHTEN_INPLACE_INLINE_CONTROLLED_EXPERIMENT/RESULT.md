# colorops.rs: confirm and fix `brighten_in_place`'s candidate-003 fast-path regression via `#[inline(always)]`

Candidate 003 converted `brighten_in_place` (and four other `colorops.rs` functions) to a
`to_pixel_view_mut`-based fast path; it consistently regressed ~17% while the same mechanism won
or was neutral elsewhere. That candidate's `RESULT.md` flagged, via `nm`, that `brighten_in_place`
was the *only* function whose baseline had no standalone symbol (fully inlined into its bench call
site) while every other function did - and hypothesized, but never tested, that the fast path's
larger prologue simply no longer clears LLVM's inlining-cost threshold at that call site. This
candidate runs the controlled experiment candidate 003 left open: add `#[inline(always)]` to the
fast-path `brighten_in_place` (and, as requested, check `contrast_in_place` for the same pattern),
plus an isolating `#[inline(never)]` control on the *unmodified* baseline loop to separate "lost
inlining" from "the fast-path loop body itself is slower".

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidate 003). Benchmark:
`benches/imageops.rs`, with `brighten_in_place`/`contrast_in_place` cases added (same
`iter_batched(.., BatchSize::LargeInput)` shape candidate 003 used - bundled into `patch.diff`).
Four prebuilt `--release` `imageops` bench binaries were compared via 6-8 alternating paired runs
per (function, mode) via a small paired-comparison script (`paired_colorops.sh`, included in this
folder), each `--warm-up-time 1 --measurement-time 1.2`. Full raw output: `raw_pairs.txt`. Symbol
presence per binary via `nm -C | grep colorops`: `nm_evidence.txt`.

| Variant | brighten_in_place vs baseline | contrast_in_place vs baseline | `nm` symbol? (brighten / contrast) |
|---|---|---|---|
| baseline (unpatched) | - | - | no symbol / symbol present |
| fast path, unannotated (reproduces candidate 003) | median +21% slower, 0/12 pairs favor patch | median +5-8% slower, 0/11 pairs favor patch | symbol / symbol |
| fast path + `#[inline(always)]` on both functions | median **27% faster**, 12/12 pairs favor patch | median +7-8% slower, 0/12 pairs favor patch | no symbol / no symbol |
| isolating control: baseline loop unchanged, `#[inline(never)]` only | median +47% slower, 0/12 pairs favor "patch" | median ~0-2% slower (within noise), mixed pairs | symbol / symbol |
| **final scoped patch (this candidate's `patch.diff`)**: fast path + `#[inline(always)]` on `brighten_in_place` only, `contrast_in_place` untouched | median **27% faster**, 14/14 pairs favor patch (8 single-core + 6 multi-core) | n/a (untouched) | no symbol / symbol (unchanged from baseline) |

Single-core (`taskset -c 3`) and multi-core (unpinned) both measured for every variant above;
directions and magnitudes matched closely between the two modes throughout (see `raw_pairs.txt`
for the per-mode breakdown - single/multi never disagreed on direction for any variant).
`/proc/loadavg` is host-wide in this container (candidate 003's same caveat), so multiple paired
runs (6-8 per cell) were used rather than trusting single comparisons, per MEASUREMENT.md.

## Result

**The inlining-context hypothesis is confirmed for `brighten_in_place`, and it fully explains and
fixes the regression - forcing inlining back with `#[inline(always)]` doesn't just recover
baseline performance, it turns the fast path into a genuine win** (median ~27% faster than
baseline, single- and multi-core, 14/14 paired runs in the final scoped measurement, matching the
same magnitude candidate 003 found for `invert`/`huerotate` once the trait-dispatch overhead is
actually removed).

The isolating control makes the causal chain explicit: applying `#[inline(never)]` to the
*original, unmodified* `get_pixel`/`put_pixel` loop (no fast-path code change at all) produces an
**even larger** regression (median +47%) than the fast path's own +21% - i.e. losing the baseline's
free inlining costs more than the fast-path loop shape itself ever did. The fast-path loop is
actually the faster of the two loop bodies once both are compared on equal (either both-inlined or
both-not-inlined) footing; candidate 003's regression was a pure inlining-context artifact, not a
cost inherent to the `to_pixel_view_mut` iteration style.

**`contrast_in_place` does NOT show the same fix.** Its baseline already has its own standalone
symbol (`nm` shows it was never inlined in the unmodified crate, unlike `brighten_in_place`,
`invert`, and `huerotate_in_place`), so it had no inlining opportunity to lose in the first place -
confirmed by the isolating control showing `#[inline(never)]` on the untouched baseline loop
changes contrast_in_place's timing by only ~0-2% (within noise). Forcing `#[inline(always)]` onto
the fast-path version doesn't fix its ~7-8% regression either (still 0/12 pairs favor the patch).
This resolves the open question from candidate 003's `RESULT.md` in the other direction: the
inlining-context theory correctly predicts `brighten_in_place`'s recovery and correctly predicts
that `contrast_in_place` *won't* recover the same way (since it wasn't inlined to begin with) -
but it doesn't explain what *does* cause `contrast_in_place`'s regression. That remains an open
question (see "What's not done" below) and is **not** part of this candidate's `patch.diff` -
`contrast_in_place` is left untouched in the real source, exactly as in the current baseline.

## Does it change behavior/output?

**No - bit-exact.** A temporary standalone example (`examples/_correctness_check_054.rs`, not part
of `patch.diff`, removed before reverting) called `brighten_in_place` on `Rgb<u8>` and `Rgba<u8>`
buffers, on a `SubImage` view (exercising the fallback path - a `SubImage`'s row stride belongs to
its parent and is never "fully packed" by the fast-path check), and with a negative value and a
value large enough to hit the upper clamp branch. FNV-1a hash of every output buffer matched
byte-for-byte between an unpatched-source build and this candidate's patched-source build.

**Test suite**: `cargo test --release --lib`: 331/331 passed, identically before and after.
`cargo test --release` (full suite, all test binaries): 550 passed / 1 failed before **and**
after - the failure is `tests/regression.rs::resizing_with_catmul`, independently confirmed (by
running it against the unpatched baseline in isolation) to already fail on this session's base
commit, unrelated to this candidate (same pre-existing failure candidate 003 already documented,
likely `pic-scale-safe` version-resolution drift - not investigated further here).

**Not a breaking change** - no public signature, panic behavior, or documented numeric result
changed.

## Why the result looks the way it does

Confirmed directly via `nm -C <bin> | grep colorops` on every one of the five bench binaries built
for this candidate (see `nm_evidence.txt` for the full table): `brighten_in_place` has no
standalone symbol in the unmodified baseline (LLVM's ordinary size/cost-based inlining decision
inlines it into its one call site), gets a standalone symbol as soon as the fast path's larger
`to_pixel_view_mut` prologue is added (no longer clears the inline-cost threshold on its own), and
loses that symbol again - recovering (and improving on) baseline performance - once
`#[inline(always)]` is added back. The isolating `#[inline(never)]`-on-baseline control shows the
inlining loss alone (independent of any loop-body change) accounts for a larger regression
(+47%) than the fast path's own +21%, meaning the fast-path loop body is not itself slower than the
original - it's actually faster once both are measured on the same (inlined) footing, consistent
with `invert`'s and `huerotate`'s wins in candidate 003.

`contrast_in_place` sits differently in this story: it was **already not inlined** in the plain,
unmodified baseline (has its own symbol from the very first `nm` check, before any patch), so the
isolating control shows removing an inlining opportunity it never had changes essentially nothing.
`#[inline(always)]` on its fast-path version does force inlining (confirmed by `nm`), but its
timing still regresses ~7-8% - meaning something about the fast-path loop shape or the
`to_pixel_view_mut` prologue itself costs `contrast_in_place` more than it saves, independent of
inlining. Given `contrast_in_place`'s per-pixel body is float-heavy (divide, multiply, clamp, two
`NumCast` conversions) compared to `brighten_in_place`'s all-integer body, a plausible next-step
hypothesis is that the prologue's `to_pixel_view_mut`/`strides_cwh` bounds/dimension check, paid
once per call either way, is a larger fraction of `contrast_in_place`'s per-call cost profile in a
way that ordinary code-reading and `nm` symbol presence alone can't distinguish from a codegen
difference in the loop body - this was not investigated further here (no `perf` available in this
environment, per `projects/image.md`'s Gotchas; `objdump` disassembly comparison would be the next
step, as candidate 003 already partially did for `contrast_in_place` without a clear answer).

## What's not done / open questions

- `contrast_in_place`'s regression is confirmed to have a *different* cause than
  `brighten_in_place`'s (not an inlining-context artifact - it was never inlined in the baseline to
  begin with), but that different cause was not identified. Left untouched in `patch.diff` and
  added as a fresh, narrower `IDEAS.md` backlog entry (the previous "check contrast_in_place for
  the same pattern" entry is now resolved - the pattern doesn't apply there - and replaced with
  this more specific follow-up).
- `huerotate`/`huerotate_in_place`/`invert` were not re-tested here (candidate 003 already measured
  them as wins/small wins); this candidate only re-touches `brighten_in_place` and investigates
  `contrast_in_place`.
- Did not try an `objdump -d` disassembly diff of `contrast_in_place`'s fast-path loop body
  specifically to distinguish "prologue cost" from "loop codegen shape" as the cause of its
  regression - bounded per SKILL.md's guidance rather than pursued exhaustively.
