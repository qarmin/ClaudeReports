# errno_from_raw_os_error_zero_panic

`rustix::io::Errno::from_raw_os_error(0)` aborts the whole process on the default `linux_raw`
backend, reachable from safe public API on caller-supplied input. Sourced from GitHub issues
#1578 (a real-world crash with an attached corefile, reported against Smithay) and #1593
(cross-backend inconsistency).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | `src/backend/linux_raw/io/errno.rs` `from_errno`/`from_raw_os_error`, and the `libc` backend equivalent | Confirmed the exact mechanism before writing any repro |
| Standalone repro, debug + release build | `cargo run` (dev profile) and `cargo run --release`, values `{0, -1, 2, 4096, 100000, i32::MIN, i32::MAX}` | Panics on every out-of-`1..4095`-range value including `0`; `2` (ENOENT) works fine |
| `git log -S` / `git blame` | on `assert!(encoded >= 0xf001)` and `raw.wrapping_neg() as u16` | Pinned the introducing commit precisely |
| `cargo test --workspace` before/after fix | default features, x86_64-unknown-linux-gnu | No regressions; suite never exercised `from_raw_os_error(0)` before |

## Status

**Confirmed**. 3/3 identical runs before the fix (deterministic pure-function panic, no flakiness
possible), 3/3 identical runs after the fix (no panics, legitimate errno round-trip still correct).

**Fully reproducible**: Yes. This is a single-participant, pure public-API function call
(`rustix::io::Errno::from_raw_os_error(raw)`) - no peer/caller needs to be faked, the repro calls
the real entry point directly with the exact input a real caller would supply.

## Class

`crash` (primary). A secondary `logic`/API-inconsistency aspect exists too (#1593): the `libc`
backend never panics for the same inputs, so behavior silently differs by backend/feature flag.

## Root cause

In `src/backend/linux_raw/io/errno.rs`, `from_errno(raw: u32)` computes
`encoded = raw.wrapping_neg() as u16` and then does `assert!(encoded >= 0xf001)`. For `raw == 0`,
`wrapping_neg()` is a no-op (`0 -> 0`), so `encoded == 0`, which fails the assert and aborts the
process - even though `0` means "no error," not an actual errno. The same assert also fires for
any `raw` outside Linux's `1..4096` errno range (negative values, `>= 4096`, `i32::MIN`,
`i32::MAX`), all reachable via the public `from_raw_os_error(raw: i32)`.

Internally, callers that already know about this hazard defensively guard against it before
calling `from_raw_os_error`/`from_errno` (e.g. `from_io_error` checks `(1..4096).contains(&raw)`
first; `socket_error()` in both `linux_raw/net/sockopt.rs` and `libc/net/sockopt.rs`
special-cases `err == 0` from `getsockopt(SO_ERROR)`), confirming this is a known-dangerous input
pattern the public constructor itself never protected against.

Notably, the `assert!` is now vestigial from a soundness standpoint: the struct's
`#[rustc_layout_scalar_valid_range_start/end(0xf001..0xffff)]` attributes that this assert used to
back (making the range a real compiler-enforced niche invariant) are commented out as of commit
aab94640 "Completely remove `rustc_attrs`" (#1663) - so today the assert buys zero soundness
benefit and purely serves as a liability that can abort the process on legitimate input (`0`).

Cross-backend inconsistency (#1593): `src/backend/libc/io/errno.rs`'s `from_raw_os_error` is
`Self(raw)` with no validation at all - it silently accepts `0` and any other value. Both issues
stem from the same design gap: no defined behavior was ever specified for `from_raw_os_error` on
`0` or out-of-Linux-errno-range inputs, so each backend independently chose different (and in
`linux_raw`'s case, actively harmful) undocumented behavior.

## Introduced by

401381b3 "Store Linux error codes in negated form." (2021-07-22, Dan Gohman). Confirmed via
`git log --all -S "assert!(encoded >= 0xf001)"` and
`git log --all -S "raw.wrapping_neg() as u16"` - both point to this single commit, which
originally used a `cfn_assert!` macro (`const_fn_assert` crate, since `assert!` wasn't usable in
`const fn` on stable Rust at the time) and was later swapped for plain `assert!` once that
stabilized.

## The fix

Removed the hard `assert!` entirely (see `patch.diff`) rather than downgrading it to
`debug_assert!`. A `debug_assert!` was rejected because it would still abort in ordinary debug
builds (`cargo run`/`cargo test` without `--release` - common during development, e.g. for the
Smithay project that hit this in the wild), so `from_raw_os_error(0)` would still crash there,
failing to actually fix #1578 in the environment where it most plausibly bit the reporter. Since
the type's range invariant is no longer compiler-enforced (the
`rustc_layout_scalar_valid_range_*` attributes were fully commented out in commit aab94640,
#1663), there is no remaining soundness reason to validate the range at all, so it's safe to just
store whatever `u16` results from the encoding, for any input. This also makes `linux_raw`
policy-consistent with `libc` in spirit (both now silently accept out-of-range/zero input rather
than panicking) - addressing #1593's cross-backend-consistency concern, though `libc`'s own file
was left unchanged since it never panicked to begin with.

One residual, intrinsic inconsistency remains and was **not** unified: `linux_raw`'s `Errno`
stores a 16-bit encoded value while `libc`'s stores the full `i32`, so extremely large
out-of-range inputs (e.g. `raw = 100000` or `i32::MIN`/`i32::MAX`) round-trip losslessly through
the `libc` backend but get truncated/wrapped through `linux_raw`'s 16-bit encoding (e.g.
`from_raw_os_error(100000).raw_os_error()` becomes `-31072`, not `100000`). Fully unifying that
would require widening `linux_raw`'s internal storage type - a bigger, more invasive change than
fits a bounded one-session fix. The alternative of changing the return type to `Option<Self>` was
also considered and rejected as an unnecessary breaking public-API change when the assert can
simply be removed instead.

- Project test suite: `cargo test --workspace`, default features - 22 passed / 0 failed / 1
  ignored (+ 7 doc-tests) before, identical after. No regressions.

## What's not done / open questions

- `linux_raw`'s 16-bit truncation of very large out-of-range `raw_os_error` round-trips (see
  above) was flagged but not fixed - a maintainer should decide whether this residual asymmetry
  with the `libc` backend matters enough to widen the internal representation.
- Other call sites in the codebase that call `Errno::from_raw_os_error` on a raw C-library return
  value without pre-checking for `0` (`src/backend/libc/pty/syscalls.rs:100`,
  `src/backend/libc/termios/syscalls.rs:528`, `src/net/wsa.rs:27` and `:46`) were noted but not
  independently investigated - on the `libc` backend they're harmless today since that backend
  never panicked, but they're the same "0 sneaks through" pattern and worth a quick look if this
  guard is ever tightened elsewhere. Not added as a new backlog candidate since they're already
  safe under the fix applied here.
- The `try_decode_raw_fd`/`try_decode_void`/`try_decode_error` functions in the same file use
  `debug_assert!(raw.is_in_range(-4095..0))` on **syscall return values** (not arbitrary external
  input) - lower risk since the input source is the kernel itself, not FFI/user data, so this was
  not treated as the same bug class and was not investigated further.
