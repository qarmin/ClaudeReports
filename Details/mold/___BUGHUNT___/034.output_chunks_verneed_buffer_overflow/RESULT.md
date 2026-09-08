# output_chunks_verneed_buffer_overflow

`VerneedSection<E>::construct()` (`src/output-chunks.cc:3088-3145`) sizes its scratch buffer for
`.gnu.version_r` as `(sizeof(ElfVerneed<E>) + sizeof(ElfVernaux<E>)) * (syms.size() + 1)` - i.e. it
budgets **one** `ElfVernaux` per versioned symbol - but the writer immediately below can emit **two**
`ElfVernaux` records for a symbol that starts a new group, because `start_group` also emits an extra
`GLIBC_ABI_DT_RELR` entry when relative-relocation packing is in play. Writing then runs past the
allocation.

This candidate came out of a **discovery sweep** rather than a specific flagged line: the assignment
was to hunt the output/layout side of mold generally (never examined by this project before - all 22
previously confirmed bugs were input-parsing), for the shape "a size computed by one function and a
layout/write performed by another, which can disagree". This is the first output-side bug confirmed
here.

> **Provenance note.** The agent investigating this candidate was stopped by the user before it
> wrote this file, but *after* it had completed every investigative step (repro, gdb, ASAN,
> repeatability, controls, post-patch verification, and the before/after test-suite runs). This
> `RESULT.md` was written by the session orchestrator strictly from the evidence captured in
> `output.txt` (reproduced verbatim there) plus a `git log -S` lookup for the introducing commit -
> no step was re-run and nothing here is inferred beyond what `output.txt` shows.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Swept output chunk classes comparing each `update_shdr`/sizing expression against its `copy_buf`/writer (the method candidate 030 used on SPARC64 `.plt`) | source reading | Found the `VerneedSection::construct` allocation budget (one `ElfVernaux` per symbol) disagrees with what `start_group` can emit (up to two, via the extra `GLIBC_ABI_DT_RELR` entry) |
| Synthesized inputs: N versioned DSOs each contributing a versioned symbol, linked with `-z pack-relative-relocs`; swept N = 1,2,3,4,8,16 tracking allocated-vs-written bytes | plain Debug build (`-DCMAKE_BUILD_TYPE=Debug`) | Written exceeds allocated from **N=3** onward (N=3: 128 allocated / 144 written; N=8: 288/384; N=16: 544/768). N>=3 aborts, exit 134 |
| `gdb` backtrace of the N=8 abort | plain Debug build | The OOB writes themselves are silent; **mimalloc**'s own heap-integrity assertion (`free.c:566`, `mi_page_usable_size_of`) trips later, when `contents.resize(384)` at `output-chunks.cc:3144` frees the 288-byte block whose tail was overwritten. Backtrace runs through `VerneedSection<X86_64>::construct` (`output-chunks.cc:3144`) <- `mold_main` (`main.cc:624`) |
| Same N=8 input under ASAN+UBSAN | `build-asan`, `-DMOLD_USE_ASAN=ON`, g++ 15.2.0 | **`AddressSanitizer: heap-buffer-overflow`, WRITE of size 2**, naming both ends precisely: the write is `verneed->vn_version = 1;` inside `start_group` (`output-chunks.cc:3123`), and the region is the 288-byte allocation made at `output-chunks.cc:3092` - the `contents.resize(...)` line itself. Address is `0 bytes after` the region |
| Repeatability + controls | ASAN build, N=8 | ASAN heap-buffer-overflow in **5/5** runs. Control 1: N=2 (where written == allocated) exits 0. Control 2: N=8 **without** `-z pack-relative-relocs` exits 0 - confirming the extra `GLIBC_ABI_DT_RELR` entry is what exhausts the budget |
| Re-ran the full sweep after applying `patch.diff` | plain Debug build, rebuilt | N = 1,2,3,4,6,8 all exit 0. Plus a mixed same-class input (4 `libc.so.*` DSOs with two `GLIBC_2.*` versions each + 3 ordinary versioned DSOs, 14 versioned symbols) exits 0, and `readelf` parses the resulting `.gnu.version_r` cleanly (7 entries, correct `GLIBC_ABI_DT_RELR`/`GLIBC_2.x`/`OTHER_x` structure) - so the fix changes only how much scratch space is reserved, not the section's contents |
| Project test suite before and after | `ctest --test-dir build-plain -j4` | **476/476 before, 476/476 after** |

