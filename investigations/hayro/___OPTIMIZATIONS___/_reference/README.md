# Reference code

Not a candidate optimization - this is the instrumentation technique used to produce the pipeline
timing breakdown in `../README.md` (temporary `Instant`+`AtomicU64` phase timers, applied to the
source, run, then reverted). Kept here so the same measurement can be reproduced or extended
without re-deriving the approach from scratch. See `../README.md`'s "Instrumentation added for
this workspace" section for what each file below covers.

- `hayro_lib_rs_profiling_instrumentation.diff` - the diff applied to `hayro/src/lib.rs` to time
  the interpret and rasterize phases (first, coarser instrumentation pass).
- `pipeline_breakdown_profiling_instrumentation.diff` - the diff applied to `hayro/src/lib.rs` and
  `hayro/src/glyph.rs` for the deeper, per-operation breakdown (second instrumentation pass:
  draw_image/draw_path/draw_glyph/clip-group ops, plus glyph outline cache lookup/miss split).
- `interpret_dispatch_instrumentation.diff` - the diff applied inside `hayro-interpret` to time
  content-stream tokenizing and `show_text_string`'s own work. Updated for candidate 007
  (`007.content_stream_dispatch_remainder`) to also add nine whole-match-arm buckets
  (`state_stack`, `transform`, `path_build`, `path_draw_whole`, `color`, `text_state`,
  `text_font_op`, `text_show_whole`, `xobject_misc`) that decompose the "remaining, undecomposed"
  slice of the dispatch-overhead bucket from `../README.md` - see that candidate's `RESULT.md` for
  what these found. The bucket classification (`__bucket_for` in `interpret/mod.rs`) reads the
  operator variant only, before the real (unmodified) match runs, so the actual dispatch logic
  stays byte-for-byte identical to upstream while being timed.
- `bench_pipeline_phases.rs` - the standalone example (`hayro/examples/`) that drives a PDF
  through the pipeline and prints the phase breakdown for the first instrumentation pass. Depends
  on `hayro_lib_rs_profiling_instrumentation.diff` plus an equivalent, not-separately-saved patch
  to `vello_common::Pixmap::into_png` (split its body into a timed `take_unpremultiplied` call and
  a timed `write_image_data`/`finish` call, same pattern as the diff above) and a temporary
  `vello_common` dev-dependency in `hayro/Cargo.toml`.
- `bench_ops_reference.rs` - the driver used with `pipeline_breakdown_profiling_instrumentation.diff`
  and `interpret_dispatch_instrumentation.diff` to produce the per-operation breakdown. Updated for
  candidate 007 to also print the nine new buckets above, and again for candidate 011
  (`../011.xobject_misc_decompose/`) to print `xobject_misc bucket minus draw_image` - the same
  device-call-subtraction technique candidate 007 applied to `path_draw_whole`/`text_show_whole`,
  applied to the one bucket 007 left undecomposed. This file is a copy kept for reference; the
  copy actually run each time lives at `hayro/examples/bench_ops_reference.rs` (applied via the
  diffs below, reverted after use like every other instrumentation in this folder).
- `img_pdf_breakdown.txt` - raw output from the second pass (pre-candidate-007) against the user's
  real `img.pdf`.
- `img_pdf_breakdown_007.txt` - raw output from the candidate-007-extended instrumentation
  against the same `img.pdf`, with the new buckets included. Note both `hayro/src/lib.rs`'s
  `Device` impl and `hayro-interpret`'s `TypedInstruction` API changed shape since the original
  July pass (`draw_glyph` -> `draw_glyph_run`, `ShowTexts` now uses
  `begin_glyph_run`/`append_text_string`/`show_glyph_run` instead of per-string
  `show_text_string`) - the diffs here are adapted to the current source, not the original
  July ones (which no longer apply cleanly).
- `img_pdf_breakdown_011.txt` - raw output from the candidate-011-extended `bench_ops_reference.rs`
  (two back-to-back runs) against the same `img.pdf`, isolating the `xobject_misc` bucket's own
  prep-vs-device-call split - see `../011.xobject_misc_decompose/RESULT.md`.
