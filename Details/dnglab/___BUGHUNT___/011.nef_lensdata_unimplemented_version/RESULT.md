# NEF LensData decoder panics on an unrecognized (but validly-formatted) lens-data sub-format version

`rawler::decoders::nef::lensdata::from_makernote` (rawler/src/decoders/nef/lensdata.rs:98) hits
`_ => todo!("Lensdata version: 0x{:x} not implemented", version)` for any Nikon Makernote LensData
tag (0x0098) whose 4-byte ASCII version prefix doesn't match one of the explicitly handled values
(0x100, 0x101, 0x201-0x204, 0x400-0x403, 0x800-0x802). This is a real-user-reported bug: GitHub
issue #583 ("not yet implemented: Lensdata version: 0x9e1ec not implemented"), still open. Every
`.nef` file goes through `NefDecoder::get_lens_description` -> `lensdata::from_makernote` as part
of `Decoder::raw_metadata`, which `dnglab convert`/`dnglab extract`/`makedng`/`rawler::analyze` all
call unconditionally as part of a normal decode.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `lensdata.rs`/`nef.rs` to trace the real call path from `rawler::get_decoder`/`Decoder::raw_metadata` down to the `todo!()` | - | Confirmed `raw_metadata` (rawler/src/decoders/nef.rs:326-334) already wraps `get_lens_description()` in a `match Ok/Err` that logs a warning and degrades gracefully on any *returned* `Err` - but `todo!()` is a panic, not a `Result::Err`, so it completely bypasses that existing graceful-degradation path |
| Scan of this project's own cached NEF test corpus (`target/rawdb_cache/Nikon/**/*.NEF`, ~47 files) via a temporary in-crate test using the crate's own internal TIFF/Makernote parsing to print each file's actual LensData version + the absolute file offset of the LensData tag's data blob (byte-search for the already-extracted data blob in the raw file) | debug build | Found a real NIKON D2H NEF (from this repo's own rawdb cache) with an unencrypted (version 0x101, no `nef_decrypt` needed) LensData tag at a known absolute file offset (8032), letting a real file be minimally patched instead of hand-built from scratch |
| Standalone repro: the D2H NEF file above, copied verbatim, with exactly the 4-byte plaintext ASCII version prefix of its LensData tag changed from "0101" to "0900" (parses to version 0x900, an unimplemented value; every other byte including the full image data section is untouched real camera output), opened via the real, public, top-level `rawler::get_decoder` + `Decoder::raw_metadata` | debug build, 3 runs; release build, 3 runs | Deterministic panic, 6/6 runs, identical panic location/message (`lensdata.rs:98:12`, "not yet implemented: Lensdata version: 0x900 not implemented"); backtrace confirms the real path `repro::main -> NefDecoder::raw_metadata -> lensdata::from_makernote` |
| Real end-to-end via the actual `dnglab` CLI binary (`dnglab convert crafted.NEF out.dng`) - the literal command a user hitting issue #583 would run | release build, 3 runs | Deterministic: panics in a rayon worker thread, rayon's panic strategy escalates it to `abort()` (SIGABRT, exit 134, core dump), 3/3 runs |
| Second boundary value in the same class: same file, version bytes changed to "0999" (0x999) instead of "0900" | release build (pristine and patched), 1 run each | Also panics/aborts on pristine tree; also succeeds (with a graceful warning) on the patched tree - confirms the fix isn't narrow to the one specific version value first tried |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed.** Reproduces 6/6 runs at the library level (3 debug + 3 release) and 3/3 runs through
the real `dnglab` CLI binary end-to-end, fully deterministic - not gated by
`overflow-checks`/build profile, a plain `todo!()` panic reached whenever `version` doesn't match
any handled arm.

**Fully reproducible**: **Yes**. `repro/` opens a real, minimally-patched-but-otherwise-genuine
NEF file (a real Nikon D2H raw file from this project's own rawdb test corpus, 4 bytes changed)
through the unmodified, public, top-level `rawler::get_decoder` + `Decoder::raw_metadata` - the
exact function every `dnglab`/rawler-based caller invokes for every `.nef` file. This was also
independently confirmed by building the actual `dnglab` binary and running
`dnglab convert crafted.NEF out.dng` on the same file, reproducing the real user-facing crash
(SIGABRT via rayon's panic-to-abort escalation) reported in GitHub issue #583. Single-participant
case (a malformed-but-real file handed to a decoder's public entry point) - no second participant
to fake, per `CONFIRMATION.md` section 11.

## Class

`crash`.

## Root cause

`lensdata::from_makernote` computes `version` as a `u32` built by treating the LensData tag's
first 4 bytes as ASCII decimal digits (`version = (version << 4) + (buf[i] - b'0')` for `i` in
`0..4`), then matches `version` against a fixed, hand-maintained list of Nikon lens-data
sub-format versions the decoder happens to implement (0x100, 0x101, 0x201-0x204, 0x400-0x403,
0x800-0x802 - i.e. real Nikon firmware version strings like "0100", "0201", "0800"). Any NEF whose
LensData version isn't one of those - a legitimately different/newer Nikon firmware sub-format, or
(per the real GitHub report, version 0x9e1ec, five hex digits from a 4-nibble construction) a
LensData blob whose first bytes aren't plain ASCII digits at all, causing `buf[i] - b'0'` to
overflow past 4 bits per byte - falls through to `_ => todo!(...)`, an unconditional panic.

The panic is especially misleading because a sibling problem was already fixed correctly one
level up: commit 1326f03f ("nef: Relax errors when lens data is not available", fixes #650)
rewrote `NefDecoder::raw_metadata` (rawler/src/decoders/nef.rs:326-334) to catch any `Err` from
`get_lens_description()` and degrade gracefully (log a warning, return metadata with no lens
info) instead of propagating the error and failing the whole decode. That fix works for every
error `from_makernote` returns as a proper `Result::Err` - but the `todo!()` arm is a panic, which
unwinds straight past that `match Ok/Err` (and, in the real `dnglab` CLI, past rayon's worker
thread entirely, escalating to a process `abort()`) without ever reaching it.

## Introduced by

ab787d9d "Nikon Lens data decoder" (2022-04-23) - the commit that introduced `lensdata.rs` and
the `todo!()` fallback arm itself (confirmed via `git log -S'not implemented'` and `git log -p
--follow` on the file; the line is unchanged since that commit, including through commit
1326f03f which touched the same file for a related but different fix without changing this line).

## The fix

Changed the fallback arm from `todo!("Lensdata version: 0x{:x} not implemented", version)` to
`return Err(format!("Lensdata version: 0x{:x} not implemented", version).into())` - one line,
turning the panic into the function's own existing `Result` error path (identical shape to every
other error this function already returns, e.g. `.ok_or(ERRMSG)?` for a short buffer). This is
picked up unmodified by `NefDecoder::raw_metadata`'s existing `match Ok/Err` (added by 1326f03f)
which already does exactly the right thing for any `Err` from this function: log a warning and
return usable metadata without lens info, instead of crashing.

Verified against two different unimplemented version values (0x900 and 0x999, both outside every
handled arm) - both panicked/aborted on the pristine tree and both now complete successfully (with
a logged warning, no lens info in the output) on the patched tree, through both `repro/`'s
library-level call and the real `dnglab convert` CLI end-to-end.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises this exact fallback arm, so this fix
  is additive coverage, not a regression risk.

## What's not done / open questions

- The underlying `version` computation itself (`buf[i] - b'0'`, `i` in `0..4`) assumes the first 4
  bytes are ASCII decimal digits and doesn't validate that - a non-digit byte can produce a
  `version` value with more than 4 significant hex digits (as in the real issue #583 report,
  0x9e1ec), which is harmless today (any such value still safely falls into the now-fixed `_ =>`
  arm) but is itself a slightly surprising computation worth a maintainer's attention if new
  version arms are ever added that assume `version` stays within a 16-bit range.
- Not checked this session: whether any *other* Nikon-makernote-adjacent decoder path
  (`parse_lensdata_0x100`/`0x101`/`0x204`/`0x4xx`/`0x800` themselves, or `nef_decrypt`) has its own
  `todo!()`/`unimplemented!()`/`unreachable!()` reachable via a different malformed-but-plausible
  buffer shape within an already-recognized version - this candidate only targeted the
  version-dispatch fallback itself.
- The five real cached files this session found already using encrypted versions (0x201-0x204,
  0x400-0x403, 0x800-0x802, requiring `nef_decrypt` before `version` dispatch runs) were not used
  for the repro (the unencrypted 0x101 D2H file was simpler and sufficient to demonstrate the same
  `_ => todo!()` arm) - the fix applies identically regardless of whether decryption ran first,
  since `version` is read from the plaintext prefix (`nef_decrypt` starts at buffer offset 4,
  `version` is read from offset 0..4) before any encrypted-path branch is taken.
