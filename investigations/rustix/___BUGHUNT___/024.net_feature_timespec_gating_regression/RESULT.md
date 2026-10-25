# net_feature_timespec_gating_regression

Sourced from open issue https://github.com/bytecodealliance/rustix/issues/1689 (#1689): a
feature-gating regression introduced in 1.1.5. Both backends' `net::sockopt` modules
(`src/backend/linux_raw/net/sockopt.rs`, `src/backend/libc/net/sockopt.rs`) reference
`crate::timespec` unconditionally (added to implement the `SO_RCVTIMEO`/`SO_SNDTIMEO` timeout
conversions correctly), but `src/lib.rs`'s `#[cfg(any(...))]` gate for `mod timespec` never
included `feature = "net"` - so a consumer that enables only `net` (not `fs`/`process`/`thread`/
`runtime`/`time`/`event`) fails to compile.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct build of the exact feature combo from the issue | `cargo build --no-default-features --features "std,net"` against this repo's own `287214b8` checkout | Reproduced immediately, deterministically: 6x `error[E0433]: cannot find \`timespec\` in \`crate\`` in `src/backend/linux_raw/net/sockopt.rs`, matching the issue's own pasted output byte-for-byte (error count, file, line numbers) |
| Same, libc backend | `cargo build --no-default-features --features "std,net,use-libc"` | Reproduced: 3x the same `E0433`, matching the issue's stated libc-backend error count exactly |
| Root-cause read | `grep`'d `crate::timespec` call sites in both `sockopt.rs` files, then read `src/lib.rs`'s `mod timespec` gate (lines 389-410) | Confirmed `feature = "net"` is absent from the gate's `#[cfg(any(...))]` list - the single, sufficient root cause; no further tool needed (this is a static feature-flag mismatch, not something that benefits from miri/ASAN/fuzzing) |
| `git log -S'crate::timespec::Timespec'` on both `sockopt.rs` files | - | Pinpointed the exact introducing commit (see "Introduced by") |
| Cross-check against upstream | Fetched issue #1689 and searched open PRs | Found two independent attempts already filed upstream: PR #1690 ("Gate `mod timespec` on the `net` feature", closed/not merged) and PR #1681 ("Compile timespec for `net` feature", open, not merged) - #1681's one-line diff (`+    feature = "net",` in the same `#[cfg(any(...))]` block) is the fix independently applied and verified below |
| Standalone end-to-end repro (`repro/`) | Real downstream crate (own `Cargo.toml`, `rustix = { path = "../clone", default-features = false, features = ["std", "net"] }`) exercising the real public API (`rustix::net::sockopt::{set_socket_timeout, socket_timeout}` against a real `AF_INET`/`SOCK_DGRAM` socket) | 3/3 full `just test` cycles: before-patch build fails with the same 6x `E0433`; after-patch build succeeds and the real socket-timeout round-trip returns the correct value |

## Status

**Confirmed**. Reproduces 3/3 full before/after cycles (`just test`, each a fresh `cargo build`/
`cargo run` with `Cargo.lock` removed beforehand) - this is a deterministic compile-time error, not
a flaky runtime condition, so a bit-for-bit repeat rate applies trivially.

**Fully reproducible**: **Yes**. This is a compile-time regression with no second participant to
fake - the "input" is a real downstream crate's `Cargo.toml` feature selection, and `repro/` is a
genuine consumer crate depending on the real `rustix` checkout via a real `cargo build`/`cargo run`
invocation, using only its public API (`rustix::net::sockopt::*`, `rustix::net::socket`), exactly
reproducing the exact feature combination (`default-features = false, features = ["std", "net"]`)
the issue reports. Beyond proving the build fails/succeeds, it also drives a real socket through
the actual code path that needed `crate::timespec` (`set_socket_timeout`/`socket_timeout`) to
confirm the fix is both compile-correct and runtime-correct.

## Class

logic (feature-gating / compile-time correctness bug - a consumer's valid, documented minimal
feature selection fails to build)

## Root cause

