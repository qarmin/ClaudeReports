# ASCII fast path in `segment_into_runs` (skip unicode-segmentation for lone ASCII bytes)

`text_layout::segment_into_runs` ran `str::grapheme_indices` over every section text; profiling `text_layout_uncached` showed `unicode_segmentation::Graphemes::next` at ~9% self time. An ASCII byte followed by another ASCII byte (or end of text) is always its own grapheme cluster, except `\r\n`, so the iterator is only needed for the other cases.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1 (egui 0.37.0). Metric: wall-clock time only (no RSS/alloc measurement - no allocation is added or removed).

## What was tried

Criterion `egui_demo_lib` bench, pinned `taskset -c 4`, `--warm-up-time 1 --measurement-time 4`, compared with `--baseline base` (saved from unpatched tree), 2 repeats. Multi-core (unpinned) not run: the code is single-threaded text layout.

| Variant | text_layout_uncached | text_layout_cached | text_layout_cached_many_lines_modified |
|---|---|---|---|
| baseline | 155.4 us | 159.4 ns | 265.8 us |
| patched | 142.1 us / 143.3 us | 160.7 ns / 162.8 ns | 261.8 us / 262.5 us |
| delta | -9.1% / -8.7% | +0.8% / +2.1% (cached path does not reach the patched code; layout-independent noise level) | -1.4% / -1.3% |

## Result

Time: about -9% on uncached layout of the long lorem-ipsum text (two repeats agree, p < 0.05, run-to-run spread ~0.5%), about -1.4% on the many-lines-modified case (most lines served from cache). The cached lookup path never calls the changed function; its +1-2% is not explained by the patch and is treated as noise/code-layout.

## Does it change behavior/output?

- **Bit-exact?** yes. The segmentation is equivalent: a throwaway test (not kept) compared the new loop against `grapheme_indices` on 200,000 random strings built from `\r \n a space e-acute U+0301 regional indicators ZWJ emoji tab Hangul jamo` and all matched.
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features`: 95 + 4 passed (1 ignored) before, same after. (`cargo test -p epaint` without `--all-features` fails to compile at HEAD - `unique_test_key` is behind `default_fonts` - pre-existing.) `cargo test -p egui` after: 86 + 3 + 188 passed, 0 failed; unpatched baseline: identical counts (86 + 3 + 188).
- Real fix to the tested project's source: `patch.diff` (+15/-2).

## Why the result looks the way it does

Per-cluster work is dominated by the grapheme state machine (table lookups per char) in the typical all-ASCII case; the rest of the per-cluster work (face cache hash lookup, `rasterize_cluster` early-out) is unchanged and still runs once per cluster, so rasterizer semantics are untouched.

## What's not done / open questions

- Per-cluster `resolve_cluster_face` hash lookup and `rasterize_cluster` call could additionally be batched for ASCII runs (separate candidate).
