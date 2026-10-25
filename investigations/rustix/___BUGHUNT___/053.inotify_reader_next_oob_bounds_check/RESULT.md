# inotify_reader_next_oob_bounds_check

`inotify::Reader::next` (`src/fs/inotify.rs:176-210`, pre-fix) refills its buffer with a plain
`read()` on whatever `Fd: AsFd` it was constructed with, then unconditionally casts the start of
the buffer to `&inotify_event` and advances by `size_of::<inotify_event>() + event.len` - unlike
the sibling `RawDir`/`Dir` readers, it has no check that `offset + size_of::<inotify_event>() <=
initialized` before the cast, and `file_name()`'s `CStr::from_ptr` has no length bound against the
buffer at all. `Reader<'buf, Fd: AsFd>` is generic over *any* fd, not just one returned by
`inotify::init()`, so this doesn't need a misbehaving kernel to trigger - a real pipe (or any other
non-inotify fd) returning a short read is enough, since only a real `getdents64`-style
fd-type-checked syscall (as `RawDir`/`Dir` use) would make the kernel itself refuse to write partial
data; plain `read()` has no such per-fd-type guarantee.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + comparison with `RawDir`/`Dir` siblings (candidate 044) | `src/fs/inotify.rs` vs. `src/fs/raw_dir.rs`/`src/backend/linux_raw/fs/dir.rs` | Confirmed the asymmetry is real; also confirmed `RawDir`'s own identical-shaped missing check (044) was ruled "Not reproducible" specifically because `getdents64` is fd-type-checked by the kernel (refuses non-directory fds outright) - `inotify::Reader` uses plain `read()` instead, which has no such per-fd-type guarantee, so that ruling-out does not transfer here |
| Approach 1: real second-participant repro - a real pipe (`rustix::pipe::pipe`), 4-byte short write, real `read()` via the unmodified public `Reader::next()` API, debug build | `cargo build` + run, 3x | **Fired**: `Ok(Event{..})` built from uninitialized stack memory every run; `cookie`/`events`/`wd` are garbage, `file_name()` leaks raw stack-pointer bytes through an unbounded `CStr::from_ptr` (e.g. `\xfc\x7f` bytes - real stack address fragments), varying every run (proving genuinely uninitialized, not a fixed wrong decode) |
| Approach 2: Miri, same scenario (`cargo +nightly miri run`, `MIRIFLAGS=-Zmiri-disable-isolation`, libc backend auto-selected via `CARGO_CFG_MIRI` in `build.rs`) | 3x | **Fired**: `error: Undefined Behavior: reading memory ... uninitialized ... inotify.rs:197:69` (the `event.len` read), bit-for-bit identical location/offsets all 3 runs |

Soft cap of 3 not needed - 2 tools (real execution + Miri) both gave an unambiguous, repeatable
positive signal; a 3rd (ASAN) would have been redundant.

## Status

**Confirmed.** Reproduces deterministically: 3/3 runs under plain debug execution (UB-shaped wrong
output every time, content varies as expected for uninitialized-memory disclosure) and 3/3 runs
under Miri (bit-for-bit identical UB diagnostic, same allocation offset and source line every time).

**Fully reproducible:** **Yes.** `repro/` drives the real, unmodified, public
`rustix::fs::inotify::Reader::next()` API end-to-end against a real pipe fd with real `read()`/
`write()` syscalls - no hand-faked kernel/inotify behavior. The "second participant" here is
whatever process/fd a caller passes to `Reader::new` (`Reader` is generic over any `Fd: AsFd`); a
plain pipe returning a genuinely short read is completely ordinary `read(2)` behavior, not a
contrived stand-in for it (unlike a real inotify fd, which the kernel always fills with whole
`inotify_event` structs - this finding is precisely that `Reader`'s safety reasoning assumed that
guarantee without the type system enforcing that the fd is actually an inotify fd).

## Class

memory-safety (reads uninitialized memory through a typed reference; `file_name()`'s unbounded
`CStr::from_ptr` additionally has no bound against the buffer at all, so a differently-shaped input
could walk further out of bounds than this specific repro demonstrates - not separately
differentiated as its own class here since this fix closes both at once).

## Root cause

`Reader::next()` performs a `read()` into a `&mut [MaybeUninit<u8>]` buffer, then does:

```rust
let ptr = self.buf[self.offset..].as_ptr();
let event = unsafe { &*ptr.cast::<inotify_event>() };   // no bounds check first
self.offset += size_of::<inotify_event>() + usize::try_from(event.len).unwrap();
```

