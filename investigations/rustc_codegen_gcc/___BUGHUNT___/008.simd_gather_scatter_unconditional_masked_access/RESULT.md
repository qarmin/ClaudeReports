# simd_gather_scatter_unconditional_masked_access

`simd_gather`/`simd_scatter` lowering (`src/intrinsic/simd.rs`, the `gather()` helper used by both
`sym::simd_gather` and `sym::simd_scatter`) unconditionally dereferences/writes through every
lane's pointer, then selects which results to keep via `shuffle_vector` - instead of skipping the
memory access for disabled lanes entirely. This contradicts the documented safety contract of
these intrinsics (a disabled lane's pointer may be invalid/dangling and must not be touched) and
of the public wrappers that expose them (`std::simd::Simd::gather_select_ptr`/`scatter_select_ptr`:
"If a pointer is disabled, ... no read is performed" / "the write to its pointee is skipped").
Source: GitHub issue #640.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (`src/intrinsic/simd.rs`, `gather()` helper + both call sites) | N/A | Confirmed the loop dereferences `pointers[i]`/writes to `*pointers[i]` for every `i` in `0..in_len` unconditionally, with the mask only used afterward as a `shuffle_vector` select index - exactly the pattern issue #640 describes. |
| Manual boundary-value repro via `std::simd::Simd::gather_select_ptr`/`scatter_select_ptr` (public, stable-shaped `#![feature(portable_simd)]` API, real end-to-end entry point - not a hand-built intrinsic call) with a disabled lane's pointer set to `std::ptr::null()`/`null_mut()`, `CHANNEL=release ./y.sh cargo build` against a real built backend `.so`, debug build | Segfaults deterministically (3/3 gather, 3/3 scatter, 3/3 on an 8-lane variant with all-disabled and boundary-disabled masks) on the pristine backend; 3/3 clean passes with the fix applied (see below) | **Reproduced** |
| Differential check against host `rustc +nightly-2026-08-04` (real LLVM backend), same source file, same `-O` | Runs to completion, prints the expected merged result, exit 0 | Confirms the crash is specific to this backend's codegen, not a general property of the (deliberately real, per-contract) input |

Only 2 of the soft cap of 3 approaches were needed - the second one gave an immediate, deterministic
signal (a segfault matching the exact mechanism read from the source), so a third tool (e.g. ASAN)
would have added nothing here.

## Status

**Confirmed**. Reproduces 3/3 runs for each of three repro binaries (`gather_scatter_repro`,
`scatter`, `gather8`), debug build (`./y.sh cargo build`, no extra sanitizer needed - a plain
segfault is enough of a signal here), against a fresh rebuild of the repro binaries each time the
backend `.so` was rebuilt (a stale-binary trap was hit and caught mid-session: the first "after"
run against the fixed backend still segfaulted because the repro binaries hadn't been rebuilt after
flipping the backend - re-running `CHANNEL=release ./y.sh cargo build --manifest-path
repro/Cargo.toml --bins` before each measurement resolved it).

**Fully reproducible**: **Yes**. The repro calls the real public API (`std::simd::Simd::
gather_select_ptr`/`scatter_select_ptr`, gated by the same `#![feature(portable_simd)]` any real
caller would use) with a genuinely disabled lane whose pointer is null - exactly the input shape
the intrinsic's own documented contract says must be safe. There is no second participant to fake
here (per `CONFIRMATION.md` section 11): the "input" is a single Rust program's own pointer/mask
values, not something produced by a separate peer/caller across a boundary.

## Class

memory-safety

## Root cause

`gather()` (the shared helper backing both `simd_gather` and `simd_scatter`) loads every lane's
value unconditionally before doing any masking:

```rust
for i in 0..in_len {
    let index = bx.context.new_rvalue_from_long(bx.i32_type, i as i64);
    let int = bx.context.new_vector_access(None, pointers, index).to_rvalue();
    let ptr_type = elem_type.make_pointer();
    let ptr = bx.context.new_bitcast(None, int, ptr_type);
    let value = ptr.dereference(None).to_rvalue();   // <-- dereferences EVERY lane's pointer
    values.push(value);
}
```

The resulting vector is only *afterward* combined with the `default`/`or` vector via a computed
`shuffle_vector` mask, so a disabled lane's dereference has already happened by the time the mask
is consulted - if that lane's pointer is null, dangling, or otherwise not safe to read (which the
whole point of a mask argument is to allow), the load itself faults before the select ever
discards it.

`simd_scatter` compounds this: it calls the same `gather()` (with `invert: true`) to first *read*
the current value at every pointer (again unconditionally, same bug) merged with the
values-to-store, then loops a second time and *writes* the merged result back to every pointer
unconditionally - so a disabled lane's pointer is both read from and written to, even though the
intrinsic's contract says "no memory access to that location is made" for disabled lanes.

