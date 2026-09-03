# back_write_bitcode_emission_lto_supported_paths

`src/back/write.rs`'s bitcode-emission block (lines 47-70 at commit 4cd7da5) adds
`-flto=auto`/`-flto-partition=one` (and, for one sub-branch, `-ffat-lto-objects`) when
`lto_supported` is true, then calls `context.compile_to_file(OutputKind::ObjectFile, ...)` - noted
in candidate `001.lto_driver_failure_segfault`'s "What's not done" section as sharing candidate
001's "no precondition check before an operation known to be able to crash" pattern (001's own
confirmed crash is a separate call site a few lines further down, `write.rs:122`, gated on
`lto_mode`/`fat_lto` rather than `lto_supported`, and reached through `compile_to_file(..,
OutputKind::Executable, ..)` plus `add_driver_option("-fuse-linker-plugin")`). Flagged, not
individually checked, until this candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real environment, boundary-value construction via rustc flags that set `config.emit_bc`/`config.emit_obj` to trigger `config.bitcode_needed()` (`--emit=llvm-bc`) | Release-mode backend build reused from the main tree's sysroot (see this workspace's `README.md`); this environment's real `gccjit::is_lto_supported()` returns `false`, so the guarded `-flto=auto` options are dead code here in practice | Compiles cleanly, exit 0, produces a real 8.4KB relocatable object |
| Source-instrumented build: temporarily hardcoded `let lto_supported = true;` in `src/back/write.rs` (plus a diagnostic `eprintln!`) to force the guarded branch live regardless of this environment's real GCC-LTO-support detection, rebuilt (`./y.sh build --release`, ~1s incremental), then drove all three sub-branches gated by that condition with real rustc flags: `--emit=llvm-bc` (line 48 first half), `-C linker-plugin-lto` (line 48 second half, `EmitObj::Bitcode`), `--crate-type=rlib --emit=obj,link -C embed-bitcode=yes` (line 59, `EmitObj::ObjectCode(BitcodeSection::Full)`) - 3 repeat runs each | All 9 runs: exit 0, no SIGSEGV/SIGABRT/gccjit-internal error, deterministic 3/3 for each of the 3 sub-branches. `objdump -h` on the resulting object from the third sub-branch confirms real `.gnu.lto_*` sections are present, i.e. `-flto=auto` was genuinely honored by GCC, not silently ignored | Change reverted with `git checkout -- src/back/write.rs` before finishing |
| Read code: compared this call site's `OutputKind::ObjectFile` + no `add_driver_option`/link step against candidate 001's confirmed crash site's `OutputKind::Executable` + `add_driver_option("-fuse-linker-plugin")` + `add_driver_option("-Wl,-r")` (which does invoke the link driver) | - | Explains why the two call sites behave differently despite sharing `-flto=auto`/`-flto-partition=one` - see Root cause below |
| Verified `just test` (`repro/`+`probe.diff`+`justfile` in this candidate folder) end-to-end from a truly fresh `git clone` of `repo_url`@`base_commit` (not this workspace's own build cache) - `just clone` (full `./y.sh prepare && ./y.sh build --sysroot --release`), `just reproduce-before` (real environment, guard naturally false), `just apply` (applies `probe.diff`, rebuilds), `just reproduce-after` (guard forced true) | Fresh clone, same 3 flag combinations x 3 repeats each, both before and after `probe.diff` | Both `reproduce-before` and `reproduce-after` print `RESULT: NOT REPRODUCIBLE` (all 9 runs each, exit 0) - matches the in-place investigation exactly; `just clean` afterward removes the fresh clone |

## Status

**Not reproducible.** Tried real-environment exercise (1 config) plus source-instrumented
worst-case exercise (`lto_supported` forced `true`, all 3 flag combinations that reach this code
block, 3 repeat runs each = 9 runs) plus a code-reading pass explaining the mechanism. No crash,
no gccjit-internal error, no wrong output observed in any run.

## Class

n/a (not reproducible)

## Root cause

Not a bug: this call site's `compile_to_file(OutputKind::ObjectFile, ...)` compiles the current
translation unit to a single, ordinary `.o` file and returns - libgccjit never invokes the link
driver (`gcc`'s own driver stage, the piece that in turn shells out to `lto1`/needs
`liblto_plugin.so`) for `OutputKind::ObjectFile`. Passing `-flto=auto -flto-partition=one` at this
stage only tells GCC's codegen to *emit* GIMPLE bytecode sections (`.gnu.lto_*`, confirmed present
in the resulting object via `objdump -h`, see `output.txt`) into the object file for a *later*
consumer to read - it does not itself trigger any LTO merge/link step, so there is nothing here
that needs `liblto_plugin.so` to be present.

