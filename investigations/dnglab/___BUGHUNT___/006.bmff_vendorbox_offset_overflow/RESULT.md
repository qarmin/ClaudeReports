# BMFF vendor-box parsing panics on `offset + size` integer overflow (debug builds)

`rawler::formats::bmff::vendor::VendorBox::read_box` computes `header.offset + header.size` (both
`u64`, both read directly from a BMFF box header's own file bytes) with no overflow check -
`HEURISTICS.md`'s "arithmetic on sizes/lengths/indices derived from input" pattern. Unlike
candidates 003/005 in this workspace, `formats::bmff::FileBox::parse` (which this is reached from)
is **not** gated behind any format-specific magic-byte check at all: `rawler::get_decoder`'s own
fallback chain calls `Bmff::new(&mut rawfile.reader())` unconditionally on any file it hasn't
already recognized as one of the earlier-checked formats, making this the broadest-reach crash
found this session - reachable from *any* unrecognized file handed to `rawler::get_decoder`, not
just files matching a specific raw format's signature.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo fuzz run fuzz_bmff_parse` (existing harness in this repo, same run that found candidates 003-005) | ~25 min, no seed corpus | Found a 54-byte crashing input - artifact saved at `rawler/fuzz/artifacts/fuzz_bmff_parse/crash-66d729698c6852e22325a4d22c672046c97cfd97` |
| Code reading of the crash location (`vendor.rs:13-18`) and how `get_decoder` reaches `FileBox::parse` | - | `Bmff::new` calls `FileBox::parse` directly (`formats/bmff/mod.rs:313-317`), and `get_decoder`'s own fallback chain (`decoders/mod.rs:958-963`) calls `Bmff::new` **unconditionally**, with no prior magic-byte gate the way `is_ciff`/`is_x3f` gate their formats - any file that reaches this point in the decoder-selection chain triggers real BMFF parsing |
| Standalone repro: the exact 54-byte fuzzer-found input written to a real file on disk, opened via the real, public, top-level `rawler::get_decoder` | debug build, 3 runs; release build, 1 run | Deterministic panic, 3/3 debug runs, identical location/message; **no panic in release** - see Status |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed**, but **debug-build-only** - unlike candidates 003/004/005 in this workspace. 3/3
debug runs panic deterministically. In a `--release` build (`overflow-checks = false`, both
dnglab's own `[profile.release]` setting and plain Rust's default `--release` behavior), the same
input does **not** panic: the addition silently wraps, `Seek::seek` either succeeds at the wrapped
position or fails gracefully through the function's own `?` operator, and `get_decoder` eventually
returns a clean `"No decoder found"` error instead. Per `CONFIRMATION.md` section 2, a debug-only
manifestation is still a real, reportable bug - fuzzing (`cargo fuzz`'s default build has
debug-assertions on) and `cargo test`/plain `cargo run` all show it.

**Fully reproducible**: **Yes**. `repro/` writes the real fuzzer-found bytes to disk and opens them
through the unmodified, public, top-level `rawler::get_decoder` - reachable with no format-specific
magic bytes required at all (see Root cause). Single-participant case, no second participant to
fake, per `CONFIRMATION.md` section 11.

## Class

`crash` (debug build only - see Status; in release the same code path degrades to a wrong-but-safe
seek position rather than any memory-unsafety, since `Seek`/`Read` on a `Cursor` are themselves
bounds-checked independent of the arithmetic that computed the target offset).

## Root cause

`VendorBox::read_box` (`formats/bmff/vendor.rs:13-18`):

```rust
fn read_box(reader: &mut R, header: BoxHeader) -> Result<Self> {
  reader.seek(SeekFrom::Start(header.offset + header.size))?;
  Ok(Self { header })
}
```

`header.offset` and `header.size` are both `u64` fields populated directly from a BMFF box's own
header bytes in the file (`BoxHeader`, `formats/bmff/mod.rs:58-67`) - a box that declares a `size`
close to `u64::MAX` (the fuzzer's input has `size = 0xffffffffffffffff` at the relevant box) makes
the addition overflow. In a debug build this panics; the underlying operation (repositioning a
`Seek`-able reader) is itself harmless even when the sum wraps, which is why release doesn't crash
- but the panic itself is a real, real-path-reachable crash regardless of what would have happened
next.

## Introduced by

30c5068 "Add BMFF parser based on mp4-rust crate" (2021-06-19) - `git log -S"header.offset + header.size"` finds this as the only commit touching that expression in `rawler/src/formats/bmff/vendor.rs`; the unchecked addition has been present since the BMFF parser was first added (ported from the mp4-rust crate), not introduced by a later change.

## The fix

Replaced the unchecked `header.offset + header.size` with
`header.offset.checked_add(header.size).ok_or_else(|| BmffError::Parse(...))?`, turning the
overflow into the function's existing `Result` error path instead of a panic (also making release
builds fail cleanly and predictably here instead of silently seeking to a semantically meaningless
wrapped position, even though that particular release-mode side effect wasn't itself unsafe).

Verified: `repro/` (the exact fuzzer-found 54-byte input) no longer panics in debug and returns a
clean `get_decoder` error instead (the same "No decoder found" outcome release builds already had,
now reached deliberately via a real error path instead of accidentally via wraparound).

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises a BMFF box with an overflowing
  size/offset, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- Other BMFF box types under `formats/bmff/` (there are many `impl ReadBox for ...` implementations
  in this module, one per box type) were not systematically audited for the same
  `offset + size`-shaped arithmetic - `VendorBox` was fixed because it's the one the fuzzer's short
  run happened to reach first via the un-gated fallback path; a sibling box type reachable only
  after successfully parsing further into a real CR3 file's structure could have the identical
  unchecked-addition shape and wasn't checked this session.
- This is the third distinct file-format-parser-layer "unchecked-offset-arithmetic-from-file-bytes"
  bug found this session (after X3F's slicing and CIFF's slicing/subtraction) - see candidate 005's
  "What's not done" note about a possible shared bounds-checking helper being worth adding to this
  crate rather than re-deriving the same fix shape by hand in each parser.
