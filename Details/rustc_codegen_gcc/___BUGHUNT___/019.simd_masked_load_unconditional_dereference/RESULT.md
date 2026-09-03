# simd_masked_load_unconditional_dereference

`sym::simd_masked_load`'s lowering (`src/intrinsic/simd.rs`, around lines 1573-1584) builds each
lane via `let then_val = bx.context.new_array_access(None, pointer, i).to_rvalue();` and only
afterward calls `bx.select(mask, then_val, else_val)`. Read at the Rust-source level, this looks
identical to the confirmed root cause of candidate 008
(`simd_gather_scatter_unconditional_masked_access`, memory-safety): a value that is seemingly
computed before the mask is even consulted. The intrinsic's own doc comment ("The memory addresses
corresponding to the "off" lanes are not accessed") plus `simd_masked_store`'s already-fixed
sibling implementation a few hundred lines below (using real `then_block`/`after_block` control
flow) made this look like the same bug class, not yet applied to the load side.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: `std::simd::Simd::<i32,4>::load_select_ptr` (real public, stable-shaped `#![feature(portable_simd)]` API - the actual entry point that lowers to `simd_masked_load`), one lane disabled with its address landing exactly on a `mprotect(PROT_NONE)` guard page immediately after 3 valid, enabled lanes | debug build (`CHANNEL=release ./y.sh cargo build --manifest-path repro/Cargo.toml --bins`, debug profile binary), 5 runs | Correct result `[10, 20, 30, -1]`, exit 0, 5/5 runs - the guard page is never touched |
| Same API, all 4 lanes disabled, base pointer from a page that was `mmap`'d then immediately `munmap`'d (fully unmapped, not just protected) | Same build, 3 runs | Correct result `[-1, -2, -3, -4]`, exit 0, 3/3 runs - the unmapped pointer is never dereferenced |
| `objdump -d` on the compiled `load_select_ptr` (inlined intrinsic body) in the debug binary produced by the first repro | Static disassembly, no extra build | Shows a real `test %eax,%eax; je <skip>; <vmovd (mem)>; jmp <after>; <skip>: <fallback>` sequence for **every one of the 4 lanes** - i.e. a genuine conditional branch guarding each lane's memory load, not an unconditional load followed by a data-select |

Only 3 approaches were used (matching the soft cap), but the third (disassembly) gave a definitive,
mechanistic explanation for why the first two produced clean negatives, so no further tool was
needed.

## Status

**Not reproducible.** Both boundary-value repros (single disabled lane against a guard page, all
lanes disabled against a fully unmapped pointer) ran cleanly 5/5 and 3/3 respectively with correct
results and no crash, and the disassembly of the actual generated code confirms why: each lane's
load is inside a real conditional branch, never executed when that lane's mask bit is zero.

## Root cause (why the suspect turned out to be safe)

The candidate's hypothesis was based on a source-level misreading of what
`bx.context.new_array_access(None, pointer, i).to_rvalue()` actually does. In libgccjit's object
model, this call does not perform a memory read at the point it's called - it constructs an
*unevaluated* expression-tree node ("the lvalue `pointer[i]`, as an rvalue"). That node only turns
into an actual load instruction once it is placed into a statement that itself belongs to some
basic block. `Builder::select` (`src/builder.rs:1554-1581`) is what actually places `then_val`
into a statement - specifically, `then_block.add_assignment(self.location, variable, then_val)` -
and `then_block` is one of three **real, freshly created basic blocks** (`then_block`, `else_block`,
`after_block`) wired together with `end_with_conditional`. So the sequence in
`simd_masked_load`'s loop:

```rust
let mask = /* cmp != 0 */;
let then_val = bx.context.new_array_access(None, pointer, i).to_rvalue(); // build the AST node, no load yet
let else_val = bx.context.new_vector_access(None, default, i).to_rvalue();
let element = bx.select(mask, then_val, else_val); // THIS is where then_val's load actually gets emitted, inside then_block
```

only emits the load itself inside `then_block`, which is only entered when `mask` is true for that
lane. This is confirmed directly by the disassembly in `output.txt`: each of the 4 lanes gets its
own `test`/`je`/load/jump sequence, i.e. 4 independent conditional loads, exactly matching a
correct masked load.

This is a structurally different situation from candidate 008's actual (fixed) bug: the old
`gather()` helper computed every lane's dereference in a single, unconditional, sequential
`for i in 0..in_len { ...; values.push(value); }` loop with **no blocks or branches created at
all** - the mask was only consulted afterward, as a data-level `shuffle_vector` select over
already-computed (already-faulted, if unsafe) values. `simd_masked_load`'s loop instead calls
`bx.select()` per lane, which - despite `then_val` being constructed as a Rust-level local
variable "before" the call - defers the actual load into a block that `select()` itself creates
and gates on the mask, giving it the same real conditional-access property as the already-correct
`simd_masked_store` sibling, just expressed with `select()`'s implicit blocks instead of
`simd_masked_store`'s explicit `then_block`/`after_block` calls. The "unconditional access, select
afterward" pattern that made candidate 008 a real bug depends on the access actually happening in
a block that always executes; `simd_masked_load`'s call shape doesn't have that property, even
though the Rust source ordering (`let then_val = ...` before `bx.select(...)`) suggested it might.

## Introduced by

N/A - not applicable, no bug was confirmed.

## The fix

No fix - nothing to fix. No `patch.diff` in this candidate folder.

- Project test suite: not run for this candidate - no source change was made (see above), so there
  is nothing to regress and no before/after to compare. N/A.

## What's not done / open questions

- Only tested with 4-lane `i32` vectors and one guard-page layout (single trailing disabled lane,
  and all-disabled). Wider vectors, other element types/widths, and a "disabled lane before the
  enabled ones" layout (guard page preceding valid memory) were not separately constructed, but
  given the disassembly shows the same `test`/`je`-gated pattern independently per lane index with
  no dependency on lane position, there is no structural reason to expect a different result for
  those variants.
- Only checked on `x86_64-unknown-linux-gnu`, this checkout's pinned libgccjit snapshot
  (`4cd7da5f9a5d9509b34ac0a766424ccdae706d33`'s `build/libgccjit`), and a debug-profile repro
  binary (`CHANNEL=release ./y.sh cargo build`, no `--release` on the repro crate itself). Not
  re-checked with an optimized repro binary or a different libgccjit version.
- This result is specific to `simd_masked_load`. It does not re-validate `simd_masked_store` (already
  fixed prior to this session, uses the same real-block pattern explicitly) or `simd_gather`/
  `simd_scatter` (candidate 008, separately confirmed and fixed for the actual unconditional-loop
  shape).
