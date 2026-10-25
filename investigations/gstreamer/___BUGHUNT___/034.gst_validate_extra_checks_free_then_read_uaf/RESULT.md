# gst_validate_check_num_instances_data_new() free-then-read at `failed:` label

`subprojects/gst-devtools/validate/gst/validate/gst-validate-extra-checks.c`'s
`gst_validate_check_num_instances_data_new()` has a `failed:` error label that does
`g_free (data); g_free (data->klass);` - reading `data->klass` (a struct member) after `data`
itself has already been freed, a textbook free-then-read use-after-free *by source-code shape*.
All three `goto failed;` sites are reachable only when a `GST_VALIDATE_CONFIG`-supplied
`extrachecks` structure is missing/malformed a mandatory field (`num-instances`, `pipeline-name`,
or `element-klass`). Source: findings.toml UAF_7.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: full call graph of `gst_validate_check_num_instances_data_new()` | Only caller is `gst_validate_add_num_instances_check()`, itself only called from `gst_validate_extra_checks_init()` (which is called from `gst_validate_init()`, i.e. reachable by any `gst-validate` invocation) for each `GST_VALIDATE_CONFIG` structure named `extrachecks` that has a `num-instances` field. All 3 error branches inside the function are immediately preceded by a call to `gst_validate_abort()`. | Confirmed the function is reachable via a genuine, user-controlled external input (`GST_VALIDATE_CONFIG` env var), not just callable in the abstract |
| Code reading: `gst_validate_abort()` (`gst-validate-report.c`, line ~1692) | Body is `g_printerr("Bail out! %s\n", tmp); g_free(tmp); exit(-18);` - unconditional, unguarded `exit()`, no ifdef, no return path. Every one of the 3 `goto failed;` statements in the target function is dead code: control flow can never reach any of them because `exit()` terminates the process first. | Concluded the free-then-read is **statically unreachable** through this function's only call site |
| Real end-to-end run: scoped ASAN+UBSAN debug build (`meson --buildtype=debug -Db_sanitize=address,undefined`, clang, core+base+devtools only, no plugins-good/bad/ugly/libav) of `gst-validate-1.0`, driven with `GST_VALIDATE_CONFIG` set to 3 different malformed `extrachecks` structures (missing `pipeline-name`; missing `element-klass`; `num-instances` present with wrong type), each run against a real pipeline (`fakesrc num-buffers=1 ! fakesink`) so `gst_validate_init()` genuinely executes | 3 distinct configs, each run twice (repeat run for the primary "missing element-klass" case, matching the literal `failed:`/`data->klass` scenario in the report) | All 6 runs: process printed `Bail out! [CONFIG ERROR] Mandatory field \`...\` not found ...` to stderr and exited with code 238 (i.e. `exit(-18)` as an unsigned byte). No ASAN heap-use-after-free diagnostic, no crash, no UBSan report tied to this code path (some unrelated UBSan misaligned-load noise from `gstutils.h` fires on every run regardless of config, a known pre-existing artifact noted in this workspace's own `README.md`) |
| Git history of the pattern | `git log -S"g_free (data->klass);"` / direct inspection of `b1b696017c` (2017-06-13, "validate: Add a plugin with potential extra checks" - the commit that introduced this file) | The exact same free-then-read shape was present in the very first version of the file, using `g_error(...)` instead of today's `gst_validate_abort(...)`. `g_error()` is also unconditionally fatal in GLib (`G_LOG_LEVEL_ERROR` cannot be made non-fatal; it aborts the process). So the code has been dead/unreachable by the same mechanism since its introduction, not something a later refactor accidentally protected. |

2 approaches used (code reading + a real instrumented end-to-end run, 6 total process
invocations across 3 distinct malformed-config scenarios, repeated once for the primary
scenario) - both converged on the same, deterministic conclusion, so the soft cap of 3 wasn't
needed: this isn't a case where a different tool (miri N/A for C, a longer fuzz burst, a
different sanitizer) could plausibly change the outcome - the reason it doesn't reproduce is a
provable control-flow fact (every path to the buggy code is preceded by an unconditional,
unguarded call to a function whose entire body ends in `exit()`), not a coverage or flakiness
question.

## Status

