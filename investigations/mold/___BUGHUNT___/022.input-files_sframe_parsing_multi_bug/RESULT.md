# input-files_sframe_parsing_multi_bug

`.sframe` (a compact stack-unwinding format) parsing in `src/input-files.cc` (`ObjectFile<E>::
parse_sframe()`, ~line 764-831, and its helper `sframe_fre_block_size()`, ~line 746-759) trusts
every header/FDE field of an untrusted section, computing several offsets from fully
attacker-controlled 32-bit fields with no validation before dereferencing them. Flagged by a prior
static-audit pass, not yet built/verified. (This candidate's own dispatched agent was killed
mid-investigation by the same recurring infrastructure safety-filter error already seen on
candidates 015/019/020 - after already producing a complete, correct repro generator with detailed
reasoning but before building/running it. The orchestrator completed the investigation directly in
the surviving worktree rather than losing that work.)

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `ObjectFile<E>::parse_sframe()` and `sframe_fre_block_size()` at current commit c340272e | static read | Confirmed the exact mechanism: `fde_off`/`fre_off` (derived from untrusted `hdr.fdeoff`/`hdr.freoff`) are never range-checked against the section's real size before being dereferenced; `sframe_fre_block_size()` itself reads `num_fres`/`addr_size`/`info` fields with no bounds checks in its loop either |
| Crafted x86_64 `ET_REL` object: `.text` + `.sframe` (header with `fdeoff=0` so the one FDE index entry is real/in-bounds, but `freoff=0xFFFFFFF0` - ~4.29GB) + `.rela.sframe` (one relocation matching the FDE index's file offset, satisfying the relocation-match check so the loop body actually computes the malicious offset instead of skipping the FDE) + `.symtab`/`.strtab` defining the target symbol; ran real `mold` (`-DCMAKE_BUILD_TYPE=Debug` plain build) on the crafted object | plain Debug build, real mold binary, real CLI, no special flag needed - `.sframe` parsing is automatic for any object that has the section | **Segfaulted** (SIGSEGV, exit 139), 5/5 identical runs |
| `gdb` backtrace (`set follow-fork-mode child`/`set detach-on-fork on`) on the crash | same build | Confirmed the exact fault location and mechanism: `mold::Integer<u16>::operator u16` (`lib/integers.h:68`, `memcpy`) called from `sframe_fre_block_size<X86_64>` (`src/input-files.cc:749`) with `offset=4294967308` (~4GB) - exactly the huge `fre_off` computed from the crafted `freoff` field, called from `parse_sframe` (`src/input-files.cc:827`), called from `parse_sframe_sections` (`src/passes.cc:511-512`) |
| Confounded-fix check: reverted the patch, rebuilt, re-ran the exact same repro | same build, unpatched | Crash returns identically (exit 139, same fault location) - confirms the fix (not something else) is what makes the crash go away |
| Patched build re-verify (after re-applying the fix and rebuilding) | same build, patched | No crash: `mold: fatal: <file>:(.sframe): corrupted .sframe section`, exit 1, 3/3 runs |
| Full project test suite (`ctest --test-dir build-plain -j4`), before and after the patch | plain Debug build | 476/476 passed both times |

All runs use the real, unmodified `mold` binary via its actual CLI entry point on a hand-crafted-
but-otherwise-standard ELF relocatable object file - a malformed object file handed to the linker's
real parsing entry point is the genuine external input (CONFIRMATION.md section 11's
two-participant requirement doesn't apply; single-participant finding, same as every other
candidate in this project).

## Status

**Confirmed.** Reproduces deterministically, 5/5 runs on the unpatched plain Debug build (SIGSEGV,
exit 139), independently confirmed via a real `gdb` backtrace pinpointing the exact fault location
and matching the hypothesized mechanism precisely, and via the confounded-fix revert/re-apply cycle
(`SKILL.md` pitfall #6).

**Fully reproducible**: **Yes.** `repro/run.sh`/`repro/gen_evil_elf.py` runs the real `mold` binary
end-to-end (`mold -o <out> <crafted.o>`) against a single, standalone, hand-crafted ELF object
file, with no special CLI flag needed - `.sframe` parsing runs automatically whenever an input
object has a `.sframe` section. Single-participant finding (a malformed input file handed to the
real linker's real CLI entry point is the genuine external artifact a real attacker/build system
would supply; no second participant to fake).

## Class

memory-safety (raw pointer OOB read via an untrusted, unbounded offset computed from attacker-
controlled 32-bit header/FDE fields - not dependent on any hardening/assertion mechanism to
manifest, unlike candidates 016/018/020's `std::vector`-index bugs; this one walks a raw `char*`
directly, so it segfaults on every build configuration, hardened or not).

## Root cause

`ObjectFile<E>::parse_sframe()` computes three offsets from a `.sframe` section's own header/FDE
fields with **no validation against the section's actual size** before using them to index into
`data` (a `std::string_view` over the section's mmap'd bytes):

```cpp
i64 hdr_len = sizeof(SFrameHeader<E>) + hdr.auxhdr_len;   // hdr.auxhdr_len: untrusted
i64 fde_off = hdr_len + hdr.fdeoff;                        // hdr.fdeoff: untrusted
i64 fre_off = hdr_len + hdr.freoff;                        // hdr.freoff: untrusted
...
i64 off = fre_off + ent.func_start_fre_off;                // ent.func_start_fre_off: untrusted
...
fde.fre = data.substr(off, sframe_fre_block_size<E>(data, off));
```

This repro drives `fre_off` (via a huge `hdr.freoff = 0xFFFFFFF0`) to ~4.29GB while keeping
`fdeoff = 0` so the FDE-index-lookup loop itself stays in-bounds and reaches the vulnerable
`off`/`sframe_fre_block_size()` call. `sframe_fre_block_size()` then immediately dereferences
`data.data() + offset` (a raw `const u8*`) with `offset` ~4GB past the section's real mmap'd
extent - far enough to walk off the mapped region entirely and fault (per candidate 004's lesson:
only a sufficiently huge offset reliably faults; a merely-one-past-the-end offset would read a
still-mapped, silently-zero byte instead). The same function's inner loop also has no bound on how
far `p` can advance based on attacker-controlled `num_fres`/`num_words`/`word_size` values, a second
independent way to walk arbitrarily far past the section once inside the function.

## Introduced by

d0aa95ab "Support the SFrame v3 stack-unwinding format" (2026-06-07) - the single commit that
introduced `.sframe` parsing from scratch, with every one of these missing bounds checks present
from its very first version (confirmed via `git log -S'sframe_fre_block_size'`/`-S'parse_sframe'`,
both bottom out at this one commit with no prior history).

## The fix

Validate every untrusted offset against `data.size()` before it's used to index into `data`, at
three points:

1. Before reading the section's own header: `data.size() >= sizeof(SFrameHeader<E>)`.
2. Right after computing `hdr_len`/`fde_off`/`fre_off`: reject negative values and require
   `hdr_len <= data.size()`, `fre_off <= data.size()`, and
   `fde_off + num_fdes * sizeof(SFrameFdeIdx<E>) <= data.size()` (bounding the whole FDE-index
   array in one check, since it's read via a raw cast per-iteration).
3. Right after computing each FDE's `off = fre_off + ent.func_start_fre_off`: require
   `off >= 0 && off + 5 <= data.size()` before calling `sframe_fre_block_size()` (which itself reads
   a 5-byte attribute header at that offset).

`sframe_fre_block_size()` itself was also hardened (threading `Context<E>&`/the `InputSection<E>&`
through for error reporting) to re-check its own starting offset and to bound its internal loop -
`p + addr_size >= data.size()` before each `info` byte read, and `p > data.size()` after advancing -
so a corrupted `num_fres`/variable-width-word combination can't walk the internal cursor past the
section either, independent of whatever the caller already validated.

Verified:
- Re-ran `repro/run.sh` against the patched build: no longer crashes, exits 1 with `mold: fatal:
  <file>:(.sframe): corrupted .sframe section` instead of segfaulting - confirmed 3/3 runs.
- Confounded-fix check (`SKILL.md` pitfall #6): reverted the patch, rebuilt, re-ran the repro -
  crash returned identically, then re-applied the fix and reconfirmed it stays fixed.
- Project test suite (`ctest --test-dir build-plain -j4`, plain Debug build): **476/476 pass before
  the patch, 476/476 pass after** - no existing test exercises `.sframe` parsing at all on this
  host (no failure either way), and the fix compiles cleanly across every architecture template
  (`cmake --build` instantiates `parse_sframe`/`sframe_fre_block_size` per-arch via the `requires
  supports_sframe<E>` constraint - only architectures that actually support SFrame instantiate the
  function, confirmed no other arch's build broke).

## What's not done / open questions

- Only the `fre_off`-via-huge-`freoff` trigger was independently built and confirmed this session.
  The audit's other three sub-cases (a truncated header before the initial `SFrameHeader` read; a
  huge `fdeoff` walking the FDE-index-array read directly; a corrupted `num_fres`/word-size
  combination inside `sframe_fre_block_size()`'s own loop) are all covered by the same patch (every
  one of the three validation points above addresses a different sub-case) but weren't
  independently driven through dedicated crafted inputs this session - a future session could
  construct 2-3 more variants for full completeness per `CONFIRMATION.md` section 7, though the
  fix's coverage was reasoned through carefully against the actual code rather than guessed at.
- No existing project test exercises `.sframe` parsing at all (0 sframe-related test names seen in
  the 476-test suite) - this whole code path may be under-tested independent of this specific bug;
  worth flagging to maintainers as a gap, not something this candidate's fix should be expected to
  address.
