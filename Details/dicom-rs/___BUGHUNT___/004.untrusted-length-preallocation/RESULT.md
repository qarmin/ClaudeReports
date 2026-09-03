# Untrusted declared-length pre-allocation aborts the process (OOM under memory pressure)

Found during a discovery round (no seeded lead, though the pattern name deliberately echoes a
prior fuzz-bug-repro campaign's own `fix-untrusted-length-preallocation` bundle, whose actual
patch no longer exists on disk in this checkout - re-derived independently by reading
`parser/src/stateful/decode.rs` against `HEURISTICS.md`'s "`.unwrap()`/allocation on data derived
from external input" and "arithmetic on untrusted sizes" patterns, then confirmed still present in
current upstream `master` (`df2b9e42`), the same way candidates 001/002 turned out to be).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `parser/src/stateful/decode.rs`'s numeric/byte value readers (`read_value_ob`, `read_value_ss/us/uv/sl/sv/fl/od/ul`, plus `self.buffer.resize_with(len, ...)` in the string/date/time readers, plus `StatefulDecode::read_u32`). All pre-allocate a buffer sized directly from the DICOM element header's declared value length - an untrusted `u32` field, up to ~4 GiB, that arrives fully before any of the element's actual value bytes. | Found 17 call sites total (8 `smallvec![0...; n]`, 8 `self.buffer.resize_with(len, ...)`, 1 `vec.resize(base + n, 0)` in `read_u32`) using plain, infallible allocation (`smallvec!`/`Vec::resize`) directly from this untrusted length, unlike `object/src/meta.rs`'s file-meta string reading, which already uses `try_reserve_exact` for exactly this reason (checked while investigating candidate 001) - an inconsistency within the same codebase. |
| Standalone repro (release build, `object`'s public API: `InMemDicomObject::read_dataset_with_ts`) | `___BUGHUNT___/004.../repro`. Hand-built a 12-byte explicit-VR-LE element header (tag + `VR=OB` + 4-byte length field declaring `0x7FFFFFFE`, ~2 GiB) with **no actual value bytes following** - the entire malicious input is 12 bytes. | Ran the parser twice: unconstrained (24 GiB RAM available on this machine) it silently succeeded at allocating ~2 GiB and then failed gracefully on the subsequent `read_exact` (`UnexpectedEof`) - not visibly a bug on a workstation with plenty of memory. Under `(ulimit -v 500000; ...)` (a 500 MB virtual-memory cap, representative of a small container/service) the SAME 12-byte input caused the process to print `memory allocation of 2147483646 bytes failed` and **abort via SIGABRT (core dumped)** - Rust's default global-allocator-failure handler, which is not a catchable `Result::Err` or even a catchable panic. |
| Repeat-run check | `for i in 1..3: (ulimit -v 500000; ./target/release/repro)` (pre-fix) | 3/3 runs aborted identically - fully deterministic, no randomness anywhere in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed, reproduced via both
a direct unconstrained run (silent large allocation, later a graceful `read_exact` failure) and a
memory-constrained run (hard abort) to show both ends of the actual severity range.

## Class

`crash` / `security` (uncatchable process abort - not a panic that a caller's `catch_unwind` or an
outer `Result` chain could ever intercept - triggered by a 12-byte, entirely attacker-controlled
input with no actual malformed structure beyond an oversized declared length; a natural DoS vector
for any service that parses untrusted DICOM data - file upload, network association handling, or
any other externally-reachable parse entry point - especially in a memory-constrained container or
sandboxed environment, which is exactly the class of deployment where an allocator failure is most
likely to actually occur rather than just being slow).

## Root cause

Every element value reader in `parser/src/stateful/decode.rs` computes a buffer size directly from
the element header's declared length (`self.require_known_length(header)?`, sourced straight from
4 bytes on the wire, attacker-controlled, up to `0xFFFFFFFE`) and then allocates a buffer of
exactly that size **before** attempting to read any of the actual value bytes - `smallvec![0; n]`
for the typed numeric readers (`read_value_ob`/`ss`/`us`/`uv`/`sl`/`sv`/`fl`/`od`/`ul`), and
`self.buffer.resize_with(len, Default::default)` for the string/date/time readers that reuse a
shared scratch buffer. Both `smallvec!`/`Vec::resize_with` use Rust's default infallible
allocation path: if the allocator can't satisfy the request (out of memory, or - as demonstrated
here - blocked by a `ulimit`/cgroup virtual-memory limit), Rust's global `handle_alloc_error`
aborts the whole process immediately. There is no bound between the declared length and how much
data has actually arrived in the underlying reader (or even could ever arrive, for a `&[u8]`
source that's already fully buffered) - the allocation is attempted purely on the strength of 4
bytes of attacker-supplied metadata.

`object/src/meta.rs`'s file-meta string reading (`read_str_body` and the per-field readers in
`FileMetaTable::read_from`) already avoids exactly this failure mode via
`v.try_reserve_exact(len).context(AllocationSizeSnafu)?` before resizing - `decode.rs` simply never
adopted the same pattern, despite handling the exact same class of untrusted length.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added two small helper methods on `StatefulDecoder`
(`checked_resize_buffer`/`checked_alloc_smallvec`) that use `try_reserve_exact` before resizing -
mirroring `object/src/meta.rs`'s already-established pattern - so an allocation that can't be
satisfied becomes a normal `Result::Err` (wrapped in the existing `ReadValueData` error variant via
a synthesized `std::io::Error`) instead of a process abort. Routed all 17 identified call sites
through these helpers (`read_value_ob` and the 8 typed numeric readers via
`checked_alloc_smallvec`; the 8 `self.buffer.resize_with` sites via `checked_resize_buffer`; the
separate `read_u32` function, used by `read_u32_to_vec`, via its own inline `try_reserve_exact`
since it operates on a plain `Vec<u32>` passed by the caller rather than `self.buffer`). See
`patch.diff` (`parser/src/stateful/decode.rs`, one file).

