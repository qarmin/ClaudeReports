# wasip2_wasip3_wasip1_threads_parity_check

Follow-up to candidate 081, which found that rustix's own `#[cfg(test)]` `Stat`-layout self-check
(`src/fs/constants.rs`) does not compile for `wasm32-wasip1`, but arbitrarily only checked that one
wasi target and explicitly left open whether the sibling targets - `wasm32-wasip2`,
`wasm32-wasip1-threads`, and `wasm32-wasip3` - hit the identical three remaining errors or
something additionally different. No existing candidate or `IDEAS.md` entry covered this.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct cross-target compile probe (same technique as 081, extended to 3 more targets): `cargo test --no-run --target <T> --features "std,fs" --lib` for `wasm32-wasip1` (081's already-confirmed baseline, re-run for a like-for-like comparison), `wasm32-wasip2`, and `wasm32-wasip1-threads` - all 3 have a prebuilt `rust-std` via plain `rustup target add`, stable toolchain, no `-Zbuild-std` needed | stable 1.98.1, 3 repeat runs per target | **Byte-identical 24-error set across all 3 targets** (after normalizing Cargo's own non-deterministic dependency-build-order noise in the log, which is irrelevant - only `^error` lines were compared): the same 18 `st_atime`/`st_mtime`/`st_ctime`(+`_nsec`) `E0609`/`E0026` field errors 081 found, plus the same unrelated `ugid`/`dup2` pair. Reproduces 3/3 runs per target, deterministic (a compile error over fixed source + a fixed `libc` version, not runtime behavior) |
| Same probe for `wasm32-wasip3`, which has **no prebuilt `rust-std`** at all (`rustup target add wasm32-wasip3` fails outright: "no prebuilt artifacts available... this may happen to a low-tier target") - escalated to `cargo +nightly test --no-run --target wasm32-wasip3 -Zbuild-std=std,core,alloc,panic_abort --features "std,fs" --lib` (nightly 1.101.0 + `rust-src` component, a real compile against the real target, not an emulation shortcut) | nightly 1.101.0-nightly (2026-09-29), `-Zbuild-std`, 3 repeat runs | **Also byte-identical** to the other 3 targets (the only difference is cosmetic rustc-version wording for one `E0433` message - "cannot find `ugid` in the crate root" on nightly vs "cannot find `ugid` in `crate`" on stable - same error code, same location, same cause). Reproduces 3/3 runs |
| Applied candidate 081's existing `patch.diff` (verbatim, unmodified) to the worktree and re-ran all 4 targets | same 4 targets, 1 run each post-patch | The 18 `st_atime`/`st_mtime`/`st_ctime` errors disappear on **all 4** targets, leaving only the same pre-existing, out-of-scope `ugid`/`dup2` pair (3 errors) on each - identical residual across all 4, same as 081 found for `wasm32-wasip1` alone |

## Status

**Confirmed.** The `st_atime`/`st_mtime`/`st_ctime` (+`_nsec`) compile-error signature on
`wasm32-wasip2`, `wasm32-wasip1-threads`, and `wasm32-wasip3` is byte-for-byte identical (modulo one
cosmetic nightly-vs-stable `rustc` diagnostic wording difference, not a different bug) to the
already-`Confirmed` `wasm32-wasip1` case from candidate 081 - reproduces 3/3 runs per target,
inherently deterministic (a `rustc` compile error, not runtime behavior). This is the **same bug**
reaching 3 more targets, not a new or different bug - candidate 081's existing fix, applied here
unmodified, resolves it identically on all 4.

**Fully reproducible**: **Yes**. Single-participant finding (rustix's own `#[cfg(test)]` code
failing to compile against the project's own real, unmodified, vendored `libc` dependency for 4
real Rust targets) - no adversarial peer/caller to simulate, so `CONFIRMATION.md` section 11's
two-participant bar doesn't apply (same category as 081 itself). `repro/check.sh` runs the real
`cargo test --no-run --target <T>` (or, for wasip3, the real `-Zbuild-std` cross-compile) against a
real, freshly-cloned checkout of the project at the exact commit this candidate was found on -
verified end-to-end via `just test` from a genuinely fresh `git clone` (not just against the
already-built worktree) before writing this up.

## Class

build-failure

## Root cause

