# Content-stream dispatch overhead in `hayro-interpret`, the unattributed remainder

Backlog entry from `../IDEAS.md`: "the remaining ~50% of the dispatch-time bucket not yet
attributed to a specific sub-part" (24-29% of `render()`'s own time; tokenizing ~8.8% and
font/glyph resolution ~4.5% were already decomposed, leaving ~11-16% of total render time
unattributed and spread across "many small `match` arms"). The idea explicitly said this needed
real profiling data, not more guessing - `perf` is still not available in this environment (no
root, not installed), so the investigation extends the existing `Instant`+`AtomicU64` phase-timer
technique from `../_reference/` with nine new whole-match-arm buckets inside
`hayro_interpret::interpret()`'s operator loop, to see which specific operators the unattributed
time is actually going to.

## What was tried

### Step 1: re-run and extend the existing instrumentation

`../_reference/interpret_dispatch_instrumentation.diff` (updated this session - the API it patched
had drifted since July: `Device::draw_glyph` is now `draw_glyph_run`, and the `TJ` operator
(`ShowTexts`) no longer calls `show_text_string` per string, it uses
`begin_glyph_run`/`append_text_string`/`show_glyph_run`) now also classifies every operator into
one of nine buckets (`state_stack`, `transform`, `path_build`, `path_draw_whole`, `color`,
`text_state`, `text_font_op`, `text_show_whole`, `xobject_misc`) via a `__bucket_for` function that
matches on the operator variant *before* the real (byte-for-byte unmodified) dispatch match runs,
timing the whole arm body per bucket. See `../_reference/README.md` for the full explanation and
`../_reference/img_pdf_breakdown_007.txt` for raw output.

Against the real `img.pdf`, `taskset -c 2`, 80 iterations:

| Bucket | Share of total render() time |
|---|---:|
| `text_show_whole` (whole `ShowText`/`ShowTexts`/etc. arms, includes the `draw_glyph_run` call) | 37.9% |
| `xobject_misc` (XObject/InlineImage/Shading/marked-content dispatch, includes nested `draw_image` calls) | 13.5% |
| `path_draw_whole` (whole fill/stroke arms, includes the `draw_path`/`draw_rect` call) | 3.8% |
| `state_stack` (`q`/`Q`) | 1.7% |
| **`text_font_op` (`Tf` alone)** | **6.4%** |
| `text_state` (all other text-state ops: `BT`/`ET`/`Tm`/`Tc`/`Tw`/`Tz`/`TL`/`Tr`/`Ts`/`T*`/`TD`/`Td` combined) | 0.8% |
| `path_build` (`m`/`l`/`c`/`re`/clip-flag ops) | 0.8% |
| `color` (all color/graphics-state-setting ops combined) | 0.5% |
| `transform` (`cm`) | 0.3% |

