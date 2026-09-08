# linker_script_parser_token_indexing

`Script<E>`'s recursive-descent parser methods in `src/linker-script.cc` consume a
`std::span<std::string_view> tok` by direct indexing (`tok[0]`, `tok[1]`, `tok[3]`) and
`tok.subspan(N)` advancement. Candidate 025 ruled the *tokenizer* safe and handed this on as the
next lead: the hypothesis was that a truncated script could leave fewer tokens than a directive's
handler unconditionally indexes, producing an out-of-range `span::operator[]`.

**That specific hypothesis did not hold - every `tok[N]`/`subspan()` site in the file is guarded.**
Sweeping malformed scripts through the parser to prove that, however, turned up a different,
fully reproducible crash in the same methods: the parser has no recursion depth limit, so a
malformed (or, in one case, a 15-byte and entirely well-formed-looking) linker script exhausts
mold's stack and kills it with SIGSEGV.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read every `Script<E>` method end-to-end and enumerate all `tok[N]` / `tok.subspan(N)` sites, checking each against its guard | code reading only, current source at c340272e (41 `tok[` reads, 22 `subspan()` calls) | **Every one of them is guarded.** Full enumeration in "Root cause" below. No unguarded site found - the assigned hypothesis is refuted by construction, not just by testing |
| Sweep 33 truncation/malformation shapes through the linker-script entry point (`mold -o out a.o script.ld`) | plain Debug build (`-DCMAKE_BUILD_TYPE=Debug`, `_GLIBCXX_ASSERTIONS` active), `scratch/sweep.sh` | Every shape produced a clean `mold: fatal: ...` and exit 1. No SIGABRT from the hardened `operator[]`, no SIGSEGV. Corroborates the code reading |
| Same shapes through the other two entry points into the same parser, `--version-script` (`parse_version_script`) and `--dynamic-list` (`parse_dynamic_list`), which bypass `filetype.cc`'s TEXT sniffing entirely | same build, `scratch/sweep2.sh`, 14 + 8 shapes | All clean `Fatal` diagnostics. Confirms the guards hold on the version-script/dynamic-list halves of the parser too, not just the `GROUP`/`INPUT`/`OUTPUT_FORMAT` half |
| Pivot: probe the parser's *recursion* instead of its indexing - nested `AS_NEEDED(`, nested `extern "C" {`, and scripts that `INPUT()` themselves | same build, `scratch/sweep3.sh` | **5 distinct SIGSEGVs (exit 139)**, across all four recursive sites in the file |
| `gdb` with `set follow-fork-mode child` (mold forks a child to do the link) to identify each crash's actual mechanism | same build | Stack exhaustion confirmed for all: 2997 consecutive `Script<E>::read_group` frames in the first 3000 for the nested-`AS_NEEDED` case; 2992 `read_version_script_commands` frames for the version-script case; an alternating `get_machine_type` / `get_script_output_type` column for the self-including-script case |
| Binary-search the nesting depth at which the crash starts, this build | same build, `scratch/threshold.sh` | `read_group`: safe at 12000, SIGSEGV at 16000. `read_version_script_commands`: safe at 16000, SIGSEGV at 100000 (larger frames in `read_group`) |
| Differential check against GNU ld (`/usr/bin/ld.bfd`) on the identical inputs | same inputs | Nested `AS_NEEDED`: GNU ld prints `deep.ld:1: memory exhausted` and exits 1 - a clean error where mold segfaults. Self/mutually-including scripts: GNU ld does not crash but does not terminate either (killed by `timeout 60`), so that particular shape is mishandled by both linkers, differently |
| Repeat-run the standalone `repro/` against the pristine build | 3 full repro runs (5 crash cases each) + a separate 5-run loop on the 15-byte self-including script | 3/3 and 5/5, every case crashing every time. Fully deterministic |
| Write the depth-limit patch, rebuild, re-run repro + 10 extra same-class inputs + 7 valid-script inputs | same build dir rebuilt | All 5 crash cases and all 10 extra malformed inputs now rejected with a clean `Fatal`; all 7 valid scripts (including 998 legally-nested `AS_NEEDED` levels) still link |
| Project test suite before and after | `ctest --test-dir build-plain -j4` | 476/476 before, 476/476 after (includes the project's own `linker-script*.sh`, `linker-script-group-as-needed.sh`, `symbol-version*.sh`, `dynamic-list*.sh` tests) |

## Status

**Confirmed** - but for a different mechanism than the one assigned. Explicitly:

- The assigned suspect (**out-of-range `tok[N]` / `subspan()` indexing**) is **Not reproducible**;
  see the full site-by-site enumeration under "Root cause". A future session should not re-suspect
  this.
- The crash actually found (**unbounded recursion -> stack exhaustion**) reproduces **3/3** full
  repro runs (all five cases crashing in every run) on the plain Debug build, plus **5/5** on a
  dedicated repeat loop for the 15-byte self-including-script case. Deterministic, no seed or
  layout dependence.

Build dependence: this is a stack overflow, so it is not Debug-only - a Release build has smaller
frames and simply needs deeper nesting for the two input-depth-driven cases. The two
self/mutually-including-script cases recurse *without bound* (the input does not encode a finite
depth) and therefore crash in any build.

**Fully reproducible**: **Yes**. `repro/run.sh` runs the real `mold` binary from the command line
on plain text files - which is exactly the external artifact a real user/attacker supplies. There is
no second participant being stood in for (CONFIRMATION.md section 11 does not apply).

## Class

crash

## Root cause

### Part 1 - why the assigned suspect is safe (the negative result)

Every span-indexing site in `src/linker-script.cc` at c340272e, and what guards it:

| Site | Guard |
|---|---|
| `skip()` `tok[0]` (:101) | explicit `if (tok.empty()) Fatal(...)` at :99 |
| `read_output_format()` `tok[0]` (:118), `subspan(1)` (:119, :122) | `!tok.empty() &&` in the loop condition; explicit `tok.empty()` check at :120 before the final `subspan(1)`; the leading `skip()` guarantees >= 1 element consumed or `Fatal` |
| `read_group()` `tok[0]` (:193, :194, :202), `subspan(1)` (:197, :205, :210) | `!tok.empty() &&` in the loop condition; explicit `tok.empty()` check at :208 before the final `subspan(1)` |
| `parse_linker_script()` `tok[0]` (:219, 221, 223, 232, 235), `subspan(1)` | `while (!tok.empty())` at :218 |
| `parse_linker_script()` defsym branch `tok[1]`, `tok[3]`, `subspan(4)` (:228-231) | `tok.size() > 3 &&` short-circuits first, so size >= 4 |
| `get_script_output_type()` `tok[0..2]` (:245-249) | `tok.size() >= 3 &&` short-circuits first |
| `get_script_output_type()` `tok[0..3]` + `subspan(2)` + `tok[2]` (:252-256) | `tok.size() >= 3` for `tok[0]`/`tok[1]`; the `AS_NEEDED` unwrap is gated on `tok.size() >= 5`, so after `subspan(2)` size is still >= 3 and the `tok[2]` at :256 is in range on both paths |
| `read_label()` `tok[0]`, `tok[1]` (:263, :268) | `tok.size() >= 1` / `tok.size() >= 2` short-circuit first |
| `read_version_script_commands()` `tok[0]` (:280, 291, 309), `subspan(1)` | `while (!tok.empty() && ...)` at :280; the two mid-body reads at :294 and :321 are each guarded by their own `!tok.empty() &&` |
| `read_version_script()` `tok[0]` (:333, 337, 341, 343, 350), `subspan(1)` | `while (!tok.empty() && ...)` at :333; the :350 read has its own `!tok.empty() &&` |
| `parse_version_script()` `tok[0]` (:363) | `if (!tok.empty())` at :362 |
| `read_dynamic_list_commands()` `tok[0]` (:371, 372, 390), `subspan(1)` (:391) | `while (!tok.empty() && ...)` at :371; the :375 read has its own `!tok.empty() &&` |
| `parse_dynamic_list()` `tok[0]` (:408) | `if (!tok.empty())` at :407 |

Every recursive/looping path also makes forward progress (each iteration either consumes >= 1
token via `subspan` or goes through `skip()`, which consumes 1 or `Fatal`s), so there is no
infinite-loop variant of this either. The empirical sweep (Part 0 of `output.txt`, re-runnable via
`repro/sweep_token_indexing.sh`) agrees: 62 malformed inputs across all three entry points, zero
crashes.

### Part 2 - the crash that is actually there

The parser is recursive descent with **no depth limit anywhere**, and four call graphs in it can
recurse to a depth the input controls, or without any bound at all:

1. **`Script<E>::read_group()` (:194-197)** - on seeing `AS_NEEDED` it calls itself. `GROUP(`
   followed by N copies of `AS_NEEDED(` produces N nested frames. On this Debug build the stack
   dies somewhere between 12000 and 16000 levels, i.e. a ~200 KB text file.

2. **`Script<E>::read_version_script_commands()` (:296-301)** and
   **3. `Script<E>::read_dynamic_list_commands()` (:377-382)** - both call themselves for each
   `extern "C" {` / `extern "C++" {`. Same shape, reachable through `--version-script` and
   `--dynamic-list` (which do not go through the file-type sniffer at all).

4. **`Script<E>::get_script_output_type()` <-> `get_machine_type()`** - this one is a *mutual*
   recursion across two files and needs no deep input at all:

   - `get_script_output_type()` (`linker-script.cc:252-257`) resolves the first file named by a
     `GROUP()`/`INPUT()` directive and calls `get_machine_type(ctx, rctx, mf)` on it;
   - `get_machine_type()` (`filetype.cc:204-205`) dispatches `FileType::TEXT` straight back to
     `Script(ctx, rctx, mf).get_script_output_type()`.

   So a linker script whose `INPUT()`/`GROUP()` names *itself* - or two scripts naming each other -
   makes the two functions call each other forever until the stack runs out. There is no
   already-visited set anywhere on this path, and each level re-`mmap`s the file and re-tokenizes
   it. The triggering file is 15 bytes:

   ```
   INPUT(self.ld)
   ```

   The same cycle is also reachable from `read_group()` -> `resolve_path()` -> the
   `check_target` branch's `get_machine_type()` call (`linker-script.cc:145`), so it fires whether
   or not `-m` was given on the command line.

In every case the process dies with a bare SIGSEGV (exit 139) with no diagnostic - the fault
surfaces wherever the guard page happens to be touched first (mimalloc's allocator, in the captured
backtrace), which is why the crash location looks unrelated to the parser until you read the frames
below it.

Reachability is the same as candidate 025 established for the tokenizer: any file argument on the
command line that `filetype.cc`'s `is_text_file()` accepts (>= 4 printable bytes), plus any path
named inside an already-open script's `GROUP()`/`INPUT()`, plus `--version-script` and
`--dynamic-list` arguments. Not reachable from a text member smuggled inside a `.a` archive.

## Introduced by

Two different commits for two of the sites; both best-effort via `git log -S` on the specific
constructs.

- The `read_group()` `AS_NEEDED` self-recursion has been there since the file's very first commit:
  fde6b02d "Add linker_script.cc" (2020-11-19) already contains
  `if (tok[0] == "AS_NEEDED") { tok = read_group(tok.slice(1)); continue; }`. Not a regression.
- The `get_script_output_type()` <-> `get_machine_type()` cycle (the 15-byte self-including-script
  case) was introduced by cf8cce6d "-r: Infer machine type even if all input files are given via
  linker script" (2022-11-28), which is the commit that added the `INPUT`/`GROUP` branch ending in
  `return get_machine_type(ctx, rctx, mf);`. Before it, `get_script_output_type` only looked at
  `OUTPUT_FORMAT` and could not recurse (verified against a2017e5d, the commit that first added the
  function).
- The two `extern`-nesting recursions in the version-script/dynamic-list readers were not traced
  individually - they are the same construct and were not worth a separate archaeology pass.

## The fix

`patch.diff` adds one shared recursion-depth counter and checks it at all four recursive entry
points.

- `src/mold.h`: a new `i64 script_depth = 0;` field on `ReaderContext`. `ReaderContext` was chosen
  over a `Script` member because case 4's recursion crosses `Script` objects (a fresh `Script` is
  constructed per file by `get_machine_type`), and over a global/`static` because
  `get_machine_type` can run on several TBB worker threads at once via
  `find_library`/`open_library` - each job owns its own `ReaderContext`, so a field on it is both
  shared where it needs to be (the reference `Script` holds, and the copy `next_child()` makes for
  an included file) and unshared across threads. No locking needed.
