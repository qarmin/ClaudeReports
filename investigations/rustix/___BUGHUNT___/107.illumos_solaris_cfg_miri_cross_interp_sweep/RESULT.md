# illumos_solaris_cfg_miri_cross_interp_sweep

Dedicated, crate-wide sweep of every `solarish` (`target_os = "illumos"` or `"solaris"`)-gated code
path, going beyond the two narrow spots candidates 033 (termios baud-rate decode table) and 090
(`event::port::get`'s `portev_pad`) already covered - per `projects/rustix.md`'s own risk map, which
explicitly calls "Solaris/illumos beyond what 033/057 already covered" unreached (057 turned out to
be FreeBSD's `pdeathsig`, not illumos/Solaris, so the real count of dedicated illumos/Solaris
candidates going into this session was 2, not 3).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: enumerated every file in `src/` containing `solarish` (28 files, ~250 individual `#[cfg(...)]` sites - `c.rs`, `event/syscalls.rs`, `fs/{dir,makedev,syscalls,types}.rs`, `io/errno.rs`, `mm/types.rs`, `net/{ext,msghdr,read_sockaddr,send_recv,sockopt}.rs`, `pipe/types.rs`, `process/types.rs`, `clockid.rs`, `pipe.rs`, `signal.rs`, `event/{epoll,mod}.rs`, `fs/{abs,constants,fd,seek_from}.rs`, `ioctl/mod.rs`, `net/{sockopt,types}.rs`, `termios/types.rs`), cross-checked against what 033/090 already cover, prioritizing `unsafe`/`assume_init`/`transmute`/raw-pointer sites over plain constant-mapping sites (the vast majority of hits are just `Self(c::SOME_CONSTANT as _)` association-list entries, inert) | reading only, no build | Identified 4 non-trivial suspects worth digging into (below); the remaining ~240 hits are constant/flag-mapping association lists with no unsafe-memory shape, not worth individual investigation |
| Suspect A - `src/backend/libc/net/read_sockaddr.rs::read_sockaddr_unix`'s solarish/freebsd-only branch (`cfg!(any(solarish, target_os = "freebsd"))`): scans the *entire* fixed 108-byte `sun_path` array for a NUL byte, ignoring `len`, because "FreeBSD and illumos sometimes set the length to longer than the length of the NUL-terminated string" (existing code comment) - worried this could read past what the kernel actually wrote into the (`MaybeUninit`-backed) receive buffer | (1) A generic Miri semantics check (not solarish-specific) confirming the shape of the risk: creating a `&T` reference over a struct with an uninitialized array tail is **not** itself UB in Miri; UB only occurs if a scan actually reaches an uninitialized byte (i.e., no NUL in the initialized prefix) - see output.txt. (2) Read illumos-gate's real `sotpi_getsockname`/`cstat`-equivalent address path (`usr/src/uts/common/fs/sockfs/socktpi.c::sotpi_getsockname`, archived in `investigation/socktpi.c`): for `AF_UNIX`, `sti->sti_laddr_sa` is populated verbatim from whatever the *original binding caller* passed to `bind()` (full `namelen` bytes, untouched by the kernel beyond that), and `bcopy(sti->sti_laddr_sa, name, MIN(*namelen, sti->sti_laddr_len))` + `*namelen = sti->sti_laddr_len` means the kernel only ever copies back exactly as many bytes as it originally stored - our own receive buffer's `len` (and therefore the array range the NUL-scan ever touches) always exactly matches what was copied. No path reaches the buffer's genuinely-uninitialized tail | **Ruled out** - not reproducible. The only way to trigger a real NUL-scan-into-uninitialized-memory would require a peer that bound an `AF_UNIX` address whose user-supplied bytes (beyond the string's own NUL) contain no zero byte anywhere up to `namelen`, which the kernel-source reading shows gets echoed back unchanged, not newly-uninitialized by the receiving side. See `investigation/socktpi.c` (archived verbatim from illumos-gate `master`) |
| Suspect B - `src/backend/libc/fs/syscalls.rs::fstat`'s `MaybeUninit::<Stat>::uninit()` -> `assume_init()` (`Stat = c::stat` verbatim on solarish, which has a `st_fstype: [c_char; _ST_FSTYPSZ]` trailing field - same general shape as candidate 039's confirmed Apple `st_lspare`/`st_qspare` bug) - cross-interpreted with Miri for `--target x86_64-unknown-illumos`, real public `rustix::fs::fstat` API, real `std::fs::File` fd (`investigation/rustix_fstat_miri`) | `cargo +nightly miri run --target x86_64-unknown-illumos`, `MIRIFLAGS=-Zmiri-disable-isolation` (required for real file I/O) | **Initially looked like a second confirmed bug**: reproducible UB, "constructing invalid value of type `libc::unix::solarish::stat`: at `.st_fstype[113]`, encountered uninitialized memory" - see output.txt. Escalated with a third approach below before trusting this |
| Suspect B, continued - root-cause probe: does Miri's illumos shim genuinely model "the real illumos kernel leaves `st_fstype` uninitialized", or is this an artifact of `-Zmiri-disable-isolation` forwarding the call straight to the **host's real (Linux) `fstat()`** while the process is merely *compiled* for illumos? (a) Fetched illumos-gate's real `usr/src/uts/common/syscall/stat.c` (`investigation/socksubr.c` naming aside - actually `stat.c`'s content is quoted inline below, see "Root cause"); every `cstat`/`cstat64`/`cstat64_32` variant does `bzero(&sb, sizeof(sb))` (or `lsb`) **before** filling any field, i.e. the real illumos kernel always fully zero-initializes the *entire* `stat`/`stat64` struct, `st_fstype` included, before `copyout`. (b) Wrote a raw-libc byte-probe (`investigation/byte_probe`): poison a same-sized/same-layout `MaybeUninit<libc::stat>` with a `0xAB` sentinel, call the raw `libc::fstat()` (same call rustix makes internally) under the identical Miri cross-interp config, and measure exactly how many bytes the real underlying syscall actually overwrote | Same Miri/target/flags as above | **(a) illumos kernel source directly contradicts the Miri signal**: `bzero(&sb, sizeof(sb))` unconditionally covers the whole struct, so `st_fstype` can never be uninitialized on real hardware. **(b) Confirms the mechanism**: `libc::stat` is 128 bytes (illumos layout), `st_fstype` lives at offset 112-127, and the real underlying syscall performed by Miri's passthrough (the **host's** real Linux `fstat()`, which has a completely different, smaller/differently-shaped struct with no `st_fstype` field at all) only actually wrote bytes up to offset 112 - everything from 113 onward was left exactly as the `0xAB` sentinel, never touched. This is a `-Zmiri-disable-isolation` cross-OS-passthrough artifact (the host's real syscall fills a *host-shaped* buffer while Miri/the type system believes it's illumos-shaped), not a reflection of what the real illumos kernel does - see output.txt |
| Suspect C - `src/event/epoll.rs::Event`'s `#[cfg_attr(all(solarish, any(target_arch = "x86", target_arch = "x86_64")), repr(packed(4)))]` (illumos exposes a real `epoll` compat layer, unlike Solaris proper - `rustix::event::epoll` is gated on `target_os = "illumos"` specifically) - worried the packed layout might not match illumos's real ABI, since CI only `cargo check`s illumos (no execution, confirmed via `.github/workflows/main.yml:140`) | Fetched illumos-gate's real `usr/src/uts/common/sys/epoll.h` (`investigation/epoll_illumos.h`), byte-for-byte comparison | **Ruled out** - illumos's own header does `#pragma pack(4)` around `struct epoll_event` under the exact same condition (`_LONG_LONG_ALIGNMENT == 8 && _LONG_LONG_ALIGNMENT_32 == 4`) rustix is modeling with `repr(packed(4))`. Git history (`6a6f7a8d "Fix tests on illumos" (#1238)`) shows this exact line was deliberately added/tuned, matching the real header precisely - no discrepancy found |
| Suspect D - `src/backend/libc/event/syscalls.rs::port_getn` (the plural sibling of candidate 090's `port_get`): `Buf::assume_init(nevents)` on a caller-supplied buffer of `c::port_event`, same `portev_pad` field, same root cause as 090 | Code reading only (not independently re-verified with its own Miri run this session) | Same underlying bug as 090, not a new/distinct finding - noted as a sibling call site sharing 090's already-confirmed root cause, not logged as its own candidate (see "What's not done" below) |

## Status

**Not reproducible.** Four suspects were investigated to a real verdict (not abandoned mid-way):
Suspect A (sockaddr_un NUL-scan) and Suspect C (`epoll` packed layout) were ruled out by direct,
cited real-kernel-source comparison. Suspect B (`fstat`'s `st_fstype`) produced a real, repeatable
Miri UB signal (reproduces every run, deterministic - same class of evidence candidate 039 trusted
for Apple) **but was independently disproven** by a second, more targeted probe showing the UB is
an artifact of Miri's `-Zmiri-disable-isolation` host-OS passthrough (the real syscall actually
executed is the x86_64 Linux host's own `fstat()`, which doesn't know about illumos's `st_fstype`
field at all) rather than a reflection of real illumos kernel behavior, which the actual
illumos-gate kernel source (`bzero(&sb, sizeof(sb))` in every `cstat*` variant) proves always fully
initializes the struct. Suspect D is the same bug as already-confirmed candidate 090, not a new one.

No genuinely new, confirmable bug was found in solarish-gated code beyond what 033 and 090 already
established, after covering the highest-signal remaining territory (unsafe/uninit-memory-shaped
code: socket address decoding, `stat`, `epoll` layout, event ports).

**Fully reproducible**: N/A (no bug to report).

## Class

N/A - no bug confirmed. See "Root cause" below for why each suspect turned out to be safe (or, for
Suspect B, actively misleading if taken at face value).

## Root cause

Not applicable (no confirmed bug). Per-suspect reasoning for why each is safe:

- **Suspect A**: illumos's own `sotpi_getsockname` (`usr/src/uts/common/fs/sockfs/socktpi.c`)
  echoes back exactly the caller-supplied `bind()` bytes, bounded by its own tracked length - the
  buffer range rustix's NUL-scan ever touches always matches what the kernel actually copied; it
  never reaches genuinely-uninitialized memory.
- **Suspect B**: real illumos kernel (`usr/src/uts/common/syscall/stat.c`, every `cstat`/`cstat64`/
  `cstat64_32` variant) unconditionally `bzero()`s the entire output struct before filling any
  field - `st_fstype` is always defined on real hardware. The Miri signal is a tooling artifact of
  cross-OS syscall passthrough under `-Zmiri-disable-isolation` (verified directly: the host's real
  `fstat()` only wrote 113 of the illumos struct's 128 bytes in the byte-probe, consistent with
  "the host's native Linux `fstat()` was actually called, not an illumos-accurate emulation").
- **Suspect C**: illumos's own `sys/epoll.h` genuinely specifies `#pragma pack(4)` under the same
  condition rustix's `repr(packed(4))` is gated on - byte-for-byte match, and git history shows this
  was deliberately tuned (`6a6f7a8d`), not an oversight.
- **Suspect D**: not independently root-caused this session - it shares candidate 090's own root
  cause verbatim (`port_event_t.portev_pad` never written by illumos's `port_copy_event()`).

## Introduced by

N/A - no bug confirmed, nothing to bisect.

## The fix

N/A - no bug found, nothing to fix. No `patch.diff` in this folder.

- Project test suite: N/A, no code was changed this session (every suspect was ruled out or
  shown to be a duplicate of an already-confirmed candidate; nothing was built against the real
  project tree beyond the read-only investigation probes in `investigation/`, which are standalone
  crates, not modifications to `src/`).

## What's not done / open questions

- **Important tooling caveat for future sessions, also added to `projects/rustix.md`**: Miri
  cross-interpretation with `-Zmiri-disable-isolation` for a non-Linux target (illumos here) can
  silently fall back to calling the **host's real syscall** for calls like `fstat`/`stat`/`statvfs`
  that have a generic libc shim, filling a host-shaped buffer while the Rust type system believes
  it's target-shaped - producing a real, deterministic, repeatable Miri UB report that does **not**
  reflect the real target OS's behavior. This is a different (and more dangerous, since it looks
  identical to a genuine finding) failure mode than the project's already-documented "32-byte
  short-read" Miri quirk. Always cross-check a Miri-illumos/BSD/Apple struct-uninit finding against
  the real kernel source (does the real syscall `bzero`/fully populate the struct before
  `copyout`?) before trusting it - candidate 039's Apple finding remains trustworthy specifically
  because it was independently corroborated by a reporter on real Apple hardware, which Suspect B
  here has no equivalent of.
- `src/backend/libc/fs/syscalls.rs::statvfs` has the structurally identical `MaybeUninit<c::statvfs>`
  -> `assume_init()` pattern, and illumos's own `struct statvfs` also has trailing
  string/reserved-padding fields (`f_basetype`, `f_fstr`, `f_filler[16]`) - not independently
  Miri-tested this session, but expected to hit the exact same host-passthrough artifact as Suspect
  B if tested, not a new bug class; skipped to avoid re-demonstrating the same already-understood
  tooling limitation.
- Suspect D (`port_getn`'s sibling `portev_pad` exposure) was reasoned through but not given its own
  Miri run or standalone repro this session - a future session wanting to extend candidate 090's fix
  to `port_getn` as well (not just `port_get`) could reuse 090's own repro approach directly.
- `src/backend/libc/net/msghdr.rs`'s solarish-specific `msg_control_len`/`msg_iov_len` conversions
  were read but not deeply investigated (plain integer narrowing via `try_into().unwrap_or(MAX)`,
  no uninitialized-memory shape) - lower priority, no `unsafe` construction involved.
- No illumos/Solaris hardware or bootable VM was available in this sandbox for any of the above -
  every conclusion rests on (a) real, independently-fetched illumos-gate kernel source (archived
  verbatim in `investigation/`) and (b) Miri cross-interpretation, consistent with every other
  illumos/Solaris finding in this project (`projects/rustix.md`'s risk map).
