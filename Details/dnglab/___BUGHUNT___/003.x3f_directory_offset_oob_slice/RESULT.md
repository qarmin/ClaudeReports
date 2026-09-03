# X3F decoder: unchecked directory-offset slice panics on any truncated/malformed file

`rawler::decoders::x3f::X3fFile::new` reads a 32-bit directory offset from the last 4 bytes of an
X3F (Sigma Foveon) file and immediately slices the whole file buffer at that offset with no bounds
check - `HEURISTICS.md`'s "index/slice arithmetic near a boundary" and "arithmetic on
sizes/indices derived from input" patterns, on a decoder that's the very first thing
`rawler::get_decoder` (and therefore any `dnglab`/rawler-based tool) runs when opening a file
whose first 4 bytes are `FOVb`. Found via a short (~25 min, unseeded, no starting corpus)
`cargo fuzz` run of the existing `fuzz_get_decoder` target already in this repo's `rawler/fuzz/`
- see `../README.md`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo fuzz run fuzz_get_decoder` (existing harness in this repo, ASAN + debug-assertions, default `cargo fuzz` build flags) | release+ASAN+debug-assertions build, ~25 min wall clock, 2 workers, no seed corpus | Found a 5-byte crashing input within the run (artifact saved as `crash-3be38e...`); the same underlying bug also crashed `fuzz_decode_metadata` and `fuzz_decode_full` on their own independently-found inputs (same file, same root cause - not filed as separate candidates, see "What's not done") |
| Code reading of the crash location (`x3f.rs:39-62`) | - | `let offset = LEu32(&buf, buf.len() - 4) as usize; let data = &buf[offset..];` - `offset` is fully attacker-controlled (any `u32`) with no bounds check against `buf.len()`; the identical unchecked pattern also appears in `X3fDirectory::new` (`x3f.rs:65-66`) and `X3fImage::new` (`x3f.rs:76-77`), both called from the same parsing loop |
| Standalone repro: the exact 5-byte fuzzer-found input (`FOVb\n`) written to a real file on disk, opened via the real, public, top-level `rawler::get_decoder` (the same function `dnglab`'s CLI calls first for every raw file) | debug build, 3 runs; release build, 1 run | Deterministic panic, 3/3 debug runs + 1/1 release run, identical panic message and location each time |
| Boundary-value sweep after applying the fix: 4-byte magic-only, 5-byte (original), 8-byte with offset=0 (in-range - should still work), 12-byte with `offset=u32::MAX`, and a 0-byte empty file | debug build | All 5 return a clean `Err(...)`, no panic; the offset=0 in-range case correctly proceeds past the fix to the pre-existing "Directory version too old" check, confirming the fix doesn't reject valid offsets |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed.** Reproduces 3/3 debug runs and 1/1 release run, fully deterministic (the panic
message includes the literal byte offset computed from the fixed 5-byte input, so it is not
layout/seed-dependent). Not gated by `overflow-checks`/build profile - a plain out-of-range slice
index panics identically in debug and release, unlike candidates 001/002 in this workspace.

**Fully reproducible**: **Yes**. `repro/` writes the real 5-byte malformed file to disk and opens
it through the unmodified, public, top-level `rawler::get_decoder` - the exact function every
`dnglab`/rawler-based caller invokes as the first step of opening any raw file. Single-participant
case (a malformed file handed to a parser's real public entry point) - no second participant to
fake, per `CONFIRMATION.md` section 11.

## Class

`crash`.

## Root cause

`X3fFile::new` (`x3f.rs:40-43`):

```rust
fn new(file: &RawSource) -> Result<X3fFile> {
  let buf = file.as_vec()?;
  let offset = LEu32(&buf, buf.len() - 4) as usize;
  let data = &buf[offset..];
  ...
```

X3F stores its directory offset in the last 4 bytes of the file (a standard footer-pointer
layout). `offset` is read directly from file bytes with no validation, then used to slice the
*entire file buffer* (`&buf[offset..]`). Any file whose stored offset exceeds the file's actual
length - trivially true for almost any malformed, truncated, or adversarially-crafted file, since
the 4 offset bytes can be anything - panics immediately with a slice-index-out-of-range message,
before any real parsing happens. `X3fDirectory::new` and `X3fImage::new` have the identical
pattern one level down (an attacker-controlled `offset` used to slice `buf` with no check), reached
from the same directory-entry loop in `X3fFile::new` once a valid-looking top-level offset is
found but a per-entry offset is bogus.

## The fix

Replaced all three unchecked `&buf[offset..]` slices with `buf.get(offset..).ok_or_else(|| ...)?`,
turning an out-of-range offset into the function's existing `Result` error path (consistent with
how this file already reports other malformed-input conditions, e.g. "Directory version too old")
instead of a panic. Also added an explicit `buf.len() < 4` check in `X3fFile::new` before the
`buf.len() - 4` subtraction, closing off the same underflow class defensively even though it
wasn't independently observed as reachable this session (`is_x3f`'s own 4-byte magic check already
guarantees `buf.len() >= 4` for any file that reaches this decoder in practice).

Verified: `repro/` (the exact fuzzer-found 5-byte input) no longer panics and returns a descriptive
error instead, in both debug and release. Re-verified against 4 additional boundary variants
(4-byte magic-only, 8-byte with an in-range `offset=0`, 12-byte with `offset=u32::MAX`, and an
empty file) - all handled cleanly, and the in-range case confirms the fix doesn't reject valid
input, it only rejects genuinely out-of-range offsets.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises the X3F decoder with malformed
  input, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- `X3fDecoder::raw_image` (`x3f.rs:105-135`, reached only *after* `X3fFile::new` succeeds with a
  well-formed-enough directory) has further unchecked slices on attacker-influenced offsets -
  `&buffer[caminfo.doffset + 6..]` and `&buffer[offset..]` (`imginfo.doffset`) - that this
  candidate's fix does not touch. These weren't independently confirmed reachable this session
  (would need a file with a valid-looking directory but a bogus `caminfo`/`imginfo` entry,
  distinct from the trivial truncated-file crash fixed here) - filed as a follow-up lead in
  `IDEAS.md` rather than fixed speculatively.
- The same short fuzzing run also found: a `CiffIFD::new` (`formats/ciff/mod.rs:59`) subtraction
  overflow on a completely empty (0-byte) input, and a `FileBox::parse` (`formats/bmff/vendor.rs:15`)
  addition overflow on a 54-byte crafted input, plus a hang in a Minolta-MRW-shaped input via the
  `#[doc(hidden)]` fuzzing-only `decode_unwrapped` entry point. None of these share a root cause
  with the X3F bug fixed here - each is a distinct candidate. Not investigated further this
  session due to time; the exact crash inputs are saved in `rawler/fuzz/artifacts/` and cited by
  file/line in `IDEAS.md` so a future session can pick them up directly without re-fuzzing.
