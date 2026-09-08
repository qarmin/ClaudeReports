# 030.sparc64_large_plt_size_mismatch

`src/arch-sparc64.cc` (`sparc_plt_ptr_offset`, `write_plt_entry`), `src/mold.h`
(`to_plt_offset`, `sparc_num_small_plt`) and `src/output-chunks.cc`
(`PltSection<E>::update_shdr`). A prior static-audit pass claimed the SPARC64 "large PLT"
layout and the `.plt` size computation disagree by a factor of two, so any link needing more
than `sparc_num_small_plt` PLT entries would write past the end of `.plt` - an out-of-bounds
write into the output mapping. The area was worth checking regardless: 4afb95ac "Support
SPARC64's large PLT format" (2026-06-07) is recent code and this host registers **zero**
SPARC64 tests (see below), so nothing in the project's own suite exercises it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `to_plt_offset`, `sparc_plt_ptr_offset`, `write_plt_entry`, `PltSection::update_shdr`, `RelPltSection::copy_buf`, and `struct SPARC64` in `src/elf.h` | source only | The audit's premise is factually wrong: `SPARC64::plt_size` is **32**, not 16. `sparc_num_small_plt` is therefore **32764**, not 65534, and `plt_hdr_size + n*plt_size` is *exactly* the size the block layout consumes. |
| Standalone arithmetic model of all three functions (`repro/plt_math.py`) | pure Python, 414 entry counts incl. `nlarge` = 1, 2, 159, 160, 161, 319, 320, 321, 1000, 5000, 65600, 100000 | `max(bytes_needed - sh_size) == 0` over every case. No case where the layout exceeds the computed size. |
| Real end-to-end link: synthesised big-endian SPARC64 `ET_REL` objects, `mold -shared` | plain Debug build (`build-plain`, gcc), 13 symbol counts from 100 to 34000 (up to 1236 large entries) | All 13 links exit 0. `.plt` `sh_size` matches `128 + n*32` exactly and covers every byte the layout writes, every time. No crash, no corruption. |
| Same 13 links re-run under ASAN + UBSAN | `build-asan` (`-DMOLD_USE_ASAN=ON`, g++ 15.2.0; pulls in `-fsanitize=address -fsanitize=undefined`) | Zero sanitizer diagnostics, all links exit 0, identical `.plt` geometry. |
| Post-link validation of the produced `.so` (`repro/check_plt.py`) | reads the linked output back with a hand-rolled BE-ELF64 reader | For every entry: stub inside `sh_size`; large-entry 8-byte data slot inside `sh_size`; the stub's `ldx [%o7 + imm13], %g1` displacement decodes to exactly that slot's address; the slot holds exactly `.PLT0 - call`; the matching `R_SPARC_JMP_SLOT` `r_offset` is the slot (large) / the stub (small) and lies inside `.plt`; no other section's file range overlaps `.plt`'s. All pass. |
| Repeat run of the whole sweep | plain build, second run | Byte-identical output to the first run. |
| Project test suite | `ctest -j4` in `build-plain` | 476/476 pass. `ctest -N \| grep -ci sparc` = **0** - no SPARC64 test registers on this host (`test/CMakeLists.txt:169` needs both `qemu-sparc64` *and* `sparc64-linux-gnu-gcc`; only `qemu-sparc64` is installed). |
| `just test` from this folder | fresh `git clone` of upstream at `c340272e`, fresh Debug build, then the full sweep | exit 0 - the folder is self-contained and the result reproduces from a pristine clone, not just from this worktree. |

## Status

**Not reproducible**

The hypothesised out-of-bounds write does not exist. Coverage, so a future session does not
retry this dead end:

- Arithmetic model: 414 distinct PLT-entry counts, including every boundary that matters
  (last small entry, first large entry, exact multiples of the 160-entry block, +/-1 around
  each). Maximum overshoot found: **0 bytes**.
- Real links: 13 counts (100, 32763, 32764, 32765, 32766, 32923, 32924, 32925, 33083, 33084,
  33085, 33100, 34000), i.e. the small/large boundary and the 1st/2nd/3rd block boundaries,
  run **twice** on the plain Debug build (identical output) and **once more** under ASAN+UBSAN.
  39 links total, all clean.
- The linked outputs were parsed back and checked field-by-field, not just "mold exited 0".

**Fully reproducible**: N/A (nothing to reproduce).

## Class

N/A.

## Root cause

There is no bug; the audit finding rests on a wrong constant. Concretely:

- `src/elf.h:2163-2164`: `SPARC64::plt_hdr_size = 128`, `SPARC64::plt_size = **32**` (the audit
  assumed 16).
