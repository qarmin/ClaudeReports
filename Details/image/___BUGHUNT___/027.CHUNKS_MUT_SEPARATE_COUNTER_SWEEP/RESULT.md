# CHUNKS_MUT_SEPARATE_COUNTER_SWEEP

Follow-up from candidate 026 (`CHUNKS_EXACT_MUT_TAKE_COUNTER_SWEEP`, Not reproducible), which only
searched for the literal `chunks_exact_mut(...).take(...)` shape. This candidate sweeps every
`chunks_mut(n)` call site (the non-`chunks_exact_mut` variant, whose last chunk can be *shorter*
than `n` instead of being dropped) for the same nominal-vs-actual counter mismatch shape: a
separately-tracked index/counter that assumes every chunk is exactly `n` elements wide.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep sweep of the whole repository (not just `src/`) for the literal `chunks_mut(` (excluding the unrelated const-generic `as_chunks_mut::<N>()`, which behaves like `chunks_exact_mut` - exact-size chunks, remainder dropped - and so is out of scope for this specific candidate's suspected shape) | `grep -rn "chunks_mut(" --include=*.rs .` (excluding `target/`) | 16 call sites total: `src/codecs/tga/encoder.rs` (2), `src/codecs/bmp/decoder.rs` (1), `src/images/buffer.rs` (1), `src/metadata/moxcms.rs` (12). No occurrences anywhere else in the repo (no workspace/sub-crates - single-crate `Cargo.toml`, confirmed no `[workspace]` section). |
| Manual code reading of every one of the 16 call sites for a separately-advanced counter used to index elsewhere, unaware the current chunk can be short | Read each site in full context | None of the 16 match the suspected shape: `tga/encoder.rs`'s two loops (`image.chunks_mut(bpp)` swapping `pixel.swap(0, 2)`) use no counter at all - the operation applies uniformly regardless of chunk length, and image byte length is always an exact multiple of `bpp` for `Rgb8`/`Rgba8`, so every chunk is full-size anyway. `buffer.rs`'s `self.subpixels_mut().chunks_mut(chunk_len)` loop has no counter either - `scratch` is re-sliced to `chunk.len()` on every iteration, deriving size from the actual chunk. `bmp/decoder.rs:2090`'s `row.chunks_mut(num_channels).enumerate()` does use a counter (`i`, from `enumerate()`), but `i` is generated in lock-step with the iterator itself (not separately advanced), and `row` is always exactly `width * num_channels` bytes (the shared `read_scanlines`-style helper's stride, verified at `decoder.rs:1833`/`1846`), so `chunks_mut` never actually produces a short last chunk here - `pixel[0..3]` indexing is always safe. `moxcms.rs`'s 12 sites all pair `input.chunks(N)` with `output.chunks_mut(M*N)` via `.zip(...)`, but every one of them derives its working size `n` from the *actual* yielded chunk's `.len()` (e.g. `let n = luma.len();` / `let n = output.len();`) each iteration, never from a nominal/fixed `N`, so a shorter trailing pair is handled correctly by construction, not by a separately-tracked counter. |
| Traced `bmp/decoder.rs:2090`'s row-length invariant back to its source (`num_channels` and the row-buffer allocation) to confirm the "always exact multiple" claim rather than assuming it | Manual trace of `self.rows(buf)` / the shared per-row callback helper (`decoder.rs:1833`, `1846`, `2047`-`2090`) | Confirmed: the row slice handed to the closure is always `width as usize * num_channels` bytes, an exact multiple of `num_channels` by construction, so `chunks_mut(num_channels)` can never yield a short chunk at this call site regardless of image width. |

## Status

**Not reproducible.** An exhaustive grep of the entire repository (not just `src/`, no workspace/sub-crates to miss) for the literal `chunks_mut(` shape found only 16 call sites, all read in full; none pair a separately-advanced nominal counter with `chunks_mut(n)` in a way that could desync from the real (possibly shorter) trailing chunk. Every site either uses no counter, derives its counter directly from the same iterator (`enumerate()`), or derives its working size from the actual yielded chunk's `.len()` every iteration. The suspected code shape simply does not exist anywhere in this codebase as of this commit.

**Fully reproducible**: n/a (not Confirmed).

## Class

n/a (not Confirmed).

## Root cause

n/a - no instance of the suspected pattern shape was found. `chunks_mut(n)`'s structurally different tail behavior (a short final chunk, vs. `chunks_exact_mut`'s dropped final chunk) is real, but every one of this codebase's 16 `chunks_mut(n)` call sites already handles it safely - by construction (invariant guarantees every chunk is full-size), by deriving size from the actual chunk each iteration, or by using no counter at all.

## The fix

Not applicable - nothing to fix.

- Project test suite: not run (no source change was made; `git status` was clean throughout, no scratch files were left behind since manual code reading was sufficient to reach a conclusive negative for every call site - no ambiguity that would have required a debug-assertions build or boundary-value test to resolve).

## What's not done / open questions

- This sweep only covered `chunks_mut(n)` calls textually matching `chunks_mut(`. A hand-rolled loop achieving the same effect via raw slice indexing (e.g. `for i in (0..len).step_by(n) { let chunk = &mut buf[i..min(i+n, len)]; ... }`) without ever calling `chunks_mut` at all was not swept - this is a different textual shape again and would need its own dedicated grep/read pass if a future session wants full coverage of the underlying bug *class* (nominal-vs-actual chunk-size counter mismatch) rather than just this specific API's literal call sites.
- `as_chunks_mut::<N>()` (const-generic, `chunks_exact_mut`-like semantics: remainder dropped, not shortened) was noted during the initial grep but deliberately excluded as out of scope for this candidate's specific suspected shape (it doesn't have `chunks_mut`'s "short trailing chunk" behavior) - candidate 026 already covers the "remainder dropped, not shortened" bug class for `chunks_exact_mut`; the same reasoning would apply to `as_chunks_mut` but it wasn't independently swept for a counter-mismatch under either candidate.
