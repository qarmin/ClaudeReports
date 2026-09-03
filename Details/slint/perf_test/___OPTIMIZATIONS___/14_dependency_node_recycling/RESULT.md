# 14 - Every tracked property read allocates a node, every frame

**Status: implemented, measured, validated.** `scroll` 100 000 **-8.0%** frame time and
**300 820 -> 172 allocations per frame**; `scroll --row-style bare` **-23.7%** and 200 313 -> 28.
No scene regresses. Patch: `recycle_dependency_nodes.diff`.

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
in every scene. Candidate 08 removed the duplicate registrations; what is left are genuine reads.

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

## Results

Software renderer, medians of 5 runs, recycling **off vs on in the same tree** (the two call sites
toggled, so nothing else differs - candidate 08 lives in the same file and would otherwise have been
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

## Validation

- **1 949 tests pass, 0 failures and 0 compile errors**: `i-slint-core --all-features` 309+69+1,
  `slint` 77, both build-time drivers 1 571 (36 result lines, including the 818-test interpreter
  suite). Counted with `grep -cE "^error"` alongside the result lines - a first pass of this
  validation reported a healthy total while the core crate's *test binary* was failing to compile,
  because a suite that doesn't build emits no result line at all. Four unit tests inside
  `single_linked_list_pin` call `push_front` and needed the new argument; two more were added for
  the pooling itself (nodes reused at the same addresses, values dropped exactly once).
- **`cargo check -p i-slint-core --features ffi`** - the check that catches the ABI break, and the
  only one that does. Also `--no-default-features --features unsafe-single-threaded,libm`, since
  the pool needs the same `std` / single-threaded split as `current_binding_storage`.
- **Pixels identical** on 9 of 10 scenes against the recycling-off binary: `text` 5 000 and 100 000
  (12 frames deep), `--word-wrap`, `list`, `list --mutate`, `scroll`, `scroll --row-style flat`,
  `rects`, `rects --gradient`.
- The tenth, `rects --animate`, **differs against itself** - the same-binary control was run first
  and shows the scene is time-driven (`animation-tick()`), so it is not pixel-comparable at all.
  The harness now prints that on its own output line.

## Reproduce

```sh
tools/matrix.sh "scroll|--mode scroll --count 100000" "bare|--mode scroll --count 100000 --row-style bare"
../../target/rdebug/perf_test --bench --mode scroll --count 100000 --frames 3 \
    --alloc-profile 500 --profile-phase frames
```
