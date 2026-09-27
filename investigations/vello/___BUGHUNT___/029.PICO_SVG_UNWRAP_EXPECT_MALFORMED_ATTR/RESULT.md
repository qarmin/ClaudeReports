# PICO_SVG_UNWRAP_EXPECT_MALFORMED_ATTR

`sparse_strips/vello_common/src/pico_svg.rs`'s public `PicoSvg::load(xml_string, scale)` entry
point panics on malformed-but-plausible SVG attribute text instead of returning `Err`. Two spots:
the `viewBox` parsing at line 94 (`.split(' ').map(|s| f64::from_str(s).unwrap())`), and
`parse_transform`'s `matrix(...)` branch at lines 242/245 (`.expect(...)` on parse failure, and a
second `.expect(...)` on wrong argument count). This looked suspicious because the sibling
`translate(...)`/`scale(...)` branches immediately below handle the identical failure shape
gracefully (`if let Ok(...) { ... } else { Affine::IDENTITY }`), and `pico_svg` is a real,
feature-gated but public API of `vello_common` (`pico_svg = ["dep:roxmltree"]` in
`vello_common/Cargo.toml`), not a private example-only helper - `vello_hybrid` and
`vello_example_scenes` both depend on it with that feature enabled.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (malformed-attribute strings crafted by hand, matching HEURISTICS.md's "parser/deserializer + no reference oracle" guidance for a small, well-understood grammar) | Standalone `repro/` crate, release build, calling the real public `PicoSvg::load` with 3 malformed SVG strings (bad `viewBox` field, non-float `matrix` argument, wrong `matrix` argument count) | All 3 panicked on the first run |
| Repeat-run check | Same repro, `cargo run --release` x3 | 3/3 runs, deterministic, all three inputs panic every time - no fuzzing/sanitizer needed since the failure mode is a plain `unwrap()`/`expect()` on a `Result`, not memory-unsafety |

A dedicated fuzz harness/miri/ASAN pass was not needed: the bug class (unwrap/expect on
attacker-controlled string parsing) is deterministic and fully explained by reading the code; a
debug-assertions build would show nothing additional since this isn't an arithmetic-overflow bug.

## Status

**Confirmed**. Reproduces 3/3 runs, release build, deterministic (no fixed seed needed - every one
of the 3 crafted inputs panics on every run).

**Fully reproducible**: **Yes**. The repro calls `vello_common::pico_svg::PicoSvg::load` - the
real, public entry point - directly with a hand-crafted SVG string. Per CONFIRMATION.md section 11
this is the exempt single-participant case: a malformed SVG document *is* the genuine external
input a real caller (an application embedding `vello_common` with the `pico_svg` feature, or one
of the two in-repo consumers `vello_hybrid`/`vello_example_scenes`) would hand to `load` from an
untrusted or merely malformed file; there is no second participant/peer being impersonated.

## Class

crash (unhandled panic on malformed input to a public API - a denial-of-service in any host
application that calls `PicoSvg::load` on file content it doesn't fully control, e.g. user-supplied
SVG assets)

## Root cause

`PicoSvg::load` is designed to return `Result<Self, Box<dyn Error>>` so parse failures can be
reported to the caller, and most of the file already does this correctly (`Document::parse(...)?`,
`node.attribute("d").ok_or(...)?`, `BezPath::from_svg(d)?`, and the `translate(...)`/`scale(...)`
transform branches all fail gracefully). Two spots didn't get the same treatment:

1. `viewBox`'s field list is parsed with `f64::from_str(s).unwrap()` inside a `.map()` used to
   build a `Vec<f64>` - a single non-numeric token (e.g. `viewBox="0 0 100 NaNpx"`) panics instead
   of the surrounding `.and_then` simply returning `None` (which the code already does correctly
   for the *wrong-length* case two lines down, via `else { None }`).
2. `parse_transform`'s `matrix(...)` branch parses its argument list with
   `.collect::<Result<Vec<f64>, _>>().expect(...)` (panics on any non-float argument) and then
   `vals.try_into().expect(...)` (panics unless exactly 6 arguments were given) - both `expect`
   calls unconditionally propagate the failure as a panic, whereas the `translate(...)`/`scale(...)`
   branches immediately below use `if let Ok(vals) = ... { match vals.as_slice() { ... } } else {
   Affine::IDENTITY }` to fall back to identity on the same two failure shapes.

Both are reachable purely by choosing the SVG text passed to the real public `PicoSvg::load`
entry point - no unsafe code, no internal invariant violation, just untrusted text reaching an
`unwrap`/`expect` before validation.

## Introduced by

f57a1d1b "`vello_hybrid` implementation (#831)" (2025-03-13) - this is when `pico_svg.rs` was
copied into `sparse_strips/vello_common` (per `git log -S`/`git blame` on both the `viewBox`
`.unwrap()` line and the `matrix(...)` `.expect(...)` lines, both attributed to this commit, and
confirmed by `git log --follow` showing no earlier history for the file in this location). The
`pico_svg.rs` header comment ("Below is copied, lightly adapted, from Vello.") suggests the
`unwrap`/`expect` calls may predate even this commit in the original Vello-classic source this was
adapted from, but that predecessor is out of scope for this repo's own history.

## The fix

Changed both spots to fail gracefully instead of panicking, mirroring the pattern already used by
`translate(...)`/`scale(...)` immediately below in the same function:

- `viewBox`: `.map(|s| f64::from_str(s).unwrap()).collect()` became
  `.map(|s| f64::from_str(s).ok()).collect::<Option<Vec<f64>>>()?` - a malformed field now makes
  the whole `viewBox` attribute act as if it were absent (the surrounding `.and_then` already
  returns `None` via `?`), exactly like the pre-existing wrong-field-count fallback two lines down.
- `matrix(...)`: replaced both `.expect(...)` calls with `.ok()`/`.and_then(...)`/`.unwrap_or(...)`,
  falling back to `Affine::IDENTITY` on either a non-float argument or a wrong argument count -
  exactly the fallback the `translate(...)`/`scale(...)` branches already use for the identical
  failure shape.

Verified by re-running `repro/` against the patched tree: all three previously-panicking inputs
now return `Ok(_)` from `PicoSvg::load` (see `output.txt`), 0 panics, exit code 0.

- Project test suite (`cargo test -p vello_common --features pico_svg`): **160 passed; 0 failed; 1
  ignored** before the patch, **160 passed; 0 failed; 1 ignored** after the patch (identical -
  `pico_svg` has no dedicated unit tests exercising these exact lines, so the existing suite
  neither exercises this bug nor regresses from the fix). Doc-tests: 0/0 both before and after.

## What's not done / open questions

- `root.attribute("width")`/`("height")` already use `.and_then(|s| f64::from_str(s).ok())` (safe)
  and `stroke-width` already uses `.map(|a| f64::from_str(a).unwrap_or(1.0))` (safe) - both checked
  while reading this file and found already-graceful, so not part of this fix.
- `parse_color`'s fallback-to-fuchsia-on-error and `modify_opacity`'s `.unwrap_or(...)` calls were
  also read and are already graceful (no panic on malformed color/opacity text) - not part of this
  candidate.
- A longer fuzz campaign (via `fuzz-bug-repro`, run separately) over `PicoSvg::load`'s full
  attribute surface (not just `viewBox`/`matrix`) was not run this session; the two remaining
  `expect`/`unwrap` calls found by manual reading were the only ones identified in this file, but a
  dedicated fuzz harness could confirm there's nothing else missed by inspection alone.
- Whether the same original Vello-classic `pico_svg.rs` this was "lightly adapted" from has the
  identical bug was not checked - out of scope (this repo's own history and public API only).
