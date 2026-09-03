# div_by_2pn_m1 overflow on out-of-range 16-bit input (src/alpha.rs:176-183)

`div_by_2pn_m1` is the shared helper behind `premultiply_rgba16`/`premultiplied_rgba16`/
`premultiply_la16`/`premultiplied_la16` for every `bit_depth` other than 10/12/16. It does `u32`
arithmetic on `pixel * alpha` assuming the product fits the caller's claimed `bit_depth`, but
nothing in the public API validates that a `&mut [u16]`/`&[u16]` buffer actually stays within
`bit_depth` bits - flagged by the prior `code-audit-structured` pass, picked as the first backlog
item in `IDEAS.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug-assertions build | `cargo run --example` (later `repro/`, dev profile, default overflow-checks) calling `premultiply_rgba16(&mut [65535u16;4], bit_depth)` for `bit_depth` in `1..=15` (excluding 10/12/16, which take dedicated fast paths not through the generic overflow-prone branch) | Reproduced immediately on the first attempt for every tested `bit_depth` except the dedicated 10/12/16 paths: `thread 'main' panicked at src/alpha.rs:182:6: attempt to add with overflow` |
| Same repro, release build (`cargo run --release`) | Same inputs | No panic (overflow-checks off by default in release) but a silently wrong, wrapped-around result (`[32769, 32769, 32769, 49152]` for `bit_depth=1` instead of a bounded/sane saturated value) - confirms the "silently produce a wrong value in release" half of the original suspicion |

Only one approach was needed; the debug-assertions build reproduced the crash on the very first
run, so no escalation to miri/ASAN/fuzzing was necessary (the arithmetic-on-untrusted-input class
is exactly what a debug-assertions build + boundary values is suited for, per `HEURISTICS.md`).

## Status

**Confirmed** - deterministic, 3/3 identical panics on repeated runs of the original hand-written
repro (same message, same `src/alpha.rs:182:6` location every time - no randomness anywhere in the
call path), plus a clean re-run of the final `repro/` binary (exit 1, panic + wrong-value report)
against the unpatched tree, and exit 0 against the patched tree.

## Class

crash (debug build) / logic (release build - silent wraparound to a wrong pixel value instead of a
panic)

## Root cause

`div_by_2pn_m1(v: u32, n: u32)` computes `round = 1 << (n - 1)` then `v + round`, then
`(v >> n) + v`, entirely in `u32`. The doc comment says "the result is expected to fit in u16",
which is true *only if* `v` itself is bounded by `(2^n - 1)^2` (i.e. both multiplicands actually
conform to `bit_depth`). The public callers (`premultiply_rgba16`, `premultiplied_rgba16`,
`premultiply_la16`, `premultiplied_la16`) take a plain `&mut [u16]`/`&[u16]` plus a `bit_depth: u32`
and only `assert!(bit_depth > 0 && bit_depth <= 16)` - there is no check that the pixel/alpha
values in the slice are actually `<= (1 << bit_depth) - 1`. A caller can pass full-range `u16`
values (0..=65535) tagged with a small `bit_depth` (e.g. 1), making `v = pixel * alpha` up to
`65535 * 65535 = 4_294_836_225`, close to `u32::MAX` (4_294_967_295). The final `(v >> n) + v`
addition then overflows `u32` for small `n` (headroom left after `v + round` is as little as
~131_070, while `v >> 1` alone is over 2 billion for `n = 1`) - `debug_assert!(n > 0 && n <= 16)`
only checks `n`'s range, not that `v` conforms to it.

## Introduced by

5e9fcd2 "Use integer trick for faster general u16 premultiply" (2026-01-09) - `git log -S"fn div_by_2pn_m1"` finds this as the commit that introduced the integer-trick `div_by_2pn_m1` implementation (replacing a presumably float-based predecessor), with the u32 overflow already possible for out-of-range input.

Still present in current HEAD (checked while filling in this section: `div_by_2pn_m1` is unchanged) - not yet fixed upstream.

## The fix

Widened the intermediate arithmetic in `div_by_2pn_m1` from `u32` to `u64` (which cannot overflow
for any `u32` input `v` combined with `n` in `1..=16`), and saturated the final result to
`u16::MAX` before the `as u16` cast (previously a bare truncating cast, which is what produced the
silent wrong value in release for out-of-range input). This is behavior-preserving for every
conforming input - proven directly, not just argued: `premultiply_rgba16(&mut [1000,2000,3000,40000], 16)`
returns the identical `[610, 1221, 1831, 40000]` before and after the patch - because for
`bit_depth`-conforming values the un-saturated `u64` result is always `<= 2^16 - 1` anyway, so the
`.min(u16::MAX as u64)` never triggers on valid data.

```diff
-/// Computes `round(v / (2**n - 1))`. The result is expected to fit in u16.
+/// Computes `round(v / (2**n - 1))`, saturating to `u16::MAX`.
+///
+/// Callers only guarantee `v`'s two factors individually fit `bit_depth` bits, not that their
+/// product does (there is no runtime validation of that upstream) - so this widens to `u64` to
+/// stay overflow-free for any `u32` `v`, rather than relying on the input actually conforming.
 #[inline(always)]
 fn div_by_2pn_m1(v: u32, n: u32) -> u16 {
     debug_assert!(n > 0 && n <= 16);
-    let round = 1 << (n - 1);
-    let v = v + round;
-    (((v >> n) + v) >> n) as u16
+    let round = 1u64 << (n - 1);
+    let v = v as u64 + round;
+    (((v >> n) + v) >> n).min(u16::MAX as u64) as u16
 }
```

Full `patch.diff` in this folder. Re-running the standalone `repro/` against the patched source:
no panic for any of `bit_depth in [1,2,3,4,5,6,7,8,9,11,13,14,15]` at full-range `u16` input, every
returned value bounded and matching an independent overflow-free oracle, and the conforming
`bit_depth=16` case unchanged - exit code 0 (previously exit 1 with a panic).

- Project test suite: `cargo test -p pic-scale-safe --lib` - `0 passed; 0 failed` before the patch,
  `0 passed; 0 failed` after (the library crate has no `#[test]`s at all, confirmed both times by
  actually running the command, not assumed from `README.md`'s note).

## What's not done / open questions

- This only widens the shared low-level helper to be overflow-free; it does **not** add any
  validation to `premultiply_rgba16`/`premultiplied_rgba16`/`premultiply_la16`/`premultiplied_la16`
  that pixel data actually conforms to the declared `bit_depth` - a caller can still silently get a
  saturated-but-still-wrong result if it mismatches `bit_depth` and the real data range. Adding a
  real validation path (return a `Result` instead of panicking/saturating) would be a breaking API
  change and was out of scope for a bounded fix in this session.
- The `bit_depth == 10 / 12 / 16` fast paths (`div_by_1023`/`div_by_4095`/`div_by_65535`) all go
  through the same now-fixed `div_by_2pn_m1`, so they're covered by the same fix - not separately
  re-verified with their own dedicated boundary-value repro beyond what's already exercised
  (`bit_depth=16` conforming case in `repro/`).
- `unpremultiply_rgba16`/`unpremultiplied_rgba16`/`unpremultiply_la16`/`unpremultiplied_la16`
  (the un-premultiply direction, further down `src/alpha.rs`) were not checked in this candidate -
  worth a follow-up pass for the same "out-of-range 16-bit input" class; added to `IDEAS.md`.
