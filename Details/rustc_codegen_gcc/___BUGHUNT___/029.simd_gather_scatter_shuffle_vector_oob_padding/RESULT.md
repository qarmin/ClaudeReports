# simd_gather_scatter_shuffle_vector_oob_padding

Candidate 005 (Confirmed, memory-safety, `shuffle_vector_oob_read_padding`) found that
`Builder::shuffle_vector` (`src/builder.rs:2124-2199`) reads `v2` out of bounds in its
mask-padding branch when `mask_num_units > 2 * vec_num_units`, and left unchecked "whether
simd_gather/simd_scatter ... can hit the same branch" (the code comment at `src/builder.rs:2124-
2125` explicitly calls out `simd_gather` as the reason that branch exists at all). This candidate
checks the internal `gather()` helper in `src/intrinsic/simd.rs` (used by both `sym::simd_gather`
and `sym::simd_scatter`, ~lines 918-964), which calls `bx.shuffle_vector(vector, default, mask)` /
`bx.shuffle_vector(default, vector, mask)` to select between gathered values and the
default/passthrough vector.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (`src/intrinsic/simd.rs`'s `gather()` helper, both call sites at `sym::simd_gather`/`sym::simd_scatter`, and `Builder::shuffle_vector` in `src/builder.rs`) | N/A | `vector`/`default` are both `args[0].immediate()`-typed (or built to match it), and both `simd_gather`'s and `simd_scatter`'s own `require!` checks (`in_len == out_len` / `in_len == out_len2` / `in_len == element_len1` / `in_len == element_len2`) force `pointers`/`mask` to have exactly `in_len` elements before `gather()` is ever called. Inside `gather()`, `vector_type = default.get_type()` (so `vec_num_units == in_len` when `shuffle_vector` later reads it), and the `mask` struct passed to `shuffle_vector` is built by a `for i in 0..in_len` loop, so it always has exactly `in_len` fields (`mask_num_units == in_len`). `shuffle_vector`'s padding branch (and the OOB-reading loop inside it) is gated by `vec_num_units < mask_num_units`, which is `in_len < in_len` - always false. The branch is structurally unreachable through this call site, for any `in_len`. |
| grep for other `shuffle_vector` call sites in `src/intrinsic/simd.rs` (to check whether `simd_scatter`'s own lowering, ~line 1086, makes a separate `shuffle_vector` call of its own) | `grep -n shuffle_vector src/intrinsic/simd.rs src/builder.rs` | Only 3 call sites total in the whole codebase: `Builder::shuffle_vector`'s own definitions (`src/builder.rs:2095`, `:2228`), `simd_shuffle`'s call (`src/intrinsic/simd.rs:460`, candidate 005's own territory), and `gather()`'s two calls (`:960`/`:962`). Read `simd_scatter`'s full lowering block (`src/intrinsic/simd.rs:1086-1210`): it calls the same `gather()` helper (with `invert: true`) to read-merge the current values, then does its own separate `for i in 0..in_len { ... }` write-back loop with no `shuffle_vector` call at all. `simd_scatter` makes no `shuffle_vector` call independent of `gather()`'s. |
| Manual boundary-value repro: real `std::simd::Simd::gather_select_ptr`/`scatter_select_ptr` (public portable-simd API), `in_len` = 4, 8, and 16, with boundary/alternating/mod-3 mask patterns, run against a real built backend, first plain then rebuilt against a backend patched with candidate 005's own `ubsan_hook.patch` (`CG_GCCJIT_UBSAN=1`, forwarding `-fsanitize=undefined` into libgccjit) | Debug build (`CHANNEL=release ./y.sh cargo build`), scratch git worktree at base_commit, reusing the main tree's `build/libgccjit`+`build/build_sysroot` | Correct output and exit 0 in both runs; the UBSan-instrumented run produced zero diagnostics (compare candidate 005's own repro, which reliably produces ~40 diagnostics for its own vulnerable input shape using the identical instrumentation hook). Confirms empirically, for concrete `in_len` values including one (16) not covered by candidate 005's own cases, that `gather()`'s `shuffle_vector` calls never touch the OOB-reading branch. |

Two approaches (code reading + grep, then one boundary-value/UBSan repro) were enough: the code
reading gives a structural proof that the branch is unreachable for any `in_len` (not just the
values tested), and the UBSan run found nothing to contradict it. A third tool (e.g. a fuzz
harness varying `in_len` further) would not add signal here, since the structural argument doesn't
depend on the specific `in_len` value at all - `vec_num_units == mask_num_units` holds by
construction for every `in_len`.

## Status

**Not reproducible.** `gather()`'s `shuffle_vector(vector, default, mask)` /
`shuffle_vector(default, vector, mask)` calls always have `vec_num_units == mask_num_units ==
in_len` (enforced both by `simd_gather`/`simd_scatter`'s own `require!` argument-length checks and
by how `gather()` itself builds `vector`/`default`/`mask`), so `shuffle_vector`'s
`vec_num_units < mask_num_units` padding branch - the only branch that can reach the OOB-reading
loop candidate 005 found - is never entered through this call site, for any `in_len`. Verified
both by reading the code (a structural argument that holds for every `in_len`, not just the ones
tested) and by an actual UBSan-instrumented run at `in_len` = 4, 8, and 16 with three different
mask patterns, which produced zero diagnostics in every case.

`simd_scatter`'s own lowering (distinct from candidate 006's `type_uint_from_ty`/`vector_ty` call)
makes no `shuffle_vector` call of its own at all - it calls the same `gather()` helper (ruled out
above) and then does a plain unconditional write-back loop with no shuffle/select of any kind.

## Class

N/A (not reproducible)

## Root cause

Not applicable to this candidate as a bug - documenting instead why the suspected spot is safe:
`shuffle_vector`'s vulnerable branch requires the mask to be *longer* than the input vectors
(`vec_num_units < mask_num_units`, and, for the actual OOB read, `mask_num_units > 2 *
vec_num_units`). `gather()` never constructs a mask longer than its input vectors - by
construction, the mask it builds always has exactly as many elements as `vector`/`default`
(`in_len`, the same length enforced on every argument to `simd_gather`/`simd_scatter` by their own
monomorphization checks before `gather()` is ever called). Candidate 005's own vulnerable case
(portable-simd's `simd_swizzle!`/`Swizzle::swizzle`) is different in exactly this respect: it
deliberately broadcasts a *shorter* input to a *longer* mask/output, which is the one shape
`gather()` never produces.

## Introduced by

N/A - not reproducible, no bug to attribute.

## The fix

N/A - nothing to fix; no bounded fix applicable.

- Project test suite: not run against a fix (none needed). No regression check needed beyond the
  boundary-value repro above, since no source change was made for this candidate.

## What's not done / open questions

- This rules out `gather()`'s two `shuffle_vector` calls and confirms `simd_scatter` makes no
  `shuffle_vector` call of its own - together with candidate 005 (which already covers
  `simd_shuffle`'s call site, the only other one in the codebase), this accounts for every
  `shuffle_vector` call site that exists in the project as of this commit. No other code path was
  left unchecked for this specific bug class.
- Not relevant to this candidate, but noticed while reading the same code: `gather()` itself has
  an unrelated, already-confirmed memory-safety issue (candidate 008,
  `simd_gather_scatter_unconditional_masked_access`) - it dereferences/writes through every lane's
  pointer unconditionally before the mask/shuffle is applied, so a disabled lane's pointer being
  null/dangling still faults. That is a separate bug from the one this candidate checked (which is
  specifically about `shuffle_vector`'s own padding-branch OOB read, not about unconditional
  pointer access) and was already recorded before this session.
