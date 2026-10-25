# Logger tests null-deref (SIGSEGV) when the user data dir can't be created

Found incidentally while setting up a `perf-optimize` session (not from a dedicated bug-hunt) -
establishing the `--headless --test` baseline in a Docker container where `$HOME/.local/share`
is mounted read-only crashed the whole test binary instead of just failing the affected tests.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Run full `--test` suite in this session's actual Docker environment (`$HOME` read-only except `$HOME/.cache`) | `scons target=editor tests=yes` build, `bin/godot.linuxbsd.editor.x86_64 --headless --test` | SIGSEGV in `tests/core/io/test_logger.cpp`, whole binary aborts (exit via signal) instead of reporting a clean test failure |
| Isolated repro: fresh `mktemp -d` HOME with `.cache` writable, `.local/share` `chmod 555` | `--test --test-case="*Logger*"` | Reliably reproduces the same SIGSEGV, 1/1 runs - see `repro/run.sh` |
| Naive repro attempt: `chmod -R 555` on the whole fake HOME (including `.cache`) | same | Does NOT reproduce - `tests/test_main.cpp`'s own temp-dir setup (`TestUtils::get_temp_path`, which uses `$HOME/.cache`, not `.local/share`) fails first and the binary exits gracefully via `ERR_FAIL_COND_V` before reaching any `TEST_CASE` - this is why the repro script deliberately only restricts `.local/share` |
| Read `core/os/os.cpp` (`OS::ensure_user_data_dir`) and `platform/linuxbsd/os_linuxbsd.cpp` (`get_data_path`/`XDG_DATA_HOME`) | code reading | Confirmed `user://` maps to `$HOME/.local/share/godot/app_userdata/<project>` by default, created via `DirAccess::make_dir_recursive_absolute` whose return value is not checked by the test file |

## Status

**Confirmed** - reproduces deterministically, 1/1 runs (and was hit unintentionally, not
deliberately fuzzed, in this session's own container before being isolated).

**Fully reproducible**: **Yes**. `repro/run.sh` builds nothing itself (it takes a pre-built
`tests=yes` binary as its argument, since a from-scratch Godot build takes 15-25 minutes) and
drives the real compiled binary through its real `--test` command-line entry point end-to-end -
no faked internals, no second participant involved.

## Class

`crash` (SIGSEGV via null-pointer dereference on a `Ref<T>`).

## Root cause

`tests/core/io/test_logger.cpp` has three `TEST_CASE`s that assume `user://logs/...` can always
be created and opened for writing/reading. When it can't (the directory tree under `user://` -
`$HOME/.local/share/godot/app_userdata/<project>` on Linux - is read-only or otherwise
uncreatable), the chain is:

1. `initialize_logs()` calls `DirAccess::make_dir_recursive_absolute(...)` and **discards the
   `Error` return value**, so the failure is silent at this point.
2. `RotatedFileLogger`'s constructor/`logf` then fails to open the log file for writing
   (logged as `ERROR: Failed to open log file for writing`), also non-fatal by itself.
3. The test then does `Ref<FileAccess> log = FileAccess::open(...)`, correctly captures `err` and
   `CHECK_EQ(err, Error::OK)` (which fails cleanly, as expected) - but the very next line
   unconditionally dereferences the same `Ref` (`log->get_as_text()`) with no `is_valid()`/
   `is_null()` guard. `FileAccess::open` returns a null `Ref` on failure, so this is a
   straightforward null-pointer dereference.
4. The same unguarded-dereference pattern also exists on `Ref<DirAccess> dir =
   DirAccess::open("user://logs")` in `cleanup_logs()` and `get_log_files()` - both call
   `dir->list_dir_begin()` immediately with no validity check. Fixing only the first `FileAccess`
   case (test case 1) was not sufficient: the crash simply moved to `cleanup_logs()`'s
   `dir->list_dir_begin()`, then (once that was also guarded) to test case 3's second unguarded
   `FileAccess` open - confirmed by re-running the repro after each incremental fix and watching
   the crash relocate to the next unguarded call site in the same file.
