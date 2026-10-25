# io_uring_ptr_padding_provenance_not_reproducible

`src/io_uring/mod.rs`'s `io_uring_ptr`/`io_uring_user_data` encode a `*mut c_void` into a `u64`
("io_uring's native API represents pointers as `u64` values") using a `#[repr(C)]` struct/union
plus explicit `__pad16`/`__pad32` fields depending on `target_pointer_width`, then transmute the
whole struct to `u64` (exercised by the `io_uring_layouts` test at `src/io_uring/mod.rs:2004-2036`).
Suspected (per candidate description) to be the same general shape as the already-confirmed
candidate 004 (`futex_val2`'s bare integer-to-pointer cast rejected by Miri) - i.e. that the
transmute relies on an unzeroed/implicit padding field, or otherwise isn't sound under Miri's
strict-provenance model.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `io_uring_ptr`'s field layout across all 4 `target_pointer_width`/ `target_endian` combinations it's `cfg`'d for | `src/io_uring/mod.rs:1203-1245` | Unlike a typical "implicit compiler-inserted padding" bug, this type has **no implicit padding at all** on any configured platform: every byte is accounted for by an explicit field (`ptr` plus `__pad16`/`__pad32` as needed), sized so the total is always exactly 8 bytes matching `u64`'s size/align, and `io_uring_ptr::new()` explicitly zero-initializes every declared pad field. The "relies on padding-field zero-fill" framing in the candidate description doesn't match what the type actually does - there is no implicit/uninitialized gap to rely on |
| `grep` for real (non-test, non-dead-code) call sites of `io_uring_ptr::new`/`io_uring_user_data::from_ptr`/`from_u64` and of any int-to-pointer `as` cast in `src/io_uring/` | whole `src/io_uring/` tree | Only 2 non-test hits: `io_uring_user_data::from_ptr`'s own body (`io_uring_ptr::new(ptr)`, forwarding an already-real pointer, no cast), and one int-to-pointer cast inside a function that is entirely commented out (`/* ... */`, dead code, `io_uring_enter_reg_wait`, lines ~290-325). rustix itself contains **no ring-buffer/mmap/CQE-read logic at all** (`src/io_uring/` has only `mod.rs` and `bindgen_types.rs`) - it only defines the types/flags and the `io_uring_setup`/`io_uring_enter`/`io_uring_register` syscalls; actually reading a completion queue's `user_data` back out is left entirely to downstream crates (e.g. `io-uring`, `tokio-uring`), so there is no real end-to-end trigger path inside rustix itself to drive with a genuine second participant (the kernel) producing a CQE |
| `cargo +nightly miri test --lib io_uring_layouts`, default (permissive) provenance, `--cfg rustix_no_linux_raw --features "std,all-apis,use-libc"` | `rustc`/`miri` nightly (component already present, `cargo +nightly miri setup` run once this session) | `ok` - 1 passed, 0 failed. Two warnings only, both on the **test's own** `MAGIC as usize as *mut c_void` cast used to fabricate a test input pointer (lines 2006, 2030) - not inside `io_uring_ptr::new`/`io_uring_user_data::from_u64`/`from_ptr` themselves |
| Same test under `MIRIFLAGS=-Zmiri-strict-provenance` | same build config | Hard error, but at the **same test-only line 2006** (`MAGIC as usize as *mut c_void`) - "integer-to-pointer casts ... are not supported with `-Zmiri-strict-provenance`". This aborts the test before it reaches the `core::mem::transmute::<io_uring_ptr, u64>` line (2012) or the `io_uring_user_data` half (2024-2036), so the existing test gives no direct signal either way about the transmute itself; it only shows that fabricating an arbitrary test pointer from a magic constant (needed to verify "pointers are stored the way io_uring expects") is itself incompatible with strict-provenance Miri - a known, inherent limitation of writing this kind of layout-equivalence test, not a defect in the production encode/decode logic |
| Standalone Miri sanity harness (own scratch crate, `/tmp/miri_sanity`, not part of the deliverable - see below) reproducing just `io_uring_ptr`/`io_uring_user_data`'s real field layout and methods, fed a **real** heap-allocated pointer (`Box::into_raw`) instead of a magic constant | `cargo +nightly miri run`, both default and `-Zmiri-strict-provenance` | Two results, see below |

### Sanity-harness result 1: realistic round trip (matches how io_uring's shared-memory ring actually works)

Constructed via `from_ptr(real_ptr)`, copied as raw bytes (`ptr::copy_nonoverlapping`, simulating
the kernel's opaque byte-for-byte copy of `user_data` from SQE to CQE through the mmap'd ring),
then decoded back via the union's `.ptr()` field accessor and dereferenced. **Clean under both
default and `-Zmiri-strict-provenance` Miri** - no warning, no error, assertion passes. This is the
realistic shape of real io_uring usage (construct once, let the kernel copy raw bytes through
shared memory, decode once) since nothing in that path ever turns the pointer into a plain integer
value through a scalar operation.

### Sanity-harness result 2: a real latent footgun, but not one `io_uring_ptr`/`io_uring_user_data`'s own API exercises

Doing the same round trip via a bare scalar `core::mem::transmute::<u64, IoUringPtr>(...)` (i.e.
skipping the memory copy and transmuting an immediate `u64` value straight into the pointer-bearing
struct, then dereferencing) **is real, reproducible UB** - confirmed under both default and
`-Zmiri-strict-provenance` Miri, same error both times: `memory access failed: ... dangling
pointer (it has no provenance)`. This is a genuinely different Miri behavior from an `as` cast:
`mem::transmute` into a pointer never goes through Miri's "exposed provenance" address-guessing
heuristic the way `with_exposed_provenance`/an `as` cast does, so the resulting pointer is always
provenance-free, unconditionally (not just under strict mode). **However, this exact
pattern - a scalar `mem::transmute` of a `u64` directly into a pointer-bearing `io_uring_ptr`/
`io_uring_user_data`, immediately dereferenced without ever passing through real memory - does not
occur anywhere in rustix's own source** (production or test): `io_uring_user_data::from_u64` only
ever stores an arbitrary `u64` (explicitly documented as such, never implicitly treated as a
pointer), and the only `mem::transmute` calls in the whole module are in the test, which never
dereferences the result (it only compares it for equality in `assert_eq!`).

## Status

**Not reproducible.** No real production code path in rustix itself constructs `io_uring_ptr`/
`io_uring_user_data` from an implicit/unzeroed padding byte (there is no implicit padding byte to
begin with - every byte is an explicit, always-zeroed field), and the one Miri-flagged operation
that genuinely strips provenance (a bare scalar `mem::transmute::<u64, T>` immediately
dereferenced) is not exercised by any call site in the crate, test or production. The crate also
has no mmap/ring-buffer/CQE-read logic of its own to exercise the real kernel round trip end to
end - that's entirely delegated to downstream crates, so there is no real end-to-end entry point
inside rustix itself for this suspected bug class to manifest through.

**Fully reproducible**: N/A (not Confirmed).

## Root cause

Not applicable (no bug found) - but to restate why the suspect turned out safe: `io_uring_ptr`'s
layout has no implicit compiler-inserted padding on any of its 4 `cfg`'d variants (16/32-bit,
big/little-endian) - the type was deliberately designed with an explicit pad field sized to make
every variant exactly 8 bytes with no gap, and `io_uring_ptr::new()` zero-initializes every declared
pad field unconditionally. The real risk class this candidate description pattern-matched against
(a bare scalar `mem::transmute` from an integer into a pointer-bearing type, later dereferenced) is
real and independently reproducible under Miri (see sanity-harness result 2 above) - but it isn't
reachable through anything `io_uring_ptr`/`io_uring_user_data` themselves do; it would require a
downstream caller to deliberately bypass the real memory round trip, which is not how the types are
used anywhere in this crate.

