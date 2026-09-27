# mp4_ilst_write_update_offsets_stco_truncating_cast_wrap

Directed candidate: whether `lofty/src/mp4/ilst/write.rs::update_offsets`'s `stco` (32-bit chunk
offset table) loop's `(i64::from(read_offset) + difference) as u32` truncating cast can silently
wrap a chunk offset (data corruption, no crash) rather than panicking or erroring, when `difference`
(the size delta from rewriting `ilst`) is large enough relative to `read_offset` - noted as an open
question in candidate `027`'s own `RESULT.md` (which fixed the sibling `co64`/`tfhd` 64-bit
overflow-panic bug but explicitly left this 32-bit truncating-cast case unverified) and in
`IDEAS.md`'s backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: took a real, `read_from`-accepted M4A (`lofty/tests/taglib/data/has-tags.m4a`), directly patched one `stco` chunk-offset entry to `0xFFFFFFFA` (4294967290, 5 short of `u32::MAX`) - a form of unvalidated/corrupted file data the read path never rejects (`stco` entries are opaque per-chunk data, never checked against the real file length) - then drove the real, public `Mp4File::read_from` -> `Ilst::insert` (a 200000-byte `covr` payload) -> `save_to` API end-to-end | Debug build (default `cargo build`, debug-assertions on), 3 repeat runs against the main project's own pristine working tree | Deterministic 3/3: `save_to` returned `Ok(())` (no panic, no error) and the on-disk `stco` entry was `200010` instead of the true (unrepresentable-in-32-bit) sum - a silently wrapped/corrupted offset |
| Manual boundary-value construction of the opposite direction (negative underflow): grew a pristine copy of the same file via the real API (5MB `covr`), then patched its resulting `stco` entry to a small value just above the (now-shifted) `ilst_offset`, then removed the tag entirely via `set_ilst(Ilst::default())` (a large negative `difference`) through the real API a second time | Same debug build, ad hoc verification script (not part of the final `repro/`, kept as a boundary-value sanity check per the "vary the input" verification step) | Confirmed on unpatched code: on-disk `stco` entry went from `2827` to `4289969463` (`0xFFB3BD37`) despite the file having shrunk by ~5MB - the negative-underflow direction of the same truncating cast, also silent (no panic, no error) |

