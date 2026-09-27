# input-files_populate_symbol_name_lengths_oob_st_name

A prior static-audit pass + an interrupted, never-verified session flagged
`InputFile<E>::populate_symbol_name_lengths()` in `src/input-files.cc` (~line 116-122), called
unconditionally from `ObjectFile<E>::parse_symbols()` right after the symbol/string tables are
read for every relocatable object file that has a `.symtab`: it derives a raw pointer from a
symbol's `st_name` (a 32-bit byte offset into `.strtab`, entirely untrusted, copied straight from
the file) with no validation against `symbol_strtab.size()`, then calls `strlen()` on it. This is
the same general shape as this project's other confirmed `sh_info`/`sh_link`/offset-into-mmap
findings (candidates 001/002/010/011), but on a per-symbol field (`st_name`) rather than a
per-section one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `src/input-files.cc` `populate_symbol_name_lengths()` (unchecked `st_name`), `parse_symbols()` (unconditional call site, has `ctx` available), `get_string()`/`get_data()` (confirmed `symbol_strtab` itself IS bounds-checked against the mmap - only the per-symbol `st_name` offset within it is not) | Confirmed the exact unguarded pointer arithmetic + `strlen()` call, matching the task's hypothesis exactly |
| Crafted-input repro against real `mold` binary (build-plain, `-DCMAKE_BUILD_TYPE=Debug`) | Minimal x86_64 `ET_REL` object: `.symtab` with the mandatory null symbol (index 0) + 1 real global symbol (index 1, `sh_info=1`, valid - does not trip the sibling `first_global` bug), `st_name = 0x40000000` (~1 GiB, chosen per this project's own lesson that a merely one-past-the-end offset lands on a still-mapped zero-filled page tail and reads silently instead of crashing) | `mold -o out.exe evil.o` -> SIGSEGV, exit 139. 9/9 runs across two batches, deterministic |
| `gdb` backtrace (`set follow-fork-mode child`/`set detach-on-fork on`, per this project's fork_child gotcha) | Same crafted object, build-plain | Confirmed fault is `__strlen_evex` called from `InputFile<X86_64>::populate_symbol_name_lengths` (`input-files.cc:120`) called from `ObjectFile<X86_64>::parse_symbols` (`input-files.cc:1121`) called from `new_object_file` (`main.cc:39`) - exactly the hypothesized call chain, no surprises |
| Same repro against build-asan (`-DMOLD_USE_ASAN=ON`, g++) | ASAN+UBSAN build | Identical SIGSEGV, exit 139, 4/4 runs. **No `AddressSanitizer:` diagnostic printed on any run** - matches this project's known quirk (candidate 009): the crash is inside a oneTBB worker thread (`read_input_files()`'s `parallel_for_each`), and ASAN doesn't always report a real SIGSEGV delivered there. Exit code + the plain-build gdb backtrace above are the actual confirmation for this build |
| Boundary-value variant (`st_name == symbol_strtab.size()` exactly, not a huge 1 GiB value) | Patched build-plain | Also correctly rejected with the same `Fatal()` diagnostic - confirms the fix isn't narrowly scoped to only the one huge magnitude used in the original repro (CONFIRMATION.md section 7) |
| Legitimate-object regression check (`st_name` pointing at a real, valid, null-terminated name) | Patched build-plain | Links successfully, exit 0, only the expected "entry symbol not defined" warning - the fix doesn't reject well-formed objects |

## Status

**Confirmed** - reproduces 9/9 runs against the unpatched plain (`-DCMAKE_BUILD_TYPE=Debug`) build
and 4/4 runs against the unpatched ASAN+UBSAN build (g++), both a deterministic SIGSEGV/exit 139.
The plain-build gdb backtrace independently confirms the exact fault location and call chain
matches the hypothesized bug precisely.

**Fully reproducible**: Yes. `repro/run.sh`/`repro/gen_evil_elf.py` runs the real `mold` binary
end-to-end (`mold -o <out> <crafted.o>`) against a single, standalone, hand-crafted ELF object
file - this is a single-participant case (a malformed input file handed to the real linker's real
CLI entry point is the genuine external artifact a real attacker/build system would supply; there
is no second participant to fake here), so CONFIRMATION.md section 11's two-participant bar does
not apply.

## Class

crash (memory-safety - out-of-bounds pointer read via `strlen()` on an attacker-controlled offset
into a bounds-checked-but-not-per-element-validated string table view)

## Root cause

`get_string()` (`src/mold.h`) correctly bounds-checks a section's own `sh_offset`/`sh_size`
against the mmap'd file size, so `symbol_strtab` (the `.strtab` view) is itself a legitimate,
in-bounds `std::string_view`. But `populate_symbol_name_lengths()` then does, for every symbol in
the `.symtab`:

```cpp
const char *p = symbol_strtab.data() + esym.st_name;  // st_name unchecked
symname_lens.emplace_back(strlen(p));                   // strlen() on a wild pointer
```

`esym.st_name` is a per-symbol, fully untrusted 32-bit byte offset (per the ELF spec, "index into
the object file's symbol string table"), copied straight from the file with no validation that it
actually falls inside `symbol_strtab`. A large `st_name` (this repro uses ~1 GiB) makes `p` point
far outside the mmap'd input file entirely; `strlen()` then walks off mapped memory and segfaults.
`populate_symbol_name_lengths()` is called unconditionally, immediately after `symbol_strtab` is
read, for every object file with a `.symtab` - no special CLI flag or linker feature is needed.

## Introduced by

Not a recent regression at the semantic level, though the exact line has moved/been renamed
several times. `git log -S'populate_symbol_name_lengths'` bottoms out at 7887246f ("Cache input
symbol name lengths", 2026-07-24) - but reading that commit's diff shows it's a pure rename/perf
refactor (`populate_symbol_names()` -> `populate_symbol_name_lengths()`, changing what's cached
from a full `string_view` to a length) that preserved the exact same unchecked
`symbol_strtab.data() + esym.st_name` pointer arithmetic verbatim; it did not introduce the missing
bounds check. Following `st_name`'s use back further (`git log -S'populate_symbol_names'`,
`git log --follow -S'esym.st_name'`) traces through `abcbd451`/`441f231d`/`5f272c41` ("perf: cache
symbol names to avoid per-call strlen", plus its own revert) and several much older commits
(`67a96ae8` "Refactor", `b1472714`, `536e0e7d`, ..., down to a handful of very early "temporary"
commits near the start of this file's own history, predating even the 2024-08-14 `5b859497`
"Reorganize directory structure" bulk-rename). The missing `st_name` bounds check itself appears
to have been present since very close to the project's original ELF symbol-table parsing code -
not worth a full `git bisect` this session given how far back the trail already goes without
landing on a single clear introducing change (same conclusion candidate 011 reached for the
sibling `first_global` bug in this same function).

## The fix

Validate every symbol's `st_name` against the actual size of the resolved string-table view,
immediately after both are read in `parse_symbols()`, before the unconditional call to
`populate_symbol_name_lengths()`:

```cpp
this->first_global = symtab_sec->sh_info;
this->elf_syms = this->template get_data<ElfSym<E>>(ctx, *symtab_sec);
this->symbol_strtab = this->get_string(ctx, symtab_sec->sh_link);

for (const ElfSym<E> &esym : this->elf_syms)
  if (esym.st_name >= this->symbol_strtab.size())
    Fatal(ctx) << *this << ": symbol name offset is out of range";

this->populate_symbol_name_lengths();
```

This mirrors the exact style of the fix candidate 011 already added a few lines above for the
sibling `first_global`/`sh_info` bug in the same function (`Fatal(ctx) << *this << ": ..."`,
checked once at the single point where the untrusted field is first available, before any
consumer can use it unsafely) - protecting every downstream consumer of `st_name`
(`populate_symbol_name_lengths()` itself, and `InputFile<E>::get_symbol_name()`'s cached-length
lookup at `src/mold.h:1935-1936`, which does the identical `symbol_strtab.data() +
elf_syms[i].st_name` arithmetic) rather than needing a check at each call site individually.

Verified:
- `repro/run.sh` (and the manual boundary/legitimate-input variants) no longer crash against the
  patched plain build - mold now exits 1 with `mold: fatal: <file>: symbol name offset is out of
  range` instead of segfaulting. Confirmed against both the exact original triggering magnitude
  (`0x40000000`) and a tight boundary value (`st_name == symbol_strtab.size()` exactly, i.e. one
  byte out of range) - both rejected identically (CONFIRMATION.md section 7).
- A legitimate object with a valid, in-range `st_name` still links successfully (exit 0) - the fix
  does not reject well-formed input.
- Project test suite (`ctest --test-dir build-asan -j4`): **before patch: 475/476 passed** (the
  one failure, `x86_64-run-clang`, is the same pre-existing ASAN/`mold-wrapper.so` LD_PRELOAD
  collision documented in `___BUGHUNT___/README.md`, unrelated to this bug). **After patch:
  475/476 passed**, identical single pre-existing failure, no new failures.

## What's not done / open questions

- `InputFile<E>::get_symbol_name()` (`src/mold.h:1934-1936`) does the same
  `symbol_strtab.data() + elf_syms[i].st_name` arithmetic as a fallback path when a symbol's
  cached length indicates a "long name" (see `7887246f`'s logarithmic-lower-bound caching scheme
  mentioned in that commit's log message) - this fix closes it off at the source (`st_name` can no
  longer be out-of-range by the time any code reads `elf_syms`), so this call site wasn't
  independently re-verified with its own dedicated repro; it shares the same root cause and should
  be covered by the same check.
- Did not check whether any *other* `ObjectFile<E>`/`InputFile<E>` method besides
  `populate_symbol_name_lengths()`/`get_symbol_name()` reads `esym.st_name` directly (a targeted
  grep for `st_name` elsewhere in `src/input-files.cc`/`src/mold.h` beyond these two call sites
  wasn't performed this session) - low risk given the fix is applied at the single point
  `elf_syms` first becomes available, but not exhaustively confirmed.
- `git log -S`/`--follow` traced the pattern back several renames without finding a single
  "introducing" commit distinct from the file's own early history - a full `git bisect` was judged
  not worth the time budget given how deep the trail already goes (see "Introduced by").
