
# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#8736** (repo: emilk/egui)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
| 2026-10-18 s1 | cubic bezier tessellation with huge control points (1e20) | randomized tessellator probe hangs (3 s watchdog) on cubic with 1e20 coordinates, not analysed | tmp probe |
| 2026-10-18 s1 | `emath::remap` debug_assert `from.start() != from.end()` via slider_core `value_from_normalized`/`normalized_from_value` | random slider fuzz: f32 `-inf..=1.0` logarithmic, step_by(1e-9), clamping Never, DragValue alongside; did not reproduce in the plain range grid, needs the exact flags | candidate #10 |
| 2026-10-18 s1 | `Slider` range -1.0..=100.0 (zero cutoff exactly 0.0) | same `logarithmic_zero_cutoff` formula yields 0 for the negative side; check negative values on the rail | candidate #10 |
| 2026-10-18 s1 | `Rect::rotate_bb` on `Rect::NOTHING` | returns [-inf,-inf]..[inf,inf] (covers everything) instead of staying empty | candidate #14 |
