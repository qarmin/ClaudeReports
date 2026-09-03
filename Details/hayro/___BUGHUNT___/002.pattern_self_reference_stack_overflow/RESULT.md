# pattern_self_reference_stack_overflow

`hayro-interpret/src/pattern.rs` (`TilingPattern::interpret`) and `hayro/src/paint.rs`
(`Renderer::set_paint`). Pre-session triage found that of 15 pre-existing `crash-*` files in
`hayro-fuzz/artifacts/fuzz_pdf_render/` (stale May 2026 local corpus), 1 -
`crash-51b715b89668b0bd90d214f9249269abac0b05a2` (4369 bytes) - still reproduces on current HEAD
(`5a5f0e24`) as an ASan `stack-overflow`, unlike the other 13 which are already fixed upstream.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct replay of the stale crash file through the real `hayro-fuzz` `fuzz_pdf_render` cargo-fuzz/libFuzzer target (ASan on by default for `cargo fuzz build`) | `cargo +nightly fuzz build --release`, single input, ~15-60s per run (ASan's own stack unwind after a real stack-overflow SIGSEGV is itself slow) | Reproduces every time: `AddressSanitizer: stack-overflow on address ...`. `ASAN_OPTIONS=symbolize=0` needed to get libFuzzer to actually exit instead of hanging trying to symbolize a ~2000+-frame corrupted stack. |
| `gdb` attached to the same ASan binary, `handle SIGSEGV stop`, `bt 4000` at the moment of the crash | dev/ASan build, `-batch -x` script, single crash file | Got a full backtrace (see `output.txt`) showing an identical 9-frame cycle repeating >2000 times: `interpret` -> `fill_path` -> `Renderer::draw_rect` -> `Renderer::set_paint` -> `TilingPattern::interpret` -> `interpret` -> ... - this is what identified the actual unbounded recursion (pattern-in-pattern), not just "cloning `Resources` overflows" (which is where the crash address happened to land, since the parent-`Resources` chain grows in lockstep with the recursion and `Resources::clone` - `#[derive(Clone)]` over `parent: Option<Box<Self>>` - is itself O(depth) recursive). |
| Manual boundary-value construction: an independently hand-crafted, minimal (733-byte) PDF whose only content is a tiling pattern (`/Pattern 5 0 R`) whose own content stream sets its fill color/pattern to itself (`/Pattern cs /P1 scn` where `/P1` in the pattern's own `/Resources` again points at object `5 0 R`) and fills a rect | dev + release builds (no ASan needed - the fast `RUST_MIN_STACK` guard-page detector reports "has overflowed its stack" immediately) | Reproduces identically: same "has overflowed its stack" abort, confirming the bug class generalizes beyond the one fuzzer-found byte sequence and is specifically "a tiling pattern that (directly or via a cycle) fills/strokes with itself", not something narrower to that one input. |

Only these were needed (no `miri` - this is safe-Rust unbounded recursion, not memory-unsafety
`miri` would add anything to diagnose; no fresh fuzz harness beyond the already-restored
`fuzz_pdf_render` - the existing stale corpus already had a live hit).

## Status

**Confirmed**, reproduces **3/3 runs** for both the original fuzzer-found `crash_input.pdf` and the
independently hand-crafted `self_pattern.pdf`, deterministically (same input -> same crash every
time, no layout/seed dependence - this is a real unbounded call-stack recursion, not a
memory-corruption-dependent crash).

**Fully reproducible**: **Yes.** `repro/` calls `hayro::render` - the crate's own public rendering
entry point, the same one any real caller (a PDF viewer, a thumbnailer) would use - directly on a
real, standalone PDF file. Per `CONFIRMATION.md` section 11, a malformed file handed to a renderer's
own public entry point *is* the genuine external input; there is no second participant (peer/caller)
being impersonated here, so the single-process, single-participant repro shape is correct, not a
shortcut. (A stack overflow aborts the whole process and can't be caught with `catch_unwind`
in-process, so `repro/` re-execs itself as a child process per input and the parent observes
whether the child survives - see `repro/src/main.rs`.)

## Class

crash (unbounded native call-stack recursion reachable from a malformed/adversarial PDF's content
stream, with no `catch_unwind` able to save the caller - stack overflow aborts the whole process;
denial-of-service in any consumer that renders untrusted PDFs, e.g. a browser extension, a
thumbnailer, a document-preview service).

## Root cause

`hayro-interpret/src/context.rs` defines `MAX_NESTED_INTERPRETATION_DEPTH = 50` with the doc
comment "Maximum nesting depth for interpreting `XObject`'s/patterns/streams" - i.e. the intent was
for this cap to cover Form XObjects, patterns, and other nested content streams alike. It's enforced
correctly for Form XObjects: `x_object/form.rs`'s `FormXObject::draw` calls
`context.begin_nested_interpretation()` before recursing into a nested Form XObject's content
stream, and bails out (returns without drawing) once the depth cap is hit.

`hayro-interpret/src/pattern.rs`'s `TilingPattern` never got the same wiring. `TilingPattern::new`
does compute and store `nesting_depth = ctx.nesting_depth() + 1` (mirroring the field that
`x_object/soft_mask.rs`'s `SoftMask` and `font/type3.rs`'s `Type3Glyph` also carry forward) - but
`TilingPattern::interpret` never checks that stored value against
`MAX_NESTED_INTERPRETATION_DEPTH`, and nothing else on the pattern-fill path calls
`begin_nested_interpretation`/`end_nested_interpretation` either. So a pattern whose own content
stream fills/strokes a shape with a pattern again (directly self-referential, as in
`self_pattern.pdf`, or through a longer cycle of distinct pattern objects) recurses with **no depth
limit** through:

```
interpret() -> fill_path()/stroke_path() -> Renderer::set_paint() (hayro/src/paint.rs)
  -> TilingPattern::interpret() -> interpret() -> ... (repeats)
```

Each cycle is ~9 stack frames plus whatever `Resources::from_parent`/`.clone()` costs at that depth
(`hayro-syntax/src/page.rs`'s `Resources` has `parent: Option<Box<Self>>` and derives `Clone`, so
cloning it is itself O(depth) recursive as the parent chain grows one link per pattern level) - by
roughly 2000+ levels deep the process's default 8 MiB stack is exhausted and the OS delivers SIGSEGV
on the guard page, which Rust's runtime reports as "thread has overflowed its stack" and aborts
(`SIGABRT`) rather than a catchable panic.

## Introduced by

de1f549 "First version of tiling patterns" (2025-06-04) - `git show` confirms `TilingPattern::interpret` was added by this commit already missing the `begin_nested_interpretation`/depth-check wiring that Form XObjects already had at the time.

## The fix

Add the same bounded-recursion check `FormXObject::draw` already uses, at the start of
`TilingPattern::interpret` (`hayro-interpret/src/pattern.rs`): if `self.nesting_depth >=
MAX_NESTED_INTERPRETATION_DEPTH`, return `None` (skip drawing that pattern instance) instead of
recursing further. See `patch.diff`.

This is sufficient, not just narrow to the one triggering input: it was verified against **two**
independently-constructed inputs in the same bug class (the original fuzzer-found file, and the
hand-crafted minimal self-referential pattern built from first principles, not derived from the
fuzzer file) - both stop crashing after the patch, 3/3 runs each (see `output.txt`).

- Project test suite (`cargo test --workspace --release --no-fail-fast`): **788 passed, 1286
  failed** before the patch, **788 passed, 1286 failed** after - identical pass/fail counts and an
  identical set of failing test names in both runs (`diff` of the two failure lists is empty). All
  1286 pre-existing failures are unrelated to this change: they are `panicked ... called
  Result::unwrap() on an Err value: Os { code: 2, kind: NotFound }` / "new reference image was
  created" failures from `hayro-tests`/`hayro-jbig2`/`hayro-jpeg2000`'s asset-suite and
  reference-image tests, which need `python sync.py` to download external test assets not present
  in this environment - not exercised by `hayro-interpret::pattern` at all. No test anywhere in the
  workspace covers tiling-pattern recursion depth before this patch.

## What's not done / open questions

- The same missing-guard pattern exists in at least one other place noticed while root-causing
  this: `hayro-interpret/src/font/type3.rs`'s `Type3Glyph` (rendering a Type 3 font glyph whose own
  glyph procedure draws text using the same or another Type 3 font) also computes and carries
  forward a `nesting_depth` field the same way `TilingPattern` did, but nothing calls
  `begin_nested_interpretation`/checks that field before recursing into the glyph's content stream
  either - this was **not** independently fuzzed/reproduced this session (no confirmed crash file
  for it), so it's filed in `IDEAS.md` as a fresh suspect rather than fixed here, per "one bug per
  candidate folder".
- `hayro-interpret/src/x_object/soft_mask.rs`'s `SoftMask` follows the identical
  compute-but-never-check `nesting_depth` shape too and was not checked either - also added to
  `IDEAS.md`.
- No formal `cargo fuzz tmin` minimization was performed on the original 4369-byte crash file -
  `tmin` runs against this particular crash class are extremely slow (each candidate reduction
  needs a full ASan stack-overflow detect-and-report cycle, and repeated attempts with generous
  timeouts (up to 300s) still didn't finish within this session's budget) - so `repro/` ships the
  original 4369-byte file as-is, alongside the independently-constructed 733-byte minimal
  `self_pattern.pdf`, which serves the same "not narrow to one exact byte sequence" purpose that
  minimization would have.