- `src/linker-script.cc`: a file-local RAII `ScopedDepth` and `MAX_SCRIPT_DEPTH = 1000`, with an
  entry check in `read_group`, `get_script_output_type`, `read_version_script_commands` and
  `read_dynamic_list_commands`. The diagnostic follows the file's existing idiom
  (`Fatal(ctx) << mf->name << ": ..."`, the same form `skip()` uses for its EOF errors).

Why 1000 is the right order of magnitude: measured crash onset on this Debug build is between
12000 and 16000 levels for the tightest (largest-frame) site, `read_group`, so 1000 leaves a >10x
margin on the build with the *biggest* frames; and real scripts nest 1-2 levels
(`GROUP(libc.so.6 libc_nonshared.a AS_NEEDED(ld-linux-x86-64.so.2))` is depth 2). The repro
verifies that 998 legally-nested, properly-closed `AS_NEEDED` levels around a real object file
still link successfully.

Why it is sufficient rather than input-specific (CONFIRMATION.md section 7): it was checked against
10 further inputs in the same class beyond the 5 that first crashed - `AS_NEEDED` at 16000/100000/
400000, `extern "C"` at 100000/300000 and `extern "C++"` at 60000, `--dynamic-list` at 100000, a
self-include routed through `AS_NEEDED` instead of a bare `INPUT`, a self-include with `-m
elf_x86_64` given (which takes the `read_group` -> `resolve_path` path into the cycle instead of the
`detect_machine_type` path), and a 1200-file chain of scripts each `INPUT()`ing the next. All ten
are rejected cleanly; none crash.

