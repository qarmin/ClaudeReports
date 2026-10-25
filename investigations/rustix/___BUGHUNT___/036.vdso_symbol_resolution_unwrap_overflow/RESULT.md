# vdso_symbol_resolution_unwrap_overflow

While walking the vDSO's classic (`DT_HASH`) or GNU (`DT_GNU_HASH`) hash table in
`src/backend/linux_raw/vdso.rs`'s `Vdso::sym()`, once a candidate symbol passes
`check_sym` (right name/type/bind/version), the code does
`let sum = self.addr_from_elf(sym.st_value).unwrap();` immediately followed by an
`assert!` bounding `sum` within `[load_addr, load_end]`. `addr_from_elf` legitimately
returns `None` when `st_value` added to the vDSO's mapped base overflows `usize` -
panicking via `.unwrap()` before the `assert!` that was clearly meant to be the real
safety check for a bogus-but-non-overflowing value ever runs. Flagged via code reading
against `HEURISTICS.md`'s "a later dedicated validity check exists, yet an earlier
unwrap can fire first" pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `Vdso::sym()` (both the `gnu_hash` and classic-`DT_HASH` branches) and `addr_from_elf`/`base_plus` | n/a | Confirmed both branches share the identical `let sum = self.addr_from_elf(sym.st_value).unwrap(); assert!(...)` shape; `base_plus` does `(self.load_addr as usize).checked_add(offset)`, so `addr_from_elf` returns `None` exactly when `load_addr + st_value.wrapping_add(pv_offset)` overflows `usize` |
| Manual boundary-value construction: a hand-built, in-process, minimal classic-`DT_HASH` `Vdso` (one bucket, one symbol) with `st_value` chosen as the exact two's-complement negation of the table's own real `load_addr`, added as a `#[cfg(test)]` test directly in `vdso.rs` (this project's own test module) since `Vdso`/`sym()` are `pub(super)` with no public entry point to drive from outside the crate | debug build, `cargo test --lib --features time,stdio,fs`, repeated 5x | Panicked 5/5 runs, deterministically, at `src/backend/linux_raw/vdso.rs:394:68`, `called 'Option::unwrap()' on a 'None' value` - exactly the claimed bug |
| Same test, release build | `cargo test --release --lib --features time,stdio,fs` | Panicked 1/1 run, same location/message - confirms this is not a debug-assertion-only manifestation (`.unwrap()` panics unconditionally regardless of profile) |
| Fix: replace the `.unwrap()` + `assert!` pair with a helper that folds the overflow case and the bounds-check case into the same `None` outcome, returning a null pointer (treat as "symbol not found", the same fallback `Vdso::sym()`'s own callers in `vdso_wrappers.rs::init()` already handle for other reasons) instead of aborting the process; re-ran the same test against the patched tree, both builds | debug + release | Both: test **passes** (no panic, `sym()` returns null) |

## Status

**Confirmed**. Reproduces 5/5 debug-build runs + 1/1 release-build run, deterministic (the
boundary value is computed from the test's own real runtime address every run via
`wrapping_neg()`, not a fixed/seeded guess - `load_addr.wrapping_add(st_value) == 0` is
guaranteed by construction to overflow `checked_add`, not a matter of chance).

**Fully reproducible**: **No**, with justification. `Vdso`/`Vdso::sym()` are `pub(super)` -
there is no public API entry point to this code at all; it's reached only once per process,
internally, by `vdso_wrappers.rs`'s own private `init()`, itself invoked lazily the first time
a public call like `rustix::time::clock_gettime()` needs it. The untrusted input this bug is
about - a vDSO ELF image whose symbol table has a corrupted `st_value` large enough to overflow
pointer arithmetic once added to the real load address - is supplied once, by the kernel, at
process start via `AT_SYSINFO_EHDR`; the kernel's own real vDSO is a legitimate, sanely-addressed
shared object, so getting a genuinely kernel-produced malformed one would need a custom/modified
kernel build (not writing a corrupted vDSO ourselves and feeding it in some other way, which
would just be faking the kernel's own output). Per `CONFIRMATION.md` section 11, this falls into
the same "a single external artifact, no live second participant" category as a malformed file
handed to a parser's public entry point - except here there's no *public* entry point to call
even with hand-constructed bytes, since the parser itself is private. The repro instead drives
the real, unmodified, private `Vdso::sym()` method directly (a `#[cfg(test)]` test added to the
same file, same module the project's own `test_vdso` already lives in - not a separate faked
"second participant"), with a hand-built minimal symbol table standing in for what a corrupted
kernel vDSO's table would contain. See `repro/README.md` for the full reasoning.

## Class

crash

## Root cause

`Vdso::sym()` finds a symbol whose name/type/bind/version match what's being looked up, then
computes its usable address via `addr_from_elf(sym.st_value)`, which in turn calls
`base_plus(elf_addr.wrapping_add(pv_offset))` - and `base_plus` does
`(self.load_addr as usize).checked_add(offset)`, returning `None` on overflow specifically so
callers can detect and handle a bogus/overflowing address. `sym()` instead immediately
`.unwrap()`s that `Option`, so an `st_value` large enough that `load_addr + st_value.wrapping_add(
pv_offset)` overflows `usize` panics right there - never reaching the `assert!` on the very next
line, which checks that the (successfully computed) address falls within `[load_addr, load_end]`
and was clearly written as the intended defense against exactly this kind of bogus value. Both
the GNU-hash branch (line 374 before the fix) and the classic-hash branch (line 394) share the
identical pattern byte-for-byte.

## Introduced by

23d9add5cf18ecf7520bcc5a64cbc2683d5e9b35 "Check that the vDSO is not writeable before attempting
to parse it." (2021-08-24). Before this commit, the equivalent code was
`return match self.load_offset.checked_add(sym.st_value) { Some(sum) => sum as *const c_void, None
=> null() };` - overflow was handled gracefully, returning a null pointer (symbol not found).
This commit replaced that with `let sum = self.load_offset.checked_add(sym.st_value).unwrap();
assert!(sum >= self.load_addr && sum <= self.load_end); return sum as *const c_void;`, introducing
the unwrap-before-assert ordering bug directly (confirmed via `git show
23d9add5cf18ecf7520bcc5a64cbc2683d5e9b35 -- src/backend/linux_raw/vdso.rs` /
`src/imp/linux_raw/vdso.rs`, the path at the time). The code was later refactored (6c6fd1e0
"Preserve provenance in the vDSO parsing code too.") into the current `addr_from_elf()`
helper and a second, byte-identical call site was added for the `DT_GNU_HASH` branch
(22684024 "Implement the `DT_GNU_HASH` mechanism for vDSO parsing. (#1259)", 2025-01-13) -
neither of those two later commits changed the unwrap-before-assert shape itself, both inherited
it as-is from 23d9add5.

## The fix

Replaced both call sites' `let sum = self.addr_from_elf(sym.st_value).unwrap(); assert!(...)`
pairs with a single new private helper, `checked_sym_addr`, that folds the overflow case
(`addr_from_elf` returning `None`) and the out-of-range case (the former `assert!`'s own check)
into the same outcome - both mean "this symbol's address can't be trusted" - returning `None`
for either, which both call sites then turn into a null pointer (`Vdso::sym()`'s documented
"not found" return) instead of aborting the whole process:

```rust
unsafe fn checked_sym_addr(&self, st_value: usize) -> Option<*mut c::c_void> {
    let sum = self.addr_from_elf(st_value)?;
    if sum as usize >= self.load_addr as usize && sum as usize <= self.load_end as usize {
        Some(sum as *mut c::c_void)
    } else {
        None
    }
}
```

This matches the pre-23d9add5 (2021) design's own choice for the overflow case specifically, and
extends the same graceful-degradation philosophy the rest of `init_from_sysinfo_ehdr` already
uses throughout (bailing out via `None`/`?` on any other structurally-invalid vDSO condition,
never panicking) to this one remaining spot that didn't follow it. `vdso_wrappers.rs::init()`
already tolerates a null return from `sym()` in the general/32-bit-missing-symbol case; this
doesn't introduce a new code path there, just reaches an existing one via a new cause.

Verified:
- Re-ran the repro test against the patched tree (both debug and release): no panic, `sym()`
  returns a null pointer as expected, test passes (`CONFIRMATION.md` section 6).
- Checked a second input in the same bug class (`CONFIRMATION.md` section 7): the pre-existing
  `test_vdso` test (which exercises the real kernel-supplied vDSO's genuine, well-formed symbols
  on this host) still passes unchanged after the fix, confirming the fix doesn't affect the normal
  non-overflowing path at all.
- `cargo clippy --lib --features time,stdio,fs` on the patched tree: no new warnings in
  `vdso.rs` (a pre-existing, unrelated `build.rs` lint only triggers under `-D warnings`, present
  identically before and after).
- Project test suite, `--features time,stdio,fs --no-fail-fast`:
  - Before fix (repro test added, fix not yet applied): lib unittests 20 passed / **1 failed**
    (the new repro test) / 0 ignored.
  - After fix: lib unittests **21 passed / 0 failed** / 0 ignored.
  - Two unrelated, pre-existing failures are present identically before and after (confirmed by
    also running them against pristine/unmodified code with the same feature set):
    `tests/io`'s `ioctl::test_int_setter` (sandbox has no real tty device, `ENOENT`), and two
    doctests (`timespec::Timespec::checked_add`/`checked_sub`) that need the `event` feature,
    not included in the `time,stdio,fs` set used here.
  - Default-feature build (`cargo test --workspace`, vdso module not even compiled - needs
    `feature = "time"` or `"thread"`): 22 passed / 0 failed / 1 ignored + 7 doc-tests, identical
    before and after (matches this project's previously-recorded baseline in
    `projects/rustix.md`).

## What's not done / open questions

- Only the symbol-address overflow case was fixed here; the pre-existing out-of-range (but
  non-overflowing) case that the original `assert!` guarded against now also returns null instead
  of aborting - a deliberate, bundled part of this same fix (both cases mean "untrustworthy
  address"), not a separate bug, but worth flagging explicitly since it's a small behavior change
  beyond just the overflow path.
- No genuine end-to-end repro with a real corrupted/adversarial kernel-supplied vDSO was possible
  in this sandboxed environment (see "Fully reproducible" above) - that would need a custom or
  modified kernel build. A session with the ability to build and boot a custom kernel could
  tighten this to a fully genuine repro.
- Not checked this session: whether any other caller elsewhere in the crate relies on `sym()`
  aborting the process (via the old assert) rather than returning null for an out-of-range
  address; a brief scan of `vdso_wrappers.rs::init()` (the only caller) shows it already tolerates
  a null return in every branch that doesn't itself immediately `assert!(!ptr.is_null())` for a
  symbol documented as always available on 64-bit platforms - that remaining assert is pre-existing,
  separate code, not touched by this fix.