Identical to candidate 081's: `src/backend/libc/fs/types.rs` defines `pub type Stat = c::stat;` for
every `libc`-backend target that isn't `linux_like`/`hurd`/`netbsd`, and wasi (all 4 variants in
scope here) falls into this group. `libc`'s own WASI binding (`libc-0.2.189`'s `src/wasi/mod.rs`)
represents `stat`'s timestamps as **nested** `st_atim`/`st_mtim`/`st_ctim: timespec` sub-structs
(matching WASI's own `__wasi_filestat_t` naming) rather than the flat `st_atime`/`st_atime_nsec`
etc. integer fields `src/fs/constants.rs`'s `#[cfg(test)]` self-check assumes. Since `libc`'s wasi
`stat` binding is identical in shape for every `wasm32-wasi*` triple (`wasip1`, `wasip2`,
`wasip1-threads`, and `wasip3`, confirmed by grepping the vendored `libc-0.2.189` source: the same
`src/wasi/mod.rs` struct definition is shared by all of them - there is no per-wasi-variant `stat`
layout in `libc` at all, they all go through the same `target_os = "wasi"` cfg branch), the exact
same field-name mismatch applies to all 4 identically. This also explains why no "additionally
different" bug turned up: there is only one `stat` layout definition in `libc` for the entire wasi
family, so there was no structural opportunity for the 3 untested siblings to diverge from
`wasm32-wasip1`'s already-confirmed case.

## Introduced by

Same as candidate 081: 7202da6e "Reorganize Stat, Statx, and SockaddrStorage (#1282)" (2025-01-29),
which introduced the current `check_renamed_struct_field!(Stat, stat, st_atime/_mtime/_ctime...)`
call chain and the `assert_eq!(some_stat.st_atime, ...)` block in their present form. This is the
same regression reaching 3 more targets, not a separately-introduced bug - no new `git log -S`/
`git blame` work was needed since the affected lines are identical to 081's.

## The fix

No new fix was written - candidate 081's existing `patch.diff` (gating the flat-timestamp-field
checks behind `#[cfg(not(target_os = "wasi"))]`) was re-applied **verbatim, unmodified** and
verified to also resolve this exact error class on `wasm32-wasip2`, `wasm32-wasip1-threads`, and
`wasm32-wasip3` - it already covers all 4 wasi targets because the cfg predicate is
`target_os = "wasi"`, which all 4 share (there is no `wasip1`-vs-`wasip2`-vs-`wasip1-threads`-vs-
`wasip3` distinction at the `target_os`/`target_env` level that rustix's own `cfg` machinery
exposes - confirmed by checking that none of the 4 targets' own `cfg` differ in anything this
patch's predicate touches). `patch.diff` here is a byte-for-byte copy of 081's for traceability
(so this folder's own `just test` is self-contained and doesn't depend on 081's folder still
existing), not a new patch.

**Checked against more than the one triggering symptom (CONFIRMATION.md section 7)**: verified on
all 4 targets independently (not just re-confirming `wasm32-wasip1`), and re-ran each 081's/this
candidate's own repro 3 times per target before and after the patch to confirm the specific error
signature disappears deterministically, not just absent on one lucky run.

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`,
  host `x86_64-unknown-linux-gnu`): **411 passed, 1 failed, identical before and after** - the one
  failure is the pre-existing, environment-caused `ioctl::test_int_setter` (`/dev/net/tun` missing
  in this sandbox, documented in `projects/rustix.md`), not a regression from this patch. (This
  patch only touches `#[cfg(test)]` code gated by `target_os = "wasi"`, so it cannot affect the
  host-target test suite at all - re-run anyway for the mandatory before/after check.)

## What's not done / open questions

- The two separate, pre-existing `ugid`/`dup2` compile errors (candidate 081's own "what's not
  done" item) remain unfixed on all 4 targets after this patch, same as on `wasm32-wasip1` alone -
  still a distinct bug class, still out of scope for this candidate, still worth a follow-up.
- `wasm32-wasip3`'s own compile path (`-Zbuild-std` on nightly, since no prebuilt `rust-std` exists
  for it yet) is inherently a slightly different *tooling* path than the other 3 targets' plain
  `rustup target add` + stable `cargo test --no-run` - worth noting for a future session that this
  target's own eventual CI story will need either a prebuilt `rust-std` (once upstream ships one)
  or a `-Zbuild-std`/nightly step, not just a `rustup target add`.
- Per candidate 081's own still-open item: a `grep -rn "\.st_atime\b\|\.st_mtime\b\|\.st_ctime\b"`
  across `src/` beyond `src/fs/constants.rs` was still not done this session either - this
  candidate only extended 081's existing finding to 3 more targets, it did not sweep for additional
  call sites of the same bug class.
