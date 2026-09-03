# PIXMAP_FROM_PARTS_LENGTH_MISMATCH_PANIC

`sparse_strips/vello_common/src/pixmap.rs`'s `Pixmap::from_parts(data: Vec<u8>, width: u16, height:
u16, pixel_metadata)` has an infallible signature (`-> Self`, no `Result`/`Option`) but its own doc
comment admits it panics via `assert_eq!` if `data.len() != width * height * 4`, and via
`bytemuck::try_cast_vec(...).expect(...)` if the vector's capacity isn't a multiple of 4. This
looked suspicious because `data`/`width`/`height` are two independently caller-supplied values with
no code path connecting them - unlike `Pixmap::from_png` (which derives buffer size from the same
header it parses, so it structurally can't mismatch) - and because the sibling type `PixmapMut::new`
already returns `Option<Self>` for the exact same "does `buf.len()` match `width * height * 4`"
check, showing the project itself already considers this a case worth handling gracefully - just
not for `Pixmap::from_parts`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + real public API (`vello_common::pixmap::Pixmap::from_parts`, called directly, no rendering pipeline needed) | `Pixmap::from_parts(vec![0u8; 60], 4, 4, PixelMetadata::new(...))` (length mismatch: 60 bytes supplied, 64 expected) - debug build | Reproduced immediately: panic at `pixmap.rs:129`, `assertion `left == right` failed: Expected `data` to have length of exactly `width * height`` |
| Same, second boundary variant in the same bug class | `Vec::with_capacity(7)` then push 4 bytes (`len() == 4` matches `width=1,height=1`, but `capacity() == 7` is not a multiple of 4) - debug build | Reproduced immediately: panic at `pixmap.rs:128`, `` The capacity of the vector needs to be divisible by 4.: OutputSliceWouldHaveSlop `` - a second, distinct panic site in the same function |
| Release-build check (`CONFIRMATION.md` section 2) | `cargo run --release` with the length-mismatch input | Panics identically - `assert_eq!` is not `debug_assert_eq!`, so this is not a debug-only artifact |

A debug-assertions build plus deliberately constructed boundary values was sufficient for a clean,
deterministic reproduction of both documented panic sites - no sanitizer/miri/fuzzing needed (this
is plain arithmetic/precondition checking on caller-supplied sizes, not memory-safety/UB), so the
soft 3-approach cap was not exhausted.

## Status

**Confirmed**. Reproduces 5/5 runs (debug build) for both the length-mismatch case and the
odd-capacity case; deterministic (same panic location/message every run, no dependency on
ASLR/thread scheduling/hash order). Also confirmed to panic identically in a `--release` build.

**Fully reproducible**: **Yes**. This is a single-participant finding, not a two-participant
protocol interaction: `Pixmap::from_parts` is `vello_common`'s own real, public constructor, and the
`Vec<u8>` + `(width, height)` triple handed to it directly *is* the external artifact this finding
is about (the same shape of exception `CONFIRMATION.md` section 11 describes for "a malformed file
handed to a parser's real public `open_file`/`from_reader` entry point" - there is no "other side"
being impersonated here; nothing about a real external image decoder's output needs to be faked to
demonstrate this, the malformed `Vec<u8>`/dimension pair a real decoder-integration bug would
produce is directly constructed and fed to the real public API). `repro/src/bin/vulnerable.rs` calls
the real, public `Pixmap::from_parts` end-to-end with no hand-constructed internal state standing in
for anything else.

## Class

crash

## Root cause

`Pixmap::from_parts` takes `data: Vec<u8>` and `width`/`height: u16` as three independent
parameters with no code path enforcing they agree before the constructor runs. Internally it:
1. Calls `bytemuck::try_cast_vec::<u8, PremulRgba8>(data)` and `.expect()`s success - this fails
   whenever the vector's byte `capacity()` isn't a multiple of 4 (`size_of::<PremulRgba8>()`),
   which can happen even when `data.len()` already matches `width * height * 4` exactly (a `Vec`
   grown via `Vec::with_capacity` in a size unrelated to 4, then never `shrink_to_fit`-ed, is a
   realistic pattern for output from an external decoder/re-encoder).
2. `assert_eq!(data.len(), width * height, ...)` (in `PremulRgba8` units, i.e. `data.len() * 4 ==`
   the original byte length) after the cast succeeds.

Both are real, reachable `panic!`s (the first via `.expect()`, the second via `assert_eq!` - neither
is a `debug_assert!`, so both panic in release builds too), and the function's signature gives the
caller no way to detect either condition before calling it, or handle it afterward, other than
`std::panic::catch_unwind` (which the codebase itself doesn't otherwise rely on for this kind of
input validation, and which doesn't work in `no_std`/`panic = "abort"` configurations vello_common
otherwise supports). Compare `PixmapMut::new(width, height, buf)`, defined just above `Pixmap` in
the same file, which performs the exact same `buf.len() == width * height * 4` check and returns
`None` on mismatch instead of panicking - the asymmetry between the two nearly-identical
constructors is itself evidence this was a gap in `Pixmap::from_parts`'s own design rather than an
intentional choice specific to `Pixmap`.

## Introduced by

3fabef93 "vello_common: Unify and improve constructor for pixmaps (#1834)" (2026-08-21) - confirmed
via `git log -S"fn from_parts"` and reading the commit's own diff. Before this commit,
`Pixmap::from_parts` took `data: Vec<PremulRgba8>` directly (the caller was responsible for having
already produced a correctly-typed, correctly-sized buffer, typically via their own prior
`bytemuck`/API-level cast) and its own body only had the `width * height` length assert - no
`bytemuck::try_cast_vec` call existed inside this function at all, since the input was already the
target type. This commit changed the parameter to raw `data: Vec<u8>` plus a separate
`pixel_metadata`, centralizing the cast (and its new panic surface: capacity not a multiple of 4)
inside `from_parts` itself, and is what first made "an arbitrary caller-supplied `Vec<u8>` +
independently-supplied `width`/`height`, with no fallible variant to validate them together" the
shape of this API - the length-mismatch assert itself is not new, but the combination that makes it
easy to trigger through ordinary decoder-integration code (rather than requiring the caller to
already have produced a `Vec<PremulRgba8>`) is.

## The fix

Added `Pixmap::try_from_parts(data: Vec<u8>, width: u16, height: u16, pixel_metadata) ->
Option<Self>` as a fallible counterpart, mirroring `PixmapMut::new`'s existing `Option`-returning
pattern for the identical check. It performs the `data.len() == width * height * 4` check *before*
attempting `bytemuck::try_cast_vec`, returning `None` on either failure instead of panicking.
`Pixmap::from_parts` itself is kept, unchanged in observable behavior (still panics on the same
inputs, same documented `# Panics` contract, so existing callers who already validate their inputs
or intentionally want a hard failure are unaffected) - it is now implemented as
`Self::try_from_parts(...).expect(...)`, i.e. a thin, behavior-preserving wrapper. This is a
deliberately narrow, additive fix: it does not change `from_parts`'s contract (doing so would risk
silently changing behavior for existing correct callers), it only closes the actual gap the
candidate is about - giving a caller integrating an untrusted/external decoder a way to avoid the
process abort entirely.

