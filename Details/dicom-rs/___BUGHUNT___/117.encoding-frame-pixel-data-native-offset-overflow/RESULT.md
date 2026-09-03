# encoding-frame-pixel-data-native-offset-overflow

Found via this session's GitHub issue raw-sync: upstream issue `Enet4/dicom-rs#817`
("dicom-encoding: unchecked usize arithmetic leading to silent frame corruption", open, no linked
PR as of `df2b9e42`) reports a complete, runnable reproduction of an integer-overflow panic (debug)
/ potential silent frame aliasing (release, demonstrated by the reporter on a 32-bit target) in
`encoding/src/adapters.rs`'s `PixelDataObject::frame_pixel_data` default method, for native
(non-encapsulated) pixel data. This is the same "two-plus attacker-influenced integers multiplied
in a type narrower than their combined range" class already confirmed in the sibling crate
`pixeldata/src/lib.rs` (candidates 086/087), but in a different crate/function neither of those
candidates covered.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `frame_pixel_data`'s native-pixel-data branch (`encoding/src/adapters.rs`, the `None => { ... }` arm of the `number_of_fragments()` match) | n/a | Confirmed the exact mechanism the issue describes: `start = frame as usize * frame_size` (and the `bits_allocated == 1` special case's own `frame as usize * samples_per_frame`) is plain, unchecked `usize` multiplication/addition, with the `end <= pixel_data.len()` bounds check only applied *after* the arithmetic already happened |
| Standalone repro adapting the issue's own reporter-supplied `PixelDataObject` implementation (65535x65535 RGB/16-bit object, `NumberOfFrames = u32::MAX`, requesting the last frame) via the real public trait method, `cargo run` (debug/dev profile, overflow-checks on by default) | 3 repeat runs | Reproduced 3/3: `thread 'main' panicked at .../encoding/src/adapters.rs:272:33: attempt to multiply with overflow`, identical message and location every run - matches the issue's own reported panic site exactly |

No further tools were needed - the issue itself already supplied a complete, minimal, exact
reproduction; independently re-deriving and re-running it against this session's own checkout was
sufficient to confirm the bug is still live on `df2b9e42` (no upstream fix has landed since the
issue was filed).

## Status

**Confirmed** - deterministic, pure integer-overflow panic with no layout/seed/timing dependency:
reproduces 3/3 runs, debug/dev profile (overflow-checks on by default in this workspace - no
custom `[profile.dev]`/`[profile.release]` override for `overflow-checks` exists in the root
`Cargo.toml`).

**Debug build only** was independently exercised this session. A `--release` build with these
exact inputs was not separately tested, but per the issue's own worked 32-bit demonstration (frame
65536 wrapping to alias frame 0's bytes) and this session's own sibling candidates 086/087 (which
documented the identical "release doesn't always panic, and a coincidental wrap could silently
return the *wrong* frame's data rather than merely surviving the overflow by chance" caveat for
the same bug class in a different function), the release-mode risk described in the issue title
("silent frame corruption") is accepted as real without an independently re-derived 64-bit
wraparound-into-bounds demonstration this session - constructing one requires solving for exact
modular-arithmetic values and was judged not worth the time given the debug-mode crash alone is
already sufficient to confirm and fix the underlying defect.

**Fully reproducible**: Yes. The repro drives the real, public
`PixelDataObject::frame_pixel_data` default trait method end-to-end via a real implementor of the
crate's own public trait - the same public entry point any real caller (a pixel-data adapter, a
higher-level `DecodedPixelData` built on top of this trait) would use. There is no second "peer"
process here: the untrusted input this bug is about is the object's own declared attribute values,
supplied through the exact same public trait implementation a real object would use, not a
hand-constructed stand-in for another process's behavior.

## Class

crash (an `attempt to multiply with overflow` panic in a debug/overflow-checked build, reachable
from ordinary use of the public `PixelDataObject::frame_pixel_data` default method on an
individually-legal-but-adversarially-combined DICOM object; the issue additionally alleges a
release-mode silent-data-corruption manifestation of the same defect, accepted as real per the
reasoning above but not independently re-demonstrated this session).

## Root cause

