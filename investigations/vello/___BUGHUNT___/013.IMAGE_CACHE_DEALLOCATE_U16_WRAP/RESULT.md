# ImageCache::deallocate's u16 recomputation of padded size vs. allocate's u32-checked value

`sparse_strips/vello_common/src/image_cache.rs`'s `ImageCache::deallocate` recomputes
`padded_width`/`padded_height` as plain `u16` arithmetic (`image_resource.width +
image_resource.padding * 2`), instead of storing/reusing `allocate`'s own `u32`-checked
`padded_width`/`padded_height` value. This relies on the (type-system-unenforced) invariant that
every `ImageResource` reachable from `deallocate` was actually produced by `allocate`, which
already proved `width + 2*padding <= u16::MAX` via a `u32` computation (checked with
`u16::try_from`) before ever constructing the resource. The suspicion: if a future code path ever
constructed or mutated an `ImageResource` bypassing that check, `deallocate`'s `u16` arithmetic
could overflow (panic in a debug/overflow-checked build, silently wrap in a plain release build).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Mathematical/invariant reasoning: since `allocate` only ever constructs an `ImageResource` after `u16::try_from(u32::from(width) + u32::from(padding) * 2)` succeeds, `width + 2*padding <= 65535` holds for every resource that exists; since `width >= 0`, this implies `padding <= 32767`, so `padding * 2 <= 65534` can never overflow `u16` either as an intermediate step or in the final sum | Static, no build | Proves the `u16` recomputation in `deallocate` cannot overflow for any `ImageResource` actually produced by `allocate` - the invariant that would need to be violated for a real bug is enforced entirely inside `allocate` before construction |
| Codebase-wide grep for any other construction/mutation path for `ImageResource`: struct literals (`ImageResource {`), a `get_mut`/other mutable accessor on `ImageCache`, and every crate (`vello_hybrid`'s `webgl`/`wgpu` backends, `vello_cpu`) that touches `image_cache` | `grep -rn "ImageResource {"` and `grep -rn "image_cache"` across `sparse_strips/` | The only `ImageResource {' construction site in the whole workspace is inside `allocate` itself; `atlas_alloc_id` is a private field so no other crate can construct one via struct literal even if it wanted to. `ImageCache` exposes only `get(&self) -> Option<&ImageResource>` (shared reference, no mutable access) alongside `allocate`/`deallocate`; every real caller (`vello_hybrid`'s `webgl`/`wgpu` render backends, `vello_cpu`'s text/glyph-atlas code) only ever calls `allocate`, `get`, and `deallocate` - never anything that could plant a resource with an invalid `width`/`padding` combination |
| Manual boundary-value construction through the real public API, debug-assertions build (`cargo test`, default dev profile with `overflow-checks = true`): a temporary `#[ignore]`d unit test swept `width` in `{0, 1, 2, 32767, 65534, 65535}` x `padding` in `0..=65535`, calling `allocate`+`get`+`deallocate` for every combo `allocate` accepted, atlas_size = `(u16::MAX, u16::MAX)` | debug build (`overflow-checks = true` by default), `cargo test -p vello_common --lib image_cache -- --ignored --nocapture`, in the isolated worktree (test removed afterwards, never committed) | 114,689 accepted `(width, padding)` combos exercised through `allocate`+`get`+`deallocate`, zero panics/overflows (see `output.txt`) |
| Standalone `repro/` crate (real, public `vello_common::image_cache` API only - `ImageCache::allocate`/`get`/`deallocate`), `overflow-checks = true` forced in both `dev` and `release` Cargo profiles, sweeping `width` in `{0, 1, 2, 3, 32766, 32767, 32768, 65535}` x `padding` in `0..=65535`, at both `atlas_size = (u16::MAX, u16::MAX)` and the realistic default `(4096, 4096)` | `cargo run --release` and `cargo run` (dev), against a fresh clone of this repo at `7516831426fb70f36d62c521c5477f6a7265b36a` via `just test` | 180,224 accepted combos at the `u16::MAX` atlas size, 8,190 at the default `4096x4096` atlas size, zero panics/overflows in either build, repeated 3/3 identical runs (see `output.txt`) |
| `git log -S`/`git show` to find which commit introduced this exact `u16` recomputation in `deallocate` | `git log -S "image_resource.width + image_resource.padding" -- sparse_strips/vello_common/src/image_cache.rs` | Exactly one match: 80062c1a "vello_hybrid: Use `u16` consistently throughout the codebase (#1826)". Before that commit, `deallocate` computed `padded_width`/`padded_height` as `image_resource.width as u32 + u32::from(image_resource.padding) * 2` (already `u32`, matching `allocate`'s own u32-checked computation at the time); that commit changed `width`'s field/parameter type from `u32` to `u16` throughout and, as part of that, rewrote `deallocate`'s recomputation to plain `u16` arithmetic instead of continuing to widen to `u32` first |

No `miri`/ASAN/fuzzing was used - this is a plain arithmetic-on-checked-sizes question (HEURISTICS.md's "arithmetic on untrusted sizes/indices" category), not a memory-safety/UB question; a debug-assertions build plus deliberately constructed boundary values (through both a temporary in-tree test and a standalone public-API repro) fully covers it, matching HEURISTICS.md's own guidance for this bug class.

## Status

**Not reproducible.** Every `(width, padding)` combination `allocate` can actually accept and hand
to `deallocate` was swept at the `u16` boundaries most likely to trigger an overflow (smallest
widths, which let `allocate` accept the largest paddings; largest widths, which force `allocate`
to accept only tiny/zero paddings; both the maximum-representable atlas size and a realistic
default 4096x4096 one) with `overflow-checks = true` forced on, and zero overflows/panics occurred
across 294,913 total accepted combos (114,689 + 180,224 + 8,190) split across two independent
implementations (a temporary in-tree test and a standalone external crate using only the public
API), each internally consistent and re-run to a deterministic 3/3 in the standalone `repro/`
(`cargo run --release` twice, `cargo run` (dev) once, same clone - see `output.txt`).

This does not cover every conceivable future code path that might someday construct or mutate an
`ImageResource` outside `allocate` - the candidate's own suspicion was explicitly about a "future
code path" bypassing the invariant, which by definition doesn't exist yet to test against. What
this candidate does establish: there is currently no such path anywhere in this workspace (see
"What was tried" above), so the suspected overflow is not reachable today through any real caller.

**Fully reproducible**: N/A (no bug found - see Status).

## Class

N/A (not Confirmed - see Status).

## Root cause

Not a live bug in the current codebase - a defensive-programming/API-hygiene observation rather
than a reachable one. `allocate` is the sole constructor of `ImageResource` anywhere in the
workspace (`atlas_alloc_id` is a private field, so no other crate can build one via struct
literal even if it tried), and it already proves `width + 2*padding <= u16::MAX` via a `u32`
computation, checked with `u16::try_from`, before ever constructing a resource. Since `width >= 0`
always, this bounds `padding <= 32767` for any resource that exists, which in turn bounds
`padding * 2 <= 65534` - so `deallocate`'s `u16` recomputation of the same expression can never
overflow for any resource actually produced by `allocate`. `ImageCache` additionally exposes no
mutable accessor to an already-allocated `ImageResource` (only `get(&self) -> Option<&ImageResource>`,
a shared reference), so nothing in this codebase can mutate `width`/`padding` post-construction
either. The `u16` recomputation is a code smell (it relies on an invariant enforced entirely by a
different function, rather than the type system or a stored, already-checked value), but it is not
currently exploitable.

## Introduced by

80062c1a "vello_hybrid: Use `u16` consistently throughout the codebase (#1826)" (2026-08-17).
Before this commit, `deallocate` computed the padded size as `image_resource.width as u32 +
u32::from(image_resource.padding) * 2` - already widened to `u32`, matching `allocate`'s own
u32-checked computation. This commit changed `ImageResource.width`/`ImageCache::allocate`'s
`width`/`height` parameters from `u32` to `u16` throughout the crate (its own stated goal: "use
u16 consistently ... and only switch to u32 on demand"), and as part of that mechanical rewrite,
`deallocate`'s recomputation was simplified to plain `u16` arithmetic instead of continuing to
widen to `u32` first - introducing the (currently unreachable, per Root cause above) reliance on
`allocate`'s own invariant instead of redoing the same widen-and-check `allocate` does.

## The fix

No fix applied - `Status` is `Not reproducible`; there is no currently-reachable bug to patch. A
future hardening change (not made in this session, since there is nothing to verify a fix against
without a live bug) could still be worth considering independently of this candidate: have
`allocate` store its own already-`u32`-checked `padded_width`/`padded_height` in `ImageResource`
(or recompute via `u32` arithmetic + `u16::try_from` in `deallocate`, mirroring the pre-80062c1a
code) so `deallocate` never re-derives a value whose safety depends on a different function's
internal invariant - this would remove the reliance on the invariant entirely rather than just
re-confirming it holds today, at the cost of either a few extra stored bytes or two more widening
casts on the (cold) deallocation path. Left as a `RESULT.md`-only note since there is nothing to
verify against with `Status: Not reproducible`.

- Project test suite: no change was made to the main project's working tree at any point in this
  session (all investigation happened in an isolated git worktree, and the one temporary
  `#[ignore]`d probe test added there was reverted before finishing - see `output.txt`'s tail
  section for its captured output). Baseline (pristine, unmodified worktree): `cargo test -p
  vello_common --lib image_cache` -> 9 passed, 0 failed, 0 ignored (151 filtered out) - unaffected
  either way since nothing was patched.

## What's not done / open questions

- Did not attempt to actually construct an `ImageResource` bypassing `allocate` (e.g. via
  `unsafe`/transmute, or a hypothetical future public constructor) to force-demonstrate the
  overflow mechanically - the candidate's own premise is that no such path exists *today*, and
  confirming that is exactly what this candidate's grep + boundary sweep did; fabricating one
  would just prove the same arithmetic overflows on values `allocate` itself would never produce,
  which doesn't demonstrate a reachable bug (see CONFIRMATION.md section 11's reasoning about not
  faking the thing that produces the input in a related but distinct sense - here the "producer"
  being faked would be `allocate`'s own validation, not a network peer, but the same principle -
  don't fake the part whose correctness is in question - applies).
- If `allocate`'s own `u16::try_from` check is ever removed or weakened in a future change (e.g. if
  someone "optimizes" the boundary check away, or a new construction path for `ImageResource` is
  added, such as a batch-import/deserialize API), `deallocate`'s `u16` arithmetic would need to be
  re-audited - worth a quick comment in the source noting this cross-function dependency (not
  added in this session since `Status` is `Not reproducible` and no source change was made).
