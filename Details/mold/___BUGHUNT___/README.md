# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

mold is C++20 (not Rust) - no miri. CMake build (`CMakeLists.txt` at root), ~37.5k LOC across
`lib/` and `src/`.

| tool | works here? | notes |
|---|---|---|
| plain debug build | yes | `build-plain/` at repo root, pre-configured (`-DCMAKE_BUILD_TYPE=Debug` or `-O0 -g`). Use for quick iteration / gdb work that doesn't need sanitizers. |
| ASAN+UBSAN | yes, with g++ only | `build-asan/` at repo root, pre-configured with `-DMOLD_USE_ASAN=ON` (this single CMake option pulls in `-fsanitize=address -fsanitize=undefined` together - not two separate flags). **clang++ 21.1.8 fails to link this configuration on this host** (`libclang_rt.asan*.a` not installed) - the dir is configured with `-DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++` for this reason. Don't touch `build-asan/`'s cmake cache casually - toggling `MOLD_USE_ASAN` forces a full from-scratch rebuild of every arch translation unit (~100-110s at `-j24`, longer at the recommended `-j5`, see below). |
| TSAN | **builds OK (2026-09-05)** | `-DMOLD_USE_TSAN=ON -DCMAKE_CXX_COMPILER=g++` (gcc 15) configures and produces a working `mold` binary - verified by candidate 033 before it was interrupted. Mutually exclusive with ASAN/MSAN, so it needs its own build dir. **Not yet exercised at scale**: no session has run a real workload under it and triaged the output, so whether it is *usable* (signal-to-noise on TBB/mimalloc internals) is still unknown - see `IDEAS.md`'s TSAN resume note. |
| MSAN | not set up | `-DMOLD_USE_MSAN=ON` exists in CMakeLists.txt (mutually exclusive with ASAN/TSAN); no build dir has ever been configured for it. |
| fuzzing | no existing harness | no `fuzz/`/cargo-fuzz-equivalent in this repo; a one-off harness would be hand-rolled per candidate if ever needed (none has been so far - every candidate found through code-reading + a crafted-input repro against the real binary). |

**Build parallelism: cap at `-j5` on this host, not `nproc` (24).** A full `-j24` ASAN build/ctest
run OOM-killed a session on 2026-09-04 (24 concurrent ASAN-instrumented processes exhausted this
host's 29GB RAM). No further memory pressure was seen after capping at `-j5`.

## Existing project harnesses worth reusing

None - no existing fuzz targets or differential-testing setup in this repo. `test/` has ~522
`*.sh` scripts driven via CTest (`test/CMakeLists.txt`); `ctest -j5` in `build-asan/` registers 476
native (x86_64) tests (~20 skipped by ctest itself for feature/environment gating - zstd/hwasan/apx/
etc; QEMU cross-arch tests behind `MOLD_ENABLE_QEMU_TESTS` don't register at all on this host).
Whole suite runs in ~2 minutes at `-j5`. **`x86_64-run-clang` fails on every ASAN build regardless
of source changes** - confirmed pre-existing on unpatched HEAD (mold's own `mold-wrapper.so`
`LD_PRELOAD` trick collides with ASAN's own preload requirement) - don't mistake this one failure
for a regression.

Malformed ELF/archive inputs are hand-crafted with a small `struct`-based Python generator
(`_reference/make_bad_elf.py`, copied into each candidate's own `repro/` as needed) - writes a bare
64-byte `Elf64_Ehdr` with caller-controlled `e_shoff`/`e_shnum`/`e_shstrndx`. Extend it (real
section header table, symtab/strtab bytes, archive headers) as needed per candidate; no
`pyelftools` or other network dependency is used.

## Instrumentation/tooling set up for this workspace

Both build dirs (`build-plain/`, `build-asan/`) were configured once and are reused across
candidates - only reconfigure if a cmake option genuinely needs to change, and always double-check
which directory you're in first (see gotcha below).

| what | why | caveat |
|---|---|---|
| `build-plain/` (`-DCMAKE_BUILD_TYPE=Debug`, default compiler) | fast iteration, gdb backtraces without ASAN's own instrumentation noise | rebuild after any source change: `cmake --build build-plain -j5` |
| `build-asan/` (`-DMOLD_USE_ASAN=ON`, g++) | catches heap OOB / UB the plain build won't; the default "does this actually crash" check | see the mmap-page and silent-SIGSEGV limitations below - it does **not** catch everything |

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.

## Known environment quirks (read before writing off a "safe" result)

- **ASAN does NOT catch a small OOB read that stays within the same mmap'd page.** mold mmaps
  every input file read-only and computes raw pointers into it (`mf->data + offset` everywhere). A
  file mapping always rounds up to a full page (the kernel zero-fills the tail), and ASAN only
  instruments `malloc`/`new` heap allocations with redzones - not raw `mmap()` regions. A truncated
  file or an offset that's merely one-past-declared-size reads silently as zero bytes, under both
  plain and ASAN builds, no crash, no diagnostic (confirmed empirically, candidate 004). **Only an
  offset/count large enough to jump entirely past the mapped region (multi-page-or-more) produces a
  real fault** - prefer deliberately huge/adversarial offsets over merely-one-past-the-end values
  when constructing a repro for an untrusted-offset finding.
- **ASAN does not always print a diagnostic for a genuine SIGSEGV.** A crash inside a oneTBB worker
  thread (mold parallelizes output writing) can segfault for real - confirmed independently via
  `gdb` backtrace and matching plain-build behavior - while printing **no** `AddressSanitizer:`
  report at all, just a bare kernel-delivered `SIGSEGV`/exit 139 (candidate 009). Check the actual
  exit code (`>=128`) and/or use `gdb`, don't treat "ASAN was silent" as "input is safe".
- **mold forks a child process to do the actual link** (`src/subprocess-unix.cc::fork_child`). A
  crash happens in that forked child; the parent re-raises the same signal it observed
  (`subprocess-unix.cc:47`). A plain shell run (`mold ...; echo $?`) or an ASAN report redirected
  with `2>&1` both correctly show the real crash with no extra handling needed - but `gdb --args
  mold ...` alone attaches to/backtraces the useless parent. Use:
  `gdb -ex "set follow-fork-mode child" -ex "set detach-on-fork on" -ex run -ex bt --args mold ...`
- Double-check the working directory before any `cmake -D...` reconfigure command - it's easy to
  mean `build-plain/` and accidentally touch `build-asan/`'s cache (or vice versa), forcing an
  unwanted full rebuild.

See `~/.claude/skills/bug-hunt/projects/mold.md` for the fuller session-by-session narrative and
the current risk map (which source files/functions are the densest cluster of untrusted-offset
patterns) - this file is deliberately just the "how to reproduce anything here" reference.
