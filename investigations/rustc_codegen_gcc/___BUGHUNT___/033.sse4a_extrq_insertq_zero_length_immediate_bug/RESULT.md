# sse4a_extrq_insertq_zero_length_immediate_bug

Suspect: `_mm_extracti_si64::<0, 0>`/`_mm_inserti_si64::<0, 0>` (SSE4a `EXTRQ`/`INSERTQ` with a
compile-time `LEN`/`IDX` immediate). Per the x86 ISA and stdarch's own doc comment on these
intrinsics, a `LEN` immediate of `0` is a special-cased sentinel meaning "64 bits" (the full
quadword) - not literally zero bits. The hypothesis was that this backend's mapping of the LLVM
intrinsic names `llvm.x86.sse4a.extrqi`/`llvm.x86.sse4a.insertqi` to the matching GCC builtins
(`src/intrinsic/archs.rs:8914-8917`) mishandles the `LEN == 0` case, truncating the extract to too
few bits / the insert to too many left untouched.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: located the only place these two LLVM intrinsic names are handled in this crate | `grep -rn "extrq\|insertq"` across `src/` | `src/intrinsic/archs.rs:8914-8917` is a pure string-to-string name table (`"sse4a.extrqi" => "__builtin_ia32_extrqi"`, etc.) with no immediate-argument logic anywhere in this crate - the `LEN`/`IDX` values passed by stdarch's `extrqi`/`insertqi` FFI shims flow through unchanged to the GCC builtin call. No custom bit-twiddling/masking of `LEN`/`IDX` exists in this backend to mishandle |
| Manual boundary-value construction: a real `#[target_feature(enable = "sse4a")]` function calling `_mm_extracti_si64::<0, 0>`/`_mm_inserti_si64::<0, 0>` on real SSE4a hardware (host CPU has `sse4a` per `/proc/cpuinfo`), single fixed input pair, compiled via `CHANNEL=release ./y.sh rustc`, `release` profile | Correct result: extract returned the full unchanged 64-bit low quadword, insert fully replaced the low quadword - matched the expected "LEN=0 means 64" semantics, no truncation observed |
| Differential sweep against the LLVM backend as oracle: the same repro (`repro/zero_len.rs`) compiled once with plain `rustc -O` (LLVM) and once with `CHANNEL=release ./y.sh rustc` (this backend, same optimization intent), run over 8x8=64 fixed boundary value pairs (0, `u64::MAX`, alternating patterns, single-bit, high/low half masks) plus 2000 pseudo-random pairs each for extract and insert (4128 total checks), every operand wrapped in `std::hint::black_box` so the immediate-0 call can't be constant-folded away by either backend's optimizer | LLVM oracle: 4128/4128 correct (0 failures). This backend: 4128/4128 correct (0 failures) - byte-for-byte identical results to the LLVM oracle on every case, see `output.txt` |
| Disassembly check of the compiled binary (`objdump -d`) to confirm the actual instruction emitted is the real hardware `extrq $0x0,$0x0` / `insertq $0x0,$0x0`, not some software bit-mask emulation that could plausibly mishandle the literal-0 special case | Same binary compiled through this backend | Confirmed: real SSE4a hardware instructions `extrq $0x0,$0x0` / `insertq $0x0,$0x0` are emitted with the literal immediate `0`, exactly mirroring what the LLVM backend also emits for the same source - the CPU's own hardware, not this backend's code, is what applies the ISA's "0 means 64" special case, and it does so correctly |

## Status

**Not reproducible.** Covered: the specific `LEN=0, IDX=0` boundary case by hand, then a 4128-case
differential sweep (64 fixed boundary-value pairs + 4000 pseudo-random pairs) for both
`_mm_extracti_si64::<0, 0>` and `_mm_inserti_si64::<0, 0>` against a real LLVM-backend oracle on
real SSE4a hardware, plus a disassembly-level check confirming the real hardware instruction (not
a software emulation this crate could have gotten wrong) is what actually executes. Zero mismatches
in every approach.

**Fully reproducible**: N/A (not applicable - no reproduction to characterize).

## Root cause (why the suspect turned out to be safe)

The candidate's premise assumed this backend's own Rust code does some translation/masking of the
`LEN`/`IDX` immediates on the way from stdarch's `extrqi`/`insertqi` FFI shims to the underlying
GCC builtin. In reality, `src/intrinsic/archs.rs`'s `sse4a.extrqi`/`sse4a.insertqi` entries are a
bare name-substitution table (LLVM intrinsic name string -> GCC builtin name string) with no
immediate-argument logic at all - the call, including its `LEN`/`IDX` arguments, is forwarded
verbatim to `__builtin_ia32_extrqi`/`__builtin_ia32_insertqi`, which libgccjit lowers directly to
the real `extrq`/`insertq` x86 instructions with the immediate values baked in as-is. The "`LEN==0`
means 64" special case is therefore entirely the responsibility of the x86 CPU's own hardware
implementation of these instructions (per the ISA), not any software logic in this crate - there is
no place in this backend's own code where that special case could be applied incorrectly, because
there is no software-side interpretation of the immediate at all.

## Introduced by

N/A - not applicable, no bug was confirmed.

## The fix

No fix - nothing to fix; see Status and Root cause. There is no code path in this backend that
processes the `LEN`/`IDX` immediates, so there is nothing to patch.

- Project test suite: N/A - no fix was applied, so no before/after comparison was run.

## What's not done / open questions

- Only the `LEN=0, IDX=0` combination (the one the candidate specifically named) was swept
  extensively. Other boundary combinations of `LEN`/`IDX` (e.g. `LEN=64` is out of range per
  stdarch's own `static_assert_uimm_bits!(LEN, 6)`, so not applicable; `LEN+IDX==64` edge cases)
  were not separately swept, since the candidate's specific hypothesis was about the `LEN==0`
  sentinel case only and the underlying mechanism (verbatim builtin-name forwarding, no immediate
  logic in this crate) applies identically regardless of the specific immediate values chosen.
- `_mm_extract_si64`/`_mm_insert_si64` (the non-immediate, runtime-`y`-operand variants, mapped by
  the same table's `"sse4a.extrq" => "__builtin_ia32_extrq"` / `"sse4a.insertq" =>
  "__builtin_ia32_insertq"` entries) were not separately tested - same reasoning applies (no
  immediate-argument logic in this crate for those either), but they use a runtime bit-packed
  operand rather than a compile-time immediate, so a `LEN==0`-at-runtime differential sweep of
  those specifically was not run this session.
- This backend's pinned/downloaded `libgccjit.so` snapshot (`download-gccjit = true`) is one
  specific GCC build; a different GCC version's `__builtin_ia32_extrqi`/`insertqi`
  lowering could in principle differ, but that would be a libgccjit/GCC bug, not one in this
  crate's own code, and is out of scope for this candidate.
