# FLAC_APPLICATION_BLOCK_LENGTH_UNDERFLOW

`read_flac_application_block` (`symphonia-metadata/src/embedded/flac.rs:179`) computes
`block_length as usize - 4`, where `block_length` is `MetadataBlockHeader.block_len`, a raw 24-bit
value read directly off the wire (`symphonia-common/src/xiph/audio/flac/mod.rs:224`) with no
minimum-length validation. The suspicion: an APPLICATION metadata block with a stated length of
0-3 bytes would underflow the subtraction (panic in a debug/overflow-checked build, or a huge
`usize` passed to `read_boxed_slice_exact` in release).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the only call site (`symphonia-bundle-flac/src/demuxer.rs:89-109`) | n/a | The demuxer always wraps `reader` in a `ScopedStream::new(&mut reader, u64::from(header.block_len))` *before* calling `read_flac_application_block(&mut block_stream, header.block_len)` - i.e. the scope length and the length parameter are always the exact same value. |
| Real end-to-end repro: hand-crafted minimal FLAC files (`"fLaC"` + one APPLICATION metadata block header with declared `block_len` in `0..=4`, is_last=1, and exactly that many body bytes) driven through the real, public `symphonia_bundle_flac::FlacReader::try_new` entry point | release build, `debug-assertions=true` + `overflow-checks=true` (so any underflow would panic instead of silently wrapping), 3/3 repeat runs against a pristine clone at the base commit | For `block_len` 0, 1, 2, 3: `Err(out of bounds)` every time, no panic. For `block_len` 4 (the boundary where the subtraction would be well-defined): `Err(malformed stream: flac: missing stream info block)`, no panic. Byte-for-byte identical across all 3 runs. |
| Control: the same low-level `read_flac_application_block` called directly (bypassing the demuxer) with a reader that actually has 4+ bytes available but a `block_length` parameter of `3` (a mismatch the real demuxer never produces) | same build, 3/3 repeat runs | Panics every time: `attempt to subtract with overflow` at `flac.rs:179:46` - confirms the underflow is real in the function's own logic in isolation, it just cannot be reached this way through the real caller. |

## Status

**Not reproducible** via the real end-to-end trigger path. `ScopedStream::read_quad_bytes`
(`symphonia-core/src/io/scoped_stream.rs`) checks `self.len - self.read < 4` and returns a clean
`out_of_bounds_error()` *before* any inner read happens. Because the demuxer always constructs the
`ScopedStream`'s `len` from the exact same `header.block_len` value it later passes as
`block_length` to `read_flac_application_block`, any declared APPLICATION block length less than 4
causes the mandatory 4-byte application-identifier read (`reader.read_quad_bytes()` on line 178,
one line above the subtraction) to fail first, and the `?` propagates that error out before line
179's `block_length as usize - 4` is ever evaluated. Verified with a real crafted FLAC file through
the real public `FlacReader::try_new` API: 3/3 identical runs, `block_len` 0 through 4, always a
clean `Err`, never a panic (debug-assertions/overflow-checks both on, so a real underflow would
have aborted immediately if reached).

`read_flac_application_block` is `pub` in `symphonia-metadata` and grepping the whole repository
confirms `symphonia-bundle-flac/src/demuxer.rs:108` is its *only* caller anywhere in the codebase -
there is no other real path (native FLAC container, OGG-FLAC mapping, etc.) that reaches it with a
different scoping discipline.

**Fully reproducible**: N/A (Not reproducible - no bug found via the real trigger path; the
underflow is real only when the function is called directly with a `block_length` argument that
does not match how many bytes the reader is actually scoped to, which the real caller never does).

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the claimed bug - the suspected path is safe by construction. The mandatory
4-byte application-identifier read on the line immediately before the subtraction acts as an
implicit lower-bound check: because the reader it's called on is always scoped to exactly
`block_length` bytes by the real caller, that read itself fails whenever `block_length < 4`,
short-circuiting before the subtraction can underflow. The subtraction is only reachable with
`block_length < 4` if a caller passes a `block_length` that disagrees with how many bytes the
reader actually has available - which is a misuse of the function's own (undocumented) invariant
by a caller, not something a malformed file can force through the one real caller that exists.

## Introduced by

Not determined precisely for "when did the underflow become theoretically present" - `git log -S`
on the literal line `block_length as usize - 4` in `symphonia-metadata/src/embedded/flac.rs`
returns only 2c85def ("common: Move FLAC metadata readers to metadata crate."), which is a file
move/refactor commit, not the original introduction (the expression predates that move; the
underlying logic goes back to when this function was first written in the older, pre-refactor
FLAC demuxer crate layout). Not chased further since the finding is Not reproducible - identifying
the exact original commit would not change the outcome.

## The fix

No fix needed/applied - no bug reachable via any real caller. (A purely defensive change - e.g.
`block_length.checked_sub(4)` with a `decode_error` on `None`, so the function is safe even if
called with a mismatched length in the future, or turning the implicit ordering dependency between
the identifier read and the subtraction into an explicit `if block_length < 4` check with a
comment - would be a reasonable low-risk hardening if a maintainer wants defense-in-depth against
future refactors that might change the calling convention, but it is not fixing an observed bug.

- Project test suite: N/A, no fix was applied (nothing to verify before/after, no behavior
  changed). For reference, `cargo test -p symphonia-metadata -p symphonia-bundle-flac` on the
  pristine working tree: 14 passed, 0 failed (2 in `symphonia-bundle-flac`, 12 in
  `symphonia-metadata`).

## What's not done / open questions

- Only the one real call site was checked (there is only one in the whole repository, confirmed by
  grep). No other format/container in this codebase feeds bytes into
  `read_flac_application_block`.
- The function's own robustness (defense against a future caller that doesn't scope the reader the
  same way, or against being used as a standalone public API by an external crate that isn't
  `symphonia-bundle-flac`) was not hardened, since this session's scope is "does the claimed bug
  reproduce through this codebase's own real usage", which it doesn't.
- No new suspects were noticed while investigating this one.
