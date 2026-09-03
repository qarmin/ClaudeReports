# 14 - Every tracked property read allocates a node, every frame

Ported from `perf_test/___OPTIMIZATIONS___/14_dependency_node_recycling/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

`scroll` 100 000 **-8.0%** frame time and **300 820 -> 172 allocations per frame**;
`scroll --row-style bare` **-23.7%** and 200 313 -> 28. No scene regresses. Patch:
`patch.diff`.

## What the profiler said

`--alloc-profile 500 --profile-phase frames`, allocation *sites* ranked, every scene:

| scene | allocations/frame | share from `register_as_dependency_to_current_binding` |
| --- | --- | --- |
| `scroll` 100 000 | ~301 000 | **99.8%** |
| `list` 100 000 | ~1 500 | **100%** |
| `rects` 3 000 | ~18 500 | **91.9%** |
| `rects` 3 000 `--animate` | ~36 600 | **96.7%** |
| `text` 100 000 | ~500 | **100%** |

Dependency registration is not *a* source of allocations in Slint, it is essentially *the* source,
in every scene. Candidate 008 removed the duplicate registrations; what is left are genuine reads.

## Why it allocates

`BindingHolder::dep_nodes` is a `SingleLinkedListPinHead<DependencyNode>`. Every property a binding
reads pushes one node, raw-allocated (`properties.rs`, `single_linked_list_pin::push_front`). When
the binding is evaluated again, the entire list is dropped and rebuilt:

```rust
// clear all the nodes so that we can start from scratch
unsafe { *(*binding_ptr).dep_nodes.get() = Default::default() };
```

A binding re-reads the same properties every time it runs, so in steady state this frees N nodes and
allocates N identical ones, every frame, per binding. The module's own header says as much: *"The
current implementation uses lots of heap allocation but that can be optimized later"*.

## The change

`recycle_into(pool, unlink)` walks the list, calls `unlink` on each value (for a `DependencyNode`
that is `remove()`, which detaches it from the property's dependency list) and moves the node to a
pool **without giving the memory back**. `push_front` takes from the pool before it allocates.

Safety rests on two things, both already true of the existing code:

- After `remove()` nothing points at the node any more, so reusing the allocation cannot alias:
  the property lists are intrusive and `remove()` is what unlinks them.
- The allocation never moves, which is what `Pin<&T>` promised before and still promises.

The pooled value stays initialized and `push_front` drops it before writing the new one, so a node
that is pooled but never reused is still dropped exactly once, by the pool's `Drop`. That keeps the
container generic instead of special-casing `DependencyNode`.

### Where the pool lives, and why that mattered

The first version gave each list its own free chain - a second pointer in
`SingleLinkedListPinHead`. That **breaks the C++ ABI**: `PropertyTrackerOpaque` in
`properties/ffi.rs` mirrors `PropertyTracker`'s layout and `assert_eq_size!` catches it. Only
`cargo check --features ffi` catches it; the default build is happy.

The fix is a single pool per thread, shared by every binding, which keeps the head one pointer wide
- and turns out to be **strictly better**:

| scene | per-binding pool | shared pool |
| --- | --- | --- |
| `scroll` 100 000 | 118.00 ms, 305 allocations | 119.46 ms, **172** |
| `scroll` `bare` | 20.58 ms, 71 | **19.95 ms**, **28** |
| `list` 100 000 | 2.42 ms (**+11.5%** regression) | **2.13 ms** (no regression) |
| `text` 100 000 | 4.04 ms (**+2.3%** regression) | **3.87 ms** (no regression) |

Both regressions came from the same thing: a short-lived binding filled a pool it never reused, then
freed it with its owner. A shared pool means a binding that shrinks or dies hands its nodes to the
others. The ABI constraint pushed the design somewhere better than where it started.

## What was tried / Results

Software renderer, medians of 5 runs, recycling **off vs on in the same tree** (the two call sites
toggled, so nothing else differs - candidate 008 lives in the same file and would otherwise have been
reverted along with it):

| scene | frame off | frame on | | allocations/frame | RSS |
| --- | --- | --- | --- | --- | --- |
| `scroll` 100 000 | 129.88 ms | **119.46 ms** | **-8.0%** | 300 820 -> **172** | 1476.0 -> 1446.0 MB |
| `scroll` 100 000 `bare` | 26.15 ms | **19.95 ms** | **-23.7%** | 200 313 -> **28** | 144.1 -> 135.3 MB |
| `rects` 3 000 | 2.16 ms | **2.04 ms** | -5.6% | 18 414 -> **14** | 18.2 -> 18.6 MB |
| `list` 100 000 | 2.14 ms | 2.13 ms | - | 1 502 -> 869 | 36.0 -> 36.4 MB |
| `text` 100 000 | 3.87 ms | 3.87 ms | - | 435 -> **223** | 38.1 -> 38.5 MB |

**No scene regresses**, and resident memory falls where it moves at all: the nodes were being
churned, not saved, and the pool is smaller than the fragmentation the churn caused.

`list`'s allocation count only falls to 869 because it is the virtualized scene - rows are created
and destroyed constantly, so a share of its bindings are evaluated exactly once and have nothing to
reuse.

## Does it change behavior/output?

- **Bit-exact?** Yes on 9 of 10 scenes checked. Pixels identical against the recycling-off binary:
  `text` 5 000 and 100 000 (12 frames deep), `--word-wrap`, `list`, `list --mutate`, `scroll`,
  `scroll --row-style flat`, `rects`, `rects --gradient`.
- The tenth, `rects --animate`, **differs against itself** (the same-binary control was run first
  and shows the scene is time-driven via `animation-tick()`), so it is not pixel-comparable at all
  regardless of the patch - not a regression caused by this change. The harness now prints that on
  its own output line.
- **Is this a breaking change?** No - internal allocator/data-structure change only, no public API
  or file-format change. `single_linked_list_pin::push_front` gained a `pool` argument, but that
  type is internal to `i-slint-core`.
- Project test suite: **1 949 tests pass, 0 failures and 0 compile errors** after the change
  (`i-slint-core --all-features` 309+69+1, `slint` 77, both build-time drivers 1 571, including the
  818-test interpreter suite). No new failures - counted with `grep -cE "^error"` alongside the
  result lines. A first validation pass had reported a healthy total while the core crate's *test
  binary* was actually failing to compile, because a suite that doesn't build emits no result line
  at all; four unit tests inside `single_linked_list_pin` needed updating for the new `push_front`
  argument, and two new tests were added for the pooling itself (nodes reused at the same
  addresses, values dropped exactly once).
- `cargo check -p i-slint-core --features ffi` passes - this is the only check that catches the ABI
  break the first (rejected) design introduced. Also verified with
  `--no-default-features --features unsafe-single-threaded,libm`, since the pool needs the same
  `std` / single-threaded split as `current_binding_storage`.

## Why the result looks the way it does

Dependency-node allocation was ~92-100% of every scene's allocations before this change, so
recycling the nodes removes nearly all of them; the frame-time win is smaller than the allocation
win because allocation is only part of frame cost, and scenes with little dependency churn
(`list`, `text`) see allocation counts drop with no measurable frame-time change since allocator
overhead wasn't dominant there to begin with.

## What's not done / open questions

- Not upstreamed/applied to the main tree - lives only as `patch.diff` here.
- The rejected per-list-pool design (which broke the C++ ABI) is documented above as a dead end so
  it isn't retried.

## Reproducibility check after upstream update (2026-08-23) - fully rebuilt and re-benched

Unlike the other 11 patched candidates (checked via `git apply --check` only this round), this one
was taken all the way through: applied for real against slint HEAD `cdb6c43f7` (femtovg HEAD
`df54a3c8`, not relevant here), built, benched with `tools/paired.sh` (9 alternating pairs), and
reverted - the full candidate-loop cycle this skill prescribes, done end to end once as a concrete
test of the workflow itself, not just a paperwork check.

`patch.diff` still applies cleanly with no changes needed.

**Two build problems had to be fixed in `perf_test/` first, unrelated to this patch**, both now
fixed permanently in `perf_test/` (not reverted - they block building against *any* clean tree,
not just this candidate):

1. `perf_test/Cargo.toml` had `dep-stats = ["i-slint-core/dep-stats"]`, forwarding to a feature
   that only ever existed as a temporary local patch to `internal/core/Cargo.toml` during the
   original session (for candidate 008's counters), reverted once that session ended. The dangling
   forward broke even a **default** build - Cargo validates the whole feature graph at resolution
   time, not just activated features. Fixed to `dep-stats = []`.
2. `perf_test/src/bench.rs` called `TextLayoutCache::paragraph_shape_count()`, a method that only
   exists when candidate `003.TEXT_DOCUMENT_SHAPING`'s own patch is applied (it's the method that
   patch adds). A pristine tree - which is what every candidate's baseline needs to build against -
   has no such method. This is a textbook case of MEASUREMENT.md section 12 ("the baseline binary
   must build against the pristine tree"), just discovered the hard way instead of anticipated.
   Stubbed to `0` with a comment; the real value needs candidate 003 applied (and rebased first,
   see its own reproducibility-check section above).

**Fresh numbers** (`scroll --row-style bare --count 100000`, software renderer, `tools/paired.sh`,
9 alternating pairs, one binary per side, both built in this same session):

| | median frame | pairs won |
|---|---|---|
| off (baseline) | 29.21 ms | - |
| on (patched) | **20.09 ms** | **9 of 9** |
| delta | **-31.2%** | consistent |

Directionally the same win as originally recorded, and larger in relative terms than the original
`-23.7%` (26.15 -> 19.95 ms) - though not a strict apples-to-apples comparison: the unpatched
baseline's allocation count is now **300 541/frame** against the originally recorded **200 313**
for the identical scene and flags, a real difference not investigated further this round (some
other upstream change between the two sessions affects baseline allocations in this scene; it
doesn't change the patch's own effect, which drops allocations to **80/frame** either way - even
lower than the originally recorded 28, consistent with the bigger relative win). **Conclusion: the
fix is still fully valid and, if anything, undersold by the original numbers on today's tree.**
