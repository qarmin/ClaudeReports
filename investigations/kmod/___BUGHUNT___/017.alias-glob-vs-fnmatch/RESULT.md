# alias-glob-vs-fnmatch: alias index wildcard lookup vs libc fnmatch(3)

`index_mm_searchwild()` (libkmod/libkmod-index.c) walks the alias trie and finishes with `fnmatch()`, with hand-written handling of `*`, `?` and `[` while descending. A differential check against `fnmatch(3)` on random patterns and keys was a good fit for finding logic bugs.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| New testbed component `alias_glob`: random synthetic modules with `alias=` patterns built from `a b c : _ 0 1 * ? [ab] [a-c] [^a] [!b] [0-9] [a-]` atoms, `depmod -b`, then `modprobe --resolve-alias -- <key>` for 40 random keys per round, compared with `ctypes` `fnmatch(pattern, key, 0)` over the same aliases | HEAD ASAN, 8 seeds x 3 rounds (long mode) | no difference after the generator was restricted as below |
| First, unrestricted run | same | differences, all explained by documented/by-design behavior: (1) a key containing `[` or `]` is rejected by `alias_normalize()` (`shared/util.c`) before the lookup; (2) patterns containing a stray `]` or a class starting with `]` (`[]a]`) are rejected by the same function when depmod normalizes aliases; (3) backslash escapes (`\*`) are not interpreted by the trie walk, only by the final `fnmatch`; (4) `-` is normalized to `_` in both pattern and key; (5) a key that itself contains a literal wildcard char (`*`) can report the same module twice (both the literal and the wildcard edge match), `modprobe --resolve-alias` prints the duplicate |

## Status

**Not reproducible** - for the supported subset the index lookup and `fnmatch` agree. The duplicate result for keys containing a literal `*` is a cosmetic artifact: real module aliases and lookup keys never contain `*`.

## Not done

- Escapes and `[]x]` classes are untested by design. Item (5) was not turned into a candidate (no realistic key contains `*`).
- The component stays in the testbed (`just testbed --only alias_glob`) and runs clean.