The accompanying `SAFETY` comment states "Assumption: the kernel will not give us partial
structs" - true for a genuine inotify fd (inotify's own kernel-side event producer always emits
whole structs), but `Reader<'buf, Fd: AsFd>`'s type signature places no constraint tying `Fd` to
one actually obtained from `inotify::init()`. Any `AsFd`-implementing value (a pipe, a regular
file, a socket) can be passed to `Reader::new`, and a plain `read()` on those has no per-fd-type
"always returns whole records" guarantee the way `getdents64` does for `RawDir`/`Dir` (candidate
044's reason for ruling that sibling finding "Not reproducible" does not apply here). When the
read is shorter than `size_of::<inotify_event>()` (16 bytes: `wd:i32, mask:u32, cookie:u32,
len:u32`), the cast-and-read still happens over the un-written tail of the buffer, and
`event.len`'s own (possibly-uninitialized) value then drives how far `file_name()`'s
`CStr::from_ptr` scans with no bound against the buffer at all.

## Introduced by

777fbc68 "Implement InotifyReader for raw inotify event iteration (#1113)" (2024-08-26) - the
original implementation already had this exact shape (no bounds check before the cast, unbounded
`CStr::from_ptr` for the name); nothing since has added one (`git log -S "is_buffer_empty"` and
`git log -S "pub struct Reader"` both bottom out at this same commit for `src/fs/inotify.rs`).

## The fix

Added, in `Reader::next()`:
1. A header-size check (`self.initialized - self.offset < size_of::<inotify_event>()` ->
   `Err(Errno::INVAL)`) before the `inotify_event` cast, mirroring the check `Dir`'s own backend
   (`src/backend/linux_raw/fs/dir.rs:165`) already does for the structurally identical
   `linux_dirent64` case.
2. A whole-entry bound (`size_of::<inotify_event>() + event.len <= remaining`, via
   `checked_add` to also reject an `event.len` so large it would overflow) before advancing
   `self.offset`, instead of trusting `event.len` unconditionally.
3. Replaced the unbounded `CStr::from_ptr(event.name.as_ptr().cast())` with a NUL search that is
   explicitly bounded to the now-verified-initialized `name_len`-byte region
   (`name_bytes.iter().position(|&b| b == 0)` + `CStr::from_bytes_with_nul`), so a missing NUL
   within that region now yields `None` instead of scanning past the buffer - the same
   byte-scanning defense `Dir`'s own backend already uses for its own `d_name` field.

Verified against both the standalone `repro/` (via `just test`, see `output.txt`) and Miri (which
now exits cleanly with `Err(Errno::INVAL)` instead of reporting UB), and against a separate
well-formed-event check (`wd`/`mask`/`cookie`/`len`/name all correctly sized and NUL-terminated)
confirming the new checks don't change behavior for valid input - see `output.txt`.

- Project test suite (`cargo test --quiet --workspace --features "std,all-apis,use-libc"
  --no-fail-fast`): 411 passed / 1 failed before the fix -> 411 passed / 1 failed after. The one
  failure (`ioctl::test_int_setter`) is identical before and after and pre-existing/unrelated
  (documented in `___BUGHUNT___/README.md`: this container has no `/dev/net/tun` device).

## What's not done / open questions

- `file_name()`'s NUL-bounding fix only covers the path reached through `next()`'s own
  `entry_len <= remaining` check; it was not separately fuzzed against a wide sweep of
  adversarial `event.len` values (0, 1, `u32::MAX`, values that would overflow
  `size_of::<inotify_event>() + len` on a 32-bit target) - the `checked_add` should make these
  safe, but only the single short-read scenario in `repro/` was run against the fix, not a
  dedicated fuzz harness.
- Not checked against a real inotify fd combined with a deliberately tiny buffer that splits a
  single read right at a struct boundary (i.e. whether `inotify_init`'s own kernel-side guarantee
  can ever be violated in practice) - per candidate 044's conclusion for the structurally similar
  `getdents64` case, this is believed to be a non-issue for genuine inotify fds specifically, but
  was not independently re-verified for inotify's own kernel code path this session.
- The `libc`-backend-vs-Miri interaction needed one false start (passing
  `RUSTFLAGS="--cfg rustix_no_linux_raw"` to force the libc backend under Miri wrongly disabled
  the `linux_raw_dep` feature that gates the `inotify` module's own types, since `build.rs` only
  turns that feature on when `!cfg_no_linux_raw`) - worth recording as a gotcha: Miri already
  forces the libc backend on its own via `CARGO_CFG_MIRI`, no extra `RUSTFLAGS` needed for this
  crate under Miri.
