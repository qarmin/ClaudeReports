# tcp_congestion_zero_len_name_assert_panic

`src/backend/{libc,linux_raw}/net/sockopt.rs`'s `tcp_congestion()` (the safe backing for the
public `rustix::net::sockopt::tcp_congestion()`) reads back `getsockopt(IPPROTO_TCP,
TCP_CONGESTION)` into a 16-byte (`TCP_CA_NAME_MAX`) buffer, then does
`assert!(slice.contains(&b'\0'))` before handing the buffer to `CStr::from_ptr`. The candidate
claimed a congestion-control algorithm whose kernel-side `name` field exactly fills all 16 bytes
with no NUL terminator would abort the process via this safe API.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of rustix's own two backend copies (libc + linux_raw) | n/a | Both backends have byte-identical logic: `assert!(slice.contains(&b'\0'))` then `CStr::from_ptr(slice.as_ptr().cast())` - confirmed the assert is reachable whenever the kernel hands back exactly 16 bytes with no `\0` anywhere in them |
| Reading the actual upstream Linux kernel source (`net/ipv4/tcp.c` `do_tcp_getsockopt`, `net/ipv4/tcp_cong.c` `tcp_register_congestion_control`), fetched live from `raw.githubusercontent.com/torvalds/linux/master` | n/a | Confirmed the kernel's own `TCP_CONGESTION` getsockopt handler does `char ca_name[TCP_CA_NAME_MAX] = {}; len = min_t(unsigned int, len, TCP_CA_NAME_MAX); memcpy(ca_name, icsk->icsk_ca_ops->name, sizeof(ca_name));` - i.e. it copies the algorithm's raw `name` field verbatim with no NUL-termination guarantee. Confirmed `tcp_register_congestion_control()`'s own validation (`tcp_validate_congestion_control`) checks only required function pointers, never that `name` is NUL-terminated within its 16-byte array - so an out-of-tree module compiled with e.g. `.name = "0123456789abcdef"` (a C string-literal initializer exactly as long as the destination array is legal C and silently drops the implicit terminating NUL) registers successfully and would trigger this |
| Attempted to build/load a real out-of-tree congestion-control kernel module in this sandbox, to get a fully genuine kernel-produced 16-byte/no-NUL response | `id`, `capsh --print`, `sudo -n true`, `ls /lib/modules/$(uname -r)`, `sysctl net.ipv4.tcp_available_congestion_control` | Confirmed infeasible in this environment: non-root uid, `cap_sys_module` explicitly stripped from both the bounding set and the IAB set, `sudo` refuses outright because `no_new_privs` is set, and there is no `/lib/modules/<running-kernel>` tree to build a module against at all. Only `reno`/`cubic` are currently loaded; no available algorithm's name is 16 bytes long anyway |
| Standalone repro: real TCP socket, real libc `getsockopt(2)` call, through rustix's real, unmodified, public `rustix::net::sockopt::tcp_congestion()` API (libc backend, via the `use-libc` Cargo feature so the LD_PRELOAD shim below actually intercepts it - the default linux_raw backend makes the raw syscall directly and would bypass any libc-level interposition) | `repro/` crate, release build, repeated 5x | Baseline (no shim, real kernel, real `cubic` algorithm): `Ok("cubic")`, no panic. With an LD_PRELOAD shim that hands back exactly 16 bytes with no NUL (the byte-for-byte shape a real out-of-tree module with a full-length name would produce, per the kernel source read above) in place of the real kernel's getsockopt response: panicked 5/5 runs, deterministically, with `assertion failed: slice.contains(&b'\0')` at the exact line identified from code reading |
| Fix: replace the assert + `CStr::from_ptr` with an explicit NUL-scan with a length fallback, re-run the same repro + a second boundary case (a 15-byte name + a real trailing NUL, to check the fix handles both shapes, not just the one that first triggered it) against the patched tree | debug/release, both backends (`use-libc` and default linux_raw) | Patched tree: baseline unchanged (`Ok("cubic")`); shim input now returns `Ok("supercongestion1")` instead of panicking; second boundary case (`"fifteenchars123"`, 15 chars + NUL) returns `Ok("fifteenchars123")` correctly - no regression on the normal NUL-terminated shape |

## Status

**Confirmed**. Reproduces 5/5 runs (release build, libc backend, deterministic - no flakiness,
since the fake 16-byte input is fixed and does not depend on memory layout/ASLR/timing).

