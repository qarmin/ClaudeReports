
# Optimization results

**Impact** (derived from the Result column, for scanning at a glance - not a recommendation, this
classifies the measured magnitude/direction only, it doesn't decide what to do about it): **Strong
win** (large, confirmed improvement) - **Small win** (real but modest improvement) - **No change**
(null result / within noise) - **Mixed** (improves one thing, regresses another) - **Small/Strong
regression** (would make things worse) - **N/A** (pure measurement/characterization, no patch
compared against a baseline).

Rows are kept sorted by the **#** column (ascending) - append in whatever order candidates get
measured, then run `just tidy` (or `just sort-summaries`) to reorder; sorting is mechanical and must
never be done by hand. The **#** is a plain number so it sorts numerically: a string sort would
render `1, 10, 11, 2`. This replaces the old "sort by impact, biggest win first" rule, which was
unmaintainable in practice - every real workspace drifted to number order or to no order at all,
because re-ranking the whole table by hand on each new row is exactly the kind of chore that gets
skipped. **Impact** is still the column you scan/filter by; it just doesn't drive row order. The
order carries no meaning beyond readability - it is not a priority ranking, see `SKILL.md`'s "the
one rule that matters most". A candidate that
combines several individually-tested fixes only appears here if it was explicitly requested and
measured as its own row (this skill tests one fix at a time by default - see `SKILL.md`).

**Candidate** and **Result** are two different lanes, not two phrasings of the same thing.
**Candidate**: the finding itself, in plain language - what's slow, or what the patch changes, and
why - written so a reader understands the problem with zero numbers (e.g. "Every tracked property
read allocates a dependency node, every frame", not a short label like "dependency node pooling").
**Result**: the technical, measured evidence - the actual numbers (timings, allocation counts, RSS,
paired-run counts), what was compared against what, and any measurement caveats. A reader should be
able to grasp the finding from Candidate alone, and judge the evidence from Result alone, without
either column repeating the other.

**Fixed by** (only for a Strong-win/Small-win/Mixed row, best-effort, checked at row-creation time
and worth re-checking on later sessions/`git pull`s): blank/empty if the underlying inefficiency is
still present in current `HEAD` (the default, ordinary case, since this workspace's own patch is
always reverted, never applied for real) - or the short commit hash + subject of the commit that
already addressed it, if a maintainer independently applied the same (or an equivalent) fix
upstream since this candidate was measured. Finding one doesn't change the Impact cell (the
original measurement was real) but means a `CORRECTION.md` per `SKILL.md`'s "When a later
candidate invalidates an earlier one's numbers" should exist alongside it, and the row should come
out of `SUMMARY-SHORT.md` if it's there - see that template.

| # | Impact | Candidate | Result | Changes behavior? | Fixed by | Link |
|---|---|---|---|---|---|---|
| 001 | Small win | Text layout runs the full Unicode grapheme-cluster iterator over every byte even for plain ASCII, where each byte is its own cluster | text_layout_uncached 155.4 us -> 142.1/143.3 us (-9%, 2 pinned criterion repeats, p<0.05); many_lines_modified -1.4%; cached path unchanged | no, bit-exact (200k random-string segmentation equivalence check, epaint 95+4 tests pass) |  | [`001.ascii_fast_path_grapheme_segmentation/RESULT.md`](001.ascii_fast_path_grapheme_segmentation/RESULT.md) |
| 002 | Small win | Computing the bounding box of every text row mesh uses NaN-aware f32::min/max (several instructions each) in a per-vertex loop | text_layout_uncached -2.0% to -3.1% (interleaved A/B, 4/4 rounds, pinned core); tessellate_text unchanged | no, bit-exact except possible 0.0 vs -0.0 tie (epaint 95+4, egui 277 tests pass) |  | [`002.mesh_calc_bounds_plain_compare/RESULT.md`](002.mesh_calc_bounds_plain_compare/RESULT.md) |
| 003 | Strong win | Every text glyph vertex goes through a closure that re-checks per-row invariant conditions (override color, color glyphs, opacity, rotation) before a trivial transform | tessellate_text -46%, demo_only_tessellate -18..22% (interleaved A/B, 4/4 rounds, pinned core) | no, bit-exact (epaint 95+4, egui 277 pass; egui_demo_lib same 2 pre-existing snapshot failures before and after) |  | [`003.tessellate_text_plain_fast_path/RESULT.md`](003.tessellate_text_plain_fast_path/RESULT.md) |
| 004 | No change | `round_ui`/`round_to_pixels`/`PointScale::round_to_pixel` call libm `roundf` (shows as 2-8% in profiles) | all four benches within -1.2%..+0.8%, inconsistent sign over 3 interleaved rounds; replacement verified bit-identical over all 2^32 f32 | no, bit-exact (exhaustive 2^32 check; emath 32 + epaint 99 tests pass) |  | [`004.fast_round_instead_of_roundf/RESULT.md`](004.fast_round_instead_of_roundf/RESULT.md) |
| 005 | No change | `get_response` and `check_for_id_clash` each take the Context lock twice per widget (separate prev-pass / options reads) | demo_no_tessellate and demo_with_tessellate__realistic ratios 0.98-1.004 over 4 interleaved rounds, mixed signs | no, bit-exact (egui 277 tests pass) |  | [`005.fewer_context_locks_per_widget/RESULT.md`](005.fewer_context_locks_per_widget/RESULT.md) |
| 006 | Small win | `Color32` hashes as a length-prefixed byte slice (2 slow ahash writes) although it is one u32; hashed up to 4x per `TextFormat` on every label layout/cache lookup | text_layout_cached -2.0..-3.2% (9/10 interleaved rounds); demo_* and text_layout_uncached within noise | no for output; `Hash` values of `Color32` change (egui 277, ecolor+epaint tests pass) |  | [`006.color32_hash_single_u32_write/RESULT.md`](006.color32_hash_single_u32_write/RESULT.md) |
| 007 | Small win | For every character of laid-out text, the font fallback chain is resolved via a hash-map probe and then a second map lookup checks the face exists, even when the character just continues the previous run | text_layout_uncached -2.4..-3.1% (5/5 interleaved rounds); demo_no_tessellate within noise | no, bit-exact (epaint 95+4, egui 277 tests pass) |  | [`007.ascii_face_table_and_run_merge_before_face_lookup/RESULT.md`](007.ascii_face_table_and_run_merge_before_face_lookup/RESULT.md) |
| 008 | Mixed | UV-colored strokes collect all path positions into a temporary Vec just to compute their bounding box | 3-point UV strokes -2..-16% (noisy); 1000-point thin UV stroke +8% slower (4/4 rounds, also with A/B order swapped), thick +1% | no, bit-exact (epaint 95+4 tests pass) |  | [`008.uv_stroke_bbox_without_vec/RESULT.md`](008.uv_stroke_bbox_without_vec/RESULT.md) |
| 009 | Small win | The per-paragraph `Glyph` vec (72 B/glyph) is grown by push through 6 doubling reallocs while laying out a long text | text_layout_uncached allocated bytes 468,912 -> 344,352 (-26.6%), allocations 128 -> 113 per layout; time -1.3% (5/5 interleaved rounds); peak live unchanged | no, bit-exact (epaint 95+4 tests pass) |  | [`009.reserve_paragraph_glyphs/RESULT.md`](009.reserve_paragraph_glyphs/RESULT.md) |
| 010 | Mixed | Every tessellated clip-rect mesh starts empty and regrows by doubling (66 reallocs >= 2 KB per demo frame) | fixed 1024-vertex initial capacity: demo_only_tessellate -4% time and -74 allocations, but +254 KB allocated and +365 KB peak live per frame | no, bit-exact (epaint 95+4, egui 277 pass) |  | [`010.tessellator_new_mesh_initial_capacity/RESULT.md`](010.tessellator_new_mesh_initial_capacity/RESULT.md) |
| 011 | No change | Meshes grow by doubling during tessellation; size them up front from text/mesh shape sizes of each clip-rect run | time within noise (4 rounds); allocations 157 -> 121 per tessellation but allocated bytes +4.7% and peak live +107 KB | no, bit-exact (epaint 95+4, egui 277 pass) |  | [`011.tessellator_sized_mesh_hints/RESULT.md`](011.tessellator_sized_mesh_hints/RESULT.md) |
| 012 | Strong win | The galley-cache key hash of every label layout does ~35 separate tiny hasher writes (each float, bool, enum, color), costing a fixed-cost mix per write | text_layout_cached -11% (6/6 interleaved rounds); whole demo UI build (demo_no_tessellate) -3.5% (6/6); allocations unchanged | no for output; Hash values of LayoutJob/TextFormat/TextWrapping/Stroke change (epaint 95+4, egui 277 tests pass) |  | [`012.pack_layoutjob_hash_writes/RESULT.md`](012.pack_layoutjob_hash_writes/RESULT.md) |
| 013 | No change | `FontId` hash does a float write and an enum write for every text section | demo_no_tessellate -0.3..-1.3%, text_layout_cached -0.5% (5 interleaved rounds, within noise) | no for output; FontId Hash values change (epaint 95+4 pass) |  | [`013.fontid_hash_single_write/RESULT.md`](013.fontid_hash_single_write/RESULT.md) |
| 014 | Small win | The per-glyph atlas cache key is hashed from six separate fields (two of them constant for the whole run) | text_layout_uncached -1.4..-2.1% (6/6 interleaved rounds) | no, bit-exact (epaint 95+4 tests pass) |  | [`014.outline_glyph_key_packed_hash/RESULT.md`](014.outline_glyph_key_packed_hash/RESULT.md) |
| 015 | Strong win | Every shaped text run compiles a fresh harfrust shape plan (OpenType feature map) although it depends only on face, script, direction, language and variations | scene_changing_labels_40 -11.7% (4/4 interleaved rounds), text_layout_uncached -2.5%, many_lines_modified -1% | no, bit-exact (layout hash identical over Latin/Arabic/CJK/emoji; epaint 95+4, egui 277 tests pass) |  | [`015.harfrust_shape_plan_cache/RESULT.md`](015.harfrust_shape_plan_cache/RESULT.md) |
| 016 | Small win | Every path point normal in the tessellator calls the out-of-line libm `hypotf` through `Vec2::length` | demo_only_tessellate -2.3..-3.7%, scene_widget_mix_100 / scene_buttons_300 -2..-3% (interleaved A/B); 0 mismatches vs f32::hypot in 400M sampled pairs | no in all sampled cases (not proven bit-identical on every libm) (emath/epaint/egui tests pass) |  | [`016.vec2_length_inline_sqrt/RESULT.md`](016.vec2_length_inline_sqrt/RESULT.md) |
| 017 | Strong win | Each tessellated mesh is regrown by doubling every frame although its size is almost the same as in the previous frame | demo_only_tessellate -7%, scene_painter_shapes_2000 -15%, buttons_300 -9%, widget_mix -4%, demo frame -2.3%; allocations 157 -> 99 and bytes -21% per tessellation, in-tessellation peak +11% (interleaved A/B + alloc harness) | no for output; additive public API (epaint Tessellator hints) (epaint 95+4, egui 284 tests pass) |  | [`017.previous_frame_mesh_size_hints/RESULT.md`](017.previous_frame_mesh_size_hints/RESULT.md) |
| 018 | Small win | The list of all shapes of a frame is built from an empty Vec with repeated appends (doubling realloc chain of 96-byte `ClippedShape`s every pass) | demo_run_ui allocated bytes -10% (379,224 -> 341,208) and peak live -23% per pass; time unchanged (interleaved A/B, within noise) | no, bit-exact (egui 277 tests pass) |  | [`018.drain_layers_exact_capacity/RESULT.md`](018.drain_layers_exact_capacity/RESULT.md) |
| 019 | No change | Stroke loops push each vertex and triangle with separate `Vec` calls | scene_polylines/painter_shapes/widget_mix and demo_only_tessellate within noise (4 interleaved rounds) | no, bit-exact (epaint 95+4 tests pass) |  | [`019.tessellator_batched_pushes/RESULT.md`](019.tessellator_batched_pushes/RESULT.md) |
| 020 | Small win | A widget registration takes the `Context` lock 5-7 times (focus bookkeeping, widget insert, id clash, highlight, response update, scroll requests) although all work on the same state | 21 -> 15 lock acquisitions per button; widget-heavy scenes -2..-4% (table up to -7%), demo UI build -1.2% (interleaved A/B) | no (same operation order; id-clash error painting now after the response computation) (egui 277 tests pass) |  | [`020.create_widget_single_lock/RESULT.md`](020.create_widget_single_lock/RESULT.md) |
| 021 | Strong win | Every rounded rectangle recomputes its outline normals (sqrt and divides per point) although they depend only on the corner shape | filled round rects -38%, stroked -27..28%, 300 buttons -16.5%, widget mix -8%, windows -7% (3/3 interleaved rounds) | yes, characterized: normals differ from per-rect computation by <=6.8e-5 typical (3.2e-4 extreme), i.e. <1e-4 px vertex shifts; topology/colors identical (epaint+egui tests pass) |  | [`021.rounded_rect_precomputed_normals/RESULT.md`](021.rounded_rect_precomputed_normals/RESULT.md) |