This is architecturally the "unconditional access + select" shape - not a sign/type mismatch like
most of this project's other confirmed candidates (001-006) - and it's a real memory-safety bug,
not just a missed-optimization: the mask exists specifically so callers can pass pointers that are
only valid for the *enabled* lanes.

## Introduced by

not determined - the `gather()`/`simd_gather`/`simd_scatter` code has been present, in this same
unconditional-load-then-shuffle shape, since before this repository's own tracked history began
tracking it as a standalone file history (`git log --follow src/intrinsic/simd.rs` bottoms out at
`afae271 "Initial commit"`, a large initial import, and `git log -S"fn gather"` only turns up
subtree-sync merge commits, not an original authoring commit) - not worth a full `git bisect`
this session since the bug is a structural property of the code as originally written, not a
regression introduced by a later change.

## The fix

Rewrote `gather()` to use real control flow (a `then`/`after` basic-block pair per lane, mirroring
the pattern the already-correct `simd_masked_store` intrinsic uses a few hundred lines below in the
same file) instead of "load every lane, select afterward": each lane's pointer is only
dereferenced inside a block that's only entered when that lane's mask value is non-zero; a disabled
lane's local result variable keeps the `default`/`or` value and its pointer is never touched.
`simd_scatter` was changed to stop reusing `gather()` altogether (the previous "read via masked
gather, then write back the merged result unconditionally" trick could never be sound, since a
disabled lane's read is exactly as invalid as its write) and instead has its own direct
guarded-write loop with the same per-lane conditional-block shape, so a disabled lane's pointer is
neither read from nor written to.

Verified against more than the original triggering input, per `CONFIRMATION.md` section 7: the
original single-disabled-lane case (both gather and scatter), plus an 8-lane variant with (a) every
lane disabled and every pointer null, and (b) only the first and last lanes disabled ("boundary"
lanes) with null pointers and interior lanes enabled and dereferenced normally - all three cases
segfault 3/3 on the pristine backend and pass cleanly 3/3 after the fix, matching the values a real
`rustc`+LLVM build of the same source produces.

- Project test suite (`./y.sh test --release --mini-tests`, `--test-libcore`, `--cargo-tests`):
  identical results before and after the fix - `mini-tests` runs to completion producing the same
  output, `--test-libcore` 578 passed / 0 failed / 0 ignored (both), `--cargo-tests` 41 passed / 0
  failed / 1 ignored (both, for both the `lang_tests` and the run-test variant it runs twice).
  None of these exercise `simd_gather`/`simd_scatter` (no existing test in this project's suite
  covers these two intrinsics at all), which is consistent with this bug going unnoticed - the fix
  changes behavior only for code paths the existing suite never reaches, so an unchanged pass count
  before/after is exactly the expected outcome, not evidence the fix is untested.
  `--extended-tests` was not run (see `___BUGHUNT___/README.md`'s parallel-worktree guidance - this
  session ran alongside other candidates' worktrees sharing `build/rand`/`build/regex`/
  `build/simple-raytracer`).

## What's not done / open questions

- **`simd_masked_load`** (same file, a few hundred lines above `simd_masked_store`) has the exact
  same "unconditional load then select" shape as the old `gather()` - `simd_masked_store` was
  already fixed to use the correct conditional-block pattern reused here, but `simd_masked_load`
  was not. Not fixed as part of this candidate (out of scope - a distinct intrinsic, distinct issue
  number if any), but it is the same bug class and should be checked/fixed separately.
- The "possibly assumes vector length is a power of 2" half of the original suspicion (from issue
  #640's title/description) was not specifically confirmed as its own separate defect - the
  original `shuffle_vector`-based implementation's `mask_types`/`mask_values` construction did not,
  on inspection, appear to assume a power-of-2 length (it built one struct field/mask value per
  `in_len`, with no shift/bitmask sized to a power of 2); this is superseded anyway now that the
  fix removes the `shuffle_vector`-based merge entirely for both `simd_gather` and `simd_scatter`.
  Not independently re-tested with a non-power-of-2 lane count, since `std::simd::Simd<T, N>` only
  supports power-of-2 `N`, making a non-power-of-2 repro through the public API impractical without
  a custom `#[repr(simd)]` type and unstable-internals-level test scaffolding.
- Only tested on `x86_64-unknown-linux-gnu`. Not checked on other targets this backend supports.
- A longer fuzzing campaign (varying lane count, element type, pointer nesting depth, and
  enabled/disabled patterns systematically) was not run - the boundary-value cases tried here
  (single lane, all lanes, first/last lanes) already demonstrate the bug and its fix hold across
  the shape of input most likely to matter, but a `fuzz-bug-repro` campaign (run separately, not by
  this skill) could still turn up an edge case in, e.g., very wide vectors or multi-level pointer
  nesting (`pointer_count > 1`) that wasn't specifically exercised here.
