# val_idx_collect_into_unreachable_merge

`ValIdx::collect_into` (`src/results/val_idx.rs:35-67`) is a k-way merge over `Vec<Vec<ValIdx<T>>>`
using an `orx_priority_queue::BinaryHeap`, `unsafe { ptr.add(idx).read() }` element moves, and
`unsafe { vec.set_len(0) }` to suppress double-drops - the same shape as the panic-unsafe merge
confirmed in candidate 006 (`006.sort_slice_mergesubtask_panic_safety`). It was already flagged as
a `dead_code` finding in `___FINDINGS___/findings.toml` (~line 197) and
`findings-04-result-runner-sizes.toml`, on the grounds that grep shows no caller anywhere. This
candidate exists to confirm that claim rather than just trust the earlier grep.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Repository-wide grep for any caller of `ValIdx::collect_into` (qualified path, and disambiguated from the many unrelated `.collect_into(` trait-method calls elsewhere in the crate) across `src/`, `benches/`, `examples/` | Plain `grep -rn` over the whole tree, ripgrep-equivalent, no build needed | Zero calls found. The only occurrence of the string `collect_into` tied to `ValIdx` is the function's own definition; all five other `ValIdx` consumers (`src/infallible*/par_runner.rs`, `src/result*/par_runner.rs`, `src/option*/par_runner.rs`) call `ValIdx::first`/`first_res`/`first_opt` instead |
| `cargo build` dead-code-lint check, across feature combinations that actually build | `cargo build` (default `std`), `cargo build --no-default-features --features std`, `cargo build --no-default-features --features wasm --target wasm32-unknown-unknown` (after a `touch` to force a real recompile, not a cached no-op) | No `dead_code` warning fires for `collect_into` on any of these (expected and uninformative here: rustc's `dead_code` lint does not flag `pub fn` items even inside a private module, since it can't rule out external use through re-exports - it does not perform the module-privacy reachability analysis that `unreachable_pub` would; `all-features`/`persistent-pool-rayon`+`transient-pool` combos don't build at all on this commit for unrelated reasons, see `pool::global_pool` / `nt_state` signature errors, out of scope for this candidate) |
| Git archaeology (`git log --oneline -- src/results/val_idx.rs`, `git log --all -S"ValIdx::collect_into"`, `git show` on the hits) to find whether/when this function last had a caller | `git log -S` on the qualified call string, across all branches | Confirmed: `78f41c19` "implement collect into with heap sort" added the function; `6c4fb6bc` "implement collect for parallel runner" wired it in as the sole caller (`ValIdx::collect_into(results_bag.into_inner().into_inner(), pinned_vec)` in `src/infallible/par_runner.rs`); `480024e3` "implement collect_arbitrary_into for Vec" (2026-08-28) removed that caller, changing `collect()`'s return type to the raw `Vec<Vec<ValIdx<X2::O>>>` instead of merging through `ValIdx::collect_into`. The same commit introduced an almost byte-for-byte copy of the merge logic as `merge_collected_into` in `src/infallible/collectables/utils.rs`. That file no longer exists at HEAD either - a further refactor relocated the (still) live merge logic to `src/collectables/alg/merge_collected/merge_ordered_into.rs` (`merge_ord_into_vec`/`merge_ord_into_split_vec`), which is `use`d by every collector in `src/collectables/{inf,inf_use,res,res_use,opt,opt_use}/{vec,vec2,split_vec_*}.rs`. `ValIdx::collect_into` was simply never deleted after its caller moved away from it in `480024e3`. |

## Status

**Not reproducible.** There is no bug to demonstrate: `ValIdx::collect_into` is genuinely dead code
at HEAD (b9214fbe), unreachable from any entry point in this crate (public or internal), under any
buildable feature combination checked. Per `HEURISTICS.md`'s own anti-pattern note ("a lint tool
flags something with no attacker-reachable path to it ... not worth a candidate folder"), an
unreachable copy of the panic-unsafe merge pattern cannot itself misbehave, so there is nothing here
that could produce a UB/crash/logic repro distinct from candidate 006 (which already covers the
*live* version of this same merge algorithm in `merge_ordered_into.rs`).

**Fully reproducible**: N/A (Not reproducible - no reproduction attempted, since the code path is
confirmed unexercised by anything in this repository).

## Class

N/A (Not reproducible; not a maintainable-bug Class, see "Root cause" below for a suggested
`dead_code`/`other` classification if this is ever elevated).

## Root cause

Not a bug - a maintenance leftover. `ValIdx::collect_into` was the *original* implementation of an
ordered multi-thread merge, briefly wired into `ParRunnerInfallible::collect` for one commit
(`6c4fb6bc`), then orphaned one commit later (`480024e3`) when that call site was rewritten to
return raw per-thread `Vec<Vec<ValIdx<T>>>` results instead, and the actual merge responsibility
moved into a new helper (first `collectables::utils::merge_collected_into`, since further relocated
to `collectables::alg::merge_collected::merge_ordered_into::{merge_ord_into_vec,
merge_ord_into_split_vec}`, which is where the same panic-unsafety class already lives and is
confirmed reachable/live in candidate 006). `src/results/val_idx.rs`'s copy was never deleted along
the way, so it now sits in the crate as a second, unreachable copy of the same unsafe merge -
harmless at runtime, but a real risk for a future maintainer who edits the *live* merge in
`merge_ordered_into.rs` and mistakes this stale copy for another call site that also needs the fix
(or vice versa: "fixes" this dead copy and believes the live path is now covered too).

## Introduced by

Not applicable - there is no bug being introduced. The function itself was added in
78f41c19 "implement collect into with heap sort" (2026-04-09); it became dead code as of
480024e3 "implement collect_arbitrary_into for Vec" (2026-08-28), the commit that removed its
only caller.

## The fix

No bounded "fix" applies to a Not-reproducible finding in the bug sense. The maintenance-liability
angle already has a suggested resolution recorded in `___FINDINGS___/findings.toml` (~line 197):
either wire `ValIdx::collect_into` into an actual call site, or delete it (and its now-unused
`orx_priority_queue`/`orx_pinned_vec` imports in that file) since `merge_ordered_into.rs` already
covers the same functionality for every real collector. No source change was made in this worktree;
nothing to verify a before/after for.

- Project test suite: 99 passed; 0 failed; 0 ignored (`cargo test --lib`, default features,
  unmodified tree) - recorded as a baseline only, since no patch was applied.

## What's not done / open questions

- This candidate does not re-verify the *live* merge's panic-safety - that is candidate 006's
  scope (`merge_ordered_into.rs`, the actual code path every real collector uses); this candidate
  only establishes that `val_idx.rs`'s copy is not an additional, independently-reachable instance
  of that same bug class.
- Whether to actually delete `ValIdx::collect_into` (and possibly `alloc::vec`/
  `orx_priority_queue`/`orx_pinned_vec` imports in `src/results/val_idx.rs` that only that function
  needs) is a maintainer call, not something this session made unilaterally - left as the existing
  `findings.toml` recommendation.
- Not re-checked: whether any *other* crate depends on this one via a git/path dependency and could
  call `ValIdx::collect_into` from outside - `results` is a private (`mod results;`, not `pub mod`)
  module, so this is not possible through the public API regardless of workspace layout; not
  re-verified against `docs.rs`-style external usage since the module is provably unreachable from
  outside the crate already.
