# `CFA::new` panics on a CFA pattern string of an unexpected length

`rawler/src/cfa.rs:117-125` `CFA::new(patname: &str)` matches `patname.len()` against a fixed set
of accepted sizes (0, 4, 16, 36, 144) and `panic!("Unknown CFA size \"{}\"", patname)` on any other
length. `CFA::new_from_tag` (`cfa.rs:93-107`) builds `patname` directly from a raw file's own
`CFAPattern` TIFF tag (0x828E) - the tag's element count (`pat.count()`) is fully attacker/file-
controlled TIFF data, with no length validation before it reaches `CFA::new`. `DngDecoder::get_cfa`
(`decoders/dng.rs:349-362`) calls `CFA::new_from_tag` and is itself called from
`DngDecoder::make_camera`, which is called from `DngDecoder::raw_metadata` - the metadata-only
public entry point reached by simply opening a DNG file, before any pixel decompression happens.
This is a real-user-reported issue: GitHub issue #586 on dnglab/dnglab, still open as of this
session's sync (see `../IDEAS.md`), which is candidate 012's assigned target.

The same file-controlled-length-into-`CFA::new` shape also exists in
`rawler/src/decoders/raf.rs:534-556` `RafDecoder::get_xtrans_cfa` (Fuji X-Trans `XTransLayout` tag)
- not independently confirmed reachable this session, see "What's not done" below.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `cfa.rs`'s `CFA::new`/`CFA::new_from_tag` and their call sites (`decoders/dng.rs`, `decoders/raf.rs`, `decoders/rw2.rs`, `decoders/camera.rs`) | - | Confirmed `CFA::new_from_tag` is the only call site fed directly by unvalidated, attacker-controlled file data through a public decode entry point (`raw_metadata`); other call sites use fixed literal strings or camera-config-derived strings |
| Standalone repro: a spec-shaped DNG (100x100, uncompressed) with a `CFAPattern` tag of 2 elements, built via rawler's own public low-level `TiffWriter`, opened via the real, public `rawler::get_decoder` + `Decoder::raw_metadata` (the same call chain a real file opener uses) | debug build, 5 runs; release build, 1 run | Deterministic panic every run, both build modes: `panicked at rawler/src/cfa.rs:124:12: Unknown CFA size "RG"` |
| Boundary-value variation: same repro with a 7-element `CFAPattern` (a different bad length) plus a valid, in-bounds `ActiveArea` tag (to exercise `get_cfa`'s `cfa.shift(left % cfa.width, ...)` branch) | debug + release, run once each before the fix (both panic, same shape), 5x debug + 1x release after the fix (all `Ok`) | Confirms the bug isn't specific to length 2, and that the fix's `is_valid()` guard in `dng.rs` prevents a second panic (division-by-zero-modulus) that the naive fallback-to-`(0,0)` fix would otherwise introduce on this second path |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed**. Reproduces 5/5 debug runs and 1/1 release run, fully deterministic - this is a plain
`panic!()` macro, not a debug-assertion-only overflow check, so it fires identically in both build
modes (unlike several other candidates in this workspace).

**Fully reproducible**: **Yes**. `repro/` writes a real, spec-shaped DNG file to disk (a normal DNG
except for its `CFAPattern` tag's element count) and calls the real, public `rawler::get_decoder` +
`Decoder::raw_metadata` - the same two-call sequence any real caller (dnglab's own CLI, or any
downstream user of the `rawler` crate) uses to open a file. Single-participant case (a malformed
file handed to a parser's real public entry point), no second participant to fake, per
`CONFIRMATION.md` section 11.

## Class

`crash` (panics in both debug and release builds).

## Root cause

`CFA::new` (`cfa.rs:117-125`):

```rust
pub fn new(patname: &str) -> CFA {
  let (width, height) = match patname.len() {
    0 => (0, 0),
    4 => (2, 2),
    36 => (6, 6),
    16 => (2, 8),
    144 => (12, 12),
    _ => panic!("Unknown CFA size \"{}\"", patname),
  };
  ...
```

`CFA::new_from_tag` (`cfa.rs:93-107`) builds `patname` with one character per element of the file's
own `CFAPattern` TIFF tag - `patname.len() == pat.count()`, a value read straight from file bytes
with no validation. Real cameras only ever write 4- or 36-element patterns (2x2 or 6x6 Bayer/X-Trans
arrays), so `CFA::new`'s author encoded that as an exhaustive match with an unreachable-in-practice
`panic!()` fallback - but "unreachable in practice" is not the same as "unreachable", and a crafted
or simply corrupted DNG can set `CFAPattern`'s count to anything, immediately reaching the panic.

## Introduced by

Not a regression in the usual sense - `CFA::new`'s design (an exhaustive match over accepted
pattern-string lengths, `panic!()` on anything else) has been present since the CFA module's very
first commit in the project's history: 688c29b1 "Move the CFA code into its own module" (Pedro
Côrte-Real, 2016-12-26), which already contained `_ => panic!(format!("Unknown CFA with size {}",
patname.len()).to_string())`. The exact panic message/macro form seen today was last touched by
165de6000 "Simplified other panic formatting in the remaining file" (johannesvollmer, 2018-08-14),
a purely stylistic `panic!(format!(...))` -> `panic!(...)` simplification, not a behavior change.
The accepted-length list itself grew over time (f2200125c "Add support for Phase One IIQ raws",
Daniel Vogelbacher, 2022-03-09, added the `16 => (2, 8)` case) without ever changing the `_ =>
panic!(...)` fallback shape. In short: this is original, unchanged design going back to the
project's 2016 predecessor (`rawloader`), not a later-introduced defect.

## The fix

Two changes, both required together (see the boundary-value variation row above for why):

1. `cfa.rs`: `CFA::new`'s `_ =>` arm no longer panics. It logs a `log::warn!` with the offending
   pattern string/length and falls back to `(0, 0)` - the exact same width/height an explicitly
   empty pattern (`CFA::new("")`) already produces, a state every existing caller already has to
   tolerate (`CFA::default()` is defined as `CFA::new("")`, and `CFA::is_valid()` exists
   specifically to let callers detect this state).
2. `decoders/dng.rs`: `DngDecoder::get_cfa`'s `ActiveArea`-shift branch computed `left % cfa.width`
   /`top % cfa.height` unconditionally. With change 1, an invalid/unrecognized `CFAPattern` now
   produces `cfa.width == cfa.height == 0`, which would turn the original panic into a *different*
   panic (`attempt to calculate the remainder with a divisor of zero`) the moment a file also has
   an `ActiveArea` tag. Guarded with `if cfa.is_valid() { ... } else { Ok(cfa) }` - the same
   pass-through-unchanged behavior `get_cfa` already uses when there is no `ActiveArea` tag at all.

Verified: `repro/`'s 2-element-`CFAPattern` DNG no longer panics in either build mode (5/5 debug, 1/1
release), and the length-7-`CFAPattern`-plus-`ActiveArea` variant (which would have hit the
division-by-zero without change 2) also returns `Ok` cleanly after the fix.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercised a `CFAPattern` of an unrecognized
  length, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- `RafDecoder::get_xtrans_cfa` (`decoders/raf.rs:534-556`) has the identical shape - a file-derived
  byte array's length flows into `CFA::new` with no validation - but through a different call chain
  (Fuji RAF `XTransLayout` metadata block rather than a DNG `CFAPattern` tag). Since `CFA::new`
  itself was fixed (not just the DNG call site), a malformed RAF file hitting this path is *also*
  fixed by this same patch, but it was not independently repro'd/confirmed reachable this session -
  added back to `../IDEAS.md` as a follow-up worth a dedicated repro if a future session wants
  independent confirmation of the RAF path specifically.
- `decoders/camera.rs:185` (`CFA::new(val.as_str()...)`, reading a `color_pattern` value from the
  bundled `cameras.toml` config) and `decoders/rw2.rs:252-255` (fixed `"RGGB"`/`"GRBG"`/`"GBRG"`/
  `"BGGR"` literals) also call `CFA::new`, but neither is attacker/file-controlled in the same
  sense (config-file/hardcoded-literal, not raw-file-tag data) - not investigated further as part
  of this candidate, consistent with "one bug per candidate."
- No attempt was made to change `CFA::new`'s panic-on-bad-length behavior into a `Result`-returning
  API across all ~10 call sites (see `decoders/camera.rs:185`'s own `panic!()`-on-bad-config
  pattern, and IDEAS.md's standing note about issue #193's `check_condition!()` macro proposal for
  the whole codebase) - that would be a larger, more invasive change than this session's bounded
  fix; the fallback-to-empty-pattern approach taken here fixes the concrete crash with a minimal,
  targeted diff instead.
