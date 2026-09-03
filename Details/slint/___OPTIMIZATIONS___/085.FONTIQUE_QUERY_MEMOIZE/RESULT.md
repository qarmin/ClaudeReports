# FONTIQUE_QUERY_MEMOIZE

The software renderer's font-routing check (`fonts::match_font` + `uses_parley`, called
unconditionally from every `text_size`/`text_content_widths`/`char_size`/`font_metrics`/
`draw_text`/cursor-rect call site in `internal/renderers/software/lib.rs`, ~9-12 sites) builds a
full `VectorFont` (a `fontique::Collection` query plus a `skrifa` metrics parse) purely to learn
"is this a vector font", then - on the default, parley-enabled desktop path - discards it and
calls into `sharedparley`, which does its own independent `FontRequest::query_fontique` call
(`sharedparley/shaping.rs:78`'s `ranged_builder`, or `sharedparley.rs:537`/`:577` for
`char_size`/`font_metrics`) to actually shape/measure. Verified by reading `fonts.rs`,
`fonts/systemfonts.rs`, `fonts/vectorfont.rs`, `lib.rs` and `sharedparley.rs`/`shaping.rs`
directly (see the task description this candidate came from for exact line numbers). The same
redundancy also exists *within* `sharedparley` itself: shaping a multi-paragraph document with a
uniform font style calls `query_fontique` once per paragraph shaped, all with an identical
`FontRequest`, with no cache anywhere keyed on it.

## What was tried

Memoized `FontRequest::query_fontique` (`internal/core/graphics.rs`) with a 1-slot thread-local
cache: if the incoming `FontRequest` equals the immediately preceding one, return the previously
resolved `fontique::QueryFont` (or `None`) without re-running the `fontique::Collection` query.
This is the single choke point every caller (the software renderer's routing check, and every
`sharedparley` resolution site) already goes through, so no call site needed to change. Chosen
over replicating `fonts::match_font`'s bitmap-font/system-font/fallback decision tree in a
separate "will this use parley" pre-check, which would have needed to reproduce that whole
fallback chain (direct bitmap match -> system font -> any registered bitmap font by
weight/italic -> `systemfonts::fallbackfont`) to avoid a real risk of the two decisions silently
diverging - see "why the result looks the way it does" for why a 1-slot request-keyed cache
still lands most of the real saving with none of that risk.

perf_test's `scroll`/`list` scenes at their default `--size 1280x860` (~20-30 rows visible) did
not show a signal - too small a share of a frame dominated by ~700k dependency-node-registration
allocations (`scroll`) or virtualization already limiting per-frame text work (`list`). Widening
the viewport (`--size 1280x20000`) to keep ~300 rows (900 `Text` items) visible simultaneously in
`list` mode made the routing overhead large enough to measure.

| Variant | single-core p50 (`taskset`) | multi-core p50 | notes |
|---|---|---|---|
| baseline, `--mode scroll --count 100000` (default 1280x860) | 157.4-159.2 ms | 157.7-159.2 ms | no signal, dwarfed by ~700k dependency-node allocations/frame |
| patched, same scene | 157.6-159.3 ms | 157.4-159.5 ms | identical within noise |
| baseline, `--mode list --count 100000 --size 1280x20000` | 51.3-52.9 ms median (`tools/paired.sh`, `RUNS=9 FRAMES=8`) | 51.3-51.7 ms median | |
| patched, same scene | 49.6-50.4 ms median | 49.6-49.6 ms median | |

`tools/paired.sh` results (alternating runs, same scene as above), each combination run twice:

- single-core, core pinned via `taskset -c 5`: **-5.2%** (9/9 pairs), **-5.1%** (9/9 pairs)
- single-core, `taskset -c 3`: -3.7% (9/9 pairs) on one run; a repeat on that same core came back
  6/9 (not consistent) - this machine had other, unrelated `perf-optimize` sessions running
  concurrently at the time (`uptime` load average 2.6-3.6 against 24 cores), and core 3
  specifically looked contended; re-running on core 5 reproduced the clean signal both times.
- multi-core (unpinned): **-4.0%** (8/9 pairs), **-3.3%** (9/9 pairs)

Allocation counts are identical before/after in every run (17468/frame in the wide-viewport
`list` scene) - this is a pure CPU-time saving from skipping the redundant `fontique` query, not
an allocation-count change.