- Project test suite: **476/476 passing before -> 476/476 passing after** (`ctest --test-dir
  build-plain -j4`, 0 failures either way; 16 tests skipped by ctest for missing
  zstd/hwasan/apx/etc. features, identically in both runs). This includes the project's own
  linker-script tests (`linker-script.sh` through `linker-script6.sh`,
  `linker-script-group-as-needed.sh`, `linker-script-defsym.sh`, `linker-script-error.sh`,
  `linker-script-relocatable.sh`, `linker-script-version.sh`), the version-script tests
  (`symbol-version*.sh`, `invalid-version-script.sh`, `default-symver-version-script.sh`) and the
  `dynamic-list*.sh` tests.

The patch was reverted from the worktree after `patch.diff` was captured; the tree is back at
pristine c340272e.

## What's not done / open questions

- **A maintainer decision the patch encodes**: 1000 is a hard cap, so a 1200-deep chain of scripts
  that each `INPUT()` the next is now rejected rather than linked. That is intentional (nothing real
  is anywhere near it) but it is a behavior change, not purely a crash fix, and the number is worth
  a maintainer's eye. An alternative design for case 4 specifically would be a visited-file set
  instead of a depth cap, which would let a legitimate deep chain through while still catching
  cycles - more code, and it would not help cases 1-3 at all, so the depth cap was chosen as the
  single mechanism covering all four.
