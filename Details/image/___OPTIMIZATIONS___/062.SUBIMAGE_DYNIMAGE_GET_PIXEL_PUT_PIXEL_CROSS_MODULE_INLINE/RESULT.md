# `SubImageInner<&mut DynamicImage>::get_pixel`/`put_pixel`: the remaining compounding case from candidate 061

Candidate 061 confirmed `SubImageInner::get_pixel`/`put_pixel` are already fully inlined into
`imageops::overlay` for `SubImageInner<&mut ImageBuffer<Rgba<u8>, Vec<u8>>>`, but explicitly did
not construct the `I::Target = DynamicImage` case (`SubImage<&mut DynamicImage>`, produced by
`imageops::crop_mut` on a `DynamicImage`) - the one case that wraps candidate 060's separately-
confirmed-real `DynamicImage::get_pixel`/`put_pixel` (a large, non-generic 12-arm `dynamic_map!`
dispatch, *not* inlined by default). This candidate builds and measures exactly that remaining
case (`IDEAS.md` row dated 2026-08-29, sourced from `061.../RESULT.md`'s "What's not done"
section).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. No existing bench exercised `overlay`
against a `SubImage<&mut DynamicImage>` target. Added a `"sub image overlay on dyn image"` case
to `benches/imageops.rs` (bundled in `patch.diff`): `imageops::crop_mut` a 1920x1080 `DynamicImage`
(RGBA8-backed) to its own full extent, then `imageops::overlay` a 300x300 partially transparent
(alpha 180/255) `DynamicImage` top onto it via the resulting `SubImageInner<&mut DynamicImage>`
(`imageops::overlay(&mut *view, &top, 100, 100)`, dereferencing `SubImage` to `SubImageInner` per
its documented "does not itself implement `GenericImage`" design) - `iter_batched(..,
BatchSize::LargeInput)` per prior candidates' pattern.

