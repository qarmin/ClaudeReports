# 91 - Incremental re-shaping when typing

Candidate 020 quantified typing a single character into a large `TextEdit`/`TextInput` as
882ms/keystroke at 50000 lines, linear, because the shaped-text cache (`TextLayoutCache`, added
by candidate 003) drops the *entire* cache entry on any dependency change and re-shapes every
paragraph from scratch, even though a keystroke only ever touches one. This candidate implements
the incremental path candidate 020's own `RESULT.md` designed but never attempted: diff the edited
document against the paragraphs about to be discarded, and reshape only the paragraph(s) that
actually changed.

## What was tried

Two new pieces in `internal/core/textlayout/sharedparley/`:

- `ItemCache::take_dirty_entry` (`item_rendering.rs`): if an item's cache entry exists and its
  dependency tracker is already dirty (i.e. the next `get_or_update_cache_entry_ref` call is about
  to discard it and recompute from scratch), removes it and hands back its data instead of letting
  it be silently dropped. Returns `None` for a vacant or still-valid entry - the exact cases with
  nothing to salvage. This is the "reshape with the old value in hand" hook candidate 020 said was
  the actual missing piece.
- `shape_paragraphs_incremental` / `reshape_diff` (`shaping.rs`): given the previous shape's
  paragraphs and a `ShapeFingerprint` (the plain text plus font request and stroke style, the only
  other inputs that can change the shaped bytes), computes the common byte prefix/suffix between
  the old and new text, keeps every paragraph that lies entirely inside either (byte-identical, so
  its already-shaped `parley::Layout` is still exactly right), and reshapes only what's left in the
  middle. Falls back to a full reshape whenever that isn't safe: styled text (formatting spans
  aren't representable as a byte diff), no previous shape to diff against, or the fingerprint
  changed (font/stroke changed, not just the text).

