# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo +nightly fuzz` (cargo-fuzz 0.12.0) | yes | ASan is on by default for `cargo fuzz build`/`run` - no extra flags needed for a basic sanitizer pass. |
| `cargo +nightly miri` | yes (`miri 0.1.0`, nightly-2026-08-21 toolchain present via rustup) | not yet exercised this session - most of hayro's suspect surface is safe Rust parsing logic (arithmetic/index bugs), so debug-assertions builds catch more per unit effort here than miri does; reach for miri specifically for `unsafe`/`bytemuck`/transmute-heavy spots (e.g. `hayro-jpeg2000` buffer code, `pic-scale`/`vello_cpu` deps) if a candidate points there. |
| UBSAN | not yet tried standalone - ASan-by-default from cargo-fuzz covers the memory-safety half; add `-Zsanitizer=undefined` manually if a candidate needs it. |
| `[profile.fuzz]` in root `Cargo.toml` | exists (`inherits = "release"`, `debug = 1`, `overflow-checks = true`) | overflow-checks already on for the fuzz profile - this is exactly what caught the `huffman_table.rs:455` overflow panic below. |

## Existing project harnesses worth reusing

`hayro-fuzz/` (cargo-fuzz, libFuzzer) - 7 targets after this session's restoration (see below):

- `fuzz_ccitt`, `fuzz_jbig2`, `fuzz_jpeg2000` - already on `main`, decoder-only, narrow.
- `fuzz_pdf_parse`, `fuzz_pdf_render`, `fuzz_cmap`, `fuzz_postscript` - **restored this session**
  from the user's own unmerged local branch `Fuzzers` (commit `4b713923347559f0b902cfe16d1956ebbb9a6fb60`,
  2026-07-21, not an ancestor of `main`/HEAD). `fuzz_pdf_render` is the broadest target: full
  parse -> render(first page) pipeline (content-stream interpreter, color spaces, fonts, shadings,
  image XObjects, PDF Functions, patterns, `vello_cpu` rasterization). `fuzz_pdf_parse` covers
  `hayro_syntax::Pdf::new` + xref/object/page walk without rendering. All targets use only the
  crates' public API.
- **Kept in the main project's working tree for the whole session** (per user decision, see the
  conversation this workspace was set up in) - `hayro-fuzz/Cargo.toml` and the four new
  `hayro-fuzz/fuzz_targets/*.rs` files are new/modified, uncommitted. This is a deliberate exception
  to the "leave the main tree clean" rule for individual bug-fix candidates - it's tooling setup
  (equivalent to installing miri), not a candidate patch. **Before ending this session, flag this
  restoration explicitly to whoever reads the report** so they can decide whether to commit it,
  cherry-pick from `Fuzzers` properly, or revert it - don't silently leave it there or silently
  commit it.
- `hayro-fuzz/artifacts/<target>/` held a **stale local crash/timeout corpus from a prior session
  around 2026-05-26/27** (gitignored, not from this skill) covering `fuzz_jbig2` (17 `crash-*`, ~30
  `timeout-*`, several `slow-unit-*`), `fuzz_pdf_parse` (1 `crash-*`), and `fuzz_pdf_render` (15
  `crash-*`) - `fuzz_cmap`/`fuzz_postscript` had corpus dirs but no crash/timeout files. Replaying
  all of these against current HEAD before starting discovery (see `IDEAS.md`) found 3 still-live
  issues out of that whole stale corpus; everything else in it no longer reproduces (already fixed
  upstream between May and August 2026) - don't re-investigate those as "new" candidates, they're
  just dead corpus now.

## Instrumentation/tooling set up for this workspace

Restoring the 4 fuzz targets above was the only nontrivial setup this session - see "Existing
project harnesses" above for what/why/caveat. Everything else (cargo-fuzz, miri, nightly toolchains)
was already present in the environment.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