**Not reproducible.** The free-then-read at the `failed:` label is real, wrong code by
inspection, but it is unreachable through `gst_validate_check_num_instances_data_new()`'s only
call site: every one of the 3 `goto failed;` statements is directly preceded by a call to
`gst_validate_abort()` (today) / `g_error()` (at the file's original 2017 introduction), both of
which unconditionally terminate the process before the `goto` can execute. Confirmed empirically:
3 distinct malformed-`GST_VALIDATE_CONFIG` scenarios (missing `num-instances`, missing
`pipeline-name`, missing `element-klass` - the last one being the literal scenario that leaves
`data->klass` in its default `g_new0`-zeroed state, exactly the field the `failed:` label reads
after freeing), run against a real ASAN+UBSAN debug build of `gst-validate-1.0` driven by a real
pipeline, 6/6 runs total (2 for the primary scenario) exit cleanly via `exit(-18)` with no ASAN
diagnostic and no crash.

**Fully reproducible**: N/A (not Confirmed - see above; this line is only required for a
Confirmed finding).

## Class

N/A (not a bug in practice - see Status). If it is useful context: by source shape alone this
would be classed `memory-safety` (free-then-read), but that class label does not apply to a
confirmed-unreachable code path.

## Root cause

Not applicable in the "why did it fail to reproduce" sense used for a real false-positive suspect
(this isn't "the value is already validated upstream" - the buggy read genuinely exists in the
source). The reason it never executes is that `gst_validate_abort()` (`gst-validate-report.c`)
is a `noreturn`-in-practice function (unconditional `exit(-18)` with no guard, no ifdef) called
immediately before every `goto failed;` in this function - so `failed:`'s `g_free (data);
g_free (data->klass);` is dead code reachable only if `gst_validate_abort()`'s behavior ever
changes to sometimes return (e.g. a future refactor that makes config-parse errors
non-fatal/recoverable instead of aborting the whole `gst-validate` process). Should that ever
happen, the free-then-read would become live immediately, since nothing else in the function
guards against it.

## Introduced by

Best-effort: b1b696017c "validate: Add a plugin with potential extra checks" (2017-06-13,
Thibault Saunier) - the commit that added this file (`validate/plugins/extra_checks/gstvalidateextrachecks.c`
at the time, later moved to its current path by 2701634c29). The free-then-read shape was present
in this commit's very first version of the function, using `g_error()` (also unconditionally
fatal) in place of today's `gst_validate_abort()` - i.e. the bug has never actually been reachable
at any point in the file's history, not just in the current `HEAD`.

## The fix

No fix was applied - see "the one rule that matters most" in SKILL.md: a fix is only applied to
real, reachable, Confirmed bugs, and this project's working tree must stay clean of any change
tied to a Not-reproducible finding. For the record (not applied, not verified, purely
informational): a bounded fix would swap `g_free (data); g_free (data->klass);` for
`g_free (data->klass); g_free (data);` (free the member before the struct that owns it) - trivial
and harmless to apply defensively even though currently dead code, since it costs nothing and
removes a landmine for any future change to `gst_validate_abort()`'s fatality.

- Project test suite: not run before/after, since no patch was applied this session (N/A - no
  fix to verify).

## What's not done / open questions

- The other two `gst_validate_abort()` call sites inside this same file's `failed:`-reaching
  branches (missing `num-instances`, missing `pipeline-name`) were checked for the same
  unreachability, not just the one matching the literal `data->klass` report - all three converge
  on the same conclusion.
- Not checked: whether any *other* file in this project has a similar `goto failed: g_free(x);
  g_free(x->member);` shape guarded by a fatal `gst_validate_abort()`/`g_error()` call immediately
  before the `goto` (same bug class, same reason it wouldn't reproduce) - not investigated this
  session, per "Critical pitfalls" #7 in SKILL.md this is worth a grep in a future session
  (`grep -rn 'goto failed' subprojects/gst-devtools/validate/` as a starting point) but wasn't
  pursued here since findings.toml's UAF_7 pointed specifically at this one function.
- A defensive drive-by fix (swap the two `g_free()` calls' order) is trivial and harmless but was
  deliberately NOT applied to the main tree, per this workspace's "never leave the main tree
  dirty" rule for a Not-reproducible candidate - left as a suggestion in "The fix" above for a
  human to apply directly if wanted, not something this session should silently commit.
