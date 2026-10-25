# dir_debug_impl_missing_fields

`Dir`'s manual `fmt::Debug` impl in `src/backend/linux_raw/fs/dir.rs:313` prints only the `fd`
field. `Dir` actually has five fields (`fd`, `any_errors`, `rewind`, `buf`, `pos`), the last four
of which hold the iterator's real internal state (whether a previous `next()` call hit an error,
whether the next `next()` call will rewind the underlying stream first, the current
`getdents64` buffer, and the read position within it). `clippy::missing_fields_in_debug`
(pedantic, allow-by-default) flags exactly this. Candidate sourced from a direct clippy-pedantic
lint, not from `IDEAS.md`'s own backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Compared `struct Dir`'s 5 fields against the manual `Debug` impl's single `.field("fd", ...)` call | Confirmed the impl textually omits 4 of 5 fields |
| `cargo clippy` with the specific pedantic lint enabled | `cargo clippy --quiet --features fs -- -W clippy::missing_fields_in_debug`, stable rustc/clippy 1.98.1 | Lint fires exactly on this `impl fmt::Debug for Dir` at `src/backend/linux_raw/fs/dir.rs:313`, naming all 4 missing fields (`any_errors`, `rewind`, `buf`, `pos`) |
| Standalone repro against the real public API, `cargo run --release` | Real temp directory with 8 real files, opened via `openat`, iterated via `Dir::read_from` + one real `next()` (populates `buf`/`pos` from a real `getdents64` syscall), then `rewind()` (sets `rewind = true`), then `format!("{:?}", d)` | `{:?}` output is `Dir { fd: OwnedFd { fd: 4 } }` -- `any_errors`, `rewind`, `buf`, `pos` are all absent from the printed output despite holding real, non-default values (`rewind == true`, `buf` containing the real directory listing bytes, `pos == 304`) |

## Status

**Confirmed**, reproduces 5/5 runs (release build, stable rustc 1.98.1, real directory I/O, no
fixed seed needed -- the omission is unconditional/textual in the `Debug` impl itself, not
layout- or timing-dependent, so every run gives byte-for-byte identical "missing fields" output
modulo the real syscall-dependent buffer bytes and fd number).

**Fully reproducible**: **Yes**. The repro calls `rustix::fs::Dir::read_from` (the real public
constructor) on a real, just-created directory opened via `rustix::fs::openat`, drives one real
`next()` (a real `getdents64` syscall) and one real `rewind()` call (both public API), then
formats the real `Dir` value with `{:?}`. No internal/private state is hand-constructed; this is a
single-participant case (a real directory on disk handed to the crate's own public entry point --
see `CONFIRMATION.md` section 11's own carve-out for exactly this shape), so no second
participant needs to be faked or genuinely driven.

## Class

other (Debug-impl completeness / diagnostics)

## Root cause

`impl fmt::Debug for Dir` was hand-written with `f.debug_struct("Dir").field("fd",
&self.fd).finish()` and was never updated to include the other fields. This was true from the
very commit that introduced the impl: at that commit the struct already had `buf`, `pos`, and
`next` (precursor to today's `any_errors`/`rewind` iteration-state fields) besides `fd`, but the
`Debug` impl only ever named `fd`. Every subsequent field addition/rename to the struct
(`any_errors`, `rewind` added later; `next: Option<u64>` later removed) likewise never touched the
`Debug` impl, so the omission has been present, unchanged, since the impl's introduction.

## Introduced by

a201b05fe "Implement `Debug` for `Dir`." (2022-03-04). Confirmed via `git blame` on
`src/backend/linux_raw/fs/dir.rs` (impl block, lines 313-317) and `git show --stat a201b05fe`
(the commit that added this exact `impl fmt::Debug for Dir` block, touching both the `linux_raw`
and `libc` backends' `dir.rs` in the same commit). `git show a201b05fe:src/imp/linux_raw/fs/dir.rs`
confirms the struct already had `buf`/`pos`/`next` fields at that point, so the omission was not a
later regression from fields added after the impl -- it was incomplete from the day the impl was
written.

## The fix

Changed the impl to chain `.field(...)` calls for all 5 fields instead of just `fd`:

```rust
impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dir")
            .field("fd", &self.fd)
            .field("any_errors", &self.any_errors)
            .field("rewind", &self.rewind)
            .field("buf", &self.buf)
            .field("pos", &self.pos)
            .finish()
    }
}
```

This is unconditional (every field is always included regardless of its value), so it isn't
narrowly tailored to the one triggering scenario -- verified against two different states: (1) the
original repro scenario (one real `next()` call plus `rewind()`, i.e. `rewind: true`, `pos: 304`,
`buf` containing real directory-listing bytes), and (2) a freshly-constructed `Dir` with no
iteration at all (`Dir::new`, all-default state: `any_errors: false`, `rewind: false`, `buf: []`,
`pos: 0`). Both print all 5 fields after the fix; `cargo clippy -- -W
clippy::missing_fields_in_debug` no longer flags `src/backend/linux_raw/fs/dir.rs` after the
change (only unrelated warnings elsewhere in the crate remain).

Before/after of re-running `repro/` against the patched tree (`just test` in this folder, real
clone of `https://github.com/bytecodealliance/rustix.git` at `287214b889865d8e1406a0ee71cc409b6f6191c8`):

- Before: `Debug output: Dir { fd: OwnedFd { fd: 4 } }` -- `fields present in struct but missing
  from Debug output: ["any_errors", "rewind", "buf", "pos"]` -- `REPRODUCED`.
- After: `Debug output: Dir { fd: OwnedFd { fd: 4 }, any_errors: false, rewind: true, buf: [...],
  pos: 304 }` -- `fields present in struct but missing from Debug output: []` -- `NOT REPRODUCED`.

See `output.txt` for the full real captured `just test` run.

- Project test suite: `cargo test --workspace` (default features) -- 22 passed / 0 failed / 1
  ignored (+ 7 doc-tests, included in the 22) both before and after the fix, identical counts, no
  new failures. No existing test asserts `Dir`'s exact `Debug` output (`grep -rn "Dir {" -- a "Dir
  {" debug-format literal only occurs inside the two `impl fmt::Debug for Dir` blocks themselves,
  not in any test), so this patch changes no test's outcome.

## What's not done / open questions

- The sibling `libc`-backend `Dir` (`src/backend/libc/fs/dir.rs:308`) was checked and does **not**
  have this bug: its `Debug` impl also only prints `fd`, but its struct genuinely only tracks
  `fd`/`libc_dir` (a raw libc `DIR*` the iteration state lives inside, owned by libc, not
  Rust-visible fields) -- there is nothing else to include there, so the lint correctly does not
  fire on it. This candidate is specific to the `linux_raw` backend's `Dir`.
- Purely a diagnostic/completeness issue (what `{:?}` prints), not a memory-safety or logic bug in
  the iterator's actual behavior -- the fix has no effect on anything except what gets printed when
  a user or maintainer formats a `Dir` with `{:?}`.