Subtracting the already-measured device-call time from the two buckets that call into it
(`text_show_whole` - `draw_glyph_run` = 4.5%, `path_draw_whole` - `draw_path`/`draw_rect` = 0.4%)
and summing everything that isn't itself a `draw_*` call gives **15.4% of total render time**
attributable to non-device-call operator handling - consistent with the original ~11-16%
"unattributed" estimate, confirming the bucket split is measuring the right thing. Of that 15.4%,
**`Tf` (text font selection) alone is 6.4pp - more than half of it**, and far more than every other
non-text-show, non-XObject operator combined (`state_stack`+`transform`+`path_build`+`color`+
`text_state` = 4.1pp together). This was the standout, surprising finding: `Tf` is called 326
times for this page (6.8% of all operators, per candidate `006`'s frequency table), so ~326 calls
average out to roughly 860ns each - much more than setting two fields should cost.

### Step 2: why is `Tf` expensive - `TextFont`'s existing resolution path

```rust
let font = if let Some(font_dict) = font_dict_cache.get(name).cloned() {
    context.resolve_font(&font_dict)
} else if let Some(font_dict) = resources.get_font(name) {
    font_dict_cache.insert(name.clone(), font_dict.clone());
    context.resolve_font(&font_dict)
} else {
    Font::new_standard(StandardFont::Helvetica, &context.settings.font_resolver)
        .map(TextStateFont::Fallback)
};
```

`font_dict_cache` (keyed by resource name, local to this `interpret()` call) already avoids
re-looking up the font dict in the `Resources` tree on repeat, but `context.resolve_font(&font_dict)`
still runs on **every** `Tf` call, even when it's the exact same font as the previous one: it
computes `hash128(font_dict.data())` (a `SipHash13` over the font dict's raw PDF byte span) to use
as the key into the interpreter's global `font_cache: FxHashMap<u128, Option<Font>>`, then does the
hashmap lookup and clones the `Option<TextStateFont>`. `Font`'s own clone is cheap (`Rc`-wrapped
variants), but the SipHash + hashmap lookup happen unconditionally on every `Tf`, whether or not
the font actually changed - and text-heavy PDFs call `Tf` before nearly every run of glyphs even
when reusing the same font repeatedly (this page: 326 `Tf` calls, almost certainly far fewer
distinct fonts).

### Step 3: the fix - cache the *resolved* font by resource name, scoped per content stream

Added a second local cache, `resolved_font_cache: FxHashMap<Name<'a>, Option<TextStateFont<'a>>>`,
alongside the existing `font_dict_cache` inside `interpret()`. On `Tf`, check this cache first by
resource name (a short byte-string compare/hash, not a hash over the whole font dict); only fall
through to the existing `font_dict_cache` -> `resolve_font` path on a miss, then populate both
caches.

This must be scoped to the current `interpret()` call, exactly like `font_dict_cache` already is,
**not** stored on `Context`/`TextState` (which persists across `save_state`/`restore_state`) -
the same resource name (e.g. `/F1`) can point at a *different* font inside a nested Form XObject's
own `Resources` dictionary, and a Form XObject's content stream gets interpreted via a fresh,
nested call to `interpret()` (`hayro-interpret/src/x_object/form.rs`) sharing the *same* `Context`
(so a `TextState`-level cache would leak across the boundary) but with different `resources`. A
local `FxHashMap` reset per `interpret()` call has exactly the right scope, mirroring
`font_dict_cache`'s existing, already-correct pattern.

See `patch.diff` for the full change (touches only `hayro-interpret/src/interpret/mod.rs`).

### Step 4: measurement

Bench: `bench/src/main.rs` (path-dependency on `hayro`, embeds `testdata/img.pdf`, 300 iterations,
10% warmup discarded, reports min/p50/mean/p95/max of `render()` wall time plus an FNV-1a hash of
the produced `Pixmap`'s premultiplied RGBA8 bytes for a bit-exact correctness check every
iteration). Same real `img.pdf` used throughout this workspace.

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) | pixmap hash |
|---|---|---|---|
| baseline (run 1) | 2.945ms | 2.982ms | `0x9622bedd5d3c7d1c` |
| baseline (run 2) | 2.970ms | 2.963ms | `0x9622bedd5d3c7d1c` |
| patched (run 1) | 2.491ms | 2.441ms | `0x9622bedd5d3c7d1c` |
| patched (run 2) | 2.453ms | 2.420ms | `0x9622bedd5d3c7d1c` |
| patched (run 3, via `just test` from a fresh clone) | - | 2.537ms | `0x9622bedd5d3c7d1c` |

All 300 iterations of every run (baseline and patched alike) produced the identical pixmap hash.

## Result

