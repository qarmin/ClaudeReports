# File meta group reading: skip zero-init before read_exact (7 sites in object/src/meta.rs)

Direct follow-on to candidates 009/014, applying the identical fix to the last remaining
known instance of the pattern in this workspace: `object/src/meta.rs`'s `read_str_body` and 6
inline call sites in `FileMetaTable`'s parsing (Implementation Version Name, Source/Sending/
Receiving Application Entity Title, Private Information Creator UID, Private Information) all did
`Vec::new(); v.try_reserve_exact(len); v.resize(len, 0); read_exact(&mut v)` - zero-filling a
buffer immediately fully overwritten by `read_exact`.

## What was tried

Added a shared `read_exact_uninit<S: Read>(source: &mut S, len: usize) -> Result<Vec<u8>>` helper
(allocate via `try_reserve_exact`, `set_len` without writing, then `read_exact`) and replaced all 7
call sites with it. Same soundness argument as candidates 009/014. Verified with **Miri**
(`cargo +nightly miri test -p dicom-object --lib meta::`, `MIRIFLAGS=-Zmiri-disable-isolation`):
all 10 `meta` module tests pass with no UB reported.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `parse` mode against `pydicom/liver.dcm` (a
small file, chosen deliberately to give the file-meta group - read exactly once per parse,
regardless of dataset size - the best possible chance of being a measurable fraction of total
parse time).

## Result

**No consistent change** (+0.3%, 3/9 - not consistent), exactly as this workspace's own
`IDEAS.md` entry predicted when logging this as a low-priority item ("likely small since
file-meta values are short strings"). The file meta group is read exactly once per file, and its
string values (AE titles, UIDs) are typically a handful to a few dozen bytes each - the memset
being removed is far too small in absolute terms to surface above this box's own noise floor,
even on a small file where it represents the largest possible fraction of total parse cost this
workspace could test.

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum` of the fully parsed-then-dumped object is identical between
  baseline and patched builds.
- **Is this a breaking change?** No public API/type changes - `read_exact_uninit` is a private
  function in `object/src/meta.rs`.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. Additionally verified with
  **Miri** (10/10 `object::meta` tests pass, no UB reported).

## Why the result looks the way it does

Same mechanism as candidates 009/014, but the absolute amount of work removed per call is smaller
here by roughly two more orders of magnitude (tens of bytes vs. hundreds of bytes to tens of
kilobytes) and this code path executes exactly once per file (not once per element or once per
large value) - there simply isn't enough total work being removed for the effect to be visible at
this box's measurement noise floor, even though the code is unambiguously doing (a tiny amount of)
unnecessary work by the same reasoning as its two more impactful siblings.

## What's not done / open questions

- This completes the "zero-init before full-overwrite" pattern sweep started by candidate 009 and
  continued by candidate 014 across this workspace's session - no further instances of this
  specific pattern were found in `parser/`, `object/`, or `core/` (not an exhaustive guarantee,
  just what turned up in the profiling and code-reading done this session).
- Kept in the patch (not reverted to the original code) despite the null result, per this
  workspace's own "one fix per candidate" and "record null results with the same care as wins"
  conventions - the code is objectively doing less unnecessary work even though it isn't
  measurable here, and a reader may still consider it worth taking on code-cleanliness grounds
  even absent a measured speed benefit.
