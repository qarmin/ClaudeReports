<!--
Measurement-only investigation - no patch to the tested project's own source. See "Does it
change behavior/output?" below for the explicit N/A statement this workspace requires.
-->

# `#[inline]`/`#[inline(always)]` candidate sweep across per-pixel/per-row hot loops (continuation of candidates 054/055)

Directed follow-up to candidates 054/055 (which found and root-caused one real
`brighten_in_place` fast-path regression fixed by `#[inline(always)]`): this session profiled a
broad set of decode/imageops/convert benches with `perf` to find the top real hot spots, then, for
each, checked with `nm`/`objdump` whether the small per-pixel/per-chunk helper functions inside
those hot loops are actually failing to inline (the mechanism that made 054 a real win). None of
the checked spots reproduce that pattern - see "Result" and "Why the result looks the way it does"
below for why, and why that itself is worth recording so a future session doesn't re-walk the same
ground.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as 054/055; working tree was never
modified this session - pure profiling/disassembly investigation, `git status` clean throughout).
`perf` confirmed usable (`perf_event_paranoid = -1`, per `projects/image.md`'s 2026-08-29 note).
`taskset -c 2 perf record -F 999 --call-graph dwarf` against pre-built `--release` bench binaries
(`decode`, `imageops`/`blur`, `convert`), `perf report --no-children --percent-limit 0.5-1` to find
self-time hot symbols, `nm --demangle [-S]` to check for a standalone (i.e. not-inlined) symbol,
`objdump -d -C -M intel` to inspect the actual codegen where relevant. Full `perf report` output
and `nm` checks saved in `evidence/perf_reports.txt` and `evidence/nm_inlining_checks.txt`.

Hot spots found and checked, in descending order of self-time share of their own bench:

| Bench (case) | Hot symbol (self-time) | Per-pixel/chunk helper(s) inside it | Already `#[inline]`/`#[inline(always)]`? | Standalone symbol found (i.e. NOT inlined)? |
|---|---|---|---|---|
| `blur` ("gaussian blur: sigma 7.0") | `filter_1d::filter_symmetric_column::<f32,f32>` (48.0%) + `filter_symmetric_row::<f32,f32,3>` (25.7%) | `math::utils::multiply_accumulate`, `filter_1d::ToStorage::to_` impls | yes, both `#[inline(always)]` already | no - fully inlined |
| `convert` (`cast_dynamic_rgba8_rgb8`) | `cicp::CicpRgb::cast_pixels_by_layout::<u8,u8>` (93.3%) | `FromPrimitive<T> for T`/`FromPrimitive<f32/u16/u8> for u8/u16/f32` impls (identity + bit-trick conversions) | **no** - none of these 8 trait impls carry `#[inline]` | no - fully inlined anyway |
| `convert` (`cast_dynamic_rgba8_l8`, exercises the CICP color-transform fallback path, not the fast identity path) | same function, `::<u8,u8>` (99.1%) | `ColorComponentForCicp::expand_to_f32`/`clamp_from_f32` impls, `CicpTransform::expand_rgb/expand_rgba/clamp_rgb_luma/clamp_rgba_luma` | **no** - none of these carry `#[inline]` | no - fully inlined anyway |
| `decode` (`load-Gif/sample_1.gif`) | `image::codecs::gif::blend_and_dispose_region` (2.9% of total GIF decode; most of the rest is the external `weezl`/`gif` crates) | `blend_and_dispose_pixel` (has the maintainers' own FIXME: *"internal dispatch on disposal method may be slow, investigate if this is properly and reliably vectorized"*) | yes, `#[inline]` already present | no - fully inlined; disassembly (not saved, inspected live) shows the disposal-method dispatch is hoisted **outside** the per-pixel loop into 3 separate SIMD (`xmm`) loop bodies, i.e. the FIXME's own worry has already been resolved by the optimizer |
| `decode` (`load-Tga/utc24.tga`) | `TgaDecoder::read_image` (55.2%) + `libc __memmove_avx512` (32.4%, buffer reads/copies) | n/a - dominated by actual data movement (`read_exact`/orientation-fixup byte copies), not a thin dispatch/getter shim | n/a | n/a - not an inlining question, it's real I/O-bound work |
| `decode` (TIFF planar path, code-read only, not a dedicated bench case) | `TiffDecoder::interleave_planes` | `utils::interleave_planes` (`#[inline(always)]`) → `trampoline::<PLANES,N>` → `interleave_planes_inner` (`#[inline(always)]`) | yes, both `#[inline(always)]` | **partially** - `trampoline::<4, 1>` (La(1)/Cmyk-shaped, one specific monomorphization) stayed a standalone 558-byte symbol despite `#[inline(always)]` on its callee; not performance-relevant here since `interleave_planes` runs once per decoded image, not once per pixel |
| standalone check: `imageops::overlay`/`Blend::blend` for `Rgba<T>` (no existing bench; built a throwaway downstream-crate-style binary, see below) | n/a (no existing bench exercises this path) | `Blend::blend` for `Rgba<T>` (30-line float alpha-compositing function) | **no** `#[inline]` on the impl | no - `overlay`+`blend` both fully inlined into `main` (731-byte symbol, no separate `overlay`/`blend` symbols) even in a *separate* crate depending on `image` via `path = ...` (no LTO configured) |

The last row is worth explaining: `overlay`/`tile` (the classic `get_pixel`/`put_pixel`/`blend`
trait-dispatch pattern already converted to a fast path elsewhere in this workspace for
`affine.rs`/`sample.rs`, per `projects/image.md`'s "Next-win leads") have no existing bench in this
repo, and `Blend::blend`'s `Rgba<T>` impl has no `#[inline]` annotation - both look, on paper, like
exactly the "small per-pixel function that might not cross a module boundary" pattern this session
was asked to check for. To test it without inventing a whole new criterion bench up front, a
throwaway standalone crate (`/tmp/overlay_check`, not saved - deleted at the end of this session,
per "the workspace is not scratch" not applying to genuinely disposable exploratory checks that
found nothing) was built with `image = { path = "..." }` and a tight `overlay` loop, release build,
default profile (no LTO, `codegen-units` left at Cargo's default 16). `nm` on the resulting binary
shows no `overlay` or `blend` symbol at all - both are fully inlined into `main`, confirming that
even for a *generic* public API function called from an actual separate crate with no LTO, the
inliner already does the job.

## Result

**No new `#[inline]`/`#[inline(always)]` candidate met this session's bar.** Every per-pixel/
per-chunk helper function found inside a genuine hot loop (by `perf` self-time share) was already
either explicitly `#[inline(always)]`-annotated, or - for the ~10 `FromPrimitive`/
`ColorComponentForCicp` trait impls in `src/color.rs`/`src/metadata/cicp.rs` that carry **no**
`#[inline]` at all - already fully inlined anyway, confirmed by the absence of any standalone `nm`
symbol in the relevant release bench binary. No source change was made; no benchmark comparison
(before/after) exists because there is no "after" - see "Why the result looks the way it does" for
why this differs from candidates 054/055's actual, measured win.

## Does it change behavior/output?

**Measurement-only investigation - N/A, no patch applied to the tested project's own source.**
`git status` was clean (only `___OPTIMIZATIONS___/`/`___BUGHUNT___/`/`___OTHER___/` untracked)
throughout - there is correctly **no `patch.diff`** in this folder.

- **Bit-exact?** N/A - no source change.
- **Is this a breaking change?** N/A - no source change.
- Project test suite: not run - no patch exists to compare a before/after against (this session's
  candidate loop never reached the "apply" step, since no candidate survived the initial
  nm/objdump check to be worth building/measuring).

## Why the result looks the way it does

Modern rustc runs its MIR-level inliner **before** codegen-unit partitioning, and for a function
whose body lives in the *same crate* as its call site (which is true of every helper checked here,
including the `Rgba<T>: Blend` impl reached from a separate downstream crate - `Blend::blend`'s
MIR is exported in `image`'s crate metadata regardless of `#[inline]`, because `Blend for Rgba<T>`
is a generic `impl`, and generic-function MIR must always be exported for monomorphization to work
downstream at all). This means, for the specific shape "small/cheap function, called from a hot
per-pixel loop, same crate" that this task's focus area named, `#[inline]`'s main lever - making a
function *available* to be inlined at all - was never actually blocking anything in the spots
checked: the MIR inliner's own cost heuristic already chooses to inline these because they're
cheap, independent of the attribute. `#[inline]` mainly changes the inliner's *threshold weighting*
(nudging a borderline-cost function over the line) or, for `#[inline(always)]`, forces it
regardless of cost - which is exactly why candidate 054's fix worked (`brighten_in_place`'s
fast-path body was large enough to sit right at that threshold) and why none of this session's
candidates needed it (they're all either trivially small, one-line bit tricks, or - for
`Blend::blend`'s ~30-line float compositing body - apparently still under the inliner's default
threshold despite being unannotated).

The one partial exception found (`interleave_planes`'s `trampoline::<4, 1>` staying standalone
despite `#[inline(always)]` on its callee) shows that even the strongest annotation doesn't
*guarantee* full inlining collapse for every monomorphization in a large `match`-dispatched
generic function, but it isn't performance-relevant here: `interleave_planes` is called once per
decoded planar-TIFF image, not once per pixel, so one un-inlined call in one rarely-hit branch
(planar `La(1)`/CMYK-shaped layout) has no measurable cost.

## What's not done / open questions

- This was a breadth-first sweep (5 different codec/imageops areas), not a deep dive into any one
  of them - a future session could instead pick one specific *algorithmic* (not inlining-shaped)
  hot spot found here and pursue it on its own merits: `cicp::cast_pixels_by_layout`'s fallback
  color-transform path (93-99% of the `cast_dynamic_rgba8_l8`-style benches) is a real, sizeable,
  previously-untouched hot spot in this crate, but the opportunity there (if any) is in the
  STEP=256-batched loop structure/SIMD shape, not in adding `#[inline]` anywhere - out of this
  session's stated focus area, so deliberately not pursued here. Recorded as a new `IDEAS.md`
  backlog entry instead.
- `TgaDecoder::read_image` (55% self-time on `utc24.tga`) was profiled but not investigated
  further - it's dominated by real `memmove`d data (`read_exact` into the output buffer), not a
  dispatch/inlining question; whether the `fixup_orientation`/`reverse_encoding_in_output` byte-
  swap loops it calls are optimally vectorized was not checked (out of this session's focus area).
- The `interleave_planes::trampoline::<4, 1>` partial-inlining anomaly was noted but not
  root-caused (would need `-Zmir-opt-level`/`--emit=llvm-ir` comparison across all 12
  monomorphizations to say why that one specifically didn't collapse) - not worth the effort given
  its call frequency (once per image, not per pixel).
- No new criterion bench was added for `imageops::overlay`/`tile` even though this session found
  they have none - the disposable standalone check found nothing worth building a permanent bench
  around from an *inlining* angle, but the pattern (they're the one remaining un-converted
  `get_pixel`/`put_pixel`/`blend`-dispatch call site of the kind candidates 001-007 already fixed
  elsewhere) is a real, different (structural fast-path, not inlining) candidate for a future
  session - added to `IDEAS.md`.
