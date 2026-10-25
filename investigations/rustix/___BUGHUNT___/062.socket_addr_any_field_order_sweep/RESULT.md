# socket_addr_any_field_order_sweep

`src/net/socket_addr_any.rs`'s `SocketAddrAny::new()` (line 103) and `SocketAddrAny::read()` (line
128) both build the return value with `Self { storage, len }`, while the struct itself is declared
as `pub(crate) len: NonZeroU32` then `pub(crate) storage: MaybeUninit<SocketAddrStorage>` (lines
81-82) - the opposite order, flagged by `clippy::inconsistent_struct_constructor`. Flagged as
"should be harmless under current Rust semantics but worth confirming no init-order hazard."

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static analysis: `cargo clippy --no-default-features --features std,all-apis --lib -- -W clippy::inconsistent_struct_constructor` | Default clippy 1.98.0, whole `net` module in scope | Confirms the lint fires at exactly both reported sites (`socket_addr_any.rs:103` and `:128`), nothing else in `src/net/` - the suspect description matches the actual tool output, not a guess. |
| Manual reasoning against the Rust reference's struct-literal evaluation-order rule, plus a source read of both constructors and the struct/field definitions | `src/net/socket_addr_any.rs` lines 76-129 | In `Self { storage, len }`, both `storage` (a function parameter / already-filled `MaybeUninit` local) and `len` (a `NonZeroU32` already computed on the line immediately above via `NonZeroU32::new_unchecked`) are simple variable moves by the time the literal executes - neither is a side-effecting expression, so the order they're *written* in the literal (which Rust evaluates left-to-right, independent of declared field order) cannot observably differ from any other order. Confirmed no `Drop` impl exists on `SocketAddrAny`, `SocketAddrBuf`, or either field type (`MaybeUninit<T>` and `NonZeroU32` have no drop glue) via `grep -n "impl Drop" src/net/*.rs` (only hit: an unrelated `impl Drop for RecvAncillaryBuffer` in `src/net/send_recv/msg.rs`) and `grep -n derive src/net/socket_addr_any.rs` (only `#[derive(Clone)]`, no `Drop`) - so even a hypothetical future reordering-sensitive field-drop scenario doesn't apply here either. |
| Runtime construction, debug build + Miri: a standalone example (`investigation/saa_check.rs`, kept in this folder, run via `cargo run --example` and `cargo +nightly miri run --example` against the real public `rustix::net::SocketAddrAny::new`/`::read`) exercising both constructors at a minimal realistic length (`sockaddr_in`-sized, 16 bytes) and at the maximum allowed length (`size_of::<SocketAddrStorage>()`, 128 bytes on this platform) | `--no-default-features --features std,all-apis`, x86_64-unknown-linux-gnu, rustc/miri 1.98.1 nightry-5c543b0b8c | Both constructors return a value whose `address_family()` correctly reflects the family byte written into `storage` in every case; Miri (which models Stacked Borrows / provenance and would flag any genuinely observable mis-ordering, use of uninitialized memory, or UB) raised no diagnostic across any of the three calls. See `output.txt` for full captured output. |

## Status

**Not reproducible.** No init-order hazard exists: the field order clippy flags is purely a
stylistic/lint-level inconsistency between how the struct is declared and how the literal is
written; it has no effect on runtime behavior because (a) Rust struct-literal fields are placed
into their named slots regardless of the order the literal lists them in, (b) the two values
plugged into this particular literal are already fully-formed, side-effect-free local bindings by
the time the literal executes, and (c) there is no `Drop` impl anywhere in the picture whose
declared-field-order-dependent drop sequence could matter. Covered: both affected call sites,
manual reasoning from the struct/field definitions, and an actual debug-build + Miri run
exercising both constructors at a minimal and a maximum-boundary `len`.

## Root cause

N/A - not a bug. The suspected risk (some runtime-observable consequence of evaluating `storage`
before `len` in the literal, versus the struct's own declared `len`-then-`storage` order) doesn't
exist for this code: Rust's struct-literal semantics evaluate listed fields in the order *written*
in the literal and then move each into its namesake slot - the declared field order in the
struct's own definition has no bearing on evaluation order or placement, and only matters at all
when a field's `Drop` has an order-observable side effect, which doesn't apply here (no `Drop`
anywhere in this type or its fields).

## Introduced by

N/A - not applicable, no bug confirmed. (For reference only, since it was looked up while tracing
the history: `SocketAddrAny::new`'s `Self { storage, len }` pattern was introduced in 5654c117 "Add
trait to reduce duplicated code between socket address types, implement netlink addresses (#1004)",
and `SocketAddrAny::read`'s copy of it in 29e75245 "Add a `read` function to convert a raw
representation to `SocketAddrAny`. (#1346)".)

## The fix

N/A - no bug found, nothing to fix. (A cosmetic clippy-silencing fix - swapping the two fields'
order in the literal to `Self { len, storage }` at both sites - is a one-line-each, zero-behavior-
change edit a maintainer could make purely to quiet the lint; not applied here since there is no
confirmed bug to fix, per this skill's own "attempt a bounded fix" step only applying to confirmed
findings.)

- Project test suite: not run - no behavior change was made to the tree (nothing was applied; the
  investigation only added a throwaway `examples/saa_check.rs` in the isolated worktree, discarded
  afterward), so there is nothing for the suite to validate before/after. N/A.

## What's not done / open questions

- Did not sweep the rest of the crate for other `clippy::inconsistent_struct_constructor` hits
  beyond these two sites (this candidate was scoped to the two named lines); if a future session
  wants a full-crate sweep of the same lint, `cargo clippy -- -W clippy::inconsistent_struct_constructor`
  lists all of them (one additional unrelated hit was observed in passing, in `fs`-backend dirent
  reading code, not investigated here since it's out of this candidate's scope).
- No non-x86_64 architecture was exercised (container is x86_64-only); the struct-literal
  evaluation-order semantics this candidate relies on are a language-level guarantee (not
  architecture-dependent), so this isn't expected to change the conclusion, but wasn't verified
  under e.g. a big-endian target.