## Status

**Confirmed.** ASAN reports the heap-buffer-overflow write in **5/5 runs** at N=8 on the ASAN build,
and the plain Debug build aborts deterministically (exit 134 via mimalloc's heap assertion) for every
N >= 3. Two controls isolate the cause: the same input at N=2 is clean, and the same N=8 input without
`-z pack-relative-relocs` is clean.

**Fully reproducible**: **Yes.** The repro drives the real `mold` binary end-to-end via its real CLI
on real compiler-produced DSOs; nothing is hand-faked. Single-participant finding (the inputs are the
external artifact), so `CONFIRMATION.md` section 11's two-participant bar doesn't apply.

## Class

memory-safety (heap-buffer-overflow **write** - note this is a write, unlike almost every other
confirmed bug in this project, which are OOB reads; it corrupts an adjacent heap block, detected here
only because mimalloc later validates the block on free)

## Root cause

```cpp
// src/output-chunks.cc:3088-3092
contents.resize((sizeof(ElfVerneed<E>) + sizeof(ElfVernaux<E>)) *
                (syms.size() + 1));
```

The budget allows one `ElfVerneed` + one `ElfVernaux` per versioned symbol (plus one spare set). But
the writer's `start_group` lambda emits an `ElfVerneed` **and** can emit an extra `ElfVernaux` for
`GLIBC_ABI_DT_RELR` (added when the output uses packed relative relocations) *before* the symbol's own
`ElfVernaux` is written - so a symbol that both starts a group and triggers the `GLIBC_ABI_DT_RELR`
entry consumes one `ElfVerneed` + two `ElfVernaux`. Once enough symbols start their own groups, the
cumulative writes pass the end of the allocation, and `verneed->vn_version = 1;`
(`output-chunks.cc:3123`) writes 2 bytes just past it.

The sizing and the writing live in the same function here, only ~30 lines apart - which is precisely
why this survived: it doesn't look like the classic "two functions disagree" split the sweep was
hunting for, but it is exactly that bug in miniature.

## Introduced by

f467ad1a "Create a symbol version dependency to GLIBC_ABI_DT_RELR" (2023-08-09) - the commit that
introduced the extra `GLIBC_ABI_DT_RELR` vernaux entry without widening the allocation budget that
predates it. (`git log -S'GLIBC_ABI_DT_RELR'` finds only this commit plus a later test tweak,
7734a932; `git log -S` on the `resize` expression itself bottoms out at the 5b859497 directory
reorganization, i.e. the budget expression is older than the entry that overruns it.)

## The fix

Widen the budget to two `ElfVernaux` records per symbol, matching what the writer can actually emit:

```diff
-  // Allocate a large enough buffer for .gnu.version_r.
-  contents.resize((sizeof(ElfVerneed<E>) + sizeof(ElfVernaux<E>)) *
-                  (syms.size() + 1));
+  // Allocate a large enough buffer for .gnu.version_r. Each symbol can at
+  // most start a new group, and start_group below may emit an extra
+  // GLIBC_ABI_DT_RELR entry for every group, so budget one ElfVerneed and
+  // two ElfVernaux records per symbol.
+  contents.resize((sizeof(ElfVerneed<E>) + sizeof(ElfVernaux<E>) * 2) *
+                  syms.size());
```

Verified: every previously-aborting N (1,2,3,4,6,8) exits 0 after the patch, a mixed same-class input
(14 versioned symbols across 7 DSOs, mixing `libc.so.*` and ordinary DSOs) links cleanly, and
`readelf` parses the resulting `.gnu.version_r` correctly - so only the reserved scratch size changed,
not the emitted section. Project test suite: **476/476 before, 476/476 after**.

## What's not done / open questions

- The sweep that produced this finding examined output chunk classes for the sizing-vs-writing
  divergence shape; a full inventory of which chunks were compared and cleared was not written before
  the investigating agent was stopped, so **the rest of the output side remains only partially
  swept** - a future session should treat `output-chunks.cc` as still-fertile rather than covered.
- The fix widens the budget rather than making the writer and the sizing share a single source of
  truth. A maintainer may prefer computing the exact count (walking the same grouping logic once to
  count, then allocating), which would be robust against a future third vernaux entry being added the
  same way `GLIBC_ABI_DT_RELR` was.
- Not investigated whether any other output chunk emits conditional extra entries against a fixed
  per-symbol budget - that is the same bug class and the obvious next question.
