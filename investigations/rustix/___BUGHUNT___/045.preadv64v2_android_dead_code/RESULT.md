# preadv64v2_android_dead_code

`src/backend/libc/c.rs`'s `readwrite_pv64v2` module (lines ~405-475) defines a raw-syscall
fallback `preadv64v2`/`pwritev64v2` pair for "non-glibc" targets, gated `#[cfg(any(target_os =
"android", all(target_os = "linux", not(target_env = "gnu"))))]`. The module's only possible
caller, `src/backend/libc/io/syscalls.rs::preadv2`/`pwritev2`, is gated
`#[cfg(all(linux_kernel, not(target_os = "android")))]` - excluding Android. That leaves the
Android half of the module's `cfg` compiled on every Android build, but never callable from
anywhere in the crate on that target: a real `cargo clippy --target aarch64-linux-android
--features std,all-apis,try_close` reports `preadv64v2`/`pwritev64v2` as "never used". This looked
suspicious as a bit-rot hazard (see HEURISTICS.md's "a cfg change that forgets a sibling cfg"
pattern) - not a runtime-incorrect-behavior bug (the public `rustix::io::preadv2`/`pwritev2` API is
simply and apparently intentionally absent on Android), but exactly the kind of latent mismatch
that could silently regress either way (code deleted as "obviously dead" losing real intent, or
a future cfg change re-enabling the caller without noticing the sibling module needs no change).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the code and cross-reference both `cfg` gates (`c.rs` vs. `io/syscalls.rs`) | Manual, a few minutes | Confirmed by inspection: the module's own `cfg` covers Android, but its only caller's `cfg` excludes Android - a definite mismatch, matching the candidate description exactly |
| `cargo clippy --target aarch64-linux-android --features std,all-apis,try_close` (real clippy invocation, cross-compiling, no NDK linker needed since clippy only type-checks/lints) | Default clippy lints, 2 clean runs after `cargo clean -p rustix` each time (3 total including the first) | Reproduced identically every time: `function \`preadv64v2\` is never used` and `function \`pwritev64v2\` is never used`, both pointing at `src/backend/libc/c.rs:414` and `:442` |
| Sibling-target check: same clippy invocation against `x86_64-unknown-linux-musl` (the module's *other* `cfg` arm, `not(target_env = "gnu")`, where it genuinely has a caller) | Default clippy lints | No `preadv64v2`/`pwritev64v2` warning and no build error - confirms the module is reachable (and the fix doesn't accidentally affect) the musl arm |

Only 2 distinct approaches were needed (code reading + a real clippy run), well under the soft
cap of 3 - the finding was unambiguous and fully explained by the two `cfg` gates not matching,
with no partial/flaky signal requiring further digging.

## Status

**Confirmed**

Deterministic, 100% reproducible: 3/3 `cargo clippy --target aarch64-linux-android` runs
(including after a clean rebuild each time) showed the same two "never used" warnings at the same
locations. This is a static-analysis fact about the crate's own `cfg` wiring, not a
layout/seed-dependent runtime observation, so there is no meaningful "reproduction rate" below
100% to report.

**Fully reproducible**: Yes.

