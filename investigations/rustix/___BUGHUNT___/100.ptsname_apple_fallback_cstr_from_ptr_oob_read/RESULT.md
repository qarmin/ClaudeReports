# ptsname_apple_fallback_cstr_from_ptr_oob_read

`src/backend/libc/pty/syscalls.rs:75-94`, the `#[cfg(apple)]` `ptsname` fallback used when the
weakly-linked libc `ptsname_r` symbol is absent (older macOS): it `ioctl`s `TIOCPTYGNAME` into a
fixed `[u8; 128]` stack buffer, then calls `CStr::from_ptr` on it with no explicit verification
that the kernel actually wrote a NUL within those 128 bytes - it trusts the kernel ioctl contract
entirely. Looked suspicious because `CStr::from_ptr` on a non-NUL-terminated buffer reads past its
end looking for one, and this trust is never independently checked in the source. Candidate 023
only checked this function's errno-zero-filtering path and candidate 092 only checked
solarish-specific patterns in the same function; neither touched this specific CStr::from_ptr
angle.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code read of the exact branch (`src/backend/libc/pty/syscalls.rs:75-94`) plus `git log -S "TIOCPTYGNAME"` | - | Confirmed the mechanism matches the candidate description exactly: `CStr::from_ptr(name.as_ptr().cast())` has no bounds/NUL check of its own, relying entirely on the kernel. Introduced in c2699e36 "Add a `rustix::pty` module. (#673)" and unchanged in shape since |
| Cross-compile reachability check: is the branch genuinely live code on the real affected target | `cargo check --target x86_64-apple-darwin --features "std,all-apis,use-libc"` (prebuilt rust-std via `rustup target add`, no SDK/linker needed for `cargo check`) | Compiles cleanly - the `ioctl`/`CStr::from_ptr` fallback is reachable code on `x86_64-apple-darwin`, not dead code; only the runtime `weak!`/`.get()` check (not a `cfg`) decides whether it executes |
| Audit the real upstream XNU kernel source (`apple-oss-distributions/xnu`, branch `main`, fetched via `raw.githubusercontent.com`) implementing every registered `TIOCPTYGNAME` handler, to check whether the candidate's premise ("kernel might not NUL-terminate") is realizable on any currently-known real macOS kernel | `bsd/sys/ttycom.h`, `bsd/kern/tty_dev.c`, `bsd/kern/tty_ptmx.c`, `bsd/kern/tty_pty.c` - see `output.txt` for exact fetched excerpts | `TIOCPTYGNAME` is encoded with size 128 (`_IOC(IOC_OUT, 't', 83, 128)`), matching rustix's own buffer; the shared dispatcher (`tty_dev.c`) calls `pty_get_name(dev, data, 128)` with that same hardcoded 128; and **both** of the only two `name()` callbacks ever registered (the modern `/dev/ptmx` driver's `ptmx_get_name` and the legacy `/dev/pty*` driver's `pty_get_name`) write via `snprintf(buffer, size, ...)`, which the C standard guarantees NUL-terminates within `size` bytes whenever `size > 0` - truncating-but-still-terminating in the worst case, and the actual device-name strings (`/dev/ttyp0` style) are nowhere near 128 bytes regardless. No code path in the current real kernel source can leave the buffer without a NUL |
| Attempted real hardware / emulation to falsify the kernel-source conclusion empirically rather than by reading | checked for qemu-system/Darling/macOS VM images | None available in this environment (same conclusion independently reached by candidates 028 and 035 in this workspace) |

Miri was not attempted as a 4th approach: the vulnerable branch bottoms out in a real `ioctl(2)`
syscall, which this project's own tooling notes (`projects/rustix.md`) already establish Miri
cannot execute in this environment - there was no new signal to chase that would have justified
it, consistent with the soft cap of 3 approaches when the first 3 produce a clean, well-supported
negative.

## Status

**Not reproducible.** No real second participant (a real macOS kernel, real hardware, or a working
emulator) is available in this environment to attempt triggering the hypothesized non-NUL-
terminated kernel response, and - going beyond "couldn't test it" - reading the actual real XNU
kernel source that implements `TIOCPTYGNAME` (section 3 above) shows the hypothesized trigger does
not exist in any currently known code path: every registered driver's `name()` callback writes via
`snprintf` with the matching 128-byte size, which structurally guarantees NUL-termination within
bounds. The candidate's own premise was explicitly hedged ("if that assumption is ever violated -
kernel bug, future ABI change, reused ioctl number on a different BSD-flavored kernel") and this
session could not find, and has concrete evidence against, any real-world way to realize it on the
actual XNU kernel this code targets.

## Root cause

N/A (not reproducible) - but the suspect turned out safe for a specific, checkable reason rather
than merely "couldn't test it": `CStr::from_ptr`'s safety here depends entirely on the kernel's
`TIOCPTYGNAME` ioctl handler always writing a NUL within the declared 128-byte buffer, and the real
XNU implementation of that handler (both the legacy and modern pty drivers, which are the only two
that have ever existed for this ioctl) uses `snprintf(buffer, 128, ...)` to produce the name
string - a call whose own documented contract makes a non-NUL-terminated result in the first 128
bytes impossible, independent of whatever path number is generated. The `CStr::from_ptr` call is
trusting a kernel contract that is, in practice, enforced by the C standard library function the
kernel itself uses to fill the buffer - not an unchecked assumption with no backing.

## Introduced by

Not applicable (Status is Not reproducible, not Confirmed) - for reference, the code was introduced
in c2699e36 "Add a `rustix::pty` module. (#673)" and has not changed shape since, per `git log -S
"TIOCPTYGNAME" -- src/backend/libc/pty/syscalls.rs`.

## The fix

None attempted - no bug was confirmed to fix. A defense-in-depth change is possible in principle
(e.g. using `CStr::from_bytes_until_nul` or an explicit `memchr`-style bounded scan instead of
`CStr::from_ptr`, so a hypothetical future kernel/ABI regression would produce a clean error instead
of an OOB read), but this session found no actual reproducible bug to justify writing and verifying
one - doing so here would be a speculative hardening change, not a confirmed-bug fix, and is out of
scope for this candidate per the skill's own "don't force a patch" guidance.

## What's not done / open questions

- Not independently verified on real macOS hardware - see "Status" above for exactly what was
  checked instead (upstream kernel source read directly, not assumed) and why real execution wasn't
  possible in this environment.
- The same "trust an ioctl/syscall to NUL-terminate or otherwise bound its output, verified only by
  reading the kernel implementation rather than by a source-level check" pattern may exist
  elsewhere in the `libc` backend (e.g. other `TIOCP*`/`TIOC*` ioctls, other `weak!`-gated
  fallbacks) - not swept this session, one-bug-per-candidate scope.
- If a future XNU release ever changes either `name()` callback away from `snprintf` (or a third
  driver is registered with a different, less careful implementation), this conclusion would need
  to be re-checked against the kernel source as of that release - the finding here is dated against
  the `apple-oss-distributions/xnu` `main` branch as fetched on 2026-10-01, not a permanent
  guarantee about all future XNU versions.
