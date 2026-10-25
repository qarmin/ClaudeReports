# Failure signature -> status

| Signature (prefix match) | Component | Status |
|---|---|---|
| `asan:unknown-crash:read_u32_mm`, `asan:SEGV:read_u32_mm`, `asan:*:read_chars_mm`, `asan:heap-buffer-overflow:read_chars_mm` | fuzz_index | 003 (mm reader trusts node contents) |
| `asan:stack-overflow:index_read`, `timeout:fuzz_index` | fuzz_index | 015 (cycles; branching-cycle exponential time is queued) and 014 (`last == 255`) |
| `diff:index-dump` | fuzz_index | 028 (value-order difference within a node and symptoms of 003/015; harness now compares sorted lines) |
| `invariant:strip-wrote-outside` | fuzz_elf | 002 (all 8 artifacts of the 10:48 run replay clean with 002's patch applied) |
| `invariant:strip-left-modversions` | fuzz_elf | not seen in the last two runs; queued in IDEAS as a possible harness assumption |
| `assert:libkmod/libkmod-elf.c:211`, `:212` | elf_sweep, fuzz_elf | 001 |
| `asan:leak:kmod_elf_get_modversions` | elf_sweep | 009 |
| `diff:compress-truncated-trailer-accepted-gz` | compress_diff | 008 (not a bug: data identical) |
| `asan:requested:kmod_file_load_zstd`, `libfuzzer:out-of-memory:kmod_file_load_zstd` | fuzz_modfile | 007 (ASAN allocator policy only) |
| `ubsan:tools/depmod.c:846` | cli_matrix | 005 |
| `model:modprobe-c-softdep` | config_model | 004 |
| `asan:heap-buffer-overflow:prepend_options_from_env` | fuzz_modprobe_env | 006 |
| `diff:modprobe-env-empty-token`, `-quote-glued`, `-other` | modprobe_env_diff | expected: the tokenizer predates shell-like semantics (empty tokens, adjacent quoted parts); behavior question, see IDEAS |
| `assert:tools/depmod.c:1902:*` | depmod_graph | 011 |
| `oom:modprobe-n:signal11`, `oom:modprobe-deps:signal11` | oom_sweep | 019 |
| `oom:*:hang` on compressed modules | manual sweep | 022 (OpenSSL init self-deadlock under OOM, not libkmod) |
| `diff:alias-glob-vs-fnmatch` (unrestricted generator) | alias_glob | by design, see 017 (bracket keys, `[]x]` classes, escapes, dash normalization) |
| `model:depmod-closure` | depmod_graph | none known (clean) |
| `asan:leak:kmod_elf_get_modversions` (also from fuzz_modfile `leak-*` artifacts, ~88 per run) | fuzz_modfile | 009 |
| `crash:fuzz_modfile` with an empty (0-byte) artifact | fuzz_modfile | testbed artifact: libFuzzer fork-mode child cut at `-max_total_time`, replay is clean; ignore |
| `diff:signature-signer-synthetic` | signature_diff | 029 |
| `oom:depmod-dense-cycle` (depmod_graph with k >= 20 in one SCC, RSS grows past the cap) | depmod_graph | 030 |
| `model:modprobe-order-duplicate` (modprobe -n -v list with duplicate modules when softdep names a hard dependency) | none yet | 032 |
| `regression:NNN:no-longer-reproduces` | regression | the HEAD under test fixed candidate NNN: verify with git log, write CORRECTION.md, fill Fixed by |
| `DIFF index-dump` with a NUL child character (WIP build, 2026-10-25) | fuzz_index | harness artifact: dumps containing NUL bytes cannot be line-sorted as C strings; comparison now skipped for them |
| `timeout:fuzz_index` / `oom` in index dump or wildcard walk | fuzz_index | 042 (shared children, no visit budget) |
| `diff:modprobe-env-*` (empty token, quote-glued, other) | modprobe_env_diff | 035/036 (empty tokens, quoting); the remaining shell-splitting differences stay a design question |
| `libfuzzer:file:write_str_safe@shared/util.c`, `asan:stack-overflow:index_mm_dump_node` | fuzz_index | 042 / 015 (dump of shared-children or cyclic index: output or recursion explodes) |
| `crash:fuzz_config` with a clean replay, `testbed:fuzz_elf:did-not-run` | fuzz_config, fuzz_elf | fork-mode artifacts (child cut at `-max_total_time`) / log regex noise; ignore |