Candidate 001's confirmed crash, by contrast, is a different call site entirely
(`write.rs:113-146`, gated by `lto_mode`/`fat_lto`, not `lto_supported`) that calls
`context.compile_to_file(OutputKind::Executable, &lto_path)` after adding
`add_driver_option("-fuse-linker-plugin")` and `add_driver_option("-Wl,-r")` -
`OutputKind::Executable` does invoke the link driver, which is exactly the step that needs
`liblto_plugin.so` and crashes when it's absent. The two call sites share the same
`-flto=auto`/`-flto-partition=one` *command-line* options (`add_command_line_option`, which affects
codegen only) but differ in the operation that follows them (`compile_to_file`'s `OutputKind`
argument, plus whether any `add_driver_option`/link-triggering flag was added at all) - that
difference in operation, not the shared flags, is what determines whether `liblto_plugin.so`'s
absence matters. The "no precondition check before an operation known to be able to crash" pattern
correctly describes `write.rs:122` (candidate 001) but does not carry over to `write.rs:47-70`: the
operation these three sub-branches perform (`compile_to_file(OutputKind::ObjectFile, ...)`) is not,
in fact, "known to be able to crash" the same way - it was never observed to crash in any of the 9
runs above, including the worst-case one where the guard condition was forced true specifically to
make sure the branch wasn't simply unreachable in this environment.

Separately (incidental to this candidate, not itself investigated further): `lto_supported`
(`module.module_llvm.lto_supported`, set from `gccjit::is_lto_supported()`) is genuinely `false` in
this environment's real, unforced state - meaning none of the three `write.rs:47-70` sub-branches
ever add the `-flto=auto`/`-flto-partition=one`/`-ffat-lto-objects` options at all under this
environment's actual downloaded `libgccjit.so`, independent of whatever this candidate found. This
matches candidate 001's own note that the downloaded `libgccjit.so` lacks a full accompanying GCC
install tree.

## Introduced by

n/a - not reproducible, no bug to date.

## The fix

No fix - nothing to fix. No bounded change was made to `src/back/write.rs`; the temporary
diagnostic instrumentation used to investigate was reverted (`git checkout -- src/back/write.rs`)
before this candidate was written up.

- Project test suite: N/A - no fix was applied this session (nothing to fix), so there is no
  before/after to state. `git status --short` in this candidate's worktree shows no changes after
  the revert.

## What's not done / open questions

- Not checked: whether `EmitObj::Bitcode`'s full downstream pipeline (i.e. actually driving a real
  `-C linker-plugin-lto` link to completion rather than stopping at `--emit=obj`) works end-to-end
  in this backend at all - the one attempt made here (see `output.txt`, "linking with `cc` failed")
  hit an unrelated `rust-lld` plugin-argument-syntax error well past the code under test for this
  candidate; that looks like a distinct, pre-existing gap in this backend's `-C linker-plugin-lto`
  support (not a crash, not part of this candidate's scope) and was not investigated further.
- Not checked: whether a real GCC install (not the `download-gccjit = true` prebuilt
  `libgccjit.so` this session's environment uses) reports `gccjit::is_lto_supported() == true`
  naturally, which would make these branches live without the source-instrumented `true` override
  used here - this candidate deliberately forced the condition instead, since building/locating an
  environment where it's naturally true was out of scope for confirming just this call site's
  crash-safety.
- This closes out the "other `compile_to_file`/`add_driver_option` call sites" open question left
  by candidate 001's `RESULT.md` for `write.rs:47-70` specifically. `write.rs:83`
  (`context.compile_to_file(OutputKind::Assembler, ...)`, the `config.emit_asm` branch) was not
  separately checked - it shares no `add_driver_option`/LTO-flag call at all, so it does not appear
  to fit the same suspected pattern, but this was not independently exercised this session.
