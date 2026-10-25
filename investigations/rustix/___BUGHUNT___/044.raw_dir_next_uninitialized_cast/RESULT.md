# raw_dir_next_uninitialized_cast

`RawDir::next`/`is_buffer_empty` (`src/fs/raw_dir.rs:195-221`) only checks `self.offset >=
self.initialized` before doing `unsafe { &*dirent_ptr.cast::<linux_dirent64>() }` over the raw
`&mut [MaybeUninit<u8>]` buffer - unlike the sibling `Dir` implementation
(`src/backend/linux_raw/fs/dir.rs`), which checks `self.buf.len() - self.pos <
size_of::<linux_dirent64>()` before trusting the header, and backs its buffer with a real `Vec<u8>`
that's explicitly resized with zero-fill (so even its worst case reads zeros, never
uninitialized memory). The candidate's claim: if the kernel's getdents64 "never splits a struct
across the returned byte count" invariant were ever violated, `RawDir::next` would read truly
uninitialized memory through a reference to a concrete (non-`MaybeUninit`) type - real UB under
Rust's memory model, not just wrong output.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading + git blame | `git blame`/`git log` on `src/fs/raw_dir.rs`, compared against `src/backend/linux_raw/fs/dir.rs` and the two prior "Fix UB in raw dir" commits (af0af278, 811e5eaa) | Confirmed the asymmetry is real and intentional: the `// SAFETY: ... Assumption: the kernel will not give us partial structs` comment (added in bc47644a3, "Simplify RawDir iteration (#515)") is the *only* protection RawDir has - no bounds check, no zero-fill fallback, unlike `Dir`. This is a deliberate "zero-compromise" design tradeoff (see #457's own commit title), not an oversight that was simply never noticed. |
| Approach 1: real boundary-value / differential sweep | Plain debug build, `examples/raw_dir_boundary.rs` (kept in `investigation/`), real tmpfs directory with 13 entries of varying name length (1-250 bytes, to shift record boundaries across every alignment), every buffer size 0 to 4096 swept, using the documented "grow buffer and retry on INVAL" convention, output compared against `std::fs::read_dir` | Clean: 0 mismatches, 0 corrupted/extra entries across the full sweep - genuine `getdents64` syscalls never produced a buffer that cut a record in half, matching the documented kernel invariant |
| Approach 2: MemorySanitizer (the directly-apt tool for "reads uninitialized memory" claims) | `cargo +nightly run -Zbuild-std --target x86_64-unknown-linux-gnu-msan` with `-Zsanitizer=memory -Zsanitizer-memory-track-origins`, same `examples/raw_dir_boundary.rs`, both linux_raw and libc (`--cfg rustix_use_libc`/`--features use-libc`) backends | **Fired**: `use-of-uninitialized-value` at `raw_dir.rs:226:40` (the `dirent.d_reclen`/`dirent.d_ino` field reads), reproducible on demand (5/5 runs, both backends) - see below for why this is a tool artifact, not a real finding |
| Approach 2 follow-up: isolate the MSan diagnostic in a standalone, rustix-free program | `investigation/msan_false_positive_main.rs`, no rustix code at all: the exact same real `getdents64` syscall via `libc::syscall`, the exact same raw-pointer-cast-to-`dirent64` field-read pattern, against the exact same real directory, varying only whether the destination buffer is a stack array (`[MaybeUninit<u8>; 4096]`) or a heap `Vec<MaybeUninit<u8>>` | The stack-buffer decode completes with **no** MSan warning; the heap-buffer decode (identical syscall, identical decode, only the allocation site differs) fires the **exact same** `use-of-uninitialized-value` diagnostic as the real `RawDir` code. This isolates the signal to a gap in MSan's `syscall()`/`getdents64` interceptor: it does not unpoison memory for this syscall when the destination buffer was obtained from Rust's global allocator (heap), only when it's a stack allocation - confirmed independently of rustix entirely, 5/5 runs |
| Approach 3: real second-participant trigger (a buggy/malicious FUSE filesystem, which is the only way a legitimate "kernel violates the invariant" scenario could occur without hand-faking kernel output) | Checked `/dev/fuse` availability and `unshare --user --mount` in this container | **Infeasible in this environment**: no `/dev/fuse` device node, no `modprobe`, `unshare --user` fails with `Operation not permitted` (no unprivileged user namespaces) - this sandboxed container has no path to mount even a user-space FUSE filesystem, so a genuine "real filesystem whose readdir implementation violates the VFS record-boundary guarantee" experiment could not be run here. (Moot anyway: FUSE's kernel-side translation layer enforces the same `dir_emit`/buffer-space invariant as every other filesystem type, so even a malicious FUSE daemon cannot make `getdents64` violate this from userspace - this is a generic VFS-level guarantee in `fs/readdir.c`, not something any individual filesystem driver controls.) |

## Status

**Not reproducible.** No genuine manifestation of the claimed UB was found via any real path:
- A real-syscall differential/boundary sweep (buffer sizes 0-4096, varied real file name
  lengths, the documented buffer-growing retry convention) never produced a mismatch, a crash, or
  corrupted data, on either backend.
- The one tool that *did* fire (MemorySanitizer) was independently proven, via a standalone
  program containing no rustix code at all, to be a known MSan limitation (its `getdents64`
  syscall interceptor doesn't unpoison heap-allocated destination buffers the same way it does
  stack-allocated ones) rather than a real diagnostic about `RawDir`'s code - see
  `CONFIRMATION.md` section 5 ("distinguish a real finding from the tool's own limitations").
- The kernel-side invariant the candidate's finding depends on being violated (`getdents64` never
  splits a `linux_dirent64` record across the returned byte count) is enforced generically by the
  VFS (`fs/readdir.c`'s `dir_emit`/`filldir64`), not by individual filesystem drivers - so even a
  malicious/buggy real filesystem (the only realistic "real second participant" that could try to
  violate it) cannot do so from userspace. This mirrors candidates 018 and 020's own conclusions
  for structurally identical "kernel-trust" claims in this codebase.
- No FUSE device was available in this container to attempt that real-second-participant
  experiment directly (see Approach 3 above), but it's moot given the previous point.

## Root cause (why the suspect turned out safe)

The missing bounds check is real (`RawDir` genuinely has less defense-in-depth than `Dir`), but it
was a deliberate, explicitly-commented design choice relying on a kernel-level guarantee
(`getdents64`'s "never emit a partial record" invariant) that is enforced in the generic VFS layer
for every Linux filesystem, not something any individual filesystem implementation - buggy,
malicious, or otherwise - can violate from userspace. Two prior commits in this same file's history
(af0af278 "Fix UB due to misaligned RawDir pointer", 811e5eaa "Fix another UB in raw dir") show the
maintainers already treat this struct's safety carefully; the current state (bc47644a3) represents
where they landed: trust the kernel's own ABI guarantee instead of paying for a redundant
bounds check on every entry, the same tradeoff `Dir`'s own comment documents making too
("Assumption: the kernel will not give us partial structs") just without `Dir`'s extra
belt-and-suspenders check on top.

## Introduced by

N/A (Not reproducible - no bug to date). For reference, the current form of the unchecked read
dates to bc47644a3 "Simplify RawDir iteration (#515)" (2023-01-17), which is also where the
"Assumption: the kernel will not give us partial structs" SAFETY comment was added.

## The fix

Not applicable - no reproducible bug. Main project tree was never modified (no patch to apply/
revert).

- Project test suite: N/A, no fix was made. For reference, pristine-tree baseline this session
  (base_commit 287214b8, `cargo test --quiet --workspace`): 22 passed / 0 failed / 1 ignored (plus
  7 doc-tests), matching the project's own previously-recorded baseline in
  `___BUGHUNT___/projects/rustix.md`.

## What's not done / open questions

- A defense-in-depth bounds check (matching `Dir`'s own `self.buf.len() - self.pos <
  size_of::<linux_dirent64>()` guard) would still be a reasonable *hardening* change for `RawDir`
  even though no real trigger was found - it costs one comparison per entry and would turn a
  hypothetical future kernel-ABI regression (or a non-Linux/emulated `getdents64`-alike with looser
  guarantees) from real UB into a clean, catchable error, consistent with how `Dir` already treats
  the exact same input. This is a hardening suggestion, not a confirmed-bug fix, so no `patch.diff`
  was produced for it per this skill's "don't force a patch that only papers over a theoretical
  input" guidance - a maintainer may still want it.
- The MSan interceptor gap found here (heap-allocated `Vec`/`Box` buffers passed to a raw
  `getdents64` syscall via either `libc::syscall()` or an inline `asm!` instruction are not
  unpoisoned, while an equivalent stack buffer is) is worth recording as a general tooling gotcha
  for this project (added to `projects/rustix.md`) - any future candidate relying on MSan against
  a real syscall writing into a `Vec`-backed buffer should expect the same false positive and
  verify with the same stack-vs-heap isolation technique before trusting the diagnostic.
- Not independently re-verified on a non-Linux kernel/non-VFS filesystem (e.g. a 9p/NFS client
  path, or a FUSE daemon on real hardware outside this sandboxed container) - the VFS-level
  argument above should hold generically, but was not empirically exercised beyond ext4/tmpfs in
  this container.
