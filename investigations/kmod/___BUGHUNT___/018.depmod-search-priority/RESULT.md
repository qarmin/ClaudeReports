# depmod-search-priority: which duplicate module depmod keeps

`depmod_module_is_higher_priority()` (tools/depmod.c) decides between two modules with the same name using `search`, `override` and `external` directives. Reading it showed an `else if` chain that skips the old-path test when the new path matches the same search entry, which looked suspicious.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Real `depmod -b <root> -C <etc>` on two synthetic `foo.ko` copies in different directories, 11 layouts: default config (both discovery orders), `search kernel updates built-in`, `search built-in updates`, `search extra built-in`, `search built-in extra`, two copies below one search dir (`extra/a`, `extra/b`), `override foo 1.0 kernel`, `override foo 1.* kernel`, `override foo 2.0 kernel` (non-matching version), `external 1.0 /nonexistent` | HEAD ASAN build | every result matches depmod.d(5): earlier `search` entry wins, `built-in` means `kernel/`, an `override` for a matching kernel version beats `search`, a non-matching version is ignored, an unreachable `external` changes nothing. The two-copies-in-one-dir case keeps the later one found, which depends on directory iteration order and is unspecified in the manual |

## Status

**Not reproducible** - no deviation from the documented behavior; the suspicious `else if` only matters when both copies sit below the same search entry, which is the unspecified tie case.

## Not done

- Tie-breaking determinism across filesystems (`readdir` order) was not tested.
