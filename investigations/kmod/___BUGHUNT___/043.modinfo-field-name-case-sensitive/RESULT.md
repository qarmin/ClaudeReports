# modinfo-field-name-case-sensitive: `modinfo -F LICENSE` prints nothing although modinfo(8) says field names are case-insensitive

man/modinfo.8.scd states for `-F`: "Field names are case-insensitive." `tools/modinfo.c` compares the requested field with `streq()` everywhere (`parm`, `name`, `filename` and the generic `streq(field, key)`), so `modinfo -F LICENSE foo.ko`, `-F Author`, `-F FILENAME` silently print nothing and exit 0. Scripts written against the documented behavior (and the old modutils tool) get empty results.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `modinfo -F <field> m.ko` on a synthetic module with license/author/parm/parmtype, fields `license`, `LICENSE`, `License`, `author`, `AUTHOR`, `filename`, `FILENAME`, `parm`, `PARM` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: lower-case works, every other spelling prints nothing, exit 0 |
| Patched | patch.diff, 5 runs | 5/5: all spellings print the same values |
| History search | `git log -S strcasecmp -- tools/modinfo.c` | the tool never used a case-insensitive comparison; the sentence comes from the man page added in f3a423b |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real binary against the documented behavior.

## Class

logic (behavior differs from documentation)

## Root cause

Field comparison uses `streq()`.

## Introduced by

not determined: the documentation sentence predates the scdoc conversion (f3a423b "man: add scdoc based man pages") and the code never matched it.

## The fix

`patch.diff` (+5/-4, tools/modinfo.c): `strcasecmp()` for the four comparisons of the requested field. Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after. The alternative is to correct the man page; which side is right is upstream's call (a field `-F Parm` currently selects nothing, so no script can depend on the case-sensitive behavior except for relying on the empty output).

## What's not done / open questions

- Keys inside the module are compared as stored; only the user-supplied field name is folded.