- **The `get_script_output_type()` <-> `get_machine_type()` cycle is a genuine infinite recursion,
  not merely a deep one.** A depth cap turns it into a clean error, which is enough to stop the
  crash, but the underlying "mold will happily re-open and re-tokenize the same script forever" is
  arguably a separate design gap worth its own look. Note GNU ld also fails to terminate on this
  input (it hangs rather than crashing), so there may be no established correct behavior to copy.
- **Unrelated pre-existing quirk noticed while testing, not a crash, not investigated further**:
  `get_script_output_type()` (`linker-script.cc:254-256`) unwraps only *one* level of `AS_NEEDED`
  before calling `resolve_path(tok[2])`, so for `GROUP(AS_NEEDED(AS_NEEDED(foo.o)))` with no `-m`
  on the command line it tries to open a library literally named `AS_NEEDED` and dies with
  `library not found: AS_NEEDED`. Confirmed present on the pristine build (i.e. not caused by this
  patch) and not fixed here since it is a different defect (wrong output, not a crash). Whoever
  looks at cf8cce6d/e0a6f8e1 next may want to fold this in.
- **`MappedFile *mf = mf;` at `src/mold.h:2199`** - `Script`'s default member initializer
  self-initializes. It is dead code today (the class's only constructor initializes `mf` in its
  member-init list, which overrides the NSDMI), so there is no reachable UB, but it is a typo that
  would become a real uninitialized read the moment anyone adds a second constructor. Left alone as
  out of scope.
- No fuzzing was run against this parser. A short libFuzzer/AFL harness over `Script<E>::tokenize`
  plus the four `parse_*` entry points would be cheap (the input is plain text, no ELF crafting
  needed) and is the obvious next step if anyone wants coverage beyond the ~65 hand-written inputs
  used here. This session's coverage was: 62 truncation/malformation shapes across all three entry
  points (`repro/sweep_token_indexing.sh`, re-runnable), plus ~20 nesting-depth shapes - all
  hand-written, none generated.
