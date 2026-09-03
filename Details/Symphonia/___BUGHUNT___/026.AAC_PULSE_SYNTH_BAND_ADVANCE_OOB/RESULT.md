# AAC_PULSE_SYNTH_BAND_ADVANCE_OOB

`symphonia-codec-aac/src/aac/ics/pulse.rs` `Pulse::synth` only guards the starting condition
(`if self.pulse_start_sfb >= bands.len() - 1 { return; }`) and then runs
`while bands[band + 1] <= k { band += 1; }`, where `k` accumulates attacker-controlled
`pulse_offset` values read straight from the bitstream (bit-width validated on read, but never
range-checked against the bands table). This looked like it depended entirely on every static
per-sample-rate scalefactor-band table happening to end with a sentinel `>= 1024` (so the
`k >= 1024` check a few lines above trips before `band + 1` can run off the end of `bands`), with
nothing in `synth` itself enforcing `band + 1 < bands.len()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual audit of every `GASubbandInfo`-derived `long_bands` table (`symphonia-codec-aac/src/aac/common.rs`, all 7: `SWB_OFFSET_{8,16,24,32,48,64,96}K_LONG`) plus the call site (`ics/mod.rs::get_bands`/`decode`) that only ever passes `bands` when `self.info.long_win` is true (`validate!(self.pulse.is_none() \|\| self.info.long_win)` at line 431) | Read source only, no build | Every one of the 7 real long-band tables ends in exactly `1024` (not just `>= 1024`); since `k >= 1024` returns before the `while` loop runs, `bands[bands.len() - 1] == 1024 <= k` is always false when the loop executes, so `band` can never advance past `bands.len() - 2`, and `band + 1` never exceeds `bands.len() - 1` (the last valid index). The starting guard (`pulse_start_sfb >= bands.len() - 1`) combined with this makes `band + 1 < bands.len()` an invariant, not a coincidence of the current table contents alone - it holds for the sentinel value itself, not merely "probably big enough" |
| Debug-assertions build (`RUSTFLAGS="-C debug-assertions=on -C overflow-checks=on"`) running a boundary-value + exhaustive + 200k-iteration random-fuzz `#[cfg(test)]` module (`repro/pulse_synth_boundary_tests.rs`) appended to the real `pulse.rs`, calling the real, private `Pulse::synth` directly with the real `SWB_OFFSET_*_LONG` tables and every `pulse_start_sfb` (0..=len), every `number_pulse` (1..=4, the full 2-bit-field+1 range), and both curated boundary `pulse_offset` patterns and a 200,000-iteration xorshift random burst per table (5-bit field range 0..=31) | `cargo test -p symphonia-codec-aac pulse_oob --lib -- --nocapture`, run via `just test` against a pristine clone at the base commit | 3/3 test functions pass, 0 panics across ~1.7M+ `synth` calls total (7 tables x 65 `pulse_start_sfb` values x 4 `number_pulse` values x 5 patterns, plus 42x32 exhaustive single-pulse sweep, plus 7x200,000 random fuzz) - see output.txt |

Manual audit alone already proves the invariant holds structurally (not just for the current table
contents); the debug-assertions exhaustive/random test run empirically confirms it across every
value the bitstream reader's own field widths can produce. Both approaches agree, so the soft cap
of 3 was not needed - no third tool was escalated to.

## Status

**Not reproducible.** Covered: full boundary-value sweep of `pulse_start_sfb` (0 through
`bands.len()`, i.e. every valid index plus the guard boundary and one past it) x `number_pulse`
(1..=4) x 5 curated `pulse_offset` patterns (all-max, all-zero, all-one, alternating max/zero, a
mixed pattern) across all 7 real `SWB_OFFSET_*_LONG` tables; an exhaustive single-pulse sweep of
every `(pulse_start_sfb, pulse_offset)` pair against the smallest table (`SWB_OFFSET_96K_LONG`);
and a 200,000-iteration-per-table random fuzz burst sampling the full field ranges the bitstream
reader permits. Zero panics, zero out-of-bounds indications, in a debug-assertions +
overflow-checks build. A future session should not retry this exact input space - if a real bug
exists here, it would require either a table entry to have been miscopied (checked by eye against
each value above, all end in 1024) or a code path that passes a `bands` slice other than one of
these 7 static long-band tables into `Pulse::synth` (checked: the only real call site,
`ics/mod.rs::synth_channel` via `get_bands()`, is gated by `validate!(self.pulse.is_none() ||
self.info.long_win)` at decode time, so `pulse.synth` is only ever reached when `get_bands()`
returns `self.sbinfo.long_bands`, never `short_bands`).

## Root cause (why the suspect is actually safe)

The starting guard `if self.pulse_start_sfb >= bands.len() - 1 { return; }` bounds
`band = pulse_start_sfb` to `0..=bands.len() - 2` before the loop starts. Inside the `for pno`
loop, `k` is checked against `k >= 1024` (return) before the `while bands[band + 1] <= k` runs, so
the `while` loop body only ever executes with `k < 1024`. Because every real `long_bands` table's
last element is exactly `1024`, the condition `bands[bands.len() - 1] <= k` is always false for any
`k < 1024`, so the `while` loop can never advance `band` as far as `bands.len() - 1`; it always
stops at latest at `band == bands.len() - 2`, keeping `band + 1 <= bands.len() - 1` (the last valid
index) for every iteration. The bug candidate's premise - that this "depends entirely" on the
tables' current contents rather than being enforced by `synth` itself - is correct as a
description of the code's structure (there is no explicit `debug_assert!`/runtime check tying the
two together), but the actual data in every currently-defined `GASubbandInfo::long_bands` table
does satisfy the required invariant (terminal sentinel `== 1024`), and the only call site that
reaches `Pulse::synth` is statically constrained (via `validate!` at decode time) to only ever pass
one of these long-band tables. No input-dependent way to reach `Pulse::synth` with a `bands` slice
that violates the invariant was found.

## Introduced by

N/A - not a bug, so no introducing commit to identify.

## The fix

None needed - no fix applied, `patch.diff` intentionally omitted since Status is Not reproducible.
(A defensive improvement worth noting for future maintainers, not applied here since it changes no
observable behavior: `synth`'s `while` loop could use `band + 1 < bands.len() &&
bands[band + 1] <= k` to make the invariant self-enforcing rather than relying on every table's
sentinel, guarding against a future table edit that omits the `1024` terminator. This was
considered out of scope since there is no confirmed bug to fix.)

## What's not done / open questions

- Not checked: whether any *short*-window bands table (`SWB_OFFSET_*_SHORT`) could ever reach
  `Pulse::synth` through a code path other than the one audited here - the current code base has
  exactly one call site and one bitstream-level validation gate, both confirmed to exclude this,
  but a future refactor that loosens `validate!(self.pulse.is_none() || self.info.long_win)` or
  adds a second caller of `Pulse::synth` would need to re-establish the same invariant for whatever
  table it passes in, since none of the `SWB_OFFSET_*_SHORT` tables were checked for a `1024`
  terminator here (out of scope: unreachable under current code).
- The defensive fix suggested above (`band + 1 < bands.len() && ...`) was not applied since there
  is nothing to verify it against (no reproducing input) - left as a documented suggestion only.
