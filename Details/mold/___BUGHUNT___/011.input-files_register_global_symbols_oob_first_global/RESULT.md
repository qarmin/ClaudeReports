# input-files_register_global_symbols_oob_first_global

`___BUGHUNT___/IDEAS.md` backlog item ("src/input-files.cc:1112-1125 ... ObjectFile<E>::parse_symbols()
copies symtab_sec->sh_info straight into first_global with no validation. sh_info is an untrusted
32-bit field...", from a prior `code-audit-structured` audit pass). `first_global` is used
throughout symbol-handling code as the boundary between local and global symbols in a file's
symbol table - it's derived from a completely ordinary, always-present `.symtab` section, so this
is reachable via any relocatable object file with symbols at all, no special feature needed.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `src/input-files.cc` `parse_symbols()` (sets `first_global`), `register_global_symbols()` (first consumer, called unconditionally right after) | Found `i64 num_globals = this->elf_syms.size() - this->first_global;` immediately followed by `has_symver.resize(num_globals)` - if `first_global > elf_syms.size()`, `num_globals` is negative, and `std::vector<bool>::resize` receives that negative `i64` implicitly converted to a huge `size_t` |
| Crafted-input repro against real `mold` binary | `build-plain/mold`, minimal x86_64 ET_REL object with a `.symtab` holding exactly 1 symbol (the mandatory null entry) but `sh_info = 5` | Uncaught `std::length_error` ("vector<bool>::_M_fill_insert"), `terminate()` -> `SIGABRT`, 3/3 runs |
| Same repro against `build-asan/mold` | ASAN+UBSAN build | Identical `std::length_error` -> `SIGABRT`, 2/2 runs - same failure mode as plain build (this is a `std::vector<bool>` allocation-size check inside libstdc++ itself, not something ASAN's own instrumentation needed to catch) |

## Status

**Confirmed** - reproduces 3/3 runs against the plain build and 2/2 against the ASAN build, both
with the identical, deterministic `std::length_error` message (this is not a memory-layout-
dependent crash the way a raw OOB pointer read is - the negative-to-huge size conversion fails
allocation the same way every time). Captured real command output in `output.txt`.

**Fully reproducible**: Yes. `repro/run.sh` runs the real `mold` binary end-to-end against a
crafted-but-otherwise-ordinary ET_REL object file.

## Class

crash (uncaught C++ exception from an invalid container-resize argument, same general shape as
candidate 008's uncaught `std::out_of_range` - mold has no top-level exception handling anywhere in
its input-parsing pipeline, so any thrown exception on this path reaches `std::terminate()`)

## Root cause

`ObjectFile<E>::parse_symbols()` reads the "index of first global symbol" field directly from the
file's own `.symtab` section header (`sh_info`, per the ELF spec) with no validation:

```cpp
this->first_global = symtab_sec->sh_info;
this->elf_syms = this->template get_data<ElfSym<E>>(ctx, *symtab_sec);
...
register_global_symbols(ctx);
```

`register_global_symbols()` then computes:

```cpp
i64 num_globals = this->elf_syms.size() - this->first_global;
has_symver.resize(num_globals);
```

If the file declares `sh_info` larger than the actual number of symbol-table entries (`elf_syms.size()`),
`num_globals` goes negative. `std::vector<bool>::resize(size_type)` takes an unsigned `size_type`,
so the negative `i64` is implicitly converted to an enormous `size_t` (~1.8×10^19 for a difference
of -4), which `std::vector<bool>`'s internal `_M_fill_insert` rejects by throwing
`std::length_error`. Nothing on this path catches it, so the program aborts.

## Introduced by

not determined (best-effort) - `git log -S` on the exact `num_globals`/`has_symver` computation
bottoms out at 205138f1 ("Refactor SharedFile class", 2022-01-12) for the current variable names,
but the underlying "subtract an untrusted `sh_info` from the symbol count with no validation"
pattern appears to predate that refactor under different naming - not worth a full `git bisect`
this session given how far back the trail already goes without landing on a single clear
introducing change.

## The fix

Validate `first_global` against `elf_syms.size()` immediately after both are read in
`parse_symbols()`, before any code (in `register_global_symbols()` or elsewhere) can compute a
negative `num_globals` from it:

```cpp
this->first_global = symtab_sec->sh_info;
this->elf_syms = this->template get_data<ElfSym<E>>(ctx, *symtab_sec);

if ((u64)this->first_global > this->elf_syms.size())
  Fatal(ctx) << *this << ": symbol table's sh_info (index of first global"
    " symbol) is out of range";
```

This mirrors the existing bounds-checked pattern already used a few lines earlier in the same file
for a different untrusted `sh_info` use (`src/input-files.cc:293`, COMDAT group signatures). Fixing
it at the single point where `first_global` is assigned protects every downstream consumer of the
field (`register_global_symbols()`, `initialize_local_symbols()`, and the several `for (i64 i =
this->first_global; ...)` loops elsewhere in this file) rather than needing a check at each call
site individually.

Verified: `repro/run.sh` no longer crashes against either the patched plain build or the patched
ASAN build - mold now exits cleanly with a `fatal:` diagnostic instead of an uncaught exception
(see `output.txt`'s "after patch.diff applied" section).

- Project test suite (`ctest -j5` in `build-asan/`): **before patch: 475/476 passed** (1 known
  pre-existing failure, `x86_64-run-clang`, same baseline already established this session for
  candidate 010 against the same unpatched `HEAD`; see `___BUGHUNT___/README.md`). **After patch:
  475/476 passed**, same single pre-existing failure, no new failures.

## What's not done / open questions

- Not independently checked whether any of the several other `for (i64 i = this->first_global; i <
  this->elf_syms.size(); i++)`-shaped loops elsewhere in `src/input-files.cc` (lines 1238, 1274,
  1351, 1442, 1482, per a grep for `first_global`) have their *own* additional assumptions about
  `first_global` beyond simple bounds (e.g. relative ordering with other fields) that this one
  bounds check might not fully cover - the fix closes off the specific negative-`num_globals`
  crash confirmed here, but a fuller audit of every `first_global` consumer was out of scope this
  session.
- The `git log -S` trail for the introducing commit was not fully run to ground (see "Introduced
  by") - a future session with more time budget could `git bisect` this properly if pinning the
  exact origin is valuable.
