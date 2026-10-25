# hex_color_accepts_plus_sign

`HexColor::from_str_without_hash` / `Color32::from_hex` in `crates/ecolor/src/hex_color_runtime.rs:88`. Looked suspicious because it delegates validation to `from_str_radix`, which has its own notion of valid input (HEURISTICS: parser relying on a std parse with looser grammar).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `s.len()` dispatch then `u16/u32::from_str_radix(s, 16)` | `from_str_radix` accepts one leading `+` |
| Boundary inputs | `#+f0`, `#+fff`, `#+12345`, `#+0000000`, `#+f0f`, `#-f0`, `# f0`, debug build | all `+` forms return `Ok`; `-` and space are rejected |

## Status

**Confirmed** - reproduces on every run (deterministic, pure function; run via `just test`, same output twice).

**Fully reproducible**: **Yes** (public `Color32::from_hex` API, no second participant involved).

## Class

logic

## Root cause

`from_str_radix` accepts a leading `+`. The length check counts the `+` as a digit, so `#+f0` is a "4-digit" string that parses as the 3 digits `f0`... giving `#00FF00FF` (Hex4), and `#+12345` is accepted as the 6-digit colour `#012345`. The documented grammar (CSS hex colours, `# ` followed by only hex digits) is violated, and user input like `#+f0` is silently turned into a wrong colour instead of an error.

## Introduced by

669473ccd "Add `Color32::from_hex_string` and `Color32::to_hex_string` (#3570)" (2024-01-07), the commit that added the function.

## The fix

Reject a leading `+` with the same `InvalidInt(InvalidDigit)` error `-` already yields (`patch.diff`, +5/-0). After the patch `repro/` reports 0 accepted strings.

- Project test suite (`cargo test` in crates/ecolor): 11 + 2 passed / 0 failed before -> same after.

## What's not done / open questions

- The compile-time `hex_color!` macro was not checked for the same leading-`+` behaviour.
- Only one leading `+` is possible in `from_str_radix`, so the single prefix check covers the class.
