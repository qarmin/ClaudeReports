# `StatefulDecoder::read_u32`'s unguarded `vec.resize` from an untrusted Basic Offset Table length (OOM abort)

From `IDEAS.md`'s backlog (added during candidate 080, itself citing candidate 004's own drafted-
but-never-merged fix for the same file): `parser/src/stateful/decode.rs`'s `read_u32` - the
function backing `read_u32_to_vec`, used by `DataSetReader`'s Basic Offset Table reading in
`parser/src/dataset/read.rs` - does `vec.resize(base + n, 0)` sized directly from an untrusted
item length, with no fallible-allocation guard.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/stateful/decode.rs::read_u32` (only caller: `read_u32_to_vec`, only caller of that: `parser/src/dataset/read.rs`'s Basic Offset Table branch, `Vec::with_capacity(len)` + `read_u32_to_vec(len as u32, &mut offset_table)`). Confirmed `read_u32`'s `vec.resize(base + n, 0)` (`n` derived straight from the item's declared byte length, up to `0xFFFF_FFFE`) uses Rust's default infallible allocation path, same class as candidates 004/080. | Found the call site is real, but **entangled** with candidate 080's already-recorded bug at the very same call path: the offset table's outer `Vec::with_capacity(len)` (a `Vec<u32>`, so it requests `len * 4` bytes for the same `len`) always needs 4x more memory than `read_u32`'s own resize (`n = len >> 2` elements, `n * 4 \| len` bytes) for the identical declared length - so on today's unmodified master (where 080's fix is not merged either, per this skill's "always revert" rule), the outer allocation always fails first, and `read_u32`'s own resize is never reached as the *proximate* cause. |
| Standalone repro (release build, `dicom_object::from_reader`, the real public top-level entry point), `ulimit -v 500000` | `repro/` here. A 236-byte crafted file (valid file-meta + encapsulated Pixel Data + one Basic Offset Table item declaring a 4,294,967,280-byte length, no actual bytes following). Run three ways: (a) pristine clone, neither patch - aborts requesting `17,179,869,120` bytes (`len*4`, candidate 080's site); (b) candidate 080's fix applied first (isolates the confound) - aborts requesting exactly `4,294,967,280` bytes (`len`, `read_u32`'s own resize, this candidate's site); (c) both patches applied - no abort, graceful error. | (a) and (b) both abort deterministically (`SIGABRT`, `memory allocation of N bytes failed`) with the *predicted, distinct* byte counts for each site, confirming both bugs are real and confirming this candidate's site specifically only becomes the proximate cause once the outer one is neutralized. (c) resolves gracefully. |
| Repeat-run check | 3 runs each, release build, `ulimit -v 500000`, no fixed seed | 3/3 for (b) and 3/3 for (c) - fully deterministic, no randomness anywhere in this code path (see `output.txt`). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed, reproduced via the real
`dicom_object::from_reader` public entry point on a 236-byte crafted file.

**Fully reproducible**: **Yes** - `repro/` calls `dicom_object::from_reader`, the real public
top-level entry point a real application would use to parse an untrusted DICOM byte stream (e.g.
one just received over the network or an upload), on a fully self-contained, hand-crafted
malformed file. Per `CONFIRMATION.md` section 11, a malformed file handed to a parser's real
public entry point *is* the genuine external input; there is no second participant being
impersonated here. Note the one caveat stated plainly above and in "Root cause" below: on a
**pristine** clone (neither this candidate's fix nor candidate 080's already-recorded fix
applied), the *same* crafted input still aborts the process - but via candidate 080's own,
already-`Confirmed`, distinct call site (a 4x-larger allocation for the same declared length),
not via this candidate's `read_u32`. This candidate's own repro therefore demonstrates its bug by
first applying candidate 080's own already-verified fix (`prereq-080.patch`, a copy of that
candidate's own `patch.diff`) to remove that confound - this is not "faking a second participant"
(the second participant here is still just this repro's own crafted bytes handed to the real
parser); it is isolating one root cause from another at the same call site, exactly the situation
`SKILL.md`'s critical pitfall #6 ("confounded fix") warns about, made explicit rather than glossed
over.

## Class

`crash` / `security` (uncatchable process abort via Rust's default global-allocator-failure
handler, not a panic or `Result::Err` any caller could intercept - triggered by a 236-byte,
entirely attacker-controlled input with no actual malformed structure beyond an oversized declared
Basic Offset Table length; the same DoS class as candidates 004/080, in the one call site those
two candidates' own fixes don't yet cover between them).

## Root cause

`StatefulDecoder::read_u32` (`parser/src/stateful/decode.rs`) is reached from exactly one path:
`read_u32_to_vec`, called from `DataSetReader::next()` (`parser/src/dataset/read.rs`) when reading
the Basic Offset Table item that always precedes an encapsulated Pixel Data element's fragments.
That call site first does `let mut offset_table = Vec::with_capacity(len);` (a `Vec<u32>`, so this
requests `len * 4` bytes for the wire's own *byte* length `len` - candidate 080's already-recorded
bug, a 4x amplification) and only then calls `read_u32_to_vec(len as u32, &mut offset_table)` ->
`read_u32((len >> 2) as usize, &mut offset_table)`, whose own `vec.resize(base + n, 0)` (with
`base = 0` since `offset_table` is always freshly created here) requests roughly `n * 4 ≈ len`
bytes. Because the outer call's request is always ~4x the inner one's for the same `len`, and
`Vec::resize` only allocates when it needs *more* capacity than it already has, `read_u32`'s own
resize never triggers a *new* allocation on unmodified master: the outer `with_capacity` either
already grabbed enough (in which case `resize` is free), or it already aborted the process first.
Empirically: a pristine clone aborts requesting `len * 4 = 17,179,869,120` bytes (080's site); the
identical input against a clone with **only** 080's fix applied (`Vec::new()` instead of
`Vec::with_capacity(len)`, so `offset_table` starts with zero capacity) aborts requesting
`len ≈ 4,294,967,280` bytes instead - now unambiguously `read_u32`'s own `vec.resize`, since
that's the only allocation-sized-from-`len` call left on that path. Both use Rust's default
infallible allocation path (`handle_alloc_error` -> process abort on failure), not a catchable
`Result::Err` or panic.

This is the exact call site candidate 004 already root-caused and drafted a fix for (its own
`patch.diff` for `parser/src/stateful/decode.rs` includes a `read_u32`-specific `try_reserve_exact`
hunk, reproduced verbatim in this candidate's own `patch.diff`) - never applied/merged to master,
per this skill's "always revert the working tree" rule, so it remains live. This candidate exists
as its own numbered entry (rather than folded into re-verifying 004) because 004's own repro never
demonstrated `read_u32` as the *specific, isolated, proximate* abort site - candidate 080's later
work at the very same call path showed that doing so requires neutralizing 080's own confound
first, which is exactly what this candidate's repro does and documents.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a `try_reserve_exact(n)` guard to `read_u32` before its `vec.resize(base + n, 0)`, converting
an allocation failure into a normal `Result::Err` (via the existing `ReadValueDataSnafu` context)
instead of a process abort - identical in spirit to `checked_resize_buffer`/`checked_alloc_smallvec`
already drafted for the other 16 call sites in candidate 004's own (also-unmerged) fix, and to
candidate 080's fix for the confounding outer site. See `patch.diff` (`parser/src/stateful/decode.rs`,
one file, one hunk - reproduces candidate 004's own `read_u32` hunk verbatim).

Verified against the real end-to-end `repro/` (see `output.txt`): 3/3 runs aborted before the fix
(with 080's own fix applied to isolate this site), 3/3 runs returned a graceful error after it, no
change to the crafted input between runs.

- Project test suite: `cargo test -p dicom-parser -p dicom-object --lib` - dicom-object 75 passed
  / 0 failed, dicom-parser 49 passed / 0 failed, identical before (clean master) and after (this
  candidate's `patch.diff` alone, no prereq needed for this check since it's a build-time
  regression check, not an allocation-triggering one). `cargo test --workspace --lib` - 351
  passed / 0 failed / 2 ignored, identical both times (no regressions, no newly-passing tests - no
  existing test constructs an oversized Basic Offset Table length).

## What's not done / open questions

- **Applying this candidate's `patch.diff` alone, to a pristine (unfixed) master, does NOT stop
  the crafted input in this repro from aborting the process** - it still aborts, via candidate
  080's own site (17.18 GB request), one call earlier. Closing the Basic Offset Table path fully
  requires **both** candidate 080's fix and this candidate's fix applied together (or,
  equivalently, candidate 004's full 17-site fix, which already includes both this exact
  `read_u32` hunk and, via its own `checked_alloc_smallvec`/`checked_resize_buffer` helpers, a
  template for guarding the other typed-value readers - though not the specific `read.rs`
  `Vec::with_capacity` sites candidate 080 covers, which are a different file/mechanism). A
  maintainer applying only this candidate's `patch.diff` in isolation, without also applying
  candidate 080's, gets no observable improvement for the Basic Offset Table scenario specifically
  - worth flagging explicitly so this isn't mistaken for a complete fix on its own. This is why
  this candidate's own `patch.diff` should be applied together with candidate 080's, not as a
  substitute for it.
- Did not re-check whether `read_u32` is reachable from any other path than the Basic Offset
  Table (grepped for all callers of `read_u32`/`read_u32_to_vec` in `parser/src/`; found exactly
  one call chain, described above) - if a future refactor adds a second caller with a non-zero
  `base`, this fix still applies unconditionally (`try_reserve_exact` reserves *additional*
  capacity beyond the current length either way), so no follow-up is needed there.
- Not fuzzed - found and confirmed by direct code reading plus hand-constructed boundary-value
  input (a single crafted 236-byte file), consistent with `HEURISTICS.md`'s guidance that manual
  boundary-value construction is the right first tool for this class of untrusted-length
  allocation bug, and consistent with how candidates 004/080 were each found the same way.