## Introduced by

N/A - no bug confirmed.

## The fix

None - no bug confirmed, nothing to fix.

- Project test suite: not run as a before/after pair (no patch was applied to the main tree at any
  point this session - no source file in the main project's working tree was ever modified). For
  reference, the existing `cargo test --lib --features "std,all-apis,use-libc" io_uring_layouts`
  passes (`1 passed; 0 failed`), same as before this session, in the isolated worktree used for
  investigation.

## What's not done / open questions

- Downstream crates (`io-uring`, `tokio-uring`, etc.) that actually implement the SQ/CQ ring-buffer
  management on top of rustix's `io_uring` types were not audited for whether any of them do a bare
  scalar `mem::transmute`-style decode of a kernel-echoed `user_data` `u64` directly into a pointer
  without routing it through a real memory read first - sanity-harness result 2 above shows that
  specific pattern (not present in rustix itself) is real, reproducible UB under Miri, independent
  of this candidate.
- The existing `io_uring_layouts` test cannot currently be run to completion under
  `-Zmiri-strict-provenance` at all, because it fabricates its test input pointer via `MAGIC as
  usize as *mut c_void` (needed to verify the layout against a specific known bit pattern) - this is
  a pre-existing, inherent limitation of testing this kind of raw bit-layout equivalence under
  strict-provenance Miri, not something a source change can fix without changing what the test
  verifies; not pursued further this session since it isn't a soundness bug, just a Miri-coverage
  gap. Noted as a possible low-value future candidate (`NewSuspectsNoticed`) only to the extent of
  "what would it even look like to write a provenance-preserving version of this test" - not
  obviously worth a dedicated session.
- The `assert_eq_align!(io_uring_ptr, u64)` static assertion in this same test is independently
  known (from this session's own GitHub issue sync, cached in `__issues_raw/issues/1645.json`) to fail
  to *compile* on `x86_64-unknown-linux-gnux32`/`mips64-openwrt-linux-musl`/
  `sparc-unknown-linux-gnu`/`aarch64-unknown-linux-gnu_ilp32` (already reported upstream as issue
  #1645) - a real, already-known bug, but a compile-time alignment-assertion failure on exotic ABIs,
  not the Miri/padding/provenance runtime-UB question this candidate asked about. Left to its
  already-open upstream issue, not re-investigated as a new candidate here.