`frame_pixel_data`'s native-pixel-data branch computes `frame_size` from
`Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` (all individually legal, `US`-typed DICOM
attributes, so each is bounded only by 16 bits / 65535), then computes `start = frame as usize *
frame_size` and `end = start + frame_size` using a caller-supplied `u32` frame index - entirely in
plain, unchecked `usize` arithmetic, with the `end <= pixel_data.len()` length check applied only
*after* the multiply/add already happened. The `bits_allocated == 1` special case has the identical
shape (`frame as usize * samples_per_frame`, plus a further `+ 1` before the second multiply).
Neither `rows`/`columns`/`samples_per_pixel`/`bits_allocated` individually, nor `frame` alone, is
out of range for its own attribute/parameter type - only their *combined* product overflows: with
`Rows=Columns=65535`, `SamplesPerPixel=3`, `BitsAllocated=16` (2 bytes/sample) and a
`frame` near `u32::MAX`, the product vastly exceeds even 64-bit `usize::MAX`. In a debug build this
is a hard, immediate panic; in a release build (overflow-checks off by default), the wrapped value
can - as the issue's own 32-bit demonstration shows concretely - happen to land back inside the
actual buffer's bounds, silently returning a *different, wrong* frame's bytes instead of erroring
or panicking.

## Introduced by

61e43337 "[encoding] Add function to access frame pixeldata" (the commit that introduced
`frame_pixel_data`'s native-pixel-data branch with this exact unchecked arithmetic shape).

## The fix

Replaced every multiply/add in both branches (the `bits_allocated == 1` special case and the
general case) with `checked_mul`/`checked_add`, short-circuiting to `None` via `?`/an early
`let...else` as soon as any step overflows - matching the function's own existing contract of
returning `None` for "no such frame" rather than trusting the arithmetic to always fit. This is
the same graceful-rejection behavior the function already provides for a frame index that is
merely too large for the buffer (the pre-existing `end <= pixel_data.len()` check) - overflowing
frame indices now take the same "no such frame" path instead of panicking (debug) or silently
computing a wrong offset (release).

Verified against the repro: before the fix, the exact panic from the issue report (3/3 runs);
after the fix, a clean `None` with no panic - see `output.txt` for both full runs.

- Project test suite: `cargo test -p dicom-encoding` - 40 passed, 0 failed, 0 ignored (lib) + 4
  passed, 0 failed, 0 ignored (doctests) before AND after (identical counts, zero regressions).
  Full workspace `cargo test --workspace --exclude dcms` (dcms excluded: untracked scratch crate
  belonging to a different session, not part of this workspace's tracked tree) - 0 failures
  anywhere before or after.

## What's not done / open questions

- A genuine 64-bit release-mode wraparound-into-bounds demonstration (a constructed
  `frame`/`frame_size` pair whose product wraps `usize` to a small value that still falls within
  `pixel_data.len()`, producing `Some(wrong_bytes)` instead of `None`) was not independently built
  this session - accepted as real by extension of the issue's own 32-bit demonstration and this
  session's own sibling candidates 086/087's identical documented caveat, but not directly proven
  on a 64-bit target. Constructing one is a solvable modular-arithmetic search, not a fundamentally
  different bug class, if a future session wants full independent 64-bit confirmation.
- Not checked: whether the encapsulated-pixel-data branches of the same function (the `Some(..) if
  ... ==` and `Some(..)` match arms just above the native-data branch this candidate fixes) share
  any comparable unchecked-arithmetic risk - a quick read suggests they use `offset_table` entries
  (already `u32`-bounded, summed via ordinary `+=` into a `usize` accumulator with no attacker
  control over the number of terms beyond `number_of_fragments`, itself bounded by the actual
  fragment count) rather than a raw multiply of two large attacker-controlled factors, so this
  specific overflow shape looks less likely to apply there - not independently verified with its
  own repro this session.
- Not checked: whether any higher-level caller in `pixeldata`/`object` reaches this exact
  `PixelDataObject::frame_pixel_data` default method directly (as opposed to `pixeldata/src/
  lib.rs`'s own separate `DecodedPixelData::frame_data`, already covered by candidates 086/087) -
  if so, the same fix should already cover that call path since it lives in the shared default
  trait method, but this wasn't independently traced this session.