**Step 1 (spot-check before patching, per 061's recommended approach):** built the bench with the
*unmodified* source and inspected `target/release/deps/imageops-*` with `nm -C --defined-only`
and `objdump -d -C`. No standalone `SubImageInner::get_pixel`/`put_pixel` symbol exists for
`SubImageInner<&mut DynamicImage>` either. `overlay`'s monomorphization for this type contains one
call to `overlay_bounds_ext` (once per invocation) plus, in the per-pixel loop, direct calls
straight to `DynamicImage`'s own `get_pixel`/`put_pixel` GOT entries (confirmed standalone,
non-inlined symbols, matching candidate 060's finding) - `SubImageInner`'s own offset-forwarding
wrapper body (`x + self.xoffset`, `y + self.yoffset`) is inlined directly into the loop around
those calls, with no separate call/return for the wrapper itself.

**Step 2 (measure anyway, since the change is cheap):** added `#[inline(always)]` to both
`SubImageInner::get_pixel`/`put_pixel` (this candidate's `patch.diff`), rebuilt, and re-ran the
same `nm`/`objdump` check - still no standalone `SubImageInner` symbol either way (as expected,
unchanged from baseline). A normalized (address/GOT-offset-stripped) `objdump` diff of the two
`overlay` monomorphizations (both exactly 0x320 bytes) showed the same pattern candidate 061 found:
not byte-identical (register allocation / instruction-scheduling differences - e.g. two `mov`/`add`
instructions swapped order, a `lea` operand order swapped, some stack-slot offsets swapped), but
identical call sites, identical branch structure, identical total instruction count (239
disassembled lines each) - no call added or removed.

Compared two prebuilt `--release` `imageops` bench binaries (baseline vs. `#[inline(always)]`-
patched source) via alternating paired runs (`paired_overlay.sh`, copied from candidate 061's
folder, `--warm-up-time 1 --measurement-time 1.2`), single-core (`taskset -c 3`) and multi-core
(unpinned, 24-core host, `uptime` load average 1.7-4.0 throughout, well under core count). A
same-binary self-check (baseline vs. an independently-copied, md5-identical copy of itself) was
run first to establish the single-core noise floor.

| Variant | single-core (`taskset -c 3`) p50 | multi-core (unpinned) p50 | notes |
|---|---|---|---|
| noise floor: baseline vs. md5-identical copy of itself (10 pairs) | A=0.9875 ms, B=0.9867 ms (ratio 0.999, B "faster" in 6/10 pairs) | not separately re-checked | establishes 6/10 (i.e. no consistent direction) as ordinary run-to-run noise on this host |
| baseline (unpatched `get_pixel`/`put_pixel`) vs. patched (`#[inline(always)]` on both) | A=0.9796 ms, B=0.9844 ms (ratio 1.005, patched faster in 2/10 pairs) | A=0.9613 ms, B=0.9567 ms (ratio 0.995, patched faster in 6/10 pairs) | |

## Result

**No consistent, reproducible timing difference**, on either single-core or multi-core. The
single-core comparison landed at 2/10 pairs favoring the patched build (i.e. the *baseline* won
most pairs) and the multi-core comparison landed at 6/10 (no direction) - both well short of
`paired.sh`'s own "consistent" bar (wins >= n-1 = 9, or wins <= 1) in either direction, and both
within the ~6/10-pairs, <1% noise floor measured on this host via the same-binary self-check. Time
was the only dimension checked; the mechanism (an inline-hint attribute added where no call was
added or removed, per the disassembly) has no plausible effect on allocation count or peak RSS, so
neither was measured.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_check_062.rs`, not
  part of `patch.diff`, removed before reverting) ran `imageops::overlay` through
  `imageops::crop_mut` on a `DynamicImage` (`SubImage<&mut DynamicImage>`) at five offset/crop
  combinations (in-bounds, negative-offset-clipped, fully-out-of-bounds-clipped, and two using a
  cropped, non-full-extent `SubImage`), reading every output pixel back via
  `DynamicImage::get_pixel` and combining an FNV-1a hash of every buffer. The combined hash
  (`ebf7d13ac74edb5e`) matched byte-for-byte between an unpatched-source build and this
  candidate's `#[inline(always)]`-patched build.
- **Is this a breaking change?** No - only `#[inline(always)]` attributes were added to two
  existing trait-impl methods (the same two candidate 061 also annotated); no public signature,
  panic behavior, or documented result changed.
- Project test suite (`cargo test --release`): **550 passed / 1 failed, identically before and
  after.** The one failure is the same pre-existing, unrelated `tests/regression.rs::
  resizing_with_catmul` every prior BMP/`pic-scale-safe`-adjacent candidate in this workspace has
  already documented (confirmed independently on this session's own baseline and patched builds).

## Why the result looks the way it does

Confirmed via `nm -C`/`objdump -d -C`: `SubImageInner::get_pixel`/`put_pixel` are trivial generic
one-line offset-forwarding wrappers regardless of what `I::Target` resolves to, and rustc's MIR
inliner (which has full MIR for a generic function available unconditionally, per candidate 059's
established finding for this crate) inlines them into `overlay`'s own compiled body on cost
grounds alone, independent of the `#[inline]` hint and independent of whether the *callee* they
forward to (`I::Target::get_pixel`/`put_pixel`) is itself inlined. This holds even when that
callee is `DynamicImage::get_pixel`/`put_pixel` - a large, non-generic dispatch that candidate 060
confirmed is genuinely *not* inlined by default and does show a real, measured win when forced
inline. The two layers are independent: the outer generic wrapper (`SubImageInner`) disappears
into straight-line code regardless, while the inner concrete dispatch (`DynamicImage`) is a real
call boundary regardless of what wraps it - so annotating the outer wrapper here has nothing left
to remove, exactly as candidate 061 predicted for this specific compounding case ("the MIR
inliner should treat it the same way candidate 061 found" - now confirmed rather than assumed).
This closes out `IDEAS.md`'s row 74, which named this exact remaining case.

## What's not done / open questions

- This was the last case named for the `SubImageInner`/`DynamicImage` inlining question; no
  further "is this cross-module accessor inlined" variant of this specific investigation (059 ->
  060 -> 061 -> this candidate) remains open in the backlog.
- `tile`, `replace`, `vertical_gradient`, and `horizontal_gradient` on a `SubImage<&mut
  DynamicImage>` target were not separately built/measured here - same reasoning as 061's "what's
  not done" note: the underlying mechanism (generic wrapper always inlined, callee inlining
  decided independently by its own size/genericity) is identical regardless of the calling
  imageops function, so a separate measurement would not be expected to show a different
  conclusion.
