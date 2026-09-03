# Ideas

Untested ideas only - see `SUMMARY.md` for what's already been tried.

Last GitHub issue/PR checked for inspiration: **#1864** (repo: linebender/vello)

## Backlog (not yet tried)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Idea | Why it looks promising | Source |
|---|---|---|---|
| 2026-09-01 | vello_hybrid: 2D texture array -> individual textures for the image atlas | Open PR #1859: dynamic mutation of 2D texture arrays is poorly supported on some GPUs (hangs on some, broken multi-texture copies on Adreno) and has worse memory footprint than plain 2D textures. Replacing the atlas's texture array with separate 2D textures (reusing the existing external-texture mechanism) is as much a stability fix as a memory-footprint improvement - worth checking the copy/binding overhead tradeoff before merge. | https://github.com/linebender/vello/pull/1859 |
| 2026-09-01 | vello_hybrid: permanent stall on Metal with very large single-encoder scenes | Open issue #1246: pushing enough draw commands onto a single `CommandEncoder` in vello_hybrid can permanently stall on the Metal backend (large monitor, complex compositing/mixing scene) - same underlying issue as gfx-rs/wgpu#1960. A proof-of-concept periodically submitting/recreating the encoder was mentioned as a workaround; worth investigating for scenes with heavy per-frame command volume. | https://github.com/linebender/vello/issues/1246 |
| 2026-09-01 | vello_cpu: bring-your-own-threadpool | Open issue #1798: `RenderContext::new_with` only lets you pick a thread count, so each `RenderContext` spins up its own thread pool; callers with multiple contexts (e.g. Servo, per servo/servo#46992) end up with redundant thread pools instead of sharing one. Allowing an externally supplied pool would cut redundant thread spawning for multi-context users. | https://github.com/linebender/vello/issues/1798 |
