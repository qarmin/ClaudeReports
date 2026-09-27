# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness and pipeline,
`SUMMARY.md` lists the candidates measured with it.

## Workload

hayro converts a PDF page to pixels in 3 stages, all on the CPU, before handing off to *some*
encoder (PNG via vello's built-in path, or - as the user actually does - JPEG via the external
`jpeg-encoder` crate, not present in this repo). Code path, for one page (`hayro::render` in
`hayro/src/lib.rs:236`, called by `hayro::render_pdf` at `hayro/src/lib.rs:297`):

| Stage | Code | What it does |
|---|---|---|
| 1. Parse | `hayro_syntax::Pdf::new` | Parse PDF object graph, xref table, page tree. |
| 2. Interpret + decode | `hayro_interpret::interpret_page`, called from `hayro/src/lib.rs:282` | Walk the content stream, build paths/text, decode embedded images (JPEG via `zune-jpeg`, JPEG2000 via `hayro-jpeg2000`, CCITT fax via `hayro-ccitt`, JBIG2 via `hayro-jbig2`, color space conversion via `hayro-interpret/src/color/*`), feed all of it into `vello_cpu::RenderContext` as scene geometry. |
| 3. Rasterize | `vello_cpu`'s `RenderContext::render`, called from `hayro/src/lib.rs:290` | Vello's CPU tiling/strip rasterizer turns the scene into a pixel buffer (`Pixmap`, premultiplied RGBA8). |
| *(4. Encode)* | Caller's choice - `Pixmap::into_png` if using vello's built-in path, or the caller's own encoder (e.g. `jpeg-encoder`) after reading pixels out of the `Pixmap`. | Out of scope for this repo if the caller uses their own encoder - see the scoping correction below. |

| mode/scene | what it stresses |
|---|---|
| `hayro-bench render_bench` | Stages 1-3 end-to-end (parse+interpret+decode+rasterize), no PNG/JPEG encode |
| `hayro-bench jpx_decode_bench` | JPEG2000/JPX decode specifically |
| Each candidate's own `bench/` (see its `RESULT.md`) | One isolated sub-operation, usually a pure function extracted for hash-checked measurement without depending on the `hayro`/`vello` crates directly |

### Important scoping correction

Stage 4 was initially assumed to be PNG (`vello_cpu::Pixmap::into_png`), and a real, large win was
found there (see the vello-side `002.png_compression_level`) - but the user's actual downstream
code encodes to JPEG via a separate crate (`jpeg-encoder`), not through that path. That fix is
still real and still worth having for anyone who *does* use vello's PNG output, but it's not
relevant to the user's own pipeline. Stages 1-3 (and, within stage 2,
`Pixmap::take_unpremultiplied` if the caller reads pixels via that method before feeding them to
their own encoder) are the parts that matter for any encoder choice, and are where attention
shifted after this correction - see candidate `005`.

## Measurement harness

`hayro-bench`'s `render_bench` and `jpx_decode_bench` (real binaries in `hayro-bench/src/`) are
the existing project benchmarks reused throughout this workspace - `render_bench` does **not**
measure PNG/JPEG encoding at all (it stops after rasterization), a distinction worth remembering
before trusting any of its numbers as "full pipeline" timing. Beyond that, most candidates needed
a purpose-built isolated bench (see each candidate's own `RESULT.md` for its exact bench and
invocation) rather than reusing `hayro-bench`, since the sub-operations under test (RGB->RGBA
expansion, IDWT filtering, unpremultiply, PNG compression) aren't independently exposed by it.

## Instrumentation added for this workspace

`perf` was not available on the test machine (not installed, no root to install it without
asking) for most of this workspace's history. Measured instead with temporary
`std::time::Instant` + atomic-counter instrumentation wrapped around each stage/operation, applied
to the source, run, then reverted. Two separate instrumentation passes were built, at different
levels of granularity, both kept in `_reference/` since they cover genuinely different scopes (not
duplicates):

**Update (candidate 020):** `valgrind`/`callgrind` **is** available in this sandbox (`perf` still
is not) - useful for exactly the cases the `Instant`-based approach can't resolve above wall-clock
noise, since `callgrind`'s instruction count (`Ir`) is deterministic run to run. One gotcha: the
default `ulimit -n` here (2,147,483,584) makes `valgrind` fail outright with `Private file
creation failed` - run it as `bash -c 'ulimit -n 4096; valgrind --tool=callgrind ...'` in the same
shell. See `020.resolved_font_cache_callgrind/RESULT.md` and its `justfile` for a full worked
example (build a probe binary against the code path of interest, run under callgrind for
baseline/patched, `callgrind_annotate --auto=yes <out> <source file>` to get exact per-line
instruction costs).

