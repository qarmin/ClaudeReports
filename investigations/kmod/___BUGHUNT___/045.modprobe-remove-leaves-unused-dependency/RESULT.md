# modprobe-remove-leaves-unused-dependency: `modprobe -r` leaves the deepest dependency of a chain loaded although nothing uses it

`rmmod_do_module()` (tools/modprobe.c, step 4) removes "other modules that became unused" by walking `kmod_module_get_dependencies(mod)` with `kmod_list_foreach()` and removing every dependency whose refcount is 0 at that moment. libkmod returns that list in load order (deepest dependency first: `parse_depline` prepends), so for `c -> b -> a` it looks at `a` first (still held by `b`, refcount 1, skipped) and then removes `b`, which drops `a`'s refcount to 0 after the loop has already passed `a`. modprobe(8) says "If the modules it depends on are also unused, modprobe will try to remove them too"; with a dependency chain deeper than one level the deepest unused module stays loaded. The project test suite has no `modprobe -r` test, which is how this survived.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| New kernel-interface simulator `testbed/sim/sysfake.c` (LD_PRELOAD: redirects `/sys/module`, `/proc/modules`, fakes `delete_module`/`finit_module` with refcounts and holders); tree c -> b -> a from `depmod`, all loaded; `modprobe -r c` (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: delete c, delete b, `a` left loaded with refcnt 0 |
| Same, patched (walk backwards) | patch.diff, 5 runs | 5/5: c, b, a removed |
| Shared dependency: `a` also held by an unrelated module `d` | patch.diff | `a` correctly kept |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** for the userspace logic, with the kernel side replaced by the simulator (no root, no kernel module needed): the real `modprobe` binary makes the real `delete_module` calls into a fake whose refcount/holders semantics follow sysfs. A real system would need loaded modules; the logic under test does not depend on anything else.

## Class

logic

## Root cause

Iteration order of the dependency list in step 4 of `rmmod_do_module()`.

## Introduced by

a872bba "modprobe: rework module removal without tree traversing"

## The fix

`patch.diff` (+2/-1, tools/modprobe.c): `kmod_list_foreach_reverse()` for the step-4 loop. Verified with the repro (5/5 exit 1 -> 5/5 exit 0), the shared-dependency variation, and the project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- Diamond-shaped dependency graphs were reasoned about, not simulated beyond the shared-holder case.
- `--remove-holders` and softdep removal order use other loops and were not examined here.