**Fully reproducible**: **No**, with justification. The underlying external input this bug is
about - a real out-of-tree TCP congestion-control kernel module whose `name` field fills all 16
`TCP_CA_NAME_MAX` bytes with no trailing NUL - is kernel state that requires `CAP_SYS_MODULE`/root
to register (build and `insmod`/`modprobe` a real `.ko`). This sandboxed environment has no root,
no `CAP_SYS_MODULE` (verified: stripped from both the capability bounding set and the IAB set via
`capsh --print`), `sudo` is blocked outright by `no_new_privs`, and there is no
`/lib/modules/<running-kernel>` tree present to build a module against at all - this is the kind
of "needs privileged access genuinely unavailable in this environment" exception
`CONFIRMATION.md` section 11 describes, not a shortcut taken for convenience. Rather than skip the
repro, or fake the whole code path, the repro runs the real, unmodified, public
`rustix::net::sockopt::tcp_congestion()` against a real socket through a real `getsockopt(2)`
libc call - the one piece that's stood in for (via an LD_PRELOAD shim intercepting only
`getsockopt(IPPROTO_TCP, TCP_CONGESTION)`, every other call passed straight through to the real
libc/kernel) is the kernel's own response shape, built to be byte-for-byte what the upstream
kernel source (read directly, see above) shows a real such module's registration would produce.

## Class

crash

## Root cause

Both backends' `tcp_congestion()` request a fixed 16-byte (`TCP_CA_NAME_MAX`) buffer from
`getsockopt(IPPROTO_TCP, TCP_CONGESTION)`. The kernel's own handler
(`net/ipv4/tcp.c::do_tcp_getsockopt`) copies the current congestion-control algorithm's `name`
field into that buffer verbatim via `memcpy`, with no NUL-termination guarantee - a `name` that
fills the full 16-byte array has no trailing NUL at all. Kernel-side registration
(`tcp_register_congestion_control`/`tcp_validate_congestion_control` in `net/ipv4/tcp_cong.c`)
never validates that `name` is NUL-terminated within its array; a C string-literal initializer
exactly as long as the destination array (`char name[16] = "0123456789abcdef"`) is legal C and
silently drops the implicit terminator, so an out-of-tree module can compile and register this
way without even a compiler error. rustix's own code then does `assert!(slice.contains(&b'\0'))`
and, if that passed, would call `CStr::from_ptr` - which would itself scan past the 16-byte buffer
into adjacent stack memory looking for a NUL that isn't there, an out-of-bounds read. The assert
exists specifically to turn that worse bug into a controlled panic instead - but a controlled
panic is still a process abort reachable through the fully safe `rustix::net::sockopt::tcp_congestion()`
API, given only an unusual-but-legal kernel/module configuration, not any unsafe caller error.

## Introduced by

a74f97f0 "Implement more socket options. (#858)" (2023-09-29) - this commit introduced
`get_tcp_congestion`/`tcp_congestion` (later renamed) in both backends with this exact
`assert!(slice.iter().any(|b| *b == b'\0'))` (later simplified to `slice.contains(&b'\0')` in a
subsequent clippy-fixes commit, f01d1c63, with no change in behavior).

## The fix

Replaced the `assert!` + `CStr::from_ptr` pair with an explicit NUL search over the known-length
`slice`, falling back to the full slice length when no NUL is present (exactly the case a
full-length `name` produces):

```rust
let len = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
Ok(core::str::from_utf8(&slice[..len]).unwrap().to_owned())
```

This also removes the now-dead `CStr` import (unused elsewhere in either file) in both backends.
Applied identically to `src/backend/libc/net/sockopt.rs` and
`src/backend/linux_raw/net/sockopt.rs` (byte-identical logic in both).

Verified:
- Re-ran the exact repro against the patched tree: no panic, returns `Ok("supercongestion1")`
  (the full 16-byte fake name) instead of aborting.
- Checked a second boundary case in the same bug class (CONFIRMATION.md section 7): a
  15-byte name with a real trailing NUL (`"fifteenchars123"`) still decodes correctly
  (`Ok("fifteenchars123")`) - the fix doesn't just special-case the one failing input.
- Both backends (`use-libc` feature and the default linux_raw backend) compile clean with no new
  warnings.
- Project test suite, same feature set before and after the patch:
  - libc backend (`--features net,alloc,fs,use-libc --lib`): 32 passed; 0 failed, both before and
    after.
  - linux_raw backend (`--features net,alloc,fs,stdio --lib`): 30 passed; 0 failed, both before
    and after (30 vs 32 is the feature-gated test set differing between backends, not a
    regression - confirmed by running the identical feature flags against the unpatched tree too).

## What's not done / open questions

- The same 16-byte-fixed-size/no-NUL-guarantee shape exists for other fixed-size string sockopts
  in this file if any share the pattern (e.g. `TCP_ULP`/`TCP_ULP_NAME_MAX`) - not checked this
  session; worth a follow-up sweep since the kernel's own `do_tcp_getsockopt` handles `TCP_ULP`
  with the same `memcpy`-verbatim shape (seen in the same kernel source read, not sockopt.rs
  itself - rustix doesn't currently wrap `TCP_ULP`, so nothing to fix here, just a note for if it
  is added later).
- No genuine end-to-end repro with a real loaded out-of-tree kernel module was possible in this
  sandboxed environment (see "Fully reproducible" above) - a session with root/`CAP_SYS_MODULE`
  access could tighten this to a fully genuine repro by building and loading a minimal
  out-of-tree `.ko` with a 16-byte no-NUL `name`.