`src/lib.rs` conditionally compiles `mod timespec;` behind an `#[cfg(any(...))]` list of features
that are expected to need it (`fs`, `event`, `process`, `runtime`, `thread`, `time`, plus two
`all(...)` compound conditions). Commit 0cea1880 (see below) added three `crate::timespec::Timespec`
construction sites to `src/backend/linux_raw/net/sockopt.rs` (6 individual `crate::timespec`
references total) and `src/backend/libc/net/sockopt.rs` (3 references), replacing an inline
`(nanos + 999) / 1000` rounding computation with a call into the shared `Timespec::to_sec_usec`
helper - a correct and deliberate fix for a separate overflow bug (the project's own candidate 016
in this workspace covers that fix's correctness). `sockopt.rs`'s own module, `net::sockopt`, is only
ever compiled under `feature = "net"` - but the commit never added `feature = "net"` to
`mod timespec`'s own gate list, so building with `net` enabled and every other `timespec`-needing
feature disabled compiles `sockopt.rs` (which now unconditionally names `crate::timespec::Timespec`
and `crate::timespec::Secs`) while `mod timespec` itself is configured out - `error[E0433]: cannot
find \`timespec\` in \`crate\`` on every reference, in both backends.

## Introduced by

0cea1880 "fix(bug): ns->us overflow handling (#1655)" (2026-09-16) - the same commit already
on record in this workspace as candidate 016's fix for the pre-existing `tv_usec` rounding-overflow
bug. That fix correctly reused `crate::timespec::Timespec::to_sec_usec` in both backends'
`sockopt.rs`, but missed adding `feature = "net"` to `mod timespec`'s own `#[cfg(any(...))]` gate in
`src/lib.rs`, which is the one-line omission this candidate is about. This commit landed 2 days
before the 1.1.5 release (287214b8) that first shipped the regression; 1.1.4 is confirmed by the
issue reporter as unaffected.

## The fix

Add `feature = "net"` to `mod timespec`'s own `#[cfg(any(...))]` gate list in `src/lib.rs` (one line
added, matching upstream's own already-proposed, not-yet-merged PR #1681 exactly - independently
re-derived and re-verified here, not copied blind). `net` genuinely needs the module (it's not a
spurious over-broad gate relaxation): `src/timespec.rs` depends only on `core`, `crate::backend::c`,
and `crate::ffi`, so widening the gate pulls in no further feature-gated code and cannot regress any
other feature combination.

Verified:
- `cargo build --no-default-features --features "std,net"` - fails before (6x E0433), builds clean
  after.
- `cargo build --no-default-features --features "std,net,use-libc"` - fails before (3x E0433),
  builds clean after.
- Standalone `repro/` (real consumer crate, real public API, real socket) - 3/3 full before/after
  cycles via `just test`.
- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc"`, `--no-fail-fast`):
  identical before and after the patch - 411 passed, 1 failed (pre-existing,
  environment-caused: `ioctl::test_int_setter`, `NotFound` opening `/dev/net/tun`, which does not
  exist in this container - already on record in `projects/rustix.md`'s "sandbox has no
  `/dev/net/tun`" gotcha, unrelated to this bug), 5 ignored. `all-apis` already includes `process`/
  `thread`/`time`/`fs`, all of which already gate `mod timespec` in regardless of this fix, so this
  full-feature suite was never capable of exercising the regression itself - it exists here purely
  to confirm the one-line change introduces no regression elsewhere, not as the regression's own
  reproduction (that's `repro/`'s job, using the actual minimal `net`-only feature set).

## What's not done / open questions

- Not checked: whether any other single-feature (or other minimal-combination) build is similarly
  missing a feature from `mod timespec`'s gate, or from any other module's own feature gate
  elsewhere in `src/lib.rs`. This candidate only confirms and fixes the specific `net`-alone gap
  the issue reports; a broader sweep of every `#[cfg(any(feature = ...))]` module gate in
  `src/lib.rs` cross-checked against what each gated module's own file actually references
  (`crate::<module>::`) would be a reasonable follow-up candidate (noted in `IDEAS.md`).
- Two upstream PRs already exist for this exact issue (PR #1681, open, same one-line fix as applied
  here; PR #1690, closed without merging, same fix). Neither has landed as of base_commit
  287214b8 (the 1.1.5 release commit, still `HEAD` on `main` at the time of this session) - this
  candidate's own fix was independently derived from reading the code and cross-checked against,
  not copied from, PR #1681 after the fact.
