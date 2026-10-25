# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `meson`/`ninja` | Yes, but NOT via apt | Not installable via apt in the sandbox (no sudo, `apt-get update` fails with a permission error on `/var/lib/apt/lists`). Installed instead via `uv tool install meson`/`uv tool install ninja` into `~/.local/bin` (see "Environment quirks" below - this does NOT survive a container rebuild/restart, re-run the one-liner below if `meson`/`ninja` are missing). |
| clang ASAN/UBSAN | Yes (as of 2026-09-15) | Needed the `libclang-rt-21-dev` apt package (matches the container's `llvm-21`); before that package was added to the Dockerfile, clang could not link `-fsanitize=address` OR `-fsanitize=fuzzer` at all (missing `libclang_rt.asan*.a`/`libclang_rt.fuzzer*.a`). |
| gcc ASAN/UBSAN | No, for this project | gcc's ASAN/UBSAN runtime works fine in general, but building gstreamer's abseil-cpp subproject under `-fsanitize=address,undefined` with gcc 15.2 hits a real compiler/library incompatibility: `absl_flags_reflection.cc` fails with `'... == 0' is not a constant expression` inside `hash_policy_traits.h`'s `constexpr` evaluation - a gcc-15-vs-abseil-cpp-20250814.1 issue, not a gstreamer bug. Use clang instead (see `tools/build_asan_fuzz.sh`). |
| AFL++ (`afl-clang-fast`) | Builds fine, but **do not use `afl-fuzz` against `ci/fuzzing/*` targets** | `AFL_USE_ASAN=1` combined with `afl-clang-fast` was tried and produced a binary with ASAN symbols linked in but that did NOT actually catch a deliberately-planted stack-buffer-overflow (exit 0, where the identical source under plain `clang -fsanitize=address` aborted as expected) - not investigated further. Worse: real `afl-fuzz` runs against `ci/fuzzing/gst-value` produced ~400 crashes of the `gst-plugin-scanner` helper in a 20s burst that looked like a real registry-cache race - confirmed NOT reproducible under 60 genuinely concurrent OS processes (0/60 crashed), so it's an AFL-forkserver-vs-internally-forking-target artifact, not a bug. Full writeup: `~/.claude/skills/bug-hunt/projects/gstreamer.md`. **Use `tools/mini_fuzz.py` against the ASAN build instead** - it execs fresh per run, sidestepping this whole class of artifact. |
| libFuzzer (`-fsanitize=fuzzer`) | Works (link-tested), not wired into `ci/fuzzing/`'s meson.build | `ci/fuzzing/meson.build` only links a `FuzzingEngine` library (oss-fuzz's own harness convention) when `cxx.find_library('FuzzingEngine')` succeeds; locally that never succeeds, so it always falls back to `localfuzzer.c` (a driver that reads one file/dir from argv and calls `LLVMFuzzerTestOneInput` once - not a real fuzzing loop). That fallback binary's argv convention (`<binary> <file>`) is exactly what both `afl-fuzz` (classic mode) and this workspace's own `tools/mini_fuzz.py` expect, so it's used as-is rather than patching the project's meson.build to wire up real libFuzzer linking. |
| `miri` | N/A | This is a C project, miri doesn't apply. |
| `valgrind` | Present, not used this session | Available as a further check (esp. memcheck for uninitialized reads ASAN's redzones don't catch) if a candidate's ASAN run comes back clean but still looks suspicious. |

## Existing project harnesses worth reusing

