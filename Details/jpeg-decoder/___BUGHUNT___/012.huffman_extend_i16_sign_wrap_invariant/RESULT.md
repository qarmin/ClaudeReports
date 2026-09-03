# huffman_extend_i16_sign_wrap_invariant

`clippy::pedantic` flags `value as i16` in `huffman::extend(value: u16, count: u8) -> i16`
(src/huffman.rs:165-172) as possibly wrapping, since `extend()`/`receive_extend()` (src/huffman.rs:90-96)
have no `assert`/`debug_assert` of their own enforcing `count <= 15` - the invariant that keeps
`value < (1 << 15)` (so `value as i16` never crosses the sign boundary) is only upheld by the three
call sites, not by the function itself. Investigated whether any reachable path (decoder.rs:1105,
decoder.rs:1165, or decoder/lossless.rs:93) can actually drive a magnitude category/count of 16 or
more through `extend()`, wrapping a value in [32768, 65535] to negative i16.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build + project's own existing test suite (`cargo test --lib --tests`, rustc 1.87.0, dev profile) with a temporary `debug_assert!(count >= 1 && count <= 15, ...)` added as the first line of `extend()` | Full test suite, including `crashtest`/`reftest`/`reftest_scaled` decoding the entire `tests/crashtest/images/` and `tests/reftest/images/` corpora (malformed/adversarial images and the full lossless corpus under `tests/reftest/images/lossless/`) | 21/21 tests passed, debug_assert never fired |
| Manual boundary-value construction, baseline (SOF0) path: two hand-crafted, real, on-the-wire JPEGs with a custom DHT whose single 1-bit code maps directly to raw symbol byte 0xFF (255, the maximum a DHT can encode) - once as the DC symbol, once as the AC symbol - decoded via the real public `Decoder::decode()` API | `cargo run --example boundary_check`, same debug_assert-instrumented tree | DC case: rejected by decode_block's `_ => Err(...)` arm before reaching `receive_extend`. AC case: `s = byte & 0x0f` masks 0xFF down to 15 before use as `count`; decodes successfully. No panic in either case. |
| Manual boundary-value construction, lossless (SOF3) path targeting decoder/lossless.rs:93 specifically: three hand-crafted real SOF3 JPEGs with DHT DC symbol 16 (the explicit bypass arm), 0xFF (out-of-range), and 15 (legal boundary) | `cargo run --example boundary_check_lossless`, same instrumented tree | Symbol 16: bypasses `extend()` entirely via the code's own `16 => 32768` arm (Ok, no panic). Symbol 0xFF: rejected by `_ => Err(...)` before `extend()` (Ok error path, no panic). Symbol 15: the maximum count any call site can pass to `extend()`; decodes fine, no panic. |

## Status

**Not reproducible.** All three call sites structurally cap the `count` argument at 15 before it
ever reaches `extend()`/`receive_extend()`, and this was verified both by reading the code and by
driving the real decoder end-to-end with crafted Huffman tables that map to the largest byte value
a DHT segment can ever encode (0xFF) at every relevant call site:

