# Cache compiled `harfrust` shape plans per font face

`shape_text` built a `harfrust::Shaper` for every text run and let `Shaper::shape` compile a fresh `ShapePlan` (OpenType feature map: `hb_ot_map_builder_t::compile`, `LangSys::feature_index_for_tag`, `LayoutTable::langsys`) every time. In the new `scenes` harness (`harness/scenes`, `scene_changing_labels_40`: 40 labels whose text changes each frame, i.e. one short uncached layout per label) these were ~14% of the profile (6.8% + 5.0% + 2.6%). The plan only depends on (face, direction, script, language, variation coordinates), so `FontFace` now keeps a `Mutex<HashMap<ShapePlanKey, Arc<ShapePlan>>>` (capped at 64 entries, cleared when exceeded so animated variable-font axes can't grow it forever) and `shape_text` passes the cached plan via `ShapeOptions::plan`.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1, harfrust 0.12.0. Metric: wall-clock time (allocation counts not separately measured).

## What was tried

Interleaved A/B of prebuilt binaries (`tools/ab.sh`, pinned core, 4 rounds), ratios patched/base:

| Benchmark | ratios |
|---|---|
| scene_changing_labels_40 (scenes harness, mimalloc) | 0.8803, 0.8820, 0.8829, 0.8859 |
| text_layout_uncached (900-char text) | 0.9727, 0.9743, 0.9711, 0.9858 |
| text_layout_cached_many_lines_modified | 0.9920, 0.9814, 0.9929, 0.9914 |

## Result

Time: about -11.7% for re-laying out short changing labels, -2.5% for one long uncached layout, about -1% for the many-lines edit case (4/4 rounds each). Cached layouts are unaffected.

## Does it change behavior/output?

- **Bit-exact?** Yes. `alloc_count` harness `SCENE=layouthash` hashes glyph positions, advances, uv rects and galley rects for Latin (kerning/ligature strings), Arabic (RTL), CJK/Japanese/Korean, emoji sequences/flags/combining marks, tabs/CRLF, accented text, at ppp 1/1.5/2 and sizes 10/14/33.3: identical hash (`34553a3c6727b927`) before and after.
- Breaking change: no (private types only). Memory: one `ShapePlan` per (script, direction, language, variation set) per face, bounded at 64.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored); `cargo test -p egui` 86 + 3 + 188 passed; same as unpatched.
- Real change to the tested project's source: `patch.diff` (+58/-5).

## Why the result looks the way it does

Plan compilation costs microseconds per call and was repeated for every run; for a short label it is a sizable share of the whole layout, for a 900-char paragraph (one run) it is amortized.

## What's not done / open questions

- User-requested OpenType features are not supported by egui today (`&[]` passed), so the key has no feature list; adding feature support would need to extend `ShapePlanKey`.
- `Shaper` itself (and the `ShaperInstance` for variable fonts) is still rebuilt per run.