| Diff | Scope | Used for |
|---|---|---|
| `_reference/hayro_lib_rs_profiling_instrumentation.diff` | Top-level split of `hayro::render()` into interpret vs. rasterize phases only | The original PDF-level breakdown (per-file table below) |
| `_reference/pipeline_breakdown_profiling_instrumentation.diff` | Deeper: every `Device` trait method (`draw_image`, `draw_path`/`draw_rect`, `draw_glyph`, clip/group ops) plus the glyph outline cache lookup/miss split | The encoder-agnostic per-operation breakdown below |
| `_reference/interpret_dispatch_instrumentation.diff` | Content-stream tokenizing (`ops.next()`) and `show_text_string`'s own work, inside `hayro-interpret` | Decomposing the "dispatch overhead" bucket into tokenizing vs. font/glyph resolution vs. unattributed |

Driver code: `_reference/bench_ops_reference.rs` and `_reference/bench_pipeline_phases.rs`
(standalone examples, not kept as real project examples - copy back alongside the relevant diff
above to reproduce). `_reference/img_pdf_breakdown.txt` has the full raw output behind the
per-operation table below.

`tools/analyze_diff/` (a small standalone Rust tool, not one of the generic `tools/*.sh`/`.py`
scripts copied from the skill's templates since it's specific to this project's own `[reference |
diff | actual]` triptych PNG format from `hayro-tests`) measures how many pixels differ and by how
much between a reference and actual render - used to characterize whether a change that trips an
exact-pixel snapshot test is a real regression or a tiny, arguably acceptable rounding difference
(see candidate `002`).

### Overall timing breakdown (measured, not guessed)

`taskset -c 2`, 20 iterations per file, release build:

| PDF (dominant content) | Parse | Interpret+decode | Rasterize | Unpremultiply (PNG-only) | Deflate (PNG-only) | Total ms/page |
|---|---|---|---|---|---|---|
| `issue391.pdf` (JPEG2000-heavy) | 0.3% | 64.1% | 5.1% | 2.8% | 26.3% | 36.1 |
| `image_cmyk_icc_jpg.pdf` (JPEG+CMYK-heavy) | 4.4% | 92.8% | 0.5% | 0.5% | 1.7% | 13.0 |
| `fonts_type1_latex.pdf` (vector/text-heavy) | 0.5% | 60.2% | 2.3% | 5.2% | 30.8% | 9.6 |
| `img.pdf` (the user's own real test file) | 0.1% | 27.0% | 5.5% | 7.3% | 59.5% | 8.4-10.4 |

The "Unpremultiply"/"Deflate" columns are stage 4 (PNG) - not part of the user's real pipeline,
kept for reference.

**Encoder-agnostic view of the same `img.pdf` measurement** (stages 1-3 only, renormalized to
100% since PNG-specific stage 4 isn't part of the user's pipeline):

| Sub-part of `render()` | Share |
|---|---:|
| `draw_glyph` (text rendering) | 38.8% |
| Content-stream dispatch overhead (`hayro-interpret`, not in any `draw_*` call) | 24.3% |
| Final rasterize (`vello_cpu::RenderContext::render`) | 16.1% |
| `draw_image` (decode + convert + resize) | 14.6% |
| `draw_path`/`draw_rect` | 3.7% |
| clip/transparency-group push/pop | 0.3% |

Further decomposition of the dispatch-overhead bucket (24-29% depending on instrumentation pass):

| Sub-part | Share of total `render()` time | Share of the dispatch bucket |
|---|---:|---:|
| Content-stream tokenizing (`ops.next()`, 4782 ops/render) | 8.8-8.9% | ~33% |
| Font/glyph resolution before reaching `device.draw_glyph` | 4.5-4.7% | ~17% |
| Remaining, undecomposed (many small match arms) | ~11-16% | ~50% |

And `draw_glyph`'s own breakdown: outline extraction/caching (`cached_outline` in
`hayro/src/glyph.rs`) is only ~11% of `draw_glyph`'s total time - the other ~89% is the
`vello_cpu::RenderContext::fill_path()` call itself (see candidate `004`).

## Environment

AMD Ryzen 9 7900 (12-core). `taskset -c 2` for all single-core numbers. `perf` was not installed
(no root available to install it without asking) - see "Instrumentation added" above.

`img.pdf` (repo root) is the user's own real test file - used specifically because it's the one
they actually care about, not just a generic sample. `hayro/src/lib.rs:33` has
`#![forbid(unsafe_code)]`, ruling out several `unsafe`-based candidate variants across this
workspace (see candidates `001`/`005`).

`vello/` (sibling of this repo) is a vendored local clone of `linebender/vello`, used because two
suspected perf issues lived inside `vello_cpu`/`vello_common` rather than hayro's own code - see
`../vello/___OPTIMIZATIONS___/README.md` for its own harness notes. `git-lfs` is not installed
locally, so that clone's PNG snapshot test assets appear as spuriously "modified" under plain
`git status`/`git diff` - see its README for the exact flags to avoid tripping over it.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code. Two candidates (`004`, `006`) are
   explicitly investigation-only with no patch ever applied to real source - noted as such in
   their own `RESULT.md` rather than carrying a fabricated diff.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `CORRECTION.md` in its
   folder explaining what changed and why, and update its `SUMMARY.md` row to point at it.
