# DICOM+JSON FD/FL values can change value across a deserialize/serialize cycle (serde_json float precision)

Found during a discovery round (no seeded lead, though the pattern deliberately echoes a prior
fuzz-bug-repro campaign's own `fix-json-float-roundtrip-precision` bundle, whose patch no longer
exists on disk in this checkout - suspected it might, like several other campaign findings this
session, never have actually been merged upstream, and confirmed that directly against current
dependencies rather than assuming the old notes were still accurate).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Dependency check | `grep serde_json json/Cargo.toml` | `serde_json = { version = "1.0.96", features = ["preserve_order"] }` - the `float_roundtrip` feature (which serde_json's own docs state trades default speed for guaranteed exact float round-tripping) is not enabled. |
| Isolated reproduction (release build, plain `serde_json` 1.0.151 - the version this workspace's `Cargo.lock` currently resolves to, newer than what the original campaign tested against) | A minimal standalone program calling `serde_json::from_str::<f64>("0.09510497055947781")` and comparing the bit pattern against `str::parse::<f64>()` on the same literal (Rust's own `FromStr` for `f64`, which round-trips exactly). | Confirmed the precision loss still reproduces with the current pinned dependency version: `serde_json::from_str` gives `0.09510497055947779443...` (bit pattern `3fb858cca23d70a3`) vs. the literal's true nearest `f64` value `0.09510497055947780831...` (bit pattern `...a4`) - genuinely a different `f64`, one ULP off, not just a different printed string. |
| Standalone repro (release build, `dicom-json`'s public API: `dicom_json::from_str`/`to_string`) | `___BUGHUNT___/011.../repro`. A DICOM+JSON document with the same literal in an `FD`-VR element **nested inside a sequence item** (matching the prior campaign's own note that a flat/top-level `FD` element with the identical literal does *not* reproduce it - re-verified true again this session, not just carried over as an assumption). | Reproduced deterministically: deserializing then immediately re-serializing changed the JSON text from `0.09510497055947781` to `0.0951049705594778` - a real, externally-observable round-trip break through the crate's actual public API, not just the underlying `serde_json` behavior in isolation. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (floating-point parsing has no randomness). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (silent value corruption: a `FD`/`FL` DICOM value changes to a different, still
plausible-looking number across an ordinary read-then-write cycle, with no error anywhere in the
chain).

## Root cause

`json/Cargo.toml` depends on `serde_json` without its `float_roundtrip` feature. Per serde_json's
own documentation, deserializing a JSON number directly into an `f64` without that feature trades
precision for parsing speed, and can legitimately land one ULP off the nearest `f64` to the
literal actually written in the JSON text - this is a deliberate, documented trade-off in
serde_json itself, not a bug in serde_json, but it directly conflicts with this project's own
established round-trip invariant (a value read then written back, with no intervening
modification, must reproduce byte-for-byte / value-for-value where the format allows it -
`json_roundtrip`'s own fuzz target, and this crate's general design intent per its doc comments
about lossless DICOM+JSON conversion). `FD` (Other/Float Double) and `FL` (Float) VR values are
deserialized as ordinary JSON numbers into `f64`/`f32` via `serde_json`'s standard number
handling, so any value affected by this ULP-level imprecision changes on the very first
deserialize, independent of anything this crate's own code does afterward.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added the `float_roundtrip` feature to `dicom-json`'s `serde_json` dependency in `json/Cargo.toml`
(`features = ["preserve_order", "float_roundtrip"]`) - a one-line diff, per serde_json's own
documented purpose for this exact class of problem. See `patch.diff`.

Verified: `repro/` re-run against the patched tree now reports the FD literal unchanged
(`0.09510497055947781` in, `0.09510497055947781` out) across the deserialize/serialize cycle,
matching the earlier isolated `serde_json`-only reproduction (which also confirmed `with
float_roundtrip` gives bit-identical results to Rust's own exact `f64::from_str`, not shown as a
separate repro since it's the same underlying mechanism serde_json documents and this crate simply
opts into).

- Project test suite: `cargo test -p dicom-json --lib` - 12 passed / 0 failed, identical both
  before and after the patch. `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored,
  identical both before and after (no regressions, no newly-passing tests - no existing test
  constructed a nested FD/FL value hitting this exact precision boundary).

## What's not done / open questions

- Did not attempt to isolate a smaller/simpler reproducing literal or a flat (non-nested)
  variant beyond re-confirming the prior campaign's finding that nesting inside an SQ item is
  necessary for *this specific* literal - the underlying `serde_json` behavior itself is
  independent of DICOM structure (as shown by the isolated `serde_json`-only test, which needed no
  DICOM/JSON structure at all), so the nesting requirement is likely specific to which exact
  literal triggers a ULP-off parse, not a structural precondition of the bug itself - not
  re-investigated further since the fix is unconditional and doesn't depend on understanding why
  this particular literal needs nesting.
- Not fuzzed - found by direct dependency inspection (checking whether a documented,
  known-relevant Cargo feature was enabled) rather than fuzzing; the original campaign's own
  discovery method (per project memory) *was* fuzzing `json_roundtrip`, which found the same class
  of issue independently - a renewed short fuzz run against `json/fuzz/fuzz_targets/json_roundtrip.rs`
  (if that harness still exists and builds) could plausibly find further floating-point edge cases
  even with this fix applied, since `float_roundtrip` guarantees round-trip fidelity for the
  literal but doesn't change how DICOM's own FD/FL value formatting rules interact with very large
  or very small magnitudes.
