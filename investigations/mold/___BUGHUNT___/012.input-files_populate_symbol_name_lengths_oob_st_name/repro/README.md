# Repro: `InputFile<E>::populate_symbol_name_lengths()` OOB `st_name` -> SIGSEGV

## What this does

`gen_evil_elf.py` writes a minimal, otherwise perfectly well-formed x86_64 `ET_REL` ELF object
whose `.symtab` has the mandatory null symbol (index 0) plus one real global symbol (index 1,
`sh_info = 1` so this doesn't trip the sibling `first_global` bug at
`src/input-files.cc:1118`) whose `st_name` (a 32-bit byte offset into `.strtab`) is set to
`0x40000000` (~1 GiB) - far larger than the actual `.strtab` (6 bytes).

`src/input-files.cc`, `InputFile<E>::populate_symbol_name_lengths()`, called unconditionally from
`ObjectFile<E>::parse_symbols()` right after the symtab/strtab are read for every relocatable
object file that has a `.symtab`:

```cpp
for (const ElfSym<E> &esym : elf_syms) {
  const char *p = symbol_strtab.data() + esym.st_name;  // st_name unchecked
  symname_lens.emplace_back(strlen(p));                  // strlen() on a wild pointer
}
```

`st_name` is never validated against `symbol_strtab.size()` anywhere upstream, so `p` ends up far
outside the mmap'd input file and `strlen()` segfaults.

## Invocation

```
python3 gen_evil_elf.py /tmp/evil.o
/path/to/mold -o /tmp/out.exe /tmp/evil.o
echo $?     # 139 (SIGSEGV) on unpatched mold; 1 with a clean "fatal: ... out of range" on patched mold
```

Or via `run.sh` (does the same thing, in a scratch tmpdir):

```
./run.sh /path/to/mold
```

No other CLI flags are needed - the crash is reached during ordinary input-file parsing, before
any symbol resolution/relocation/output-writing stage, so no special linker flag or feature
(`--gc-sections`, `--icf`, archives, etc.) is required, just `mold -o <out> <evil.o>`.