- decoder.rs:1105 (baseline/progressive DC): `match value { 0 => 0, 1..=11 => receive_extend(...), _ => Err(...) }` - any Huffman-decoded symbol outside `0..=11` is rejected before `receive_extend` is called.
- decoder.rs:1165 (baseline/progressive AC): `s = byte & 0x0f` - a 4-bit mask that mathematically bounds `s` to `0..=15` regardless of what `byte` (an arbitrary, attacker-controlled u8 from the DHT's `values` array) actually is; `s == 0` is handled separately, so `receive_extend` only ever sees `1..=15`.
- decoder/lossless.rs:93: `match value { 0 => 0, 1..=15 => receive_extend(...) as i32, 16 => 32768, _ => Err(...) }` - category 16 is special-cased to bypass `extend()` entirely (the difference is assigned directly as an `i32`, never narrowed through `i16`), and anything above 16 is rejected. `receive_extend` here only ever sees `1..=15`.
- src/huffman.rs:235 (the AC fast-lookup-table builder, `HuffmanTable::new`): `magnitude_category = value & 0x0f` - the same 4-bit mask applies at table-build time, so `extend()` is never called there with `magnitude_category > 15` either.

Covered explicitly: both the debug-assertions-instrumented full existing test/crash/reftest corpus
(no fuzzing beyond what that corpus already exercises - no dedicated fuzz campaign was run this
session) and hand-constructed adversarial DHT tables at every one of the four call sites listed
above, including the theoretical worst case (a Huffman symbol byte of 0xFF/255) at each site that
receives an attacker-controlled byte value.

**Fully reproducible**: N/A (not applicable - Status is Not reproducible, not Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not a bug: the invariant the pedantic lint is questioning (`count <= 15` before `value as i16` in
`extend()`) is real and does hold, but it's enforced structurally at each of the four call sites
rather than inside `extend()`/`receive_extend()` themselves - by a `match` arm range restriction at
three of them (decoder.rs:1105, decoder.rs:1165's `s==0` split, decoder/lossless.rs:93) and by a
`& 0x0f` bitmask at the other two (decoder.rs:1165's `s = byte & 0x0f`, huffman.rs:235's
`magnitude_category = value & 0x0f`). Since a Huffman-table-decoded symbol byte is always `u8`
(0..=255, fully attacker-controlled via a crafted DHT segment), the mask sites bound `count` to
0..=15 unconditionally (mathematically, not just "in practice"), and the match-arm sites reject any
out-of-range symbol with a `Format` error before it can reach `extend()`. Category 16 specifically
(the one value between the masked/matched safe range and where `value as i16` would first start
wrapping) is explicitly special-cased in decoder/lossless.rs to bypass `extend()`/the `i16` narrowing
altogether. There is no path from any of the three call-site locations named in the candidate
description (decoder.rs:1105, decoder.rs:1165, decoder/lossless.rs:93) that can reach `extend()`
with `count >= 16`.

## Introduced by

N/A (Not reproducible - no bug to trace to an introducing commit).

## The fix

No fix needed - no reachable bug. Not applied: adding a `debug_assert!(count <= 15)` (or similar)
directly inside `extend()`/`receive_extend()` would be a reasonable *defense-in-depth* change to
make the invariant self-documenting and to catch a future call site that violates it, but that is a
code-quality/robustness improvement, not a bug fix for a reachable defect - out of scope for this
skill's "bounded fix for a confirmed bug" step. (The temporary debug_assert added for this
investigation was reverted; the working tree is clean.)

- Project test suite: N/A (no change was kept; the temporary debug_assert used to search for a
  reproduction did not fail before or after - 21/21 passed both times, see `output.txt`).

## What's not done / open questions

- No dedicated fuzz harness was run against `extend()`/`receive_extend()` specifically this
  session (soft cap of 3 approaches was reached with the debug-assertions test-suite run plus two
  rounds of manual boundary-value construction, all negative); a short one-off fuzz burst targeting
  DHT-table construction specifically could be tried in a future session if this suspect resurfaces,
  though the masking/match-arm argument above is a mathematical guarantee, not merely an empirical
  one, so a fuzz campaign is unlikely to add real signal here.
- Did not check whether `decode_fast_ac`'s precomputed `ac_lut` (src/huffman.rs:60-78) could ever
  store a `count`-equivalent (`run_size & 0x0f`) above 15 - by construction (`magnitude_category =
  value & 0x0f`, same mask), it cannot, so this was not treated as a separate call site needing its
  own crafted-input test.
- A reasonable, low-risk follow-up (not attempted here, since it's a hardening change rather than a
  fix for a reproduced bug) would be adding `debug_assert!(count <= 15)` directly inside `extend()`
  so a future call site that violates the invariant fails loudly in a debug build instead of relying
  on every caller getting the range check right independently.