The repro (`repro/run_repro.sh`) runs the real, unmodified entry point a maintainer or CI would
actually use - `cargo clippy` invoked directly on rustix's own source tree with the exact
target/feature combination from the candidate description - and reads its real output. There is no
second participant to drive here (unlike a network-protocol finding): the bug is entirely about
whether a function is reachable from any caller *within this one crate's own compiled dependency
graph* under a given `cfg`/feature combination, which `cargo clippy` settles directly and
deterministically by actually performing that reachability analysis - nothing about this finding
depends on bytes or state supplied by an external peer/caller, so CONFIRMATION.md section 11 (which
is about faking a second participant's input) does not apply to this candidate's bug class.

## Class

other (dead-code / bit-rot hazard from two cfg gates on the same syscall pair that no longer agree)

## Root cause

`fda7cbe6` ("rustix: on Linux, support a build without linux-raw-sys", #1478) changed
`io/syscalls.rs`'s `preadv2`/`pwritev2` wrapper functions' `cfg` from `#[cfg(linux_kernel)]` to
`#[cfg(all(linux_kernel, not(target_os = "android")))]`, specifically to stop requiring
`ReadWriteFlags`'s `linux-raw-sys` dependency on Android. That commit did not touch
`backend/libc/c.rs`'s own `readwrite_pv64v2` module at all, whose `cfg` (`any(target_os =
"android", all(target_os = "linux", not(target_env = "gnu")))`) still includes Android. Since the
module's Android half of the `cfg` is now unreachable from any caller (the only caller excludes
Android), rustc/clippy correctly flag it as dead code - this is real, just non-urgent: the compiled
Android artifact is unaffected (dead code generates no observable runtime behavior difference), but
the warning is a true signal that two related `cfg` gates have drifted out of sync with each
other's intent.

## Introduced by

fda7cbe6 "rustix: on Linux, support a build without linux-raw-sys (#1478)" - confirmed directly by
`git show fda7cbe6 -- src/backend/libc/io/syscalls.rs`, which shows exactly the `cfg` change from
`#[cfg(linux_kernel)]` to `#[cfg(all(linux_kernel, not(target_os = "android")))]` on both
`preadv2` and `pwritev2`, with no corresponding change to `backend/libc/c.rs`.

## The fix

Added `#[cfg_attr(target_os = "android", allow(dead_code))]` to the `readwrite_pv64v2` module in
`src/backend/libc/c.rs`, with a comment explaining why (points at the gating mismatch and at
`fda7cbe6`) so a future reader doesn't mistake the `allow` for an oversight or delete the module
outright. This is a minimal, surgical fix for the specific finding (silences the false "this is
unreachable, maybe delete it" signal on Android while visibly documenting that it's deliberate) -
it does not change the module's `cfg` itself, since narrowing that `cfg` to drop Android entirely
would be a larger, more opinionated change (effectively deciding the dead code should be deleted)
that the candidate description explicitly flagged as a two-way risk (deleting it could silently
regress if Android support is ever re-enabled without someone checking this module first).

Verified:
- `cargo clippy --target aarch64-linux-android --features std,all-apis,try_close`: the
  `preadv64v2`/`pwritev64v2` "never used" warnings are gone (0 occurrences, confirmed 2x after a
  clean `cargo clean -p rustix` rebuild each time). Other, unrelated warnings already present in
  the pristine tree (`KernelSigSet`, `clippy::wrong_self_convention` on `timespec.rs`) are
  unaffected - this patch does not touch them.
- `cargo clippy --target x86_64-unknown-linux-musl --features std,all-apis,try_close` (the arm
  where this module genuinely has a caller): identical output before and after the patch, 0
  `preadv64v2`/`pwritev64v2`-related lines and no new errors - confirms the `cfg_attr` is scoped
  tightly enough to not affect the arm where the code is actually used.
- Project test suite (`cargo test --workspace`, default `x86_64-unknown-linux-gnu` host target,
  since Android/musl cross-targets can't actually execute tests in this environment without
  real device/emulator or full qemu+sysroot setup): **22 passed / 0 failed / 1 ignored, plus 7
  doc-tests passed**, identical before and after the patch - expected, since the fix is `cfg`/lint
  metadata only and does not touch any code path reachable on the host target.

## What's not done / open questions

- The fix is purely a `#[cfg_attr(..., allow(dead_code))]` + comment; it does not address whether
  `rustix::io::preadv2`/`pwritev2` *should* actually be supported on Android (that's a product
  decision for the maintainers, not something this session attempts to resolve - the candidate
  description itself frames the current absence as "apparently intentional").
- Not checked: whether any other `cfg`-gated internal module in `backend/libc/c.rs` has the same
  shape (a module gated broader than its only caller) - this session scoped to exactly the
  `preadv64v2`/`pwritev64v2` pair named in the candidate. A `grep`-based sweep across `c.rs`/
  `syscalls.rs` for other `cfg(any(... android ...))` modules whose caller's `cfg` excludes
  Android would be a reasonable follow-up (see `IDEAS.md`).
- No NDK/cross-linker was available in this environment, so the fix was only exercised through
  `cargo clippy` (type-check + lint, no final link step) for the Android target - this is exactly
  what reproduces and fixes the dead-code warning itself, so it's sufficient for this candidate's
  actual finding, but a full `cargo build --target aarch64-linux-android` (requiring a real
  Android NDK toolchain) was not attempted and would be needed to additionally confirm the crate
  still *links* cleanly on Android with this change (expected to be unaffected, since the change
  is `cfg`/attribute-only with no code deleted or moved).
