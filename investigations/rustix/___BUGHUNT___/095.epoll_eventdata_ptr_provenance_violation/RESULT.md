# epoll_eventdata_ptr_provenance_violation

`src/event/epoll.rs`'s `EventData` union stores either a `u64` (`as_u64`) or a `*mut c_void`
(`sixty_four_bit_pointer`) in the same 8 bytes. `ptr()` unconditionally reads the
`sixty_four_bit_pointer` field (`unsafe { self.sixty_four_bit_pointer.pointer }`) even when the
union's last-written field was actually `as_u64` - a plain integer with no pointer provenance. The
doc comment itself admits this ("If the stored value was a `u64`, the least-significant bits of
the `u64` are returned as a pointer value"). Suspected to be the same class of strict-provenance
violation already confirmed under Miri for bare `as *const T`/`as *mut c_void` casts elsewhere in
the crate (candidates 004, 054, 055), but via a union read of an inactive field rather than an
explicit cast - this candidate builds a small Miri harness (store a `u64` via `new_u64`, read it
back via `.ptr()`, then do something provenance-sensitive with the result) to check whether it's
flagged the same way.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `EventData`/`SixtyFourBitPointer` (`src/event/epoll.rs:251-331`) and a crate-wide `grep -rn '\.ptr()'` over every real call site (production + tests) | whole `src/`/`tests/` tree | `.ptr()` is called in exactly 2 places outside this investigation's own harness: `src/io_uring/mod.rs`'s own layout test (`assert_eq!(user_data.ptr(), ...)`) and `tests/event/epoll.rs` (6x `d.ptr() as u64` / `d.ptr().is_null()`) - **every single real call site only compares the returned pointer's bit pattern or checks `.is_null()`; none of them ever dereferences it.** No real end-to-end path inside rustix itself (production code or its own tests) ever dereferences a `.ptr()`-derived pointer |
| Standalone Miri harness #1 (`examples/miri_check_095b.rs` in an isolated worktree, not part of this folder's deliverable): `EventData::new_u64(0x1234)` then `.ptr()`, printed but never dereferenced | `cargo +nightly miri run --example miri_check_095b --features "std,event"`, both default (permissive) Miri and `MIRIFLAGS=-Zmiri-strict-provenance` | **Clean in both modes** - no warning, no error. The union read itself (construct as int, read back as pointer, never dereference) is not flagged at all, unlike candidates 004/054/055's bare `as` casts, which Miri's strict-provenance checker flags purely from the cast operation itself, independent of any later dereference |
| Standalone Miri harness #2 (`examples/miri_check_095.rs`, same worktree): real allocation's address exposed via `as u64` (`&*Box::new(42u8) as *const u8 as u64`), stored via `EventData::new_u64`, recovered via `.ptr()`, then **dereferenced** (`unsafe { *(recovered as *const u8) }`) | `cargo +nightly miri run --example miri_check_095 --features "std,event"`, 3 repeat runs, default Miri, then repeated once more under `MIRIFLAGS=-Zmiri-strict-provenance` | **UB every time**: `error: Undefined Behavior: memory access failed: attempting to access 1 byte, but got 0x...[noalloc] which is a dangling pointer (it has no provenance)`. Reproduces 3/3 default-Miri runs and 1/1 under strict-provenance (identical error both ways - the union read itself doesn't go through Miri's "exposed provenance" address-matching heuristic the way an `as`/`with_exposed_provenance` cast does, so the result has no provenance unconditionally, not just under strict mode) |

The two harnesses together isolate exactly where the UB actually originates: **not** in `EventData`/`.ptr()`'s own construction/round-trip (harness #1, clean), but only if and when a caller *additionally* performs their own raw-pointer dereference of the result in their own `unsafe` block without having established real provenance for it (harness #2) - which is precisely the same caller-side contract every raw pointer in Rust already carries (including one legitimately built via `core::ptr::without_provenance`), not something specific to or fixable in `EventData`'s API. The crate-wide grep confirms rustix's own code (production and tests) never takes that second step itself.

## Status

**Not reproducible** (as a library-side bug). The suspected UB does not originate in `EventData`'s
own code: constructing a value via `new_u64` and reading it back via `.ptr()` is completely clean
under Miri (default and `-Zmiri-strict-provenance` alike, harness #1) - a union-field
reinterpretation of fully-initialized integer bytes into a pointer value is well-defined Rust
behavior that produces a provenance-free pointer, the same outcome `core::ptr::without_provenance`
would give by design, not a violation. The only way to get a Miri UB report (harness #2) requires
*additionally* dereferencing that pointer in a new, separate `unsafe` block supplied by the caller -
an operation that is always the caller's own obligation to justify for any raw pointer, library-
constructed or not. No real call site anywhere in rustix (production code or its own test suite)
takes that step, so there is no real end-to-end path inside the crate where this manifests as
actual UB today.

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A - no library-side bug confirmed.

## Root cause

Not applicable (no bug found) - but to state why the suspect turned out safe: `EventData::ptr()`'s
union read is not the same operation class as candidates 004/054/055's bare `as *const T`/`as *mut
c_void` casts. Those casts are intercepted by Miri's strict-provenance checker as a named MIR-level
operation (`PointerFromExposedAddress`/an explicit `as` cast) and get flagged purely from being
*constructed*, regardless of whether the result is ever dereferenced. A union field
reinterpretation is a different, lower-level operation (reading raw, fully-initialized bytes as a
different scalar type) that Miri does not intercept the same way - it just produces a pointer value
with no provenance, deterministically, functionally identical to calling `core::ptr::without_provenance`.
That's the *intended*, documented behavior the doc comment already describes ("the least-significant
bits of the `u64` are returned as a pointer value") - it is not promising the result is
safe-to-dereference, exactly as `without_provenance`'s own documentation doesn't either. The crate's
own usage of `.ptr()` (io_uring's layout test, epoll's own test suite) only ever compares the
resulting bit pattern or checks for null, matching exactly what the API's contract actually
supports.

This also means the specific worry in the candidate description (same bug class as
004/054/055 "but via a union rather than an explicit cast") does not hold empirically: the union
technique turns out to already be a Miri-sound way to fabricate a non-dereferenceable pointer from
an integer, in contrast to the bare `as` casts those other candidates found - if anything, it is
closer to the *fix* pattern used for 004 (`core::ptr::without_provenance`) than to the bug those
candidates found.

## Introduced by

N/A - no bug confirmed. For reference: the `EventData` union (`new_u64`/`new_ptr`/`u64()`/`ptr()`
exactly as they exist today) was introduced by ac4a66f0 "Improve strict-provenance compatibility in
the `epoll` API. (#693)" (Dan Gohman, 2023-06-19) - ironically, that commit's own stated purpose was
to improve `epoll`'s provenance compatibility (replacing a bare `u64` user-data field with this
union specifically so real pointers stored via `new_ptr` keep their provenance). It was later moved
verbatim from the `libc`-backend-specific file into the current shared `src/event/epoll.rs` by
c700ad75 "Unify epoll backend implementations (#1131)" (Alex Saveau, 2024-08-26), with no change to
the union/accessor logic itself.

## The fix

None - no bug confirmed, nothing to fix. No source file in the main project's working tree was
modified at any point this session (all two Miri harnesses were written and run only in the
isolated investigation worktree, then deleted before closing out the candidate).

- Project test suite: not run as a before/after pair (no patch was ever applied). For reference,
  `cargo test --workspace --features "std,all-apis,use-libc"` in the isolated worktree (pristine
  checkout, HEAD 287214b8): 51 passed/0 failed, 0 passed/1 ignored, 14 passed/0 failed, 70
  passed/0 failed, and 16 passed/1 failed (the 1 failure is the pre-existing, environment-caused
  `ioctl::test_int_setter` - missing `/dev/net/tun` in this sandbox, already documented in
  `projects/rustix.md` as the expected baseline, unrelated to this candidate) - 151 passed / 1
  failed total, matching the documented baseline exactly.

## What's not done / open questions

- Downstream crates that use `rustix::event::epoll::EventData` and might call `.ptr()` on a value
  they can't actually prove was constructed via `new_ptr` (e.g. code using `new_u64` for sentinel
  values alongside `new_ptr` for real per-connection state in the same `epoll` instance, then
  calling `.ptr()` unconditionally on every returned event without an out-of-band tag) were not
  surveyed - that is exactly the shape of real-world misuse that *would* hit harness #2's UB, but
  it would be a downstream caller bug, not something `EventData`'s own API can prevent short of
  adding a runtime discriminant (a design change, not a bug fix, and one the crate has deliberately
  avoided to keep the type ABI-compatible with the kernel's plain 8-byte `epoll_data_t`).
  Documenting this caveat more explicitly in `.ptr()`'s own doc comment (spelling out that the
  result must not be dereferenced unless the caller independently knows this specific `EventData`
  was constructed via `new_ptr`) could be a worthwhile low-risk documentation-only follow-up, but
  that's an API-clarity suggestion, not a confirmed bug, so no patch was drafted for it this
  session.
- The systemic int-to-pointer cast sweep already tracked in `projects/rustix.md` ("Pattern 2",
  candidates 004/054/055) remains about bare `as` casts specifically; this candidate's result
  narrows that pattern's actual scope - a union-based reinterpretation is not an instance of it,
  so no further union-based accessors elsewhere in the crate need to be swept for this exact shape
  on the strength of this candidate alone.
