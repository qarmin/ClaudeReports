# `SubImageInner::get_pixel`/`put_pixel`: `#[inline(always)]` spot-check across the `sub_image.rs` -> `imageops::overlay` module boundary

`SubImageInner`'s `GenericImageView::get_pixel` (`src/images/sub_image.rs:205`) and
`GenericImage::put_pixel` (`:241`) are un-annotated one-line offset-forwarding wrappers
(`self.image.get_pixel(x + self.xoffset, y + self.yoffset)`), called from a different module
(`src/imageops/mod.rs`'s `overlay`, which loops `top.get_pixel`/`bottom.get_pixel`/
`bottom.put_pixel` directly with no `to_pixel_view()` fast-path gate) every time `overlay` runs
against a `SubImage`-cropped target (`imageops::crop`/`crop_mut`). Flagged in `IDEAS.md` (row
dated 2026-08-29) as a lower-confidence variant of candidate 060's `DynamicImage::get_pixel`/
`put_pixel` finding, explicitly not yet confirmed by disassembly, with the recommendation to
nm/objdump-spot-check for a real opportunity before committing to a full patch-and-rebench cycle.

Unlike `DynamicImage`'s 12-arm `dynamic_map!` match (candidate 060), `SubImageInner::get_pixel`/
`put_pixel` are **generic** functions (`impl<I> GenericImageView for SubImageInner<I> where I:
Deref, I::Target: GenericImageView`), monomorphized per concrete `I`. Candidate 059's own
established finding for this crate is that generic functions have their MIR exported
unconditionally (needed for monomorphization in a downstream crate/module), so the inliner's cost
heuristic - not the `#[inline]` attribute nor the cross-module boundary - is what decides whether
a generic helper gets inlined; 059 also predicted candidates like this one are likely already-inlined
no-ops precisely because they're small.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. No existing bench exercised `overlay`
against a `SubImage`-backed target (`benches/copy_from.rs`'s `bench_copy_subimage_from` uses
`SubImage::copy_from`, which `SubImageInner`'s own `copy_from` override delegates straight to the
inner image's `copy_from`, bypassing `get_pixel`/`put_pixel` entirely - not a fair proxy for this
candidate). Added a `"sub image overlay"` case to `benches/imageops.rs` (bundled in `patch.diff`):
`imageops::crop_mut` a 1920x1080 `RgbaImage` to its own full extent (to get a real
`SubImageInner<&mut ImageBuffer<..>>`, not a no-op), then `imageops::overlay` a 300x300 partially
transparent (alpha 180/255) top image onto it - `iter_batched(.., BatchSize::LargeInput)` per prior
candidates' pattern.

**Step 1 (spot-check before patching, per the recommended approach):** built the bench with the
*unmodified* source and inspected `target/release/deps/imageops-*` with `nm -C --defined-only`
and `objdump -d -C`. No standalone `SubImageInner::get_pixel`/`put_pixel` symbol existed for
either the un-annotated baseline. `overlay`'s monomorphization for
`SubImageInner<&mut ImageBuffer<Rgba<u8>, Vec<u8>>>` contains exactly one call
(`overlay_bounds_ext`, called once per invocation, not per pixel) plus cold `panic_fmt`/
`slice_index_fail` bounds-check trampolines - the entire per-pixel body (offset arithmetic, the
nested `ImageBuffer::get_pixel`/`put_pixel` indexing, the `Rgba<u8>::blend` call) is already
inlined into straight-line pointer arithmetic, confirming the same already-inlined pattern
candidate 059 found for other small generic per-pixel helpers in this crate.

**Step 2 (measure anyway, since the change is cheap and the recommendation only advises against
committing to a *full* cycle, not against a quick check):** added `#[inline(always)]` to both
methods, rebuilt, and re-ran the same `nm`/`objdump` check - still no standalone symbol either way
(as expected, since it was already fully inlined). A normalized (address/register-name-stripped)
`objdump` diff of the two `overlay` monomorphizations showed they are **not** byte-identical -
register allocation and instruction ordering differ slightly (same call sites, same branch
structure, ~5 extra instructions in one code path) - so a paired timing comparison was run anyway
to see whether this codegen churn has any real, reproducible effect.

Compared two prebuilt `--release` `imageops` bench binaries (baseline vs. `#[inline(always)]`-
patched source) via alternating paired runs (`paired_overlay.sh` from candidate 060's folder,
`--warm-up-time 1 --measurement-time 1.2`), single-core (`taskset -c 3`) and multi-core (unpinned).
A same-binary self-check (baseline vs. an independently-copied, md5-identical copy of itself) was
run first to establish the single-core noise floor.

| Variant | single-core (`taskset -c 3`) p50 | multi-core (unpinned) p50 | notes |
|---|---|---|---|
| noise floor: baseline vs. md5-identical copy of itself (8 pairs) | 0.7155 -> 0.7201 ms (+0.6%, 1/8 pairs favor "B") | not separately re-checked | establishes ~0.6-1% as ordinary run-to-run noise on this loaded host (`uptime` load average 2.4-3.8 throughout this session, above the 4-core baseline this machine otherwise idles at in earlier sessions) |
| baseline (unpatched `get_pixel`/`put_pixel`) | run 1: 0.7064 ms; run 2: 0.7155 ms; run 3: 0.6917 ms | 0.7085 ms | |
| patched (`#[inline(always)]` on both) | run 1: 0.6955 ms (**-1.5%, 10/10 pairs**); run 2: 0.7104 ms (-0.7%, 7/10 pairs); run 3: 0.6911 ms (-0.1%, 7/10 pairs) | 0.7079 ms (-0.1%, 7/10 pairs) | |

