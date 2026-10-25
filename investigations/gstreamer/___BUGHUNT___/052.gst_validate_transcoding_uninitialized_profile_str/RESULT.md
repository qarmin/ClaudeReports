# gst_validate_transcoding_uninitialized_profile_str

`gst-devtools/validate/tools/gst-validate-transcoding.c`'s `real_main()` declares `const gchar
*profile_str;` with no initializer. It is only ever assigned by GOption if `-o`/`--output-format`
is passed on the command line; otherwise the code falls straight through to `if (profile_str ==
NULL)` at line 354, reading and branching on whatever garbage happened to be on the stack.
Flagged by `findings.toml` (`PANIC_6`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Build the real `gst-validate-transcoding-1.0` tool (scoped meson build: core+base+bad+devtools, plain debug, clang not required) and run it under `valgrind --track-origins=yes` with two dummy positional URIs and no `-o` flag | `___BUGHUNT___/tools/../..` root, `meson setup --buildtype=debug -Dbase=enabled -Dbad=enabled -Ddevtools=enabled` (rest disabled), `valgrind` on the resulting binary | **Reproduces**: valgrind reports "Conditional jump or move depends on uninitialised value(s)" at `gst-validate-transcoding.c:354`, origin traced to the stack allocation at line 227 (the `real_main` frame). The garbage `profile_str` pointer is then passed into `gst_transcoder_new()` -> `create_encoding_profile()` -> `gst_value_deserialize()` -> `g_strsplit()` -> `strstr()`, which reads through it as a C string ("Use of uninitialised value of size 8" at `strstr`), 41 distinct valgrind findings across the one run, all rooted at the same uninitialised stack allocation. |

## Status

**Confirmed** - reproduces 3/3 runs, identical valgrind report each time (deterministic stack
layout for this simple, single-process invocation; no ASLR-dependent variation observed across
repeats).

**Fully reproducible**: **Yes**. `repro/run.sh` builds and runs the real, unmodified
`gst-validate-transcoding-1.0` binary via its real CLI entry point (`main()` -> `real_main()`)
exactly as a genuine user/CI invocation without `-o` would - no synthetic harness stands in for
`real_main()`, and no second participant is involved in this finding (it's a pure CLI-argument-
handling bug, not a reaction to another party's input).

## Class

UB (use of an uninitialized/indeterminate value).

## Root cause

`profile_str` is read at `gst-validate-transcoding.c:354` (`if (profile_str == NULL)`) before it
has ever been written, whenever the `-o`/`--output-format` option is absent from `argv`. Because C
gives no default value to an uninitialized automatic-storage-duration pointer, its value is
whatever bit pattern happened to already be on the stack in that frame. In this observed build/
platform that garbage pointer is non-NULL (so the "use the default profile" fallback at line 357
is skipped) but still not a valid pointer to a NUL-terminated string, so it's later dereferenced as
if it were one, all the way down into `strstr()`. In the specific instance captured here the
process did not crash - the garbage happened to look enough like non-string bytes that
`gst_value_deserialize()`/`create_encoding_profile()` failed gracefully with "No way to create a
profile for description: <garbage bytes>" - but this is exactly the kind of undefined behavior
whose actual effect (silent misbehavior vs. a segfault vs., in principle, something worse) depends
entirely on uncontrolled stack contents, which is the reason it's flagged as a real bug rather than
dismissed as "happened to work."

## Introduced by

ab98bd242ed65e2926b51fefb5c20d8e3a178cc3 "qa-transcoding: Add a binary program to easily test
transcoding" (2013-07-18) - the file's original commit; `profile_str` has never had an initializer
in its history (`git log -S'profile_str'` on this file only turns up a later bulk subproject-move
commit, not a real content change).

## The fix

Initialize `profile_str = NULL;` at its declaration (matching the pattern already used for the
sibling `gchar *output_file = NULL;` two lines above it). This makes the `if (profile_str == NULL)`
check at line 354 well-defined and lets the intended "use the default profile" fallback actually
trigger when `-o` isn't passed, instead of depending on uninitialized stack contents.

Verified: rebuilt with the one-line fix and re-ran `repro/run.sh` against the same binary/build -
valgrind now reports 0 errors (0 uninitialised-value findings), and the tool's own log shows it
correctly fell back to the default profile string (`video/x-theora` visible in the "No way to
create a profile for description: video/x-theora" error, vs. garbage bytes before the fix). The
tool still exits non-zero in this environment because the default theora/vorbis profile can't
actually be created here (a separate, pre-existing environment/dependency issue unrelated to this
bug, not investigated further) - the fix under test is specifically the elimination of the
uninitialized-value read, which is fully verified.

- Project test suite: `meson test -C build-bad-devtools 'gstreamer:validate.simplest'` - 1/1 passed
  before and after (this is the only meson test target that exercises validate scenario execution
  in this scoped build; there is no dedicated unit test for `gst-validate-transcoding.c`'s CLI
  argument handling itself in either state).

## What's not done / open questions

- Not checked whether any other CLI tool in `gst-devtools/validate/tools/` (e.g.
  `gst-validate-1.0` itself) has the same "declare without initializer, conditionally assign via
  GOption, branch on NULL" pattern for one of its own options - this candidate only covers
  `gst-validate-transcoding.c`.
- The actual downstream effect of the garbage pointer (segfault vs. graceful failure vs. something
  else) is inherently stack-content-dependent and wasn't explored further (e.g. by deliberately
  priming the stack with a non-NULL, non-string pattern via `-ftrivial-auto-var-init=pattern` to
  force a worse outcome) - the valgrind-confirmed uninitialized read itself is sufficient to
  establish this as a real bug regardless of which specific downstream effect a given run happens
  to hit.
