# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `miri` (nightly component, already installed) | Partially | Only useful for pure-computation code paths with no real syscall in them. `linux_raw` backend does real syscalls via inline `asm!`, which Miri cannot execute at all. The `libc` backend's real syscalls (`libc::syscall(...)`) also generally can't run to completion under Miri (see candidate 004's residual "incorrect c-variadic argument type" limitation on `SYS_futex`) - Miri is still useful for catching a specific UB-shaped expression (e.g. an integer-to-pointer cast) even if the surrounding function can't fully execute under it. Force the `libc` backend for anything Miri-adjacent: `RUSTFLAGS="--cfg rustix_no_linux_raw"`. |
| ASAN/UBSAN | Not tried this session | No candidate needed it - most confirmed issues were logic/type-soundness bugs provable via a real syscall + real assertion, not memory-corruption-shaped. |
| `cargo-fuzz` | No `fuzz/` dir in this repo | A candidate needing fuzzing would need its own one-off harness from scratch. |
| Real `io_uring` syscalls | Yes | `io_uring_setup`/`io_uring_register` work normally in this container (`/proc/sys/kernel/io_uring_disabled` is `0`, not blocked by seccomp here). |
| Real TCP/loopback sockets | Yes | Used for every `net`-area candidate's real two-participant repro (server thread + client thread over `127.0.0.1`). |
| `/dev/net/tun` | No | Not present in this sandbox - blocks `ioctl::test_int_setter` (a pre-existing, unrelated test failure seen in every `cargo test` run this session) and any TUN/TAP-dependent candidate (e.g. issue #1051's `TUNSETOFFLOAD` report was not investigated for this reason). |

## Existing project harnesses worth reusing

None found - no `fuzz/` directory, no differential-testing setup. The project's own `tests/`
directory (`cargo test --workspace`, broaden with `--features "std,all-apis,use-libc"` to actually
exercise `net`/`thread`/etc. - the bare default feature set is `std` only and skips most of the
crate's own test coverage) is the only existing harness, and is fast (seconds) - always run it
before/after a candidate's fix.

## Instrumentation/tooling set up for this workspace

Nothing needed installing - `rustc`/`cargo` 1.98.1 stable, nightly + `miri` component were already
present. The one real setup cost was **discovering the right feature flags to actually exercise
`net`/`thread`/`io_uring` code in `cargo test`** - the bare `cargo test --workspace` only enables
`std` and silently skips most of the crate's own tests; use
`--features "std,all-apis,use-libc"` (or the specific area's feature, e.g. `net`) to get real
coverage.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real (independently
   re-verified by the orchestrating session against the main tree, not just trusted from the
   investigating agent's own report), plus the project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
5. Two parallel candidate agents must never be told to use the same fixed scratch path (e.g.
   `/tmp/some-fixed-name`) - see `projects/rustix.md` (this skill's own cross-session notes) for a
   real collision this session hit and how it was worked around.