Three independent single-core paired runs were taken specifically because the first one (10/10
pairs, -1.5%) looked like a real result; the second and third runs (7/10 pairs each, both well
under `paired.sh`'s own "consistent" bar of `n-1`=9/10) walked the apparent effect back to
approximately the same ~0.6-1% magnitude as the measured noise floor, in both directions across
runs. Multi-core matched the second/third single-core runs (7/10, -0.1%).

## Result

**No consistent, reproducible timing difference.** The one 10/10-pairs single-core run is not
reproducible on repeat measurement (two follow-up runs landed at 7/10 pairs, within the ~0.6-1%
same-binary noise floor established on this session's host) - `paired.sh`'s own consistency bar
(`wins >= n-1` or `wins <= 1`) is met only in that first run, not in the two repeats or in
multi-core mode. This matches the disassembly finding: `#[inline(always)]` did not remove any
call, indirect or direct, from `overlay`'s hot loop, because both variants were already fully
inlined before the attribute was added - the small `objdump`-visible codegen differences between
the two builds are register-allocation/scheduling noise from LLVM re-processing the same already-
inlined IR, not a change in what code actually executes. Time was the only dimension checked; the
mechanism (an inline-hint attribute with no call being added or removed) has no plausible effect
on allocation count or peak RSS, so none was measured.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_check_061.rs`, not
  part of `patch.diff`, removed before reverting) ran `imageops::overlay` against a `SubImage`
  (`imageops::crop_mut`) target at four offsets (in-bounds, negative, and out-of-bounds-clipped)
  and combined an FNV-1a hash of the resulting buffers. The combined hash (`f7ca951ec5179738`)
  matched byte-for-byte between an unpatched-source build and this candidate's `#[inline(always)]`-
  patched build.
- **Is this a breaking change?** No - only `#[inline(always)]` attributes were added to two
  existing trait-impl methods; no public signature, panic behavior, or documented result changed.
- Project test suite (`cargo test --release`): **550 passed / 1 failed, identically before and
  after.** The one failure is the same pre-existing, unrelated `tests/regression.rs::
  resizing_with_catmul` every prior BMP/`pic-scale-safe`-adjacent candidate in this workspace has
  already documented (confirmed independently on this session's own baseline build too).

## Why the result looks the way it does

Confirmed via `nm -C`/`objdump -d -C` on the bench binary before and after adding the attribute:
neither `SubImageInner::get_pixel` nor `put_pixel` has, or ever had, a standalone symbol in
`overlay`'s monomorphization for `SubImageInner<&mut ImageBuffer<Rgba<u8>, Vec<u8>>>` - the whole
per-pixel body (offset addition, the nested `ImageBuffer::get_pixel`/`put_pixel` bounds-checked
indexing, the `Rgba<u8>::blend` call) is inlined into `overlay`'s own compiled body regardless of
the attribute, because these are small generic functions and rustc's MIR inliner (which runs
before codegen-unit partitioning, independent of the `#[inline]` hint, and always has full MIR
available for a generic function) already inlines them on cost grounds alone - the same mechanism
candidate 059 documented for `Blend for Rgba<T>`, the `FromPrimitive`/`ColorComponentForCicp`
trait impls, and TIFF's `interleave_planes` trampoline. This differs from candidate 060's
`DynamicImage::get_pixel`/`put_pixel`, which are **not** generic (a concrete 12-arm
`dynamic_map!`/match dispatch over `DynamicImage`'s enum, large enough - especially with
`get_pixel`'s `to_rgba().into_color()` conversion - to sit above LLVM's default inlining cost
threshold) and therefore had real standalone symbols and a real, measured win once forced inline.
The pre-verification note's speculative "lower confidence than 060" framing turns out to be
correct for a straightforward reason distinguishing the two candidates: 060's target was a large
non-generic dispatch shim, this one is a trivial generic wrapper the inliner already handles for
free.

## What's not done / open questions

- The small but real `objdump`-visible register-allocation/instruction-ordering difference
  between the two builds (confirmed via a normalized diff, ~5 extra instructions in one code path)
  was not root-caused further (e.g. via `--emit=llvm-ir`) - given three independent paired runs
  already show no consistent timing signal attributable to it, this was judged not worth the
  additional investigation time candidate 055's similar-shaped "leaner disassembly, no timing
  win" question received.
- `tile`, `replace`, `vertical_gradient`, and `horizontal_gradient` (candidate 060's own "what's
  not done" list, still open) also call `get_pixel`/`put_pixel` per-pixel and would hit the same
  already-inlined `SubImageInner` path if driven through a `SubImage` - not separately checked
  here since the underlying mechanism (already fully inlined, generic-function MIR export) is
  identical regardless of caller.
- This closes out the `IDEAS.md` row that named both `DynamicImage` (candidate 060, a real win)
  and `SubImageInner` (this candidate, no-op) as a single lower-confidence pair - the spot-check-
  first approach the note recommended is what caught the difference between them before a
  full bench/rebench cycle was sunk into a change with nothing to show for it.