- **`_reference/h266_harness/h266_fuzz.c`** (built by this session, NOT an upstream `ci/fuzzing/`
  target) - a standalone driver for `gsth266parser.c` (VVC/H.266), which has zero coverage from
  any existing `ci/fuzzing/*` target (their corpora are H264/H265/AV1 only) despite having the
  highest density of real, historically-confirmed bugs found anywhere in this codebase this
  session (see `018.h266parser_hardening_cluster_verify/RESULT.md` and
  `~/.claude/skills/bug-hunt/projects/gstreamer.md`'s risk map). Walks Annex-B NAL units via the
  parser's own public API (`gst_h266_parser_identify_nalu` + `gst_h266_parser_parse_nal`, plus
  `gst_h266_parser_parse_sei` for SEI NALs) and needs no `ci/fuzzing`/`oss_fuzz` build option -
  only `gst-plugins-bad:videoparsers` (already enabled in this workspace's scope) for
  `libgstcodecparsers-1.0`. Seed corpus at `_reference/h266_harness/corpus/*.bin`, extracted from
  the real VVC test vectors in `subprojects/gst-plugins-bad/tests/check/libs/h266parser.c` via
  `_reference/h266_harness/extract_h266_vectors.py` (a small regex-based
  `guint8 foo[] = {0x.., ...}` extractor - works on any similar C test-vector file, not just
  this one). Build:
  `PKG_CONFIG_PATH=../build-asan-fuzz/meson-uninstalled pkg-config --cflags/--libs
  gstreamer-codecparsers-1.0 gstreamer-1.0`, then plain `clang -fsanitize=address,undefined`.
- `ci/fuzzing/*.c` + `ci/fuzzing/meson.build` (behind `-Doss_fuzz=enabled`) - eleven real,
  already-written oss-fuzz-style targets: `gst-audio`, `gst-audio-converter`, `gst-codec-utils`,
  `gst-discoverer` (builds a real `GstDiscoverer`/decodebin pipeline - the deepest of the eleven,
  most likely to reach real demuxer/parser/decoder bugs), `gst-subparse`, `gst-tag`, `gst-uri`,
  `gst-value`, `gst-video`, `gst-video-converter`, `typefind` (runs every registered typefinder's
  `sniff()` against arbitrary bytes - cheap way to touch many demuxers' header-sniffing code at
  once). Seed corpora: `ci/fuzzing/*.corpus` (lists of in-repo files to zip) and
  `ci/fuzzing/seeds/gst-value/*` (hand-written GstValue serialization strings).
- `ci/fuzzing/build-oss-fuzz.sh` - the real oss-fuzz CI build script; this workspace's
  `tools/build_asan_fuzz.sh`/`build_afl_fuzz.sh` mirror its plugin-selection scope (a known-good,
  known-buildable subset: core+base+isomp4/matroska/flv/avi/audioparsers/wavparse/id3demux (good)
  +mpegtsdemux/videoparsers/jpegformat (bad)) but build UNINSTALLED (via `gst-env.py`, the
  project's own uninstalled-devenv helper) instead of oss-fuzz's install+patchelf dance, since
  there's no `$OUT`/oss-fuzz harness to hand the result to here.
- `gst-env.py` (project root) - sets up `GST_PLUGIN_PATH`/`LD_LIBRARY_PATH`/`PATH` for running any
  binary against an uninstalled meson build dir, avoiding a `ninja install` step entirely. Usage:
  `python3 gst-env.py --builddir=<dir> --only-environment > <dir>/env.sh && source <dir>/env.sh`.

## Instrumentation/tooling set up for this workspace

| what | why | script |
|---|---|---|
| `meson`/`ninja` via `uv tool install` | No apt access (no sudo) in the sandbox; `uv` was already present and works without root | one-liner below |
| ASAN+UBSAN build, clang, scoped plugin set | Primary tool for confirming/root-causing a crash with real sanitizer diagnostics | `tools/build_asan_fuzz.sh` |
| AFL++-instrumented build, same scope, no sanitizers | Coverage-guided fuzzing (execs/sec matters more than sanitizer precision here) - replay any crash found against the ASAN build above for diagnostics | `tools/build_afl_fuzz.sh` |
| `tools/mini_fuzz.py` | No afl-fuzz was available in the container at session start (fixed mid-session, see above) - this is a dependency-free mutation fuzzer (bit flips/chunk delete/insert/duplicate/splice) driving any `ci/fuzzing/*` binary directly, for when a quick one-off fuzz burst is wanted without the AFL++ output-directory ceremony, or on a target too small to bother setting up a real AFL run for | `tools/mini_fuzz.py` |

Environment quirks discovered this session (2026-09-15):

- **The container was restarted mid-session** (after `libclang-rt-21-dev`/`afl++` were added to the
  Dockerfile's append-only staging layer and the image rebuilt) - this reset `~/.local/bin`, so the
  `uv`-installed `meson`/`ninja` shims disappeared even though the underlying `uv tool` installs
  under `~/.local/share/uv/tools/` survived. Re-run `uv tool install meson && uv tool install ninja`
  (or `uv tool update-shell`) if `command -v meson` comes up empty after a restart - this is a
  disposable-`~/.local/bin` issue, not a real reinstall.
- A **plain `nohup bash script.sh &` background job does not source `~/.bashrc`**, so it doesn't see
  `~/.local/bin` on `PATH` even in a session where interactive `Bash` tool calls do. Both
  `build_asan_fuzz.sh` and `build_afl_fuzz.sh` now `export PATH="$HOME/.local/bin:$PATH"` themselves
  at the top for this reason - don't remove that line.
- **`afl-fuzz` needs `AFL_SKIP_CPUFREQ=1`** in this container (no root, so it can't set the CPU
  scaling governor to `performance` the way it wants to) - it hard-aborts with "Suboptimal CPU
  scaling governor" otherwise.
- **Warm the GStreamer plugin registry cache before the first `afl-fuzz`/timing-sensitive run**: a
  cold-cache run of any `ci/fuzzing/*` binary forks a `gst-plugin-scanner` helper that takes ~2.4s
  (confirmed 2026-09-15), which blows past afl-fuzz's default 40ms exec timeout on the dry run and
  aborts fuzzing outright ("results in a timeout"). Run the target binary once by hand first (any
  seed file, output discarded) - the registry then caches to `$XDG_CACHE_HOME/gstreamer-1.0/` and
  every subsequent run (including afl-fuzz's) drops to ~10-15ms.
- **Every `ci/fuzzing/*.c` harness already calls `g_log_set_always_fatal(G_LOG_LEVEL_CRITICAL)`
  itself** (confirmed 2026-09-15: `grep -l g_log_set_always_fatal ci/fuzzing/*.c` matches all 11) -
  any `g_critical()` (a violated `g_return_if_fail`-style internal invariant) already aborts the
  process on its own, once per fresh process (each `mini_fuzz.py` subprocess re-triggers this on
  first call, since it's guarded by a `static gboolean initialized` that resets per-process). No
  need to additionally force `G_DEBUG=fatal-criticals` yourself for these targets - it's already
  baked in by oss-fuzz's own harness design ("We want critical warnings to assert so we can fix
  them").
- **`ASAN_OPTIONS=detect_leaks=0`** should be set for any `ci/fuzzing/*` run: the plugin-scanner
  helper (same one as above) leaks ~500 bytes through fontconfig/pango one-time init on its first
  invocation, unrelated to the fuzzed input - `tools/mini_fuzz.py` already sets this by default,
  set it yourself too for a manual/afl-fuzz run (`AFL_PRELOAD`/env passthrough both work).
- **`ci/fuzzing/gst-discoverer`'s own internal timeout is 10s** (`gint timeout = 10;` inside its
  `LLVMFuzzerTestOneInput`, waiting for `GstDiscoverer` to finish or give up on a pipeline) -
  `mini_fuzz.py`'s default `--timeout 5` subprocess kill is shorter than that, so it flags every
  slow-but-not-actually-hung input as a false "crash" (bucket key `<timeout>`, exit code -9999).
  Confirmed 2026-09-15: the very first `<timeout>` "crash" this produced was exactly this artifact,
  not a real hang. Use `--timeout 15` (or more) specifically for `gst-discoverer`.
- **UBSAN is deliberately non-halting** (`UBSAN_OPTIONS=print_stacktrace=1`, no `halt_on_error`):
  even with the build's own `-fno-sanitize=function`, GST_READ_UINT32-style direct-cast buffer
  reads trip UBSan's alignment checks constantly on ordinary, correct-on-x86 code throughout
  `gstpluginloader.c` and typefind functions - halting on those would abort every run on
  boilerplate before the fuzzed input is meaningfully exercised. Real ASAN memory-safety errors
  still abort on their own regardless of this setting.

One-liner to restore `meson`/`ninja` after a container restart:
```
export UV_TOOL_DIR="$HOME/.local/share/uv/tools" UV_TOOL_BIN_DIR="$HOME/.local/bin"
uv tool install --force meson && uv tool install --force ninja
```

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
5. Per `AGENTS.md` at the project root: nothing from this workspace is ever posted to GitLab, and
   no MR/issue/commit-message text is AI-authored - this workspace is a local investigation record
   for a human to act on, not something submitted anywhere automatically.
