# Fewer `Context` lock acquisitions per widget (`get_response`, `check_for_id_clash`)

`Context::create_widget` takes the context lock about 6 times per widget; two of them are trivially mergeable: `get_response` read `highlight_next_pass` through a separate `prev_pass_state` lock before its main `write`, and `check_for_id_clash` used `pass_state_mut` then a separate `options` read. Merged each pair into one `write`.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core, 4 rounds). Ratios patched/base (ab.sh absolute values are doubled, only ratios matter):

| Benchmark | r1 | r2 | r3 | r4 |
|---|---|---|---|---|
| demo_with_tessellate__realistic | 0.9996 | 1.0024 | 0.9901 | 0.9968 |
| demo_no_tessellate | 1.0024 | 1.0040 | 0.9815 | 0.9936 |

## Result

Within noise (-1.9% .. +0.4%, mixed signs). Time only.

## Does it change behavior/output?

- **Bit-exact?** Yes - same values read under one lock instead of two (the viewport entry used is the same one `prev_pass_state` / `pass_state_mut` use).
- Breaking change: no.
- Tests: `cargo test -p egui`: 86 + 3 + 188 passed after (same as unpatched run earlier in session).
- Real change to the tested project's source: `patch.diff` (+11/-6).

## Why the result looks the way it does

An uncontended parking-lot style lock is only a few ns; removing 1 of ~6 acquisitions per widget is below the benchmark noise floor (a few %). `Context::get_response` self time (5%) is dominated by the hash-set lookups and flag logic, not locking.

## What's not done / open questions

- Merging all acquisitions in `create_widget` into one (including `memory()` for `allows_interaction`, `ignore_focus`, the scroll `write`) was not tried; the savings would scale with the number removed but each is a few ns.
