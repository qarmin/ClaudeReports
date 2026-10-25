# Patches dictionary and splines parsers/renderers fuzzed directly

Whole-file fuzzing reaches `PatchesDictionary::read_internal` / `Splines::read` rarely. `read_internal` has `positions.last().x as i32 + delta_x` with an attacker-controlled i32 delta (looked like an overflow lead). A scratch `bh_fuzz` module in the bh_diff worktree calls the parsers (and `add_one_row`, `initialize_draw_cache`) on raw bytes.

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz `patches_unit`: parse, then run `add_one_row` over all rows/chunks with random reference frames and extra channels | first run 17 crashes, all in `add_one_row` with extra channels of HALF the colour size (my harness's `shrink_ec`); reference frames in the real decoder always store all channels at the same size (frame/decode.rs:252-263), so they were harness artefacts. Re-run without shrinking: 3 forks, ~1500 s, ~2.0M execs, seeded with the 5 byte streams from the repo's unit tests | 0 crashes after the fix of the harness |
| cargo-fuzz `splines_unit`: `Splines::read` then `initialize_draw_cache` | 2 forks, 1500 s, ~660k execs | 0 crashes |
| Read the `i32` delta arithmetic | patches.rs read_internal | not triggered by the fuzzer; not proven impossible (needs a delta near i32::MAX with a valid entropy stream) |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No crash in the patches/splines parsers or renderers with consistent reference frames.

## What's not done / open questions

- The `x as i32 + delta_x` / `-delta_x` (i32::MIN) arithmetic in patches.rs is still an unverified lead; a hand-built entropy stream with a large delta would settle it.