5. **Notable secondary finding**: the first fix attempt changed the first `CHECK_EQ(err, ...)` to
   `REQUIRE_EQ(err, ...)`, expecting doctest's `REQUIRE` to abort the test case before reaching the
   null-deref line, the same way `tests/core/io/test_image.cpp` relies on `REQUIRE(f_bmp.is_valid())`
   elsewhere in this codebase. **It did not help - the crash still happened at the exact same
   line.** Godot's build disables C++ exceptions by default (`disable_exceptions=True` in
   `SConstruct`, passing `-fno-exceptions`), and doctest's `REQUIRE` macros normally rely on
   throwing to unwind out of the test case; without exceptions available, `REQUIRE` degrades to
   behaving like `CHECK` (log the failure, keep executing the rest of the test case). This means
   `REQUIRE`/`REQUIRE_EQ` do **not** actually guard against a following statement executing in this
   project's default (`disable_exceptions=yes`) build - only an explicit `if (x.is_valid())` guard
   (or an explicit early `return`) actually stops execution. This is a broader, project-wide
   caveat about the effectiveness of `REQUIRE*` under `disable_exceptions=yes`, not something
   scoped to just this file - see "What's not done" below.

## Introduced by

not determined - `git log -S"get_as_text" -- tests/core/io/test_logger.cpp` and `git blame` were
not run to find the exact introducing commit this session (this file appears to date back close
to when the test itself was added); not worth the time for a test-only robustness gap versus a
production-code bug.

## The fix

Guarded every unchecked `Ref<FileAccess>`/`Ref<DirAccess>` dereference in this file with an
`is_valid()` check before using it - `cleanup_logs()`, the first `TEST_CASE`'s `log->get_as_text()`,
`get_log_files()`, and the `CompositeLogger` `TEST_CASE`'s two `log->get_as_text()` calls. See
`patch.diff`. Deliberately did **not** touch the two `REQUIRE_EQ(err, Error::OK)` sites in the
"Rotates logs files" `TEST_CASE` (lines ~120/141 in the original file) - those loops only execute
when `log_files.size() == number_of_files`, which is already false (0) whenever directory access
fails, so they're not reachable under this bug's trigger condition and were left alone to keep the
patch minimal and scoped to the confirmed crash.

- Project test suite (`--headless --test`, `XDG_DATA_HOME` pointed at a writable dir so the normal,
  intended case is exercised): **1427 test cases, 1426 passed, 1 failed, 3 skipped** both before and
  after this patch - identical (the one pre-existing failure is `tests/core/io/test_ip.cpp`'s IPv6
  loopback check, an unrelated environment limitation of this Docker container having no IPv6, see
  `___OPTIMIZATIONS___/README.md`). No new failures, no newly-passing tests.
- Targeted repro (`repro/run.sh`, `$HOME/.local/share` unwritable): before the patch, SIGSEGV on the
  very first `TEST_CASE`; after the patch, all 3 `Logger` test cases fail cleanly with normal
  doctest `CHECK_EQ`/`ERROR` output (exit status 1, no crash) - see `repro/output_before.txt` and
  `repro/output_after.txt`.

## What's not done / open questions

- The `REQUIRE`-doesn't-abort-under-`disable_exceptions=yes` behavior (point 5 above) is a
  Godot-project-wide caveat, not specific to this file. Other `TEST_CASE`s that use
  `REQUIRE(x.is_valid())` as their *only* guard before dereferencing `x` (e.g.
  `tests/core/io/test_image.cpp`) could have the same latent crash-instead-of-clean-failure
  behavior under a build with `disable_exceptions=yes` (the default) if their own preceding
  condition can actually fail in some environment - this was not audited codebase-wide this
  session; flagging it as worth a targeted search (`grep -rn "REQUIRE(" tests/` followed by
  checking what's dereferenced immediately after each hit) rather than doing that here, since it's
  a much larger, separate investigation than the one crash this candidate confirms and fixes.
- Whether Godot's non-test production code has any analogous "trust `Ref<T>` is valid right after
  `FileAccess`/`DirAccess::open` without checking" spots reachable from a real user-facing code path
  (not just this test file) was not checked - this session only looked at `tests/core/io/
  test_logger.cpp` itself, prompted by the crash actually being hit, not a deliberate sweep.
- `git log -S`/`git blame` for "Introduced by" was not run (see above) - low priority for a
  test-only fix.