## Result

Real, paired-consistent win in the scene built to isolate this cost: ~3.3-5.2% faster per frame
(single-core and multi-core) when several hundred `Text` items are visible and re-measured each
frame, which is the visible-viewport analogue of what a large `ScrollView`/`ListView` full of
text does at normal window sizes scaled down. At `perf_test`'s existing default scene sizes
(~20-30 visible rows) the same removed work is real but too small a fraction of frame time to
clear the noise floor - not a null result, just not the dominant cost at that scale.

## Does it change behavior/output?

- **Bit-exact?** Yes, modulo a pre-existing harness nondeterminism unrelated to this patch: the
  last ~10 rows at the very bottom edge of the `--size 1280x20000` snapshot differ by a few
  hundred bytes between *two runs of the same unpatched baseline binary* (confirmed directly -
  see below), so that edge region is excluded from the comparison. Everything above that edge
  (76 723 827 of 76 800 000 pixel-data bytes) is byte-identical between baseline and patched.
- **Is this a breaking change?** No. `FontRequest::query_fontique` is `pub` but only reachable
  through `#[cfg(feature = "shared-fontique")]`, itself an internal, non-public-API build
  configuration; no public type or signature changes.
- Project test suite, same pass/fail counts before and after:
  - `cargo test -p i-slint-core --all-features`: 319 lib tests + 71 doctests + 1 compile-fail
    doctest, all passing, both before and after.
  - `cargo test -p slint`: every reported `test result: ok` line identical before and after
    (e.g. 24/24, 18/18, 18/18+2 ignored, etc. - no new failures in either run).

## Why the result looks the way it does

The redundant `fontique` query is real (confirmed by code reading) but only shows up once a
frame's *other* costs shrink enough for it to matter. `scroll` mode's steady-state frames are
dominated by ~700k property-dependency-registration allocations (candidates 007/008/014's
territory) that swamp anything font-related by two-plus orders of magnitude; `list` mode is
virtualized, so at the default window size only ~20-30 `Text` items pay this cost per frame at
all. Widening the viewport to keep hundreds of `Text` items on screen simultaneously (still a
realistic scenario - a maximized window, or a tablet/kiosk-sized display) is what made the signal
visible. Text mode's single large `TextInput` document does not exercise the same redundancy at
the routing level the way `list`/`scroll`'s many-small-items scenes do: the routing check
(`fonts::match_font`/`uses_parley`) fires once per method call per *item*, not once per paragraph,
so a document's paragraph count does not multiply it the way row count does in `list`/`scroll`.

The `sharedparley`-internal redundancy (one `query_fontique` call per paragraph shaped, same
`FontRequest` every time within one document) that this same cache also happens to fix was not
separately isolated in this candidate's measurements - the `text` mode scenes tried
(`--word-wrap --resize-step 40`) did not show the paragraph-reshape volume expected from
`projects/slint.md`'s notes on that path in this build; not chased further here since `list`'s
wide-viewport measurement above already demonstrates the same underlying fix has a real effect.

## What's not done / open questions

- The routing-level waste this candidate's target description centers on - `fonts::match_font`
  building and then always discarding a full `VectorFont` (the `skrifa` metrics parse plus the
  `get_swash_font_info` lookup) whenever parley is enabled - is only partially addressed: this
  patch removes the redundant *`fontique` query* underneath it, but `systemfonts::match_font`
  still unconditionally builds the (still fully cached-lookup-cheap, but non-zero) `VectorFont`
  wrapper around whatever `query_fontique` returns, every call, before the caller discards it.
  Avoiding that construction entirely would need either a cheap-and-provably-equivalent
  "will this resolve to a vector font" pre-check (rejected here as too easy to silently diverge
  from `match_font`'s real fallback chain - see "what was tried") or a larger restructuring of
  `fonts::match_font`'s return type to defer that construction. Left as a follow-up idea.
- Not measured: the `sharedparley`-internal per-paragraph redundancy this same cache fixes,
  under a scene that actually reaches many-paragraph cache misses in one frame (see "why the
  result looks the way it does").
- The cache is a single slot (last request only); a scene alternating between two or more
  distinct `FontRequest`s call-for-call would thrash it. Not measured here - `perf_test`'s
  scenes all use one uniform font style per scene.
