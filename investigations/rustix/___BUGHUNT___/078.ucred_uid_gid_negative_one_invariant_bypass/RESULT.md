# ucred_uid_gid_negative_one_invariant_bypass

`src/ugid.rs`'s `Uid::from_raw`/`Gid::from_raw` document "`raw` must be the value of a valid Unix
user ID, and not `-1`", but enforce it only via `debug_assert_ne!(raw, !0)`, compiled out entirely
in a release build. `Uid`/`Gid` wrap a plain `u32` (`RawUid`/`RawGid`) with no niche, so
constructing `Uid::from_raw(u32::MAX)` is not Undefined Behavior - unlike candidates 063/064/065's
`Pid(NonZeroI32)` niche-violation findings (which this candidate is explicitly distinct from, and
which only looked at the `pid` field). The suspicion was that `src/net/send_recv/msg.rs`'s
`cvt_msg` (`SCM_CREDENTIALS` arm, `read_unaligned::<UCred>()`) and both backends'
`socket_peercred` (`src/backend/{linux_raw,libc}/net/sockopt.rs`) construct `UCred`'s `uid`/`gid`
fields directly from raw kernel/getsockopt data without ever calling `from_raw` at all - candidate
063's own RESULT.md "open questions" section explicitly flagged this uid/gid angle as not checked.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: confirmed `Uid`/`Gid` are `#[repr(transparent)]` over plain `RawUid`/`RawGid` (`ffi::c_uint`), no niche, no `NonZeroXxx` anywhere in the type | n/a | Confirms this is not a memory-safety/niche issue like 063/064/065 - any consequence must be a logic bug, not UB |
| Code reading: traced every consumer of a `Uid`/`Gid` value for anything that treats a specific raw bit pattern specially | n/a | Found `src/ugid.rs::translate_fchown_args` (`pub(crate)`, used by `chown`/`fchown`/`chownat`/`lchown` in both backends) uses `!0` (`u32::MAX`) as its own sentinel for "`None` = don't change", mirroring POSIX `chown(2)`'s own `-1 = don't change` convention - the exact value the type's own doc comment says must never occur |
| Debug-assertions build, real public API (`rustix::fs::Uid::from_raw`, `catch_unwind`) | `cargo run` (debug), x86_64-unknown-linux-gnu, `Uid::from_raw(u32::MAX)` | Panics immediately: `assertion \`left != right\` failed, left: 4294967295, right: 4294967295` at `src/ugid.rs:32` - the debug_assert catches direct misuse here, 3/3 runs |
| Release build, real public API (`rustix::fs::fchown`, `rustix::fs::Uid::from_raw`), real temp file, real unprivileged process | `cargo run --release`, x86_64-unknown-linux-gnu, 3/3 repeat runs | **Confirmed**: `Uid::from_raw(u32::MAX)` does not panic; `fchown(&fd, Some(bad_uid), None)` returns `Ok(())` and leaves the file's owner completely unchanged - the kernel's real permission check (which correctly returns `EPERM` for a baseline attempt to chown to a different, validly-shaped foreign uid `99999` in the same process) is never even reached, because `translate_fchown_args` turns `Some(Uid(u32::MAX))` into the exact same raw syscall arguments (`-1, -1`) as `None, None` |
| Same check repeated for `Gid::from_raw(u32::MAX)` via `fchown(&fd, None, Some(bad_gid))` (CONFIRMATION.md section 7, second input in the same bug class) | `cargo run --release`, same environment | Same shape: `Gid::from_raw(u32::MAX)` doesn't panic in release; `fchown` silently no-ops the group change too |

2 escalating approaches were used (debug-assertions build, then a real release-build end-to-end
`fchown` call through the public API) plus targeted code reading to find the actual observable
consequence (the `translate_fchown_args` sentinel collision) before writing anything down - a
third tool (miri/ASAN) was not needed, since this is a plain logic bug with a 100%-deterministic,
directly-observable symptom (a wrong `Ok(())` + an unchanged file owner) in a normal release
build; no instrumentation is required to see it.

