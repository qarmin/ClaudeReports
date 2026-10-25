# start_new_frame / scan_frames_only on malformed animations

`start_new_frame` was rewritten a week before this session (6f00061 'keep matching stored frames and seek to earliest missing slot', 91480a0 'Seed noise correctly after a seek'): it tracks reference-slot state across seeks, so malformed or unusual animation structure could break the seek-equals-sequential invariant that `test_seek_matches_sequential` checks only on 9 well-formed files.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz `seek`: scan with `scan_frames_only`, then random seeks / partial decodes on a second decoder, crash/panic oracle only | `harnesses/fuzz_formats`, 6 forks, 1538 s, ~168k execs, seeded with 234 prefixed test files | 0 crashes |
| cargo-fuzz `seekdiff`: sequential decode, scan (frame count must equal), then 4 seeks with 1-3 following frames on one reused decoder, every frame compared bit-exactly with the sequential result | 10 forks, 1508 s, ~507k execs, seeded with the animation test files (<100 kB) x 4 random prefixes | 0 crashes / mismatches |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No violation of 'seek then decode == sequential decode' and 'scan frame count == decoded frame count' was found on any mutated input that the sequential decoder accepted.

## What's not done / open questions

- Large animations (icos4d, newtons_cradle, > 100 kB) were not used as seeds, and the fuzzer's execs/s was low (8-57 per fork) because every exec decodes several frames three times.
- Only default options (coalescing on, default pixel format) were used for the seeks.
