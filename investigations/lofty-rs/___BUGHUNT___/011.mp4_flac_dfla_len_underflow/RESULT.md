# mp4_flac_dfla_len_underflow

`lofty/src/mp4/properties.rs:1004`, in `flac_properties` (the FLAC-in-MP4, `fLaC`/`dfLa` codec
path): `if dfla.len - 12 < 18` subtracts 12 from the on-disk `dfLa` atom's `len` *before* checking
it's even that large - a candidate carried over from `IDEAS.md`, originally noticed while working
candidate 010 as the same subtract-before-check shape as candidate 003's confirmed ALS
`u16 as u8 + 1` bug, since `AtomInfo::read` (`lofty/src/mp4/atom_info.rs`) only guarantees
`len >= ATOM_HEADER_LEN` (8), not `>= 12`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Debug-assertions build + a hand-crafted, real MP4/M4A file on disk read through `lofty::read_from_path` | `cargo run` (dev profile, default `overflow-checks = true`); a 164-byte file with `ftyp`/`moov`/`trak`/`mdia`/`mdhd`/`hdlr`/`minf`/`stbl`/`stsd`/`fLaC` atoms, whose child `dfLa` atom declares `len = 8` (exactly `ATOM_HEADER_LEN`, no version/flags/STREAMINFO content at all) | Panicked immediately: `attempt to subtract with overflow` at `properties.rs:1004:8`, reproduced 3/3 runs (see `output.txt`); release build (no `overflow-checks`) instead wraps to a huge `u64`, fails the `< 18` check, and returns a `FileParseError` from the doomed `Block::read` call instead of panicking |

Only one approach was needed - the arithmetic pattern (`len - 12` with `len`'s only guaranteed
lower bound being 8) was already clear from reading `AtomInfo::read` alongside `flac_properties`,
and reproducing it through the real end-to-end parser on the first attempt was a clean positive
with no ambiguity, so the soft cap of escalating tools was not needed.

## Status

**Confirmed**. Reproduces 3/3 runs, debug build (default `cargo` dev profile, `overflow-checks =
true`), deterministic (no RNG/seed involved - the crafted file bytes are fully hand-constructed
and fixed).

**Fully reproducible**: **Yes**. The repro builds a real 164-byte MP4/M4A file on disk and reads
it back with `lofty::read_from_path` - lofty's real public entry point, exercising the actual
`Mp4File::read_from` -> `AtomReader` -> `read_stsd` -> `flac_properties` parsing path any caller
opening an untrusted `.m4a`/`.mp4` file (FLAC-in-MP4) would go through. Per `CONFIRMATION.md`
section 11, this is the "malformed file handed to a parser's real public entry point" case: the
crafted bytes on disk are the genuine external input, there is no second participant to fake here.

## Class

crash

## Root cause

`flac_properties` (in `lofty/src/mp4/properties.rs`) parses the child atoms of an MP4 `fLaC`
sample entry, looking for a `dfLa` atom that should carry a FLAC STREAMINFO block. After locating
and identifying the atom via `stsd.next()?`, it does:

```rust
stsd.seek(SeekFrom::Current(4))?; // skip version(1) + flags(3)

if dfla.len - 12 < 18 {
	// The atom isn't long enough to hold a STREAMINFO block, also not worth an error.
	return Ok(());
}
```

`dfla.len` comes straight from `AtomInfo::read`, which only guarantees `len >= ATOM_HEADER_LEN`
(8) - the 4-byte size field plus the 4-byte fourcc, nothing about the atom actually containing a
version/flags header or STREAMINFO data. Any `dfLa` atom whose declared `len` is in `8..=11`
(there is no requirement it be any larger; `AtomInfo::read`'s only other check is that the
declared size doesn't exceed the physically remaining bytes, which a tiny atom trivially
satisfies) makes `dfla.len - 12` underflow a `u64` *before* the `< 18` bounds check that was
supposed to guard exactly this "atom too small" case ever runs. Under this project's default
dev/test profile (and cargo-fuzz builds), `overflow-checks = true` turns this into an
unconditional panic; in a release build the subtraction instead wraps to a value near `u64::MAX`,
which fails the `< 18` check the "wrong way" (treats the undersized atom as enormous) and falls
through to `crate::flac::block::Block::read`, which then fails with a `FileParseError` reading a
STREAMINFO block that was never really there - not a panic, but still parsing logic operating on
a value the check was explicitly meant to reject.

This is the same "subtract untrusted-but-bounds-checked-elsewhere size before checking it's large
enough" shape as candidate 003's confirmed ALS `u16 as u8 + 1` bug in the same file, just
subtraction instead of a truncating add, and reached via the FLAC-in-MP4 (`fLaC`/`dfLa`) codec
path rather than AAC/`esds`/ALS.

## Introduced by

eff0d0d9 "MP4: Attempt to detect invalid atom identifiers" (2023-10-16) - `git log -S"dfla.len - 12"` finds this as the commit that introduced the `dfLa` atom length check, already as an unchecked subtraction.

## The fix

Compare against the sum instead of subtracting first, so the check can never underflow no matter
how small `dfla.len` is (the sum `12 + 18 = 30` fits trivially in `u64`, so this can't overflow in
the other direction either):

```rust
// `dfla.len` is only guaranteed to be >= `ATOM_HEADER_LEN` (8) by `AtomInfo::read`, so
// subtracting 12 first could underflow; compare against the sum instead.
if dfla.len < 12 + 18 {
```

Verified against the same crafted file: `dfla.len = 8` (the value used in the repro) now correctly
takes the "atom too small" early return with no panic and no bogus STREAMINFO read attempt,
reporting `sample_rate`/`channels` from the earlier `stsd` fixed fields only (`bit_depth` stays
`None`, matching "no STREAMINFO was present"). Values at and above the true boundary (`dfla.len ==
29` still too small, `dfla.len == 30` exactly large enough) are unaffected by construction, since
the comparison is now against the same effective threshold (30) the original code intended,
just computed without ever going negative.

- Project test suite (`cargo test -p lofty`, includes unit tests + doctests):
  before (bug present) = 176 passed; 0 failed; 23 ignored; 0 measured; 0 filtered out.
  after (patch applied) = 176 passed; 0 failed; 23 ignored; 0 measured; 0 filtered out.
  No new failures, no regressions.

## What's not done / open questions

- Only the exact boundary case `dfla.len == 8` (the minimum possible per `AtomInfo::read`) was
  used for the repro; `dfla.len` in `9..=11` was reasoned about but not independently
  file-crafted and re-run - all three still satisfy `len < 12` so the same underflow applies by
  the same arithmetic, but this wasn't separately exercised this session.
- The release-build fallthrough behavior (wrap to near-`u64::MAX`, then fail via `Block::read`'s
  own `FileParseError` rather than panicking) was observed but not exhaustively verified against
  every intermediate wrapped value - only the one concrete case from the crafted file.
- No other `AtomInfo::len` arithmetic elsewhere in `lofty/src/mp4/` was swept for the same
  subtract-before-check shape this session beyond this one spot and candidate 003's ALS branch;
  worth a targeted grep-and-check pass (`grep -n '\.len -' lofty/src/mp4/`) if this area is
  revisited.
