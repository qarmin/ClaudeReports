# zune-png `decoder.rs` doctest examples reference a phantom APNG function / undefined variable

Found incidentally while working `___OPTIMIZATIONS___/001.PNG_PAETH_SSE41_PINSRB` (running the
full `zune-png` test suite, including doctests, as before/after validation for an unrelated
performance patch to the Paeth SSE4.1 defilter) - no connection to that candidate's own fix, purely
a pre-existing documentation defect noticed along the way.

`crates/zune-png/src/decoder.rs` has two `no_run` doctest examples that fail to *compile* (doctests
are compiled even when marked `no_run`), so `cargo test --doc -p zune-png` fails 2 of 11 doctests:

1. The crate-level "Example: Decoding an APNG" example (doc comment starting at line 55) does
   `use zune_png::{PngDecoder, FrameInfo, post_process_image_apng};` and then calls
   `post_process_image_apng(...)` as a free function. This function does not exist anywhere in the
   crate - `error[E0432]: unresolved import` `zune_png::post_process_image_apng`.
2. `PngDecoder::more_frames`'s own doc example (line 507) is a bare code fragment
   (`while decoder.more_frames() { ... }`) with only a comment ("Assuming decoder is initialized")
   standing in for actual setup - `error[E0425]: cannot find value` `decoder` `in this scope`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo test --doc -p zune-png` against a pristine checkout at `base_commit` | rustc 1.90.0, cargo 1.90.0, default features | Confirmed: 9 passed / 2 failed, both doctests above fail exactly as described |
| `grep -rn post_process_image_apng crates/zune-png/src/` | read-only source search | Confirmed the function is never defined anywhere in the crate - only referenced in `decoder.rs`'s doc comments and, oddly, inside the *deprecation note* of a different, existing function `post_process_image` ("...use post_process_image_apng instead") - that note's suggested replacement doesn't exist either |
| Read `crates/zune-png/src/apng.rs`, `README.md`, `tests/test_random.rs`, `crates/zune-image/src/codecs/png.rs` for the real, current, working APNG-compositing API | read-only | Found it: `ApngContext<T>` (struct) with `ApngContext::new(&info, colorspace)` + `ctx.process_frame(&frame, &pixels, &mut output)` - already used correctly by `README.md`, by `apng.rs`'s own doctest (which passes), and by `zune-image`'s PNG codec and the crate's integration tests. `post_process_image_apng` looks like a renamed-away-from / aspirational API name that the `decoder.rs` doc comments were never updated to match. |
| `git log -S "post_process_image_apng" --oneline -- crates/zune-png/src/decoder.rs` | read-only | Found introducing commit: `6332c284` "png: Improve docs" (2026-05-15) - this commit added ~200 lines of new module-level documentation to `decoder.rs`, including both broken examples |
| `cargo test --doc -p zune-png` again with the patch below applied, same checkout | rustc 1.90.0, cargo 1.90.0 | Confirmed fixed: 11 passed / 0 failed |
| `repro/repro.sh` against a fresh `git clone` of the public repo at `base_commit` (via `just test`) | HTTPS clone (SSH unavailable in this sandbox), `base_commit` verified by `git rev-parse HEAD` after checkout | Confirmed end-to-end, outside this working checkout: reproduces before the patch (exit 1), fixed after (exit 0) |

## Status

**Confirmed**.

Reproduces deterministically and unconditionally (it's a compile error, not a flaky runtime
condition) - every run of `cargo test --doc -p zune-png` against unpatched `HEAD`/`base_commit`
fails these same 2 doctests with the same two error codes.

**Fully reproducible**: **Yes**. `repro/repro.sh` runs the real, public-facing check
(`cargo test --doc -p zune-png`, the same command CI/`cargo publish`/docs.rs effectively exercise
when building this crate's documentation) against a genuine fresh clone of the public repository at
`base_commit`, not a hand-constructed stand-in. No second participant is involved (this is a
single-crate documentation/compile defect, not an interaction between two components).

## Class

other (documentation/doctest compile defect - `no_run` doctests are still type-checked by rustdoc,
but the affected code never executes at runtime; no compiled-library behavior is affected, only
`cargo test --doc`, `cargo doc`, and docs.rs builds of this documentation).

## Root cause

`decoder.rs`'s module-level APNG documentation (added in `6332c284`) was written against an older
or aspirational free-function API shape (`post_process_image_apng(info, colorspace, frame, ...)`)
that was never actually implemented under that name - the crate's real, current APNG-compositing
API is the stateful `ApngContext<T>` struct (`ApngContext::new` + `.process_frame(...)`), which
*is* correctly documented and used elsewhere (`README.md`, `apng.rs`'s own passing doctest,
`crates/zune-image/src/codecs/png.rs`, `crates/zune-png/tests/test_random.rs`). The existing
`post_process_image` free function (still present, `#[deprecated]`) is a distinct, intentionally
broken legacy shim kept only for backward-compat source compatibility - its body unconditionally
returns an `Err` telling the caller to "use post_process_image_apng instead", which compounds the
confusion since that suggested replacement doesn't exist either. Separately, the `more_frames`
doctest (E0425) is a documentation fragment that was written as illustrative pseudocode (a bare
`while` loop with a comment instead of real decoder setup) but marked with a `no_run` code fence
the same as a fully-compilable example, which rustdoc always type-checks regardless of `no_run`.