Two approaches were used (well under the soft cap of 3): the first (positive-overflow direction)
gave a clean, deterministic reproduction on the first attempt; the second (negative-underflow
direction) was done specifically to satisfy `CONFIRMATION.md` section 7 ("vary the input within the
same bug class") before considering the eventual fix verified, and confirmed the same truncating
cast wraps in both directions on unpatched code. No sanitizer/miri/fuzzing was needed - this is a
plain integer-cast boundary bug, fully explained and reproduced by direct construction.

## Status

**Confirmed**

Reproduces deterministically 3/3 runs (debug build, standalone `repro/` against the project's
public API, run directly against the main project's working tree), and again via the fully
standalone `just test` clone-based reproduction (fresh `git clone` of the real repo at
`base_commit`, unmodified working tree) - see `output.txt`. The negative-underflow boundary variant
was separately confirmed 1/1 via an ad hoc verification script (not included in `repro/`, since one
canonical direction is sufficient to demonstrate and fix the bug class; the fix was verified to
close both directions - see "The fix" below).

**Fully reproducible: Yes.** This is a single-participant finding (the library's own write path
mishandling a file it itself already accepted at read time) - no second participant/peer is
involved, so `CONFIRMATION.md` section 11's "real second participant" bar doesn't apply; `repro/`
runs the real, public `Mp4File::read_from` -> `Ilst::insert`/`set_ilst` -> `save_to` API end-to-end
against a real crafted file, exactly as any real caller adding a large embedded picture to this
file's tag would do.

## Class

logic

## Root cause

`lofty/src/mp4/ilst/write.rs::update_offsets`'s 32-bit `stco` loop rewrites every chunk offset at
or past the rewritten `ilst` atom by adding `difference` (how many bytes the file grew or shrank
because of the tag edit) and truncating the result back to `u32`:

```rust
write_handle.write_u32::<BigEndian>((i64::from(read_offset) + difference) as u32)?;
```

`read_offset: u32` is read directly from the file's `stco` atom content with no validation that
it's a sane offset relative to the file's actual size - `stco` entries are opaque per-chunk data,
only ever used as an offset into `mdat` by a media player, never dereferenced or bounds-checked by
this codebase's own read path. Unlike the sibling `co64`/`tfhd` bug candidate `027` fixed (a
`u64`-to-`i64` *reinterpretation* that could itself overflow `i64`'s addition and panic), the
`stco` case's `i64::from(read_offset)` is always a safe, lossless widening (any `u32` fits in
`i64`), so the addition itself never overflows for any realistic `difference` and never panics.
Instead, the bug is in the **result**: `i64::from(read_offset) + difference` can legitimately land
outside `0..=u32::MAX` in either direction:

- **Positive overflow**: if `read_offset` is already close to `u32::MAX` (a real, valid 32-bit
  `stco` entry value; nothing in the file format or this codebase caps how close a chunk offset can
  legitimately be to 4 GiB) and `difference` is positive (the tag grew, e.g. a caller inserted an
  embedded cover picture), the true sum exceeds `u32::MAX` - a value that literally cannot be
  represented in a 32-bit `stco` entry - but `as u32` doesn't error, it silently truncates
  (`4295167290 as u32` -> `200010` in the confirmed input).
- **Negative underflow**: if `difference` is a large negative value (the tag shrank drastically,
  e.g. a caller removed a multi-megabyte embedded picture) larger in magnitude than `read_offset`
  itself, the true sum is negative - again unrepresentable in `u32` - and `as u32` two's-complement
  wraps it to a huge, numerically unrelated positive value (`2827 - ~5000000` wrapped to
  `4289969463` in the confirmed variant).

Neither direction panics (a raw `as` numeric cast is never checked, even in a debug-assertions
build - only arithmetic operators like `+`/`-`/`*` are), and neither direction is currently checked
against `u32`'s valid range before the value is written back to disk. The result: `save_to` returns
`Ok(())` and the caller has no indication anything went wrong, but the file's `stco` table now
contains a chunk offset with no relationship to where the actual chunk data lives - silent, durable
data corruption in a real, publicly-reachable write path (`Mp4File::save_to`), triggered purely by
writing a metadata tag to a real, already-`read_from`-accepted MP4/M4A file whose `stco` table
happens to contain an offset near the top or bottom of the range this update can shift it through (a
form of corruption or unusual encoding the read path has no reason to reject, since it never
dereferences these offsets itself).

## Introduced by

304feb94 "MP4: Properly handle offset updates for shrinking tags" (2024-01-20) - same origin commit as sibling candidate 027 (this candidate's `stco` 32-bit loop was introduced alongside the `co64`/`tfhd` 64-bit ones by the same commit, per `git log -S"i64::from(read_offset) + difference"`).

## The fix

Replaced the raw `(i64::from(read_offset) + difference) as u32` truncating cast with a checked
conversion, returning a graceful `FileEncodingError` (via the same `AtomParseError`/`FileParseError`
path this function already uses for its other "malformed atom" checks, e.g. the adjacent "found an
extended `stco` atom" check) instead of silently writing an unrepresentable value:

```rust
let new_offset = i64::from(read_offset) + difference;
let new_offset = u32::try_from(new_offset).map_err(|_| {
	FileParseError::from(AtomParseError::message(
		Some(stco.ident.clone()),
		"`stco` offset update would overflow a 32-bit offset",
	))
})?;

write_handle.seek(SeekFrom::Current(-4))?;
write_handle.write_u32::<BigEndian>(new_offset)?;
```

`u32::try_from(i64)` rejects any value outside `0..=u32::MAX` - both the positive-overflow and the
negative-underflow directions - so this closes the whole bug class in one change, not just the one
triggering input; re-verified against the negative-underflow variant separately (see "What was
tried" above): the patched code returns the same graceful `FileEncodingError` for that input too,
instead of wrapping to `4289969463`.

- Project test suite (`cargo test -p lofty --lib --tests -- --test-threads=1`): **646 passed, 0
  failed, 73 ignored** both before and after the fix (identical per-binary breakdown:
  256/122/56/2/15/191/4 passed) - no new failures, no change.
- `repro/` (the positive-overflow crafted input): before the fix, `save_to` returns `Ok(())` and
  writes the wrapped offset `200010` to disk (reproduces 3/3); after the fix, `save_to` returns
  `Err(FileEncodingError { format: Some(Mp4), .. })` with the message `` `stco` offset update would
  overflow a 32-bit offset `` (does not reproduce 3/3) - see `output.txt` for both.

## What's not done / open questions

- Candidate `027`'s own fix for the sibling `co64`/`tfhd` 64-bit case (an `apply_offset_difference`
  helper doing a checked `i64::try_from` + `checked_add` + `u32`/`u64::try_from`) was never merged
  (that candidate's tree was reverted per this skill's workflow, same as this one). Since both
  candidates target the same function but different atom types, a maintainer applying both patches
  together would end up with two separate but overlapping checked-conversion helpers (`stco`'s
  inline `u32::try_from` here, and `027`'s standalone `apply_offset_difference` for `co64`/`tfhd`) -
  worth unifying into one shared helper if both are taken, rather than a functional problem with
  either patch individually.
- This candidate only touched the `stco` loop. The `co64`/`tfhd` 64-bit loops have their own
  separate, already-`Confirmed` bug (candidate `027`, a different mechanism - `i64` reinterpretation
  overflow causing a panic, not a truncating-cast silent wrap) and were not re-examined here.
- Not re-run through a fresh fuzz burst against this specific fix - verified only via the two
  boundary-value-constructed inputs (positive overflow, negative underflow) plus the project's
  existing test suite, per the "bounded fix, verified against the confirmed input(s) + test suite"
  bar. A longer follow-up campaign (candidate `026`'s harness, kept at
  `___BUGHUNT___/_reference/026_fuzz_harness/`) would be worth pointing at this patched code.
- Whether a real-world encoder/muxer would ever legitimately produce an `stco` table with an offset
  close enough to `u32::MAX` to hit the positive-overflow direction without any corruption involved
  (e.g. a file that's itself close to the 4 GiB boundary and still uses 32-bit `stco` instead of
  `co64`) was not investigated - the confirmed reproduction relies on a crafted/corrupted `stco`
  entry, same threat model as candidate `027`'s `co64` finding, not a claim that ordinary large
  files trigger this without any corruption.