- `src/mold.h:3715`: `sparc_num_small_plt = (0x100000 - 128) / 32 = **32764**` (the audit said
  65534). `128 + 32764*32 == 0x100000` exactly, so large entries start precisely at the
  0x100000 boundary where a small entry's `ba,a %xcc, .PLT1` branch runs out of reach.
- A full large block is 160 stubs x 24 bytes + 160 pointers x 8 bytes = 3840 + 1280 = **5120**
  bytes, i.e. **32 bytes per entry** - the same 32 bytes a small entry costs. That is why
  `plt_hdr_size + n * plt_size` is the correct size for both regions, and why the deliberate
  `is_sparc<E>` branch in `PltSection<E>::update_shdr` (`src/output-chunks.cc:1894-1895`) is
  there at all: `to_plt_offset<E>(n)` would return only the *stub* offset of entry `n` and
  would under-count the final block's pointer region, so the code intentionally uses the
  closed form instead.
- Algebraically, for `nlarge` large entries with `b = (nlarge-1)/160` full blocks before the
  last one and `k = nlarge - b*160` stubs in the last block, the highest byte the layout
  touches is `0x100000 + b*5120 + k*24 + (k-1)*8 + 8 = 0x100000 + b*5120 + k*32 =
  0x100000 + nlarge*32`, which is identical to the computed
  `128 + (32764 + nlarge)*32`. The difference is exactly zero for every `nlarge`, not "exactly
  half".

Two related things were also checked and found correct, since the same code was already open:

- The `ldx [%o7 + imm13], %g1` displacement always stays inside the signed 13-bit field: within
  a block of `k` stubs, entry `i` uses `k*24 - i*16 - 4`, whose extremes are `3836` (i=0, k=160)
  and `8*k + 12 > 0` (i=k-1) - both well inside +/-4095. Partial last blocks are handled because
  `sparc_plt_ptr_offset` derives `nstub` with `min(160, nlarge - block*160)`.
- The small entry's `ba,a %xcc, .PLT1` displacement bottoms out at `(32 - (0x100000-32) - 4)/4
  = -262129`, inside disp19's `-262144..262143`.
- The `.plt` data-pointer writes are 8-byte aligned (`0x100000`, `5120` and `nstub*24` are all
  multiples of 8, and `.plt` itself is 256-aligned in the produced output).

## Introduced by

N/A - no defect. For reference, the code under suspicion was added by
4afb95ac "Support SPARC64's large PLT format" (2026-06-07), which added the
`is_sparc<E>` branch in `PltSection<E>::update_shdr` in the *same* commit as the layout
functions, specifically so the two agree.

## The fix

None - no bug to fix, so there is no `patch.diff`.

The task suggested, as a bounded fix, replacing the sizing expression with a
`sparc_plt_size(ctx, nsyms)` helper returning `0x100000 + nblocks*5120 + partial`. That
would compute the identical value the current code already computes, so it would be pure
churn; it was deliberately not written.

- Project test suite: 476/476 passing before, no change made, so 476/476 after. No SPARC64
  test exists on this host to run (0 registered), so the suite provides no coverage of this
  code either way - the end-to-end links in `repro/` are the only real exercise it got.

## What's not done / open questions

- `.plt` correctness was verified *statically* against the produced `.so`. It was **not**
  executed: `qemu-sparc64` is installed but `sparc64-linux-gnu-gcc` and a SPARC64 glibc are
  not, so no runtime check of whether the loader actually resolves through these stubs was
  possible. The static checks (ldx target, slot contents `= .PLT0 - call`, JMP_SLOT addend
  `= -call`) match what glibc's `sparc64_fixup_plt` expects, but that is a code-reading claim,
  not an executed one.
- Only `-shared` links were tried. A dynamic executable, `-z now`, `--pack-dyn-relocs`, ifunc
  (`R_SPARC_IRELATIVE`) and canonical-PLT paths through the same layout were not exercised.
- Only `R_SPARC_WPLT30` was used to force PLT creation; the other PLT-forcing relocation types
  (`R_SPARC_PLT32/PLT64/HIPLT22/LOPLT10/PCPLT*`, `R_SPARC_WDISP30`) go through the identical
  `NEEDS_PLT` path in `scan_relocations`, so they were not tried separately.
- The largest link tried was 34000 PLT entries (1236 large). Nothing beyond ~8 blocks was
  exercised, though the arithmetic model covers up to 100000 large entries.
- Worth noting for a future session: mold's own test suite gives this whole architecture zero
  coverage on this host, so any SPARC64 finding here has to be checked by hand-built objects
  the way this candidate did.