## Introduced by

6332c284 "png: Improve docs" (2026-05-15) - identified via `git log -S "post_process_image_apng"
--oneline -- crates/zune-png/src/decoder.rs`; this commit's own diff shows it added the entire
"Animated Images (APNG)" module-doc section (including the broken free-function example) and the
`more_frames` doc-comment block (including the undefined-`decoder` fragment) in one commit.

## The fix

Two independent, minimal doc-only edits to `crates/zune-png/src/decoder.rs` (no non-doc-comment
code touched):

1. Rewrote the "Example: Decoding an APNG" example to use the real, current `ApngContext` API
   (`ApngContext::<u8>::new(&info, colorspace)` + `ctx.process_frame(&frame, &frame_pixels, &mut
   output_canvas)`) instead of the nonexistent `post_process_image_apng` free function, removing
   the now-unneeded manual `backup_canvas`/`prev_frame_info` bookkeeping that `ApngContext` already
   owns internally (mirroring the working pattern already used by `apng.rs`'s own doctest and
   `README.md`). Also updated the one sentence of surrounding prose that named the phantom function.
2. Gave the `more_frames` doctest real decoder setup (reading `animated.png`, constructing the
   `PngDecoder`, calling `decode_headers()`) instead of a bare fragment, so it's an actual
   self-contained compilable example rather than pseudocode wearing a compilable-example's fence.

As a drive-by consistency fix in the same spirit (not itself required to make `cargo test --doc`
pass, but part of the same "doc references a function that doesn't exist" root cause):
`crates/zune-png/src/apng.rs`'s `post_process_image` deprecation note and doc comment, which told
callers to "use post_process_image_apng instead", now point at the real replacement,
`ApngContext::process_frame`.

Verified against the checkout this bug was found in (not just the standalone `repro/` clone):

- `cargo test --doc -p zune-png`: 9 passed/2 failed -> 11 passed/0 failed.
- `cargo test --release -p zune-png` (lib + integration, non-doc): 2 lib / 35 integration tests
  passed both before and after (unaffected, as expected for a doc-only change).
- `cargo clippy --release -p zune-png --all-targets`: same 2 pre-existing warnings in
  unrelated files (`headers/readers.rs` range-loop, one other), both before and after; no new
  warnings introduced.
- `patch.diff` diffstat: `+18/-25` across the two files (`git diff --numstat` at capture time).

## What's not done / open questions

- Not checked: whether any other crate in this workspace (`zune-image`'s own docs, any example
  under `examples/`) also references the phantom `post_process_image_apng` name - a quick `grep -r
  post_process_image_apng .` (excluding `target/`) at the time of this session only found it inside
  `zune-png/src/decoder.rs` and `zune-png/src/apng.rs`'s deprecation note (both fixed here), but a
  fuller workspace-wide sweep wasn't performed.
- Not checked: whether `cargo doc -p zune-png` (as opposed to `cargo test --doc`) has any other
  broken intra-doc links (` [\`ApngContext\`] `-style) elsewhere in the crate - only the two
  compile-failing doctests that `cargo test --doc` surfaces were investigated.
- The deprecated `post_process_image` free function itself is unconditionally broken by design
  (always returns `Err`, per its own doc comment/implementation, kept only for source
  compatibility) - that's pre-existing, intentional legacy behavior per its own docstring, not part
  of this candidate's fix, and wasn't changed beyond correcting the now-inaccurate function name in
  its error message/deprecation note.