**Self-inflicted regression caught and fixed before finalizing**: a `sed`-based mechanical rewrite
of all 8 `self.buffer.resize_with(len, Default::default);` call sites accidentally also matched
and rewrote the identical line *inside* `checked_resize_buffer`'s own body, turning it into an
infinite self-recursive call. This passed a `cargo build` (no type error - both the original line
and the replacement are valid, differently-typed calls) but caused `cargo test -p dicom-object
--lib` to fail with a stack overflow in `collector::tests::test_lazy_dicom_read` (confirmed via
`gdb -batch -ex run -ex "bt 40"`: the backtrace showed `checked_resize_buffer` calling itself at
its own line 447, with `self` and `len` values from increasingly stale/corrupted stack frames).
Fixed by restoring the direct `self.buffer.resize_with(len, Default::default);` call inside the
helper itself. Lesson: a mechanical multi-site rewrite must exclude the helper's own definition,
or verify the helper function's body isn't textually identical to one of the call sites being
replaced - re-running the full test suite (not just a build) is what caught this, underscoring why
the candidate loop's "run the project's existing test suite before and after" step is mandatory
even for a change that compiles cleanly.

- Project test suite: `cargo test -p dicom-parser -p dicom-object --lib` - 49 + 75 passed / 0
  failed, identical both before and after the (corrected) patch. `cargo test --workspace --lib` -
  351 passed / 0 failed / 2 ignored, identical both before and after (no regressions, no
  newly-passing tests - no existing test constructed an oversized declared length).

## What's not done / open questions

- The fix converts an uncatchable abort into a graceful `Result::Err`; it does **not** cap the
  *attempted* allocation size to something small. A declared length that's large but still within
  what the allocator can actually satisfy (e.g. a few hundred MB on a machine with several GB
  free) will still be allocated in full before the `read_exact` that follows discovers there isn't
  actually that much data - a real, if less severe, resource-exhaustion angle (many small
  malicious requests each triggering a large-but-successful allocation) that a stricter fix could
  additionally address by capping initial allocation to a bounded chunk size and growing
  incrementally as bytes are confirmed to actually exist (closer to what `std::io::Read::take` +
  `read_to_end` naturally does) - not attempted here as it would require restructuring the typed
  numeric readers' `decode_XX_into(&mut [T])`-based interface, a larger change than this session's
  bounded-fix scope.
- Did not check `pixeldata`/`ul` crates for the same untrusted-length-preallocation pattern in this
  session (per `HEURISTICS.md`/`SKILL.md` pitfall #7, "the same underlying bug can live in more
  than one place") - `ul/src/pdu/reader.rs` was skimmed while investigating a different, unrelated
  suspect earlier this session and its length-arithmetic call sites all had explicit `ensure!`
  guards before any subtraction/allocation, but a full pass specifically for infallible
  pre-allocation-from-untrusted-length (as opposed to arithmetic underflow) wasn't done there or in
  `pixeldata`.
- Not fuzzed - found by direct code reading against a known anti-pattern (re-derived from a
  previously-diagnosed-but-lost campaign fix), then confirmed by hand-constructing a minimal
  triggering input, per `HEURISTICS.md`'s guidance that a debug-assertions build plus deliberately
  constructed boundary values is the right first tool for arithmetic/size-prediction bugs - a fuzz
  harness would very likely have found the same class quickly too, given how directly reachable it
  is from a single malformed element header.

## Addendum (2026-08-25 session): likely independently reconfirmed via fresh fuzzing

Same caveat as candidate `001`'s addendum: this skill's fixes are never applied to the real tree
between sessions, so this bug remains fully live. A 150-second `ASAN_OPTIONS=symbolize=0 cargo
+nightly fuzz run decode_image_file` burst against `pixeldata/fuzz/fuzz_targets/decode_image_file.rs`
(previously not run this session) found a libFuzzer `out-of-memory` abort (its own 2GB RSS ceiling,
not real system memory pressure) within seconds, on a 173-byte crafted file-meta-only input.
Symbolizing the crash was not possible in this sandbox for this specific run (the documented
`ASAN_OPTIONS=symbolize=0` workaround avoids a hang during coverage-guided fuzzing, but replaying a
single fixed crash input through the built binary *with* symbolization enabled hung too, and no
`addr2line`/`llvm-symbolizer` was available to resolve the raw addresses offline) - so this was
**not** verified at the same function-level precision as candidate `001`'s addendum, only
circumstantially: the symptom (an unbounded, infallible allocation triggered by a tiny crafted
input with no real underlying data) matches this candidate's own description exactly, and
`pixeldata`'s pixel-data decode path routes through the same `parser/src/stateful/decode.rs`
readers this candidate's `patch.diff` already covers. **Treat this as a strong but not
function-level-confirmed re-discovery** - if a future session has working symbolization in its
sandbox, worth a two-minute check to nail down the exact call site and confirm it's covered by the
existing `patch.diff` rather than a distinct sibling instance of the same class elsewhere in
`pixeldata`'s own decode path (which candidate `004`'s original "what's not done" section already
flagged as unchecked).
