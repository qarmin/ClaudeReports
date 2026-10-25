# egui performance workspace

Session 2026-10-18 (Europe/Warsaw), base commit `86099aca1` (egui 0.37.0), rustc 1.95.0, Linux, 24 cores. Results ledger: `SUMMARY.md` (all candidates), `SUMMARY-SHORT.md` (wins only), backlog: `IDEAS.md`.

## Harness

No new harness was written: the existing criterion benches are used as they are.

- `cargo bench -p egui_demo_lib --bench benchmark` - end-to-end demo frame (`demo_with_tessellate__realistic`, `demo_no_tessellate`, `demo_only_tessellate`), `text_layout_uncached`, `text_layout_cached`, `text_layout_cached_many_lines_modified`, `tessellate_text`.
- `cargo bench -p epaint --bench benchmark` - tessellator micro-benchmarks (circles, strokes, dashed lines).

## Allocation / memory harness (session 2)

`harness/alloc_count/` is a standalone cargo project (own `[workspace]`, path deps on the repo's crates) with a counting global allocator: `cd harness/alloc_count && cargo run --release` prints allocations/iter, bytes/iter and peak-live delta for `demo_run_ui`, `demo_run_ui+tessellate`, `demo_only_tessellate`, `text_layout_uncached`, `text_layout_cached`, `short_label_cached`. Env switches: `BIG=1` lists allocations >= 2 KB in one uncached layout (finds realloc chains), `BIGTESS=1` the same for one `Context::tessellate`, `SCENE=ui` runs 300 UI frames only (for `heaptrack`: `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only cargo build --release`, `SCENE=ui heaptrack target/release/alloc_count`, `heaptrack_print -n 40 -s 6`), `SCENE=startup` prints startup allocation costs. Baseline numbers (system malloc, base commit): demo_run_ui 1113 allocs / 379 KB; run_ui+tessellate 1258 / 1,306 KB; only_tessellate 157 / 955 KB; text_layout_uncached 128 / 469 KB; text_layout_cached 2 / 1.5 KB. The harness' `target/` is deleted after use (rebuild ~25 s).

## Scenes harness (session 3)

`harness/scenes/` is a standalone criterion bench project (own `[workspace]`, path deps on the repo crates, mimalloc by default like the repo's own benches; `--no-default-features` for system malloc, needed for heaptrack) that measures one UI frame (build + tessellate) for: 300 buttons, 100 mixed widget rows, 1000-label grid, 1000-row table, 18k-char multiline TextEdit (static and typing), 2000 painter shapes, polylines (2x5000 points), images, show_rows, selectable labels, nested collapsing headers, 30/120 windows, 50-button input events, changing labels (uncached relayout), button scaling (100/1000/3000), and `shape_*_1000` (one benchmark per shape kind: rect variants, circles, ellipse, lines, polygon, beziers, dashed). Build with `CARGO_TARGET_DIR=<repo>/target/scenes cargo bench --no-run` from `harness/scenes`; compare prebuilt base/patched binaries with `tools/ab.sh <base> <patched> '<regex>'`; `tools/scn.sh <bin>` prints all midpoints. Baseline midpoints (mimalloc, pinned core, session 3 start) are in `_reference/scenes_base.txt`.

`alloc_count` also has `SCENE=layouthash` (hash of glyph positions/advances/uv rects over Latin/Arabic/CJK/emoji text at several ppp/sizes: base value `34553a3c6727b927`), used as the bit-exactness oracle for text-layout candidates.

## Method

- Build the baseline once (`git stash` / clean tree), copy `target/release/deps/benchmark-*` to `/tmp`, build the patched tree, copy again, then compare with `tools/ab.sh <base-bin> <patched-bin> '<criterion filter regex>' [rounds]` - interleaved runs pinned to one core (`taskset -c 4`). Only the printed ratio is meaningful (the script's absolute numbers are ~2x criterion's because of how it collects the unit/midpoint columns).
- Criterion's own `--baseline` comparison drifted between runs under load on this machine (a saved baseline "moved" by several %), so it was only used for the first candidate and never trusted afterwards.
- Load average was 4-7 on this machine during the session (unrelated builds), so only large or consistently signed ratios were called wins.
- Profiling: `CARGO_PROFILE_BENCH_DEBUG=line-tables-only CARGO_TARGET_DIR=target/prof cargo bench ... --no-run`, then `perf record -g` and `perf annotate -l`; `DEBUGINFOD_URLS=` set empty. `RUSTFLAGS="-C force-frame-pointers=yes"` into `target/prof2` for callgraphs (call graphs through inlined criterion frames were not useful).
- Tests used for every candidate: `cargo test -p epaint --all-features` (95 + 4 passed, 1 ignored; without `--all-features` epaint's tests fail to compile at HEAD), `cargo test -p egui` (86 + 3 + 188 passed). `cargo test -p egui_demo_lib` has 2 snapshot failures at HEAD in this container (`demo::demo_app_windows::tests::demos_should_match_snapshot`, `rendering_test::tests::rendering_test`), identical before and after candidate 003.

## Layout notes

- `tools/ab.sh` is the interleaved A/B runner; other scripts in `tools/` are the skill's generic templates (not adapted/used this session except `tidy.py` via `just tidy`).
- `mkjustfile.sh` generates each candidate's self-contained `justfile` (clone egui at `base_commit`, bench baseline with criterion `--save-baseline`, apply `patch.diff`, bench again with `--baseline`; the criterion comparison inside `just test` is the less reliable method, use the A/B approach above for precise ratios).
- `_reference/` holds session-only raw output (baseline criterion run, baseline test output).
- `__issues_raw/` is the local cache of open GitHub issues/PRs (935 issues, 126 PRs at sync time), not part of the deliverable.

## Candidate 004 note

`004.fast_round_instead_of_roundf/exhaustive_check.rs` is a standalone program (not part of the egui repo) verifying the replacement against `f32::round` for all 2^32 bit patterns.
