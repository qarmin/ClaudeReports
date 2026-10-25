# Ideas

Unchecked suspects only - see `SUMMARY.md` for what's already been investigated.

Last GitHub issue/PR checked for inspiration: **#475** (repo: kmod-project/kmod)

## Backlog (not yet investigated)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Suspect | Why it looks suspicious | Source |
|---|---|---|---|
| 2026-10-25 | testbed `invariant:strip-left-modversions` (fuzz_elf) and `strip-wrote-outside` | the second is 002's strlen class; the first (`kmod_elf_strip` with FORCE_MODVERSION leaves `__versions` allocated for sections the sh_flags clear does not reach?) needs a minimal input and a check whether it is only a harness assumption on invalid sections. | testbed:fuzz_elf |
| 2026-10-25 | upstream #286: module names containing `/` (lsmod/rmmod/holders path building) | `kmod_module_get_holders`/sections build `/sys/module/<name>/...` without the `!` mangling; needs a /sys fixture, only the path construction can be tested without root. | issue #286 |
| 2026-10-25 | `libkmod-signature.c` PKCS#7 trailer: openssl `BIO_new_mem_buf(pkcs7_raw, sig_len)` takes an `int`, `sig_len` is `size_t` (file > 2 GiB) | only matters for gigantic modules; low plausibility, check with a sparse-file `.ko`. | code reading |
| 2026-10-25 | upstream #33 (multiple `softdep` stanzas, only the first honored) | `kmod_module_get_softdeps()` breaks after the first match on purpose ("as modprobe from module-init-tools does"); a design question rather than a defect, PR #459 changes it. Not pursued. | issue #33 |
| 2026-10-25 | depmod: unreadable subdirectories / unsupported file types are logged and skipped with exit status 0 (`depmod.c` ~1375-1400 `continue` after ERR) | same swallowed-error shape as 023 but may be intended best-effort behavior; check module-init-tools semantics. | code reading (023 follow-up) |
| 2026-10-25 | `depmod -n` (stdout) ignores writer return codes (`if (fp == out) continue;` before checking `r`) | errors from `itr->cb()` are lost in dry-run mode; low impact. | code reading |
| 2026-10-25 | `sleep_until_msec()` (shared/util.c): `clock_nanosleep(...) < 0 && errno != EINTR` can never be true, the function returns the error number directly | error check is dead code; no observable effect found except possibly `modprobe -r --wait` with an invalid timespec; also `tend = now + wait` wraps for huge `--wait` values (timeout reported as already reached). Needs a loaded module to exercise. | code reading, session 4 |
| 2026-10-25 | depmod exits 0 although it logged ERROR for unreadable/corrupt/truncated modules and still lists them (without dependencies) in modules.dep | same swallowed-error shape as 023, probably intended best-effort; check module-init-tools semantics before calling it a bug | session 4 probe |
| 2026-10-25 | `kmod_new()` failure path after `kmod_config_new()` succeeded leaks `ctx->config` (hash_new failure) | OOM-only leak; needs a leak-checking allocation-failure run (LSAN does not combine with the LD_PRELOAD shim) | code reading, session 4 |
| 2026-10-25 | depmod `CRIT()` (tools/log.c exits inside `log_printf`) in `index__checkstring` for a non-ASCII alias/symbol: aborts mid-publish, leaves `tmpfileXXXXXX` in the module dir and a mix of old/new index files | partly by design (7-bit ASCII only), but the litter/inconsistent set is avoidable by validating before writing | session 4 probe |
| 2026-10-25 | `kmod_module_new_from_path()` error branch uses `m->path` after `kmod_module_unref(m)` (gcc -fanalyzer) | not reachable today (another holder keeps the module alive), fragile ordering | gcc -fanalyzer |
