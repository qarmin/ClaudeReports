# gdcm-pvrg-kakadu-codec-unreachable-dead-code

From `IDEAS.md`'s backlog (2026-08-25 entry, a follow-on from candidate `100`): `gdcmBitmap.cxx`'s
`Bitmap::TryPVRGCodec` and `Bitmap::TryKAKADUCodec` each guard their final `memcpy` only with a
compiled-out `assert(len <= outbv->GetLength())`, the same unchecked shape as `100`'s confirmed
`TryJPEGLSCodec` bug. The open question left by `100` was whether either codec is actually
reachable from `gdcm_rs`'s transfer-syntax dispatch in the exact build this repo produces.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `gdcmBitmap.cxx`'s `Bitmap::GetBuffer` dispatch (the only caller of both `TryPVRGCodec`/`TryKAKADUCodec` anywhere in `GDCM/Source/`, confirmed via `grep -rn` across the whole tree) | vendored GDCM 3.1 sources bundled inside `gdcm-rs-0.6.0` (pinned exact version, checked against this repo's own `Cargo.lock`) | `TryKAKADUCodec`'s only call site (`gdcmBitmap.cxx:1005`) is commented out (`//if( !success ) success = TryKAKADUCodec(buffer, lossyflag);`) - it is unconditionally dead code, never invoked from anywhere, regardless of build configuration. `TryPVRGCodec` *is* called (line 1004), but only as a fallback after `TryJPEGCodec` fails, and its entire body is gated behind `if( codec.CanDecode(ts) )` |
| Code reading of `gdcmPVRGCodec.cxx`'s `PVRGCodec::CanDecode`/`::Decode`, and `GDCM/Source/MediaStorageAndFileFormat/CMakeLists.txt` + `GDCM/CMakeLists.txt`'s `GDCM_USE_PVRG`/`GDCM_USE_KAKADU` option defaults, cross-checked against `gdcm-rs-0.6.0/build.rs` | Confirmed `build.rs` invokes `cmake::Config::new("GDCM")` with only `GDCM_BUILD_TESTING`/`GDCM_DOCUMENTATION`/`GDCM_BUILD_EXAMPLES`/`GDCM_BUILD_DOCBOOK_MANPAGES`/`CMAKE_CXX_STANDARD` set - it never touches `GDCM_USE_PVRG`/`GDCM_USE_KAKADU`, which both default to `option(... OFF)` in `GDCM/CMakeLists.txt` | `PVRGCodec::CanDecode` is wrapped in `#ifndef GDCM_USE_PVRG ... return false; #else ...`, i.e. with the macro undefined (the only way this crate ever builds it) the function is compiled to unconditionally `return false`, so `Bitmap::GetBuffer`'s `if( codec.CanDecode(ts) )` guard around the vulnerable `memcpy` can never be true. Even a hypothetical `GDCM_USE_PVRG=ON` build would additionally require an external `pvrg-jpeg`/`gdcmjpeg` executable present on disk at runtime (`System::FileExists(pvrg_command.c_str())` inside `PVRGCodec::Decode`) - a second independent gate this crate ships nothing for |
| Inspection of this repo's own actual build artifacts (`target/{release,debug}/build/gdcm-rs-*/out/build/CMakeCache.txt`, produced by a real `cargo build --features gdcm` run in this checkout) | `grep -i "GDCM_USE_PVRG\|GDCM_USE_KAKADU" .../CMakeCache.txt` | `GDCM_USE_KAKADU:BOOL=OFF` and `GDCM_USE_PVRG:BOOL=OFF` confirmed in the real, already-built configuration this repo's own `pixeldata` crate links against - not just the CMakeLists.txt default, the actual cache from this project's build |
| Standalone dynamic confirmation: a small C++ program (`#include <gdcmPVRGCodec.h>`/`<gdcmKAKADUCodec.h>`, `PVRGCodec().CanDecode(TransferSyntax::JPEGBaselineProcess1)` / `KAKADUCodec().CanDecode(TransferSyntax::JPEG2000)`) compiled and linked against this repo's own already-built static libraries (`target/release/build/gdcm-rs-e9d0db321e440218/out/lib/libgdcmMSFF.a` + friends) | `g++ -std=c++14 ...`, run once, deterministic (no randomness/input dependence in this call) | Program compiled, linked, and ran cleanly; printed `PVRG CanDecode(JPEGBaselineProcess1) = 0` and `KAKADU CanDecode(JPEG2000) = 0` - i.e. both codecs report they cannot decode *any* transfer syntax in the exact binary this repository's own build produces, confirming the static-code-reading conclusion empirically against the real compiled artifact rather than just the source |

## Status

**Not reproducible.** Both `Bitmap::TryPVRGCodec` and `Bitmap::TryKAKADUCodec` are unreachable in
the actual `gdcm-rs 0.6.0` build this repository (and any default `cargo build --features gdcm` of
`dicom-pixeldata`) produces:

- `TryKAKADUCodec` has no live call site at all (commented out in `Bitmap::GetBuffer`) -
  unconditionally dead code independent of any build flag.
- `TryPVRGCodec` is called, but its vulnerable body is entirely behind `PVRGCodec::CanDecode(ts)`,
  which is compiled to `return false` unconditionally because `GDCM_USE_PVRG` is never defined
  (confirmed against both the CMakeLists.txt default and this repo's own real `CMakeCache.txt`),
  and empirically confirmed to return `0` (false) at runtime via a real program linked against this
  repo's own compiled `libgdcmMSFF.a`.

This closes the open question left in candidate `100`'s "What's not done" section for good: no
repro was attempted (there is no reachable path to attempt one through), consistent with
`CONFIRMATION.md`'s guidance that a definitive negative from reading real, pinned source plus this
project's own real build configuration (not a guess at what a hypothetical config might do) is a
valid, complete "Not reproducible" result on its own, without needing a fuzz/sanitizer run against
a path that provably cannot execute.

**Fully reproducible**: n/a (Not reproducible - no bug to reproduce).

## Class

n/a (Not reproducible).

## Root cause

Not a bug in the reachable sense: the `assert(len <= outbv->GetLength())`-then-unchecked-`memcpy`
pattern in both functions is real and shares candidate `100`'s vulnerable shape verbatim, but
neither function's vulnerable code ever executes in this crate's actual build:

1. `TryKAKADUCodec`'s call from `Bitmap::GetBuffer` is source-level commented out - upstream GDCM
   itself never enables this path, for any caller, not just `gdcm-rs`.
2. `TryPVRGCodec` is reached from `Bitmap::GetBuffer`, but only after `TryJPEGCodec` (the IJG
   libjpeg-based decoder, which handles all the same JPEG-family transfer syntaxes `PVRGCodec`
   claims to support - `JPEGBaselineProcess1` through `JPEGLosslessProcess14_1`) has already been
   tried and failed. Even in that fallback case, `PVRGCodec::CanDecode` is preprocessor-gated on
   `GDCM_USE_PVRG`, a CMake option `gdcm-rs`'s `build.rs` never sets and which defaults `OFF`
   upstream - so the function body compiled into this crate always returns `false`, and
   `Bitmap::GetBuffer`'s guard around the vulnerable code is never satisfied.

## The fix

No fix needed/attempted - there is nothing to patch on a path that cannot execute in this crate's
build. (Note for any future GDCM upgrade or build-flag change: if `gdcm-rs` or a downstream
consumer ever sets `GDCM_USE_PVRG=ON` and ships a `pvrg-jpeg`/`gdcmjpeg` executable, `TryPVRGCodec`
would become reachable and should get the same real, always-compiled `if (len > outbv->GetLength())
{ ...; return false; }` guard candidate `100` recommended for `TryJPEGLSCodec`, matching the
pattern `TryJPEGCodec` already uses.)

- Project test suite: not run before/after, since no code was changed - N/A (no fix attempted).

## What's not done / open questions

- This only rules out `PVRGCodec`/`KAKADUCodec` as reached through `gdcm::Bitmap::GetBuffer` (the
  path `gdcm_wrapper.cc`/`pixeldata/src/gdcm.rs` actually uses via `gdcm::ImageReader`). It does
  not audit every other internal GDCM entry point that might independently construct a
  `PVRGCodec`/`KAKADUCodec` and call `Decode` directly (none were found via the `grep -rn` above,
  which covered the whole `GDCM/Source/` tree, but a narrower manual read of just
  `MediaStorageAndFileFormat` callers was what actually happened, not a full-repo semantic audit).
- Not reported upstream to the GDCM project - `TryKAKADUCodec`'s dead call site and
  `TryPVRGCodec`'s missing non-assert guard are both real latent defects in GDCM's own source
  (would matter to any caller that *did* build with `GDCM_USE_PVRG=ON`), just not reachable through
  `gdcm-rs`/`dicom-rs` today - same "not reported upstream" caveat already on record for candidates
  `091`/`100`.
- If `gdcm-rs` ever bumps its GDCM submodule or changes `build.rs` to enable `GDCM_USE_PVRG`, this
  candidate's negative result would need re-checking - it is a property of the current build
  configuration, not an inherent impossibility in the source.