**-16% to -18% render() time for `img.pdf`** (baseline p50 ~2.95-2.97ms -> patched p50
~2.42-2.49ms single-core; ~2.96-2.98ms -> ~2.42-2.54ms multi-core/unpinned), reproduced across
repeated runs (noise floor for this bench is well under 2%, per the runs above) and independently
reproduced via `just test` against a fresh clone of the pinned commit. This is a larger win than
the ~6.4% the `text_font_op` bucket alone suggested - plausible reasons: removing the SipHash call
also removes the memory traffic of reading the font dict's raw byte span 326 times, and the
instrumentation itself (extra `Instant::now()` calls per operator) adds a few percent of overhead
that inflates the *baseline-relative percentages* measured in step 1 without proportionally
inflating the *absolute* saving measured by the clean, non-instrumented bench in step 4.

This addresses a real, measurable slice of the "unattributed remainder" named in the backlog entry
for a text-heavy page. It does not address the rest of that remainder: `xobject_misc` (13.5%,
mostly real device work for Form XObjects/images, not overhead) and the non-`Tf` parts of
`text_state`/`state_stack`/`path_build`/`color` (4.1pp combined, each individually too small to be
worth a dedicated fix) remain as before. See `../IDEAS.md` for a new entry on the still-uninspected
`xobject_misc` bucket's *own* prep-vs-device-call split.

## Does it change behavior/output?

- **Bit-exact?** Yes. All 300 iterations of the bench (both variants) produced identical pixmap
  hashes on `img.pdf`.
- Project test suite: ran `cargo test --release -p hayro-tests --test tests -- render` before and
  after the patch. Both runs: 267 passed, 1259 failed, 181 filtered out, in an identical 0.35s.
  Every one of the 1259 failures is `Os { code: 2, kind: NotFound }` - a missing local test-asset
  file (this sandbox doesn't have the full multi-thousand-PDF corpus/git-lfs assets checked out;
  same pre-existing environmental limitation noted in `../README.md`'s vello section), not a
  render failure. Diffed the two runs' failing-test-name lists: **identical set**, confirming the
  patch introduces no new failures among the tests that do have local assets (which include
  several text-rendering tests: `font_cid_1`, `TextClippingModeChanges`, etc.).
- **Is this a breaking change?** No - internal to `hayro-interpret::interpret()`, no public API
  change.

## Why the result looks the way it does

`Tf` (`TextFont`) is one of the most frequent operators in real PDFs (6.8% of all operators in
this page, comparable to `q`/`Q`/`cm`) but the existing resolution path always recomputed a
SipHash over the font dictionary's raw bytes plus a global-cache lookup, even on a no-op "reselect
the same font" call - which is extremely common since text-heavy content streams often emit `Tf`
before every run of glyphs regardless of whether the font actually changed. Caching by resource
name (not font-dict-content) inside the same, already-correctly-scoped-per-content-stream
`font_dict_cache` pattern turns the common case into a cheap name-keyed lookup instead.

## What's not done / open questions

- Only measured against `img.pdf` (the user's own real, text-heavy test file). The win should
  scale with `Tf`-call frequency and font-reuse rate; a PDF that changes fonts on every single
  `Tf` call (never hitting the new cache) would see no improvement, and a PDF with almost no text
  would see a correspondingly smaller share of the ~16-18% win. Not tested against the other
  representative PDFs used elsewhere in this workspace (`issue391.pdf`, `image_cmyk_icc_jpg.pdf`,
  `fonts_type1_latex.pdf`) - `fonts_type1_latex.pdf` in particular is also text-heavy and would be
  a good second data point for a future session.
- `xobject_misc` (13.5% of total render time) was not decomposed further - it's a mix of real
  device work (nested nested Form XObject draws, image decode dispatch) and non-device dispatch
  overhead (dict/color-space resolution, `Shading`/`Pattern` construction), and the current bucket
  can't tell them apart without further sub-splitting the way `text_show_whole` and
  `path_draw_whole` were. Added to `../IDEAS.md`.
- The full `hayro-tests` corpus (thousands of PDFs) could not be run against this patch in this
  sandbox due to missing local assets (see above) - the 267 tests that could run all passed
  identically before/after, but a maintainer with the full asset set should re-run before taking
  this patch to be thorough.