Also checked (code reading only, not independently re-verified under miri/ASAN this session):
`cvt_msg`'s `SCM_CREDENTIALS` arm and both backends' `socket_peercred` construct `UCred`'s
`uid`/`gid` fields via `read_unaligned`/`getsockopt` directly into the struct, never calling
`Uid::from_raw`/`Gid::from_raw` at all - so even the (pre-fix) debug-only check never ran for data
arriving through those paths. Confirmed via `git log -S`/reading `src/backend/*/net/sockopt.rs`
and `src/net/send_recv/msg.rs:572`, not via an instrumented run (see "What's not done" below for
why a real two-participant repro of *that* specific path wasn't built this session).

## Status

**Confirmed**

Reproduces deterministically in a release build, real public API, no instrumentation needed: 3/3
runs, identical output and exit code every time, for both the `Uid` and the `Gid` field (two
inputs in the same bug class, CONFIRMATION.md section 7). Does not reproduce as "silently wrong"
in a debug build - `debug_assert_ne!` already panics there, same observable shape as the fix
(CONFIRMATION.md section 2: the bug needing a specific build to manifest doesn't make it less
real - release is rustix's own documented default/recommended build for production use).

**Fully reproducible**: **Yes**.

No second participant is faked (or needed) here - CONFIRMATION.md section 11 doesn't apply to this
specific confirmed consequence. This is a single process calling rustix's own real, public API
(`rustix::fs::Uid::from_raw`, `rustix::fs::fchown`) with an input value its own documentation says
must never occur, and observing that `fchown` silently does the wrong thing as a direct,
deterministic result - the same category as calling `slice::get_unchecked` out of bounds, not a
claim about what a network peer or the kernel would send. The original suspicion (a real peer/
kernel delivering `uid`/`gid == u32::MAX` through `SCM_CREDENTIALS`/`SO_PEERCRED`) was investigated
by code reading only and is *not* what's being reported as the confirmed, reproduced bug - see
"What's not done" below for why that narrower, second-participant claim was left uninvestigated at
the instrumented-repro level this session.

## Class

logic

## Root cause

`Uid`/`Gid` are repr-transparent wrappers over a plain, niche-free `u32` - so there is no type-
system mechanism forcing the "not `-1`" precondition from `Uid::from_raw`'s own doc comment to
hold; the only runtime check is `debug_assert_ne!(raw, !0)`, which `cfg!(debug_assertions)`
compiles away entirely in a release build (rustix's own recommended build config). Separately,
and independently of whether `from_raw`'s check runs at all, `src/ugid.rs::translate_fchown_args`
(used by every `chown`/`fchown`/`chownat`/`lchown` backend implementation) deliberately uses the
bit pattern `!0` (`u32::MAX`) as its own internal sentinel for "`None` was passed, don't change
this id" - mirroring POSIX `chown(2)`'s own `uid_t/gid_t == -1` "don't change" convention. Because
`Uid`/`Gid` can silently hold exactly that sentinel value (precondition violated, no panic in
release), `translate_fchown_args` cannot tell "caller explicitly passed `Some(Uid(u32::MAX))`"
apart from "caller passed `None`" - both produce the identical raw `(-1, -1)` syscall arguments.
The practical, observable consequence: `fchown`'s real kernel-side permission check (does this
process actually have the privilege to chown to that specific id) is never reached for that one
value, and the call returns `Ok(())` having silently changed nothing, instead of either performing
the requested change or returning the `EPERM` a privilege check against a real, different uid
would produce (confirmed directly: the baseline `fchown(fd, Some(Uid::from_raw(99_999)), None)` in
the same unprivileged process correctly returns `EPERM`).

## Introduced by

721bedc7 "Make `Uid::from_raw` a safe function. (#1294)" (2025-01-31) - this commit changed
`from_raw` from `pub const unsafe fn` (with a `# Safety` doc section putting the "not -1"
precondition entirely on the caller, who had to write `unsafe` to violate it) to today's safe
`pub fn from_raw` guarded only by `debug_assert_ne!(raw, !0)`. The commit's own message states the
reasoning explicitly: "These types don't lead anything to invoke undefined behavior, so their
constructors don't need to be unsafe" - true for memory safety, but this change is what first made
it possible to construct a contract-violating `Uid`/`Gid` from ordinary safe code with zero
indication of danger, in a release build, with no runtime check at all. `translate_fchown_args`'s
own `!0`-as-sentinel convention predates this by years (already present in f9a321ef, one of the
earliest commits touching this file) and was not itself changed - 721bedc7 only removed the
`unsafe` barrier that used to stand between ordinary safe code and triggering the sentinel
collision. (`git log -S "debug_assert_ne!(raw, !0)"` / `git log --follow -- src/ugid.rs` used to
locate this; not bisected, the `-S` search pinned it directly.)

## The fix

Changed both `Uid::from_raw` and `Gid::from_raw` in `src/ugid.rs` from `debug_assert_ne!(raw, !0)`
to `assert_ne!(raw, !0, "...: raw must not be -1")` - the same invariant, now enforced
unconditionally (debug and release alike), matching the function's own documented precondition
literally instead of only in debug builds. This is a minimal, 2-line, 4-total-line-changed fix
that closes the specific silent-bypass consequence found this session: a `Uid`/`Gid` holding
`u32::MAX` (and therefore the `translate_fchown_args` sentinel collision) can no longer be
constructed through the public `from_raw` constructor in any build. `from_raw_unchecked` (already
named to signal "no check happens here", used internally and documented the same precondition) is
intentionally left untouched - this fix targets the *checked*, safe constructor only.

This is a deliberate behavior change worth flagging explicitly for the reader: any existing caller
that was (knowingly or not) relying on `Uid::from_raw(u32::MAX)` silently succeeding in a release
build will now get a hard panic instead. Given the function's own doc comment already states this
value must never be passed, and the demonstrated consequence of allowing it is a silent,
unannounced no-op on a real `fchown`/`chown` call (worse than a panic, in the sense that a panic at
least surfaces the contract violation instead of silently doing nothing), this is considered a
net-safer default. A non-panicking alternative (returning `Option<Uid>`/`Result` from `from_raw`)
would avoid introducing a new panic path but is a breaking API change out of scope for a bounded
fix; `assert_ne!` was chosen as the smallest change that actually closes the hole, consistent with
this project's own prior use of hard `assert!`/unconditional checks for similar reachable-from-
safe-code invariant violations (see candidates 042/047/048 in this same workspace).

Verified against two inputs in the same bug class (CONFIRMATION.md section 7): `Uid::from_raw(u32::MAX)`
via `fchown`'s owner argument, and `Gid::from_raw(u32::MAX)` via `fchown`'s group argument - both
now panic instead of silently no-op'ing, confirmed by re-running against the patched tree.

Before -> after, same `repro/fchown_bypass`, release build:
- Before: `BUG: fchown silently returned Ok(()) and left the owner completely unchanged ...`
  (exit 1)
- After: `FIXED: Uid::from_raw(u32::MAX) was rejected (panicked) before ever reaching fchown ...`
  (exit 0)

- Project test suite (`cargo test --workspace --features "std,all-apis,use-libc" --no-fail-fast`):
  **411 passed, 1 failed, identical before and after** the patch. The one failure
  (`ioctl::test_int_setter`) is pre-existing and unrelated - `/dev/net/tun` doesn't exist in this
  sandbox, already documented in this workspace's `README.md` and in candidate 063's RESULT.md. No
  new failures introduced by the fix.

## What's not done / open questions

- The originally-suspected, narrower claim - that a real second participant (the kernel, mediating
  a genuine `SO_PEERCRED`/`SCM_CREDENTIALS` exchange) can actually *deliver* `uid`/`gid == u32::MAX`
  at all - was investigated by code reading only, not by an instrumented end-to-end repro, and the
  reading suggests it is very hard to trigger for real: Linux's kernel-internal uid/gid types
  (`kuid_t`/`kgid_t`) reserve exactly the bit pattern `-1` globally as `INVALID_UID`/`INVALID_GID`
  and the kernel does not assign it to any real task's credentials; the unmapped-uid-in-a-foreign-
  namespace substitution that 063 investigated for `pid == 0` uses the "overflow uid/gid"
  (`/proc/sys/kernel/overflowuid`/`overflowgid`, default 65534), which the kernel's own historical
  16-bit-uid-compatibility range caps well below `u32::MAX`. This is *not* the same claim as what
  was actually confirmed above (which needs no peer/kernel cooperation at all - any caller can
  trigger it directly) - it's flagged here only because it was the original hypothesis and is worth
  a maintainer's sanity check if ever revisited, not because it's believed reachable in practice.
- `cvt_msg`'s `SCM_CREDENTIALS` arm and both `socket_peercred` backends still construct `UCred`'s
  `uid`/`gid` fields via raw memory (`read_unaligned`/`getsockopt`) without ever calling
  `Uid::from_raw`/`Gid::from_raw` - so even with this session's fix applied, a `UCred` arriving
  through those paths bypasses the (now-unconditional) `assert_ne!` entirely, same as it bypassed
  the old `debug_assert_ne!`. Per the point above this isn't believed practically reachable for
  `uid`/`gid` specifically, but it's a latent inconsistency (these are the only three places in the
  crate that construct a `Uid`/`Gid` from truly external data without going through `from_raw`) -
  not fixed or further characterized this session, flagged as a new suspect below.
- `translate_fchown_args`'s `!0`-as-sentinel design itself (shared with `chown(2)`'s own
  convention) was not changed - a maintainer could alternatively choose to make `fchown`/`chown`
  reject an owner/group `Uid`/`Gid` whose raw value is exactly `!0` at the `translate_fchown_args`
  call site instead of (or in addition to) tightening `from_raw` itself; not pursued here since the
  `from_raw`-level fix is smaller and closes the only reachable path found this session.