Verified:
- `repro/src/bin/vulnerable.rs` (calls `from_parts`) still panics identically before and after the
  patch - this is intended (documented behavior preserved), not a leftover bug; see `output.txt`.
- `repro/src/bin/safe.rs` (calls the new `try_from_parts`, which only exists after the patch) turns
  both the length-mismatch and the odd-capacity inputs into a graceful `None` instead of a panic,
  and still returns `Some` for well-formed input (`CONFIRMATION.md` section 7 - checked more than
  just the one input that first triggered the bug). See `output.txt`.
- Reproduces/resolves deterministically 5/5 runs for both binaries (debug build).
- Project test suite (`cargo test -p vello_common`): **159 passed, 0 failed, 1 ignored** before ->
  **162 passed, 0 failed, 1 ignored** after (3 new tests added for `try_from_parts`, no existing
  test broken).
- `cargo test -p vello_cpu --lib`: **81 passed, 0 failed** before and after (unaffected, as
  expected - `vello_cpu`'s own unit tests don't exercise `Pixmap::from_parts`'s length-mismatch
  path).
- `cargo test -p vello_sparse_tests`: **89 passed, 3538 failed** before *and* after (identical
  counts, confirmed via `git stash`/`git stash pop` around the same test run) - this large failure
  count is a pre-existing environment limitation in this sandbox (no working `git-lfs` on `PATH`
  at the time these specific runs were made, so every LFS-tracked reference PNG this test suite
  compares against decodes as `Unsupported(UnsupportedError { format: Unknown, kind: Format(Unknown)
  })`), not something this patch caused or changed. (A working `git-lfs` binary was later placed on
  `PATH` specifically for the standalone `repro/`'s own `just test` verification - see
  `output.txt`'s clone log - but was not used for this particular before/after test-suite
  comparison.)

## What's not done / open questions

- `sparse_strips/vello_common/src/mask.rs` has its own, differently-shaped `from_parts(data:
  Vec<u8>, width: u16, height: u16)` (for `Mask`, not `Pixmap`) that was not checked this session -
  it may have the same class of gap and would be a reasonable follow-up candidate.
- No attempt was made to add a `try_from_png`/fallible PNG path - `from_png` was already excluded
  from this finding since it derives its buffer size from the same header it parses and structurally
  cannot mismatch, so it doesn't need one.
- Whether `vello_cpu`/`vello_hybrid`'s own examples or any downstream integration should be migrated
  from `from_parts` to `try_from_parts` was not investigated - this session only added the fallible
  API, it did not audit existing in-repo callers for whether they'd benefit from switching to it.