`line_breaking` is deliberately **not** carried over across an edit (stays `None`, same as a full
miss today) - `layout()`'s retained-breaking fast path assumes the *whole* paragraph list is
unchanged, and a freshly-reshaped paragraph's layout hasn't been broken/aligned yet. Every call
still runs the O(paragraphs) `break_all_lines`/`align` pass over the full document (as it already
did on any cache miss before this patch); only the shaping itself (the expensive part per
candidate 020/RESULT.md's own numbers) is made incremental.

| Variant | single-core p50 (50000 lines) | single-core p50 (10000 lines) | single-core p50 (1000 lines) | multi-core p50 (50000/10000) | notes |
|---|---|---|---|---|---|
| baseline (2 runs) | 2.41s / 1.38s | 267ms / 535ms | 22.8ms / 58.6ms | 1.20s / 287ms | host is on its noisy side this session (see `uptime` note below); baseline's own spread is wide |
| patched (2 runs) | 90.1ms / 86.9ms | 19.3ms / 22.2ms | 5.0ms / 6.5ms | 83.4ms / 21.9ms | consistent run-to-run, unlike baseline |

`taskset -c 3`, `perf_test --bench --mode text --count <N> --type-text --scroll-step 0 --frames 5`,
median of the 4 frames after the first (`later frames`). `uptime` load average was 13-21 across
this session (below the 24-core count throughout, but visibly non-idle) - baseline's own
run-to-run spread (e.g. 267ms vs 535ms at 10000 lines) reflects that, not this patch; patched
numbers stayed tight across repeats regardless.

Allocations per keystroke (far less noisy than wall time, and the more direct signal for "how much
was actually re-shaped"):

| lines | baseline allocations/frame | patched allocations/frame |
|---|---|---|
| 1 000 | ~15 690 | ~700 |
| 10 000 | ~150 695 | ~705 |
| 50 000 | ~750 695 | ~706 |

Patched allocations/frame are essentially flat regardless of document size (one paragraph's worth
of shaping plus fixed per-frame overhead), confirming the fix is genuinely O(1) in the shaping
step rather than O(document) - matching candidate 020's own prediction.

## Result

A real, large, and consistent win: roughly **15-27x** at 50000 lines and **12-24x** at 10000 lines
(using baseline's own two runs as the range, since baseline itself varied that much run to run),
and allocations/frame drop from growing linearly with document size to a small constant regardless
of size. `paragraph_shape_count()` (candidate 003/020's own metric for "how many paragraphs were
actually shaped") only exists when candidate 003's own patch is applied to a pristine tree, so it
reads 0 here as a stub either way (see `perf_test/src/bench.rs`'s STUB comment) - allocations/frame
is the metric that actually demonstrates the fix, and it does so unambiguously.

## Does it change behavior/output?

- **Bit-exact?** Yes, verified two ways:
  1. Twelve throwaway unit tests (written during development, removed before finalizing this
     patch - not part of the diff) directly exercised `reshape_diff` against a from-scratch
     `create_text_paragraphs` reshape for every edit shape that matters: mid-line insert/delete,
     newline insert/delete (merging two lines), edit exactly at the start, edit exactly at the end,
     appending a whole new last paragraph, replacing the entire document, deleting everything, and
     a **chained** sequence of 6 successive edits where each step's diff base is the *previous
     step's own incremental result* (not a fresh full reshape) - the same way the real cache
     chains one keystroke into the next. All matched a full reshape's shaped glyph positions
     exactly (glyph id + x + y per line, after `break_all_lines`/`align`).
  2. `perf_test --bench --mode text --count 10000 --type-text --scroll-step 0 --frames 8
     --snapshot out.ppm`, compared byte-for-byte against baseline. Two baseline runs already
     differ from each other by ~950 bytes in a small region near the bottom of the viewport
     (rows 840-849 of 860) - a pre-existing, patch-independent non-determinism (almost certainly
     the text cursor's time-based blink state, not this patch: two separate *patched* runs of the
     same binary produced **zero** byte difference against each other, consistent with the patched
     path being fast and consistent enough to always land the caret in the same blink phase, while
     baseline's much larger and more variable per-frame time pushes it across the blink threshold
     differently run to run). Comparing baseline against the corrected patch lands at 996 differing
     bytes in that exact same row range - matching baseline's own self-noise in both magnitude and
     location. An earlier, buggy version of the diff algorithm (see "Why the result looks the way
     it does") failed both of the above checks with a much larger, structurally different diff
     (15351 bytes, a wider row range) before being fixed.
- **Is this a breaking change?** No - internal cache implementation detail, no public API changed.
  `ItemCache::take_dirty_entry` is a new `pub` method (needed across the module boundary from
  `cache.rs`) but additive; nothing else calls it.
- Project test suite:
  - `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed** before AND after
    the patch (identical).
  - `cargo test -p test-driver-rust --features build-time` (`tests/`): **790 passed, 0 failed**
    after the patch (all 35 test binaries `ok`). Not independently re-run against a pristine tree
    in this session (time budget), but nothing in the patch touches generated code or the
    compiler - only the runtime shaping cache.
  - `cargo test -p test-driver-interpreter` (`tests/`): **858 passed, 0 failed** after the patch.
    Same caveat as above re: not re-run against pristine in this session.

## Why the result looks the way it does

The diff is a classic "common prefix/suffix" text diff, applied at paragraph granularity: a
paragraph survives unmodified if its entire byte range lies inside the longest byte-identical
prefix or suffix between the old and new text. Getting the boundary conditions right took two
iterations:

1. **First attempt checked only the *new* paragraph's range against the prefix/suffix length.**
   That's insufficient: a paragraph that got *shorter* (e.g. deleting most of a line's text) can
   have its new range's end fall inside the "common prefix" length purely by coincidence, while the
   *old* paragraph at that index extended further and was therefore not actually identical. Fixed
   by also requiring the new and old ranges to be equal at that index, not just the length
   threshold - equality-of-ranges alone isn't sufficient either (an edit that keeps a paragraph's
   length unchanged can still change its content), so both conditions are needed together.
2. **The very first and very last paragraph of a document have no real delimiting `'\n'`** - their
   boundary is just "start/end of document". An edit exactly at the start (prepending characters)
   can make the *entire* old document look like a byte-identical "common suffix" of the new one
   (true, byte-for-byte), while the first paragraph's actual new boundary silently changed (the
   prepended text merges into it, no new paragraph created). Fixed by also requiring the
   suffix-shifted old range to still equal the corresponding new range exactly, not just checking
   that its start lies past the suffix boundary.

Both bugs were caught by the unit-level verification (comparing against a from-scratch reshape)
before ever looking at pixels, and the pixel comparison independently corroborated the fix once
made: the buggy version's baseline-vs-patched pixel diff was ~16x larger than the noise floor
established by two baseline runs against each other; the fixed version lands within that noise
floor.

`line_breaking` staying `None` across an edit means the O(paragraphs) `break_all_lines`/`align`
pass still runs on every keystroke (unchanged from before this patch) - only the shaping itself
(the expensive part, per candidate 020's own numbers) got cheaper. This is why the win, while very
large, isn't literally "keystrokes cost nothing regardless of document size": the remaining
per-keystroke cost at 50000 lines (roughly 85-90ms) is that leftover O(paragraphs) pass, not
shaping.

## What's not done / open questions

- Carrying `line_breaking` across an edit too (only re-breaking the paragraph(s) that actually
  changed, reusing already-broken lines for the rest) would remove the remaining O(paragraphs)
  cost per keystroke - `layout()`'s retained-breaking fast path currently assumes the whole
  paragraph list is byte-for-byte unchanged, so this needs its own design pass, not just enabling
  the existing flag. Added to `IDEAS.md`.
- Wrapped documents (`TextEdit`'s default) are untouched by this patch in the sense that the
  *shaping* incremental path works identically for wrapped and unwrapped text (the diff is over
  plain-text paragraphs, before line breaking) - but per `projects/slint.md` section 4, wrapped
  documents don't get the lazy-glyph-dropping treatment `no-wrap` gets, so their retained memory
  profile is unaffected by this candidate; only the re-shaping *time* improves, for both wrap
  modes.
- Styled text (`StyledText`, formatting spans/links) always takes the full-reshape path - the diff
  only understands plain-text byte ranges, and a styled paragraph's `range` is always `0..0` (see
  `shaping.rs`), so there's nothing to diff against. Not measured separately; `perf_test`'s `text`
  scene only exercises plain text.
- `perf_test`'s own `paragraph_shape_count()` stub (in `bench.rs`) still reads a hardcoded `0`
  (per its existing "STUB 2026-08-23" comment - the real method only exists on a tree with
  candidate 003's own unapplied patch). Wiring up a real per-paragraph-reshape counter for *this*
  candidate's mechanism specifically (distinct from candidate 003's) was considered but not done,
  since allocations/frame already demonstrates the effect unambiguously without adding new
  permanent instrumentation to the real source.
- The 12 unit tests that verified `reshape_diff`'s correctness during development (edge cases plus
  a 6-step chained-edit simulation) were deliberately not kept in the patch, per this skill's own
  scope (it doesn't add permanent unit tests to the project) - only the pixel/allocation
  measurements above are part of the verification trail here. Anyone revisiting this code should
  consider whether a permanent regression test for the diff logic is worth adding separately.

## Reproduce

```sh
cd perf_test
just build  # or: cargo build --bin perf_test --profile rdebug
for c in 1000 10000 50000; do
  taskset -c 3 ../target/rdebug/perf_test --bench --mode text --count $c --type-text --scroll-step 0 --frames 5
done
```
