# Initial capacity for each new tessellated `Mesh` (vertices 1024, indices 1536)

`harness/alloc_count` (`BIGTESS=1`) showed one `Context::tessellate` of the demo frame doing 66 allocations >= 2 KB, almost all of them doubling realloc chains (e.g. 2112 -> 56320 bytes) of each clip-rect/texture `Mesh`'s `vertices`/`indices`. Experiment: give every new mesh a fixed starting capacity so the early doublings disappear.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: time and allocation (counting allocator, system malloc).

## What was tried

| Variant | demo_only_tessellate allocs/iter | bytes/iter | peak live delta | time (patched/base, interleaved A/B, 3 rounds) |
|---|---|---|---|---|
| baseline | 157.0 | 954,716 | 519,864 | 1.0 |
| fixed initial capacity | 83.0 | 1,209,488 | 884,824 | 0.9535, 0.9594, 0.9645 (demo_only_tessellate); demo_with_tessellate__realistic 0.9854, 0.9887, 0.9961 |

(`demo_run_ui+tessellate`: 1258 allocs / 1,306,196 bytes / 532,424 peak -> 1184 / 1,560,968 / 897,384.)

## Result

Mixed: -74 allocations and about -4% tessellation time (about -1% of a full frame), but +254 KB allocated and +365 KB peak live memory per frame, because many meshes are far smaller than the fixed 1024-vertex reservation and the memory is held until the frame's primitives are dropped. The time gain is real but is paid for in memory.

## Does it change behavior/output?

- **Bit-exact?** Yes (capacity only).
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed, `cargo test -p egui` 86 + 3 + 188 passed after; identical counts before.
- Real change to the tested project's source: `patch.diff` (+5/-1).

## Why the result looks the way it does

Most of the demo's meshes grow to tens of KB (text-heavy), so skipping the first 3-4 doublings saves copies, but a fixed constant over-allocates for the many small meshes. A size hint derived from the shapes (e.g. summing `galley.num_vertices` for consecutive text shapes with the same clip rect, or last frame's per-primitive sizes) would give the time gain without the memory cost; not tried.

## What's not done / open questions

- Sized (not fixed) reservation; hints from the previous frame stored in `Context`.
