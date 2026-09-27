# type3_self_reference_stack_overflow

`hayro-interpret/src/font/type3.rs` (`Type3::render_glyph`). Flagged in `IDEAS.md` while
root-causing candidate 002 (`002.pattern_self_reference_stack_overflow`): `Type3Glyph` computes
and carries forward `nesting_depth = ctx.nesting_depth() + 1` (`hayro-interpret/src/font/mod.rs:359`),
exactly like `TilingPattern` did before candidate 002's fix, but nothing in the glyph-drawing path
checked that field against `MAX_NESTED_INTERPRETATION_DEPTH` before recursing into the glyph
procedure's own content stream.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: an independently hand-crafted, minimal (~750-byte) PDF whose only content is a Type3 font (`/F1 5 0 R`) whose own `CharProcs` glyph proc for the one used glyph ("A") shows text using the *same* font object (`/F1` in the proc's inherited `/Resources` again resolves to `5 0 R`, the font currently being rendered) | dev build, no ASan needed (Rust's own `RUST_MIN_STACK` guard-page detector reports "has overflowed its stack" immediately), 3 repeat runs | Reproduces every time: `thread 'main' has overflowed its stack` / `fatal runtime error: stack overflow, aborting` (exit 134/SIGABRT), fed straight to `hayro::render`, the crate's own public entry point |
| Repeat the same input through the standalone `repro/` (release build, fresh clone at `base_commit`) via `just test` | release build, single input, one full clone-apply-reclone cycle | Reproduces identically before the patch (child process aborts, non-zero exit), and no longer reproduces after `patch.diff` is applied to the same clone (renders a 100x100 pixmap, clean exit) |

Only these two were needed - same reasoning as candidate 002: this is safe-Rust unbounded
recursion (not memory-unsafety), so `miri`/ASan add nothing a debug/release build with Rust's own
stack-overflow guard page doesn't already show; no fresh fuzz harness was needed since the bug
class and exact fix shape were already known from candidate 002, and manual construction
reproduced it on the first attempt with a clear, unambiguous signal.

## Status

**Confirmed**, reproduces **3/3 runs** for the hand-crafted `self_type3.pdf`, deterministically
(same input -> same crash every time, no layout/seed dependence - this is a real unbounded native
call-stack recursion, not a memory-corruption-dependent crash). Also reproduces via the
standalone `repro/`'s `just test` chain against a fresh clone (release build).

**Fully reproducible**: **Yes.** `repro/` calls `hayro::render` - the crate's own public
rendering entry point, the same one any real caller (a PDF viewer, a thumbnailer) would use -
directly on a real, standalone PDF file. Per `CONFIRMATION.md` section 11, a malformed file
handed to a renderer's own public entry point *is* the genuine external input; there is no second
participant (peer/caller) being impersonated here, so the single-process, single-participant
repro shape is correct, not a shortcut. (A stack overflow aborts the whole process and can't be
caught with `catch_unwind` in-process, so `repro/` re-execs itself as a child process for the
input and the parent observes whether the child survives - see `repro/src/main.rs`.)

## Class

crash (unbounded native call-stack recursion reachable from a malformed/adversarial PDF's content
stream, with no `catch_unwind` able to save the caller - stack overflow aborts the whole process;
denial-of-service in any consumer that renders untrusted PDFs, e.g. a browser extension, a
thumbnailer, a document-preview service).

## Root cause

`hayro-interpret/src/context.rs` defines `MAX_NESTED_INTERPRETATION_DEPTH = 50` with the doc
comment "Maximum nesting depth for interpreting `XObject`'s/patterns/streams" - the intent covers
Form XObjects, patterns, and other nested content streams alike, and is enforced correctly for
Form XObjects via `x_object/form.rs`'s `FormXObject::draw` (`begin_nested_interpretation`/
`end_nested_interpretation`) and (after candidate 002's fix) for tiling patterns via
`pattern.rs`'s `TilingPattern::interpret`.

`hayro-interpret/src/font/type3.rs`'s `Type3::render_glyph` never got the same wiring.
`hayro-interpret/src/font/mod.rs:359` does compute and store `nesting_depth = ctx.nesting_depth()
+ 1` on the `Type3Glyph` it builds (mirroring the field `TilingPattern`/`SoftMask` also carry
forward) - but `Type3::render_glyph` builds its nested `Context` with that stored value
(`Context::new_with(..., glyph.nesting_depth)`, `hayro-interpret/src/font/type3.rs:154`) and then
calls `interpret(iter, &resources, &mut context, device)` directly, without ever checking
`glyph.nesting_depth` against `MAX_NESTED_INTERPRETATION_DEPTH` first. So a Type3 font whose own
glyph procedure shows text using the same (or another, cyclically) Type3 font recurses with **no
depth limit** through:

```
interpret() -> draw_glyph_run() -> Type3::render_glyph() (hayro-interpret/src/font/type3.rs)
  -> interpret() -> ... (repeats)
```

Each cycle grows the native call stack until the process's stack guard page is hit and Rust's
runtime reports "thread has overflowed its stack" and aborts (`SIGABRT`) rather than a catchable
panic - the identical bug shape and consequence as candidate 002's `TilingPattern` finding, just
in the Type3-glyph-rendering path instead of the pattern-fill path.

## Introduced by

052a1c9 "First somewhat working version of Type3 glyphs" (2025-05-06) - `git log -S"fn render_glyph"` finds this as the commit that added `Type3::render_glyph` from scratch, already without the depth-check wiring `FormXObject::draw` had at the time.

## The fix

Add the same bounded-recursion check `TilingPattern::interpret` now uses (candidate 002) and
`FormXObject::draw` already used, at the start of `Type3::render_glyph`
(`hayro-interpret/src/font/type3.rs`): if `glyph.nesting_depth >= MAX_NESTED_INTERPRETATION_DEPTH`,
return `None` (skip drawing that glyph instance) instead of recursing further. See `patch.diff`
(+10/-1 lines: one import change, one early-return guard with an explanatory comment).

This was verified against the real end-to-end repro, not just the specific triggering file
in-place: `repro/`'s `just test` clones the pristine repo at `base_commit`, confirms the crash on
the unpatched clone, applies `patch.diff`, and confirms the crash is gone on the patched clone -
see `output.txt` for the full transcript of both the local scratch-harness runs (3/3 before, 3/3
after) and the standalone `just test` cycle.

- Project test suite (`cargo test --workspace --release --no-fail-fast`): **788 passed, 1286
  failed** before the patch, **788 passed, 1286 failed** after - identical pass/fail counts and
  an identical set of failing test names in both runs (diff of the two sorted failure-name lists
  is empty). All 1286 pre-existing failures are the same environment issue candidate 002 already
  documented: `panicked ... called Result::unwrap() on an Err value: Os { code: 2, kind:
  NotFound }` from `hayro-tests`'s asset-suite/reference-image tests, which need `python sync.py`
  to download external test assets not present in this environment - not exercised by
  `hayro-interpret::font::type3` at all. No test anywhere in the workspace covered Type3-glyph
  recursion depth before this patch.

## What's not done / open questions

- The same missing-guard pattern was already flagged (also from candidate 002's root-causing) for
  `hayro-interpret/src/x_object/soft_mask.rs`'s `SoftMask`, which likewise computes
  `nesting_depth = context.nesting_depth() + 1` but - as far as a grep of
  `begin_nested_interpretation`/`end_nested_interpretation` call sites shows - never checks it
  before recursing into the soft mask's own transparency-group content stream. Not checked this
  session either; it remains in `IDEAS.md`.
- No formal `cargo fuzz tmin`-style minimization was attempted on `self_type3.pdf` - it was
  already minimal by hand construction (7 objects, ~750 bytes), so minimization wasn't needed to
  demonstrate the bug isn't narrow to a specific byte sequence.
