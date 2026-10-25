# readlinkat_set_len_debug_assert_oob

`_readlinkat` in src/fs/at.rs:116-145 gates an unconditional `unsafe { buffer.set_len(nread) }`
behind `debug_assert!(nread <= buffer.capacity())` (compiled out in release) instead of a real
`assert!` like its sibling `_readlink` in src/fs/abs.rs:130 uses. `nread` comes straight from the
raw `readlinkat` syscall's return value. Candidate 013 already reasoned, from the generic VFS
`readlink_copy` helper alone, that a conforming kernel clamps the returned length to the caller's
buffer size - but flagged non-standard filesystems (FUSE, overlayfs, /proc magic links) as an
unchecked gap, since not every filesystem necessarily routes through that one generic helper.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive kernel-source audit: shallow+sparse clone of `torvalds/linux` (`fs/` only), `grep -rn "\.readlink[[:space:]]*=" fs/` to enumerate **every** `inode_operations::readlink` assignment in the whole tree (not just reasoning from the one generic path) | `git clone --filter=blob:none --no-checkout --depth=1`, `git sparse-checkout set fs`, current `torvalds/linux` master | Exactly 6 non-generic implementations exist in the entire tree: `bad_inode_readlink` (dead/bad inode, returns an error, never copies data), `proc_pid_readlink` (procfs magic links: `/proc/*/exe`, `/proc/*/cwd`, `/proc/*/root`, `/proc/*/fd/N`, `/proc/*/map_files/*`), `proc_ns_readlink` (`/proc/*/ns/*`), and `afs_readlink` (×2 call sites). Read each: `proc_pid_readlink` -> `do_proc_readlink` does `if (len > buflen) len = buflen;` before `copy_to_user`; `proc_ns_readlink` calls the generic clamped `readlink_copy` helper directly; `afs_readlink` does `len = umin(strlen(content), buflen);` before `copy_to_user`. Every filesystem that does *not* appear in this list (ext4, tmpfs, overlayfs, FUSE, NFS, 9p, cifs, etc.) has no `i_op->readlink` at all and goes through the fully generic `vfs_readlink` -> `readlink_copy` path in fs/namei.c, which does `if (unlikely(copylen > (unsigned) buflen)) copylen = buflen;` and returns that clamped `copylen`, never the unclamped `linklen`. Also checked FUSE specifically (`fs/fuse/dir.c`): `fuse_readlink_folio` bounds the userspace daemon's reply to `desc.length = folio_size(folio) - 1` and additionally has `WARN_ON(res >= PAGE_SIZE)`, and then still flows through the same generic clamped `get_link`/`readlink_copy` path - a malicious/buggy FUSE daemon cannot make the final returned length exceed the caller's `buflen`. Overlayfs defines no `.readlink` at all (delegates via `get_link` to the real underlying dentry), so it also goes through the generic clamp. |
| Real end-to-end execution of the actual `rustix::fs::readlink`/`readlinkat` public API (the real code at src/fs/at.rs:116-145, debug build so `debug_assert!` is active) against real symlinks on the filesystems actually available in this sandboxed container | `cargo run --example readlinkat_boundary_check` (debug build; container's root filesystem is itself `overlay`, `/tmp` is on that same overlay) - exercised: (a) a 4000-byte symlink target (forces the capacity-growth retry loop through multiple iterations), (b) the real procfs magic links `/proc/self/exe`, `/proc/self/cwd`, `/proc/self/root`, `/proc/self/fd/N` (the exact `proc_pid_readlink`/custom-`i_op` family flagged as the unchecked gap), (c) boundary-value reuse-buffer capacities of 0, 1, and 2 bytes against a real target | All checks passed, 0 debug_assert fires, 0 corrupted data. Real captured output: `debug_assertions enabled: true` / `OK: long symlink target (4000 bytes) round-tripped exactly` / `OK: /proc/self/exe -> ".../readlinkat_boundary_check" (102 bytes)` / `OK: /proc/self/cwd -> "..." (54 bytes)` / `OK: /proc/self/root -> "/" (1 bytes)` / `OK: /proc/self/fd/3 -> "/dev/null"` / `OK: boundary-value reuse-buffer capacities (0,1,2) all round-tripped correctly` / `ALL CHECKS PASSED`. |
| Attempt a 3rd, more adversarial filesystem (a custom FUSE server returning deliberately oversized `FUSE_READLINK` replies, to probe the one layer - `fs/fuse/dir.c` - where an external, untrusted participant supplies the raw length) | Checked container capabilities first: `ls /dev/fuse` -> no such device; `capsh --print` -> `cap_sys_admin` not in the bounding set; `unshare --mount --user --map-root-user` -> `Operation not permitted` | Not executable in this environment - no FUSE device node and no permission to create a user/mount namespace. Not counted as a negative signal (it's an environment limitation, not evidence either way) - substituted with the kernel-source read of `fuse_readlink_folio` in the first row instead, which shows the reply is bounded to `folio_size(folio) - 1` and still routed through the generic clamped path regardless of what the daemon sends. |

## Status

**Not reproducible.** Covered: (1) every `inode_operations::readlink` implementation in the current
Linux kernel source tree, found via an unbiased, exhaustive `grep` (not a sample of a few
filesystems) - all six clamp to the caller's `buflen` before copying, and every filesystem without
a custom `.readlink` goes through the one generic clamped path; (2) a real, debug-build run of the
actual `rustix::fs::readlink`/`readlinkat` public API against real symlinks on every filesystem
genuinely reachable in this container (the container's own overlayfs root/`/tmp`, and procfs magic
links specifically - the exact gap candidate 013 flagged as unchecked); (3) FUSE's own reply-length
handling read at the source level, since a live FUSE mount wasn't buildable in this sandbox
(no `/dev/fuse`, no `CAP_SYS_ADMIN`, `unshare` denied).

**Fully reproducible**: N/A (Not reproducible, no repro/ folder - see "the one rule that matters
most" in SKILL.md, a repro is only mandatory when Status is Confirmed).

## Class

N/A - no bug confirmed.

## Root cause

N/A - no bug found. The suspected hazard (a filesystem reporting a `readlinkat` length larger than
the caller-supplied buffer, bypassing the generic VFS clamp) does not exist anywhere in the current
Linux kernel: every reachable code path - the generic `vfs_readlink`/`readlink_copy` path used by
the overwhelming majority of filesystems (including overlayfs and FUSE), and every one of the small
number of filesystems with a custom `inode_operations::readlink` (procfs's magic links, AFS,
`bad_inode`) - independently clamps its returned length to `buflen` before returning. `nread` from
the raw `readlinkat` syscall can therefore never exceed `buffer.capacity()` in `_readlinkat`'s retry
loop (the capacity passed as `buf.len()` into the syscall), so the `debug_assert!` this candidate
was built around genuinely can never fire on Linux - it is intentionally redundant defense-in-depth
for a precondition the kernel's own contract already guarantees, not a live gap. The sibling
`_readlink` in src/fs/abs.rs:130 using a real `assert!` for the same check is a stylistic
inconsistency (one function pays a tiny branch in release builds the other doesn't), not evidence
that one of them is actually exposed to a real violation the other isn't - neither path currently
has a way to see a kernel that violates this invariant.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix. (The `debug_assert!`/`assert!` inconsistency between
`_readlinkat` and `_readlink` noted above is a cosmetic harmonization opportunity, not a fix for a
real defect - not applied, since there is nothing it would actually prevent given the kernel
contract confirmed above; flagged here only for the record.)

- Project test suite: N/A, no fix was made (Not reproducible candidate, no patch to verify against
  the test suite).

## What's not done / open questions

- Non-Linux targets this crate supports (the various BSDs, macOS/Apple, Solaris/illumos via the
  `libc` backend) were not audited at the kernel-source level the way Linux was here - the fix for
  candidate 013 and this candidate both reasoned from Linux kernel source specifically. A different
  BSD/Darwin VFS could in principle have a filesystem that doesn't clamp; not checked this session
  (would need each OS's own kernel source, not just Linux's).
- A live FUSE mount returning a deliberately oversized reply was the strongest remaining lead and
  could not be executed in this sandboxed container (no `/dev/fuse`, no `CAP_SYS_ADMIN`/user
  namespaces). The source-level read of `fuse_readlink_folio` substitutes for it here, but a future
  session with a less restricted environment (real root, or a VM) could still build the literal
  end-to-end FUSE repro to close this out with a live run rather than a source read.
- This closes the specific lead candidate 049 was opened to check (non-standard filesystems
  bypassing the VFS clamp on Linux). It does not change the standing, harmless stylistic
  inconsistency between `debug_assert!` (src/fs/at.rs:127) and `assert!` (src/fs/abs.rs:130) for the
  same logical check - a maintainer may still want to harmonize the two for consistency's sake, but
  that is a style choice, not a bug fix, given the finding above.
