# BUCKETER_BUCKET_COMMANDS_UNBOUNDED_RECURSION

`sparse_strips/vello_cpu/src/coarse/bucketer.rs`'s `CommandBucketer::bucket_commands` (coarse
rasterization stage) recurses into itself once per nested `RecordedLayerKind::Regular` layer while
walking the recorded scene-graph node tree - the code itself already carries a `// TODO: Avoid
recursion to prevent stack overflows for deeply nested layers.` comment at the exact call site.
Recursion depth equals the scene's own layer-nesting depth (`push_clip_layer`/`push_blend_layer`/
`push_opacity_layer`/`push_mask_layer`, i.e. nested clip/opacity/blend/mask groups - the kind a
deeply nested SVG `<g>` tree or a programmatically generated scene would produce), which is
entirely caller/input-controlled and has no depth limit anywhere in the recording (`Recorder::
push_layer` in `vello_common/src/record.rs`) or bucketing path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: standalone repro using only `vello_cpu`'s real public API (`RenderContext::push_opacity_layer`/`pop_layer`/`render`), varying nesting `depth` | Release build (`cargo run --release`), depths 1,000 / 10,000 / 15,000 / 20,000-40,000 / 50,000 / 100,000 | Binary search: succeeds through depth=10,000, overflows the stack (`thread 'main' has overflowed its stack`, `fatal runtime error: stack overflow, aborting`, `SIGABRT`/exit 134) at depth=15,000 and every depth tried above that (up to 100,000) |
| Same repro, debug (`debug-assertions`) build | `cargo build` (dev profile), depths 3,000 / 5,000 / 8,000 / 10,000 / 15,000 | Succeeds through depth=10,000, overflows at depth=15,000 (debug build needs a slightly larger stack per frame than release due to less inlining, but the threshold is in the same order of magnitude) |
| `gdb` backtrace on the debug-build crash (depth=15,000) to confirm the exact fault location | `gdb -q -batch -ex run -ex "bt 10"` | `SIGSEGV` deep inside a recursive call chain, every frame identical: `vello_cpu::coarse::bucketer::CommandBucketer::bucket_commands (...) at src/coarse/bucketer.rs:331` calling itself - the exact recursive self-call the candidate's TODO comment points at, confirmed, not a different code path |

No `miri`/ASAN/ or a fuzz harness were needed - this is a plain, deterministic, boundary-value
stack-depth issue (recursion depth = an ordinary `usize` counter, not any kind of memory-unsafety
that needs a sanitizer to surface), so manual boundary-value construction plus a debug build was
the right first tool per `HEURISTICS.md`'s "arithmetic on untrusted sizes/indices" guidance
(nesting depth is exactly that: an untrusted "size" driving recursion depth), and it reproduced
immediately.

## Status

**Confirmed.**

Reproduces deterministically, 3/3 repeat runs, release build, depth=100,000 (and separately
verified at depth=50,000 and depth=15,000; verified 3/3 in a fresh, standalone, end-to-end repro
run through the candidate's own `justfile`'s `clone`/`reproduce-before` recipe against a real
fresh `git clone` of the project). No seed/layout/timing dependency at all - the crash depth is a
fixed, reproducible property of the recursion's per-frame stack usage vs. the thread's fixed
stack size (`ulimit -s` = 8192 KiB in this environment), not something that varies run to run.

**Fully reproducible**: **Yes**. This is a single-participant case per `CONFIRMATION.md` section
11's own carve-out: the "input" is a single external artifact (the scene's own nesting structure -
e.g. what a real SVG-to-Vello frontend would build while walking a deeply nested `<g>` tree, or
any other caller that nests layers deeply) handed straight to `vello_cpu`'s real public API
(`RenderContext::push_opacity_layer`/`pop_layer`/`render`) exactly the way a real embedding
application would build and render such a scene - there is no second "peer" being impersonated
here, so no second real participant is needed to demonstrate it.

## Class

crash (stack overflow / uncontrolled recursion depth driven by caller-supplied scene structure)

## Root cause

`CommandBucketer::bucket_commands` implements a pre-order/post-order tree walk over the recorded
scene graph (`Node`/`RecordedLayer` in `vello_common/src/record.rs`): for every node with an
attached `RecordedLayerKind::Regular` layer, it calls `self.push_layer(props)`, then **recurses**
into `self.bucket_commands(&layer.nodes, ...)` to process that layer's own child nodes, and only
calls `self.pop_layer(strips)` after that recursive call returns. Every level of `Regular`-layer
nesting in the original scene (every `push_clip_layer`/`push_opacity_layer`/`push_blend_layer`/
`push_mask_layer` a caller issues, one level deeper than its enclosing layer) therefore consumes
one more native call-stack frame in `bucket_commands`, with no limit anywhere in the recording
(`Recorder::push_layer`) or bucketing code. A caller building a scene with enough nesting (roughly
11,000-15,000 levels in this environment's 8 MiB default thread stack, comfortably reachable by a
handful of KB of deeply nested SVG markup or a trivially small recursive scene-builder) overflows
the stack and aborts the process - this is a hard process abort (`SIGSEGV` inside the recursive
call chain, caught by Rust's stack-overflow guard page handler as `fatal runtime error: stack
overflow`), not a panic that can be caught with `catch_unwind`.

## Introduced by

dc23e93841d20060743e490ef49c1e0d0d22cf40 "vello_common: Generalize the command recorder
abstraction and add more metadata (#1746)" (2026-07-20). Found via `git blame` on the exact
recursive-call lines (321-339) of `bucket_commands`, which all point at this single commit,
including the `// TODO: Avoid recursion...` comment itself. This commit introduced the current
`Node`/`RecordedLayer` recording abstraction and `CommandBucketer::bucket_commands`'s recursive
walk over it as part of a larger recorder-abstraction refactor (`git log -S"Avoid recursion to
prevent stack overflows"` confirms this is the only commit that ever added that comment/call
site - it was not moved from an earlier equivalent recursive function under a different name).
Note this repo's history around this area is a fast-moving, very recent (2026) rewrite of the
`vello_cpu` frontend (see `9c292860` "A full rewrite of the frontend" a month earlier); an
equivalent (potentially also-recursive) predecessor may have existed briefly between those two
commits, but was not separately investigated since dc23e938 is what introduced the code as it
exists today, including its own explicit acknowledgement of the gap.

## The fix

Rewrote `bucket_commands`'s traversal to use an explicit heap-allocated stack of `Frame`s (each
`Frame` holding the remaining `&[Node]` slice for one nesting level, a cursor into it, and whether
finishing that frame must be followed by a `pop_layer` call) instead of a native recursive call.
Every point where the original code recursed (`self.bucket_commands(&layer.nodes, ...)`) now
instead pushes a new `Frame` for `layer.nodes` onto the `Vec`-backed stack; every point where it
returned from that recursive call and called `self.pop_layer(strips)` now instead happens when a
`Frame`'s node list is exhausted and it's popped off the stack. This preserves the exact original
processing order (all of a layer's children are fully processed, in order, before its own
`pop_layer` runs, before its parent's next sibling node is processed) while making recursion depth
independent of native call-stack space - the heap-allocated `Vec<Frame>` can grow far beyond what
an 8 MiB thread stack could ever hold. `self.viewport_origin()`'s tile-alignment `debug_assert` is
now checked once before the traversal starts instead of once per (former) recursion level, which
is behaviorally identical since `self.viewport` is fixed for the whole bucketing pass (confirmed
by reading `viewport_origin()`/`reset()`: nothing during `push_layer`/`pop_layer` ever mutates
`self.viewport`).

- Verified against the original repro: **gone**. Depth=100,000 and depth=1,000,000 (10x further
  into the same bug class than any depth that originally crashed, per `CONFIRMATION.md` section 7)
  both now render successfully in both release and debug builds, where depth=15,000 previously
  overflowed the stack in both.
- Project test suite:
  - `cargo test -p vello_cpu --release --lib`: 81 passed / 0 failed, identical before and after.
  - `cargo test -p vello_sparse_tests --release`: 3541 passed / 86 failed, identical before and
    after - the 86 failures are the exact same set of `_hybrid`-suffixed tests both before and
    after (confirmed via `diff` of the sorted failing-test-name lists), and are pre-existing
    failures unrelated to this change (they require a real GPU/`wgpu` device, unavailable in this
    environment - not something this candidate's fix touches or could regress).
  - One pre-existing, unrelated doctest failure (`glifo::atlas::GlyphCacheStats` import gated
    behind `debug_assertions`, in `vello_cpu/src/text_debug.rs`) is present identically before and
    after this change; not counted in the pass/fail numbers above since it's orthogonal to this
    candidate (confirmed present on the completely unmodified tree too).

See `patch.diff` for the full change (against `sparse_strips/vello_cpu/src/coarse/bucketer.rs`
only, +39/-12 lines).

## What's not done / open questions

- This fix only addresses `bucket_commands`'s own recursion over `RecordedLayerKind::Regular`
  layers. `RecordedLayerKind::Filter` layers are handled without recursion inside
  `bucket_commands` itself (they call `filter_ctx.filter_layer(id)`, whose own contents were
  already bucketed by a separate, earlier top-level call), so they were not a source of unbounded
  recursion in this function - but the *filter* rendering pipeline itself (rasterizing each
  filter layer's own contents, potentially built from an arbitrarily deep tree of nested filter
  layers) was not separately audited for its own possible recursion in this session; a scene with
  deeply nested *filter* layers specifically (as opposed to the `Regular`/opacity/clip/blend
  layers this candidate's repro uses) might still hit a different, unaudited recursive path
  elsewhere in the filter-compositing code and would be worth a follow-up candidate.
- The scene *recording* stage itself (`Recorder::push_layer`/`pop_layer` in
  `vello_common/src/record.rs`) is not recursive (it maintains an explicit `layer_stack: Vec<...>`
  already), so it was not a second instance of this bug class - only the bucketing/coarse-rasterization
  consumer of the recorded tree was affected.
- No depth limit was added anywhere (e.g. rejecting/truncating scenes past some maximum nesting
  depth) - the fix removes the *recursion*, not the *size* of the traversal; an extremely deep
  scene (millions of levels) will still take proportionally more time and heap memory to bucket,
  it just no longer crashes the process. Whether an explicit sane maximum nesting depth should
  also be imposed (returning an error / silently truncating, similar to this project's own
  existing "silent stop" pattern for other resource limits, e.g. candidate 027's bump-allocation
  limit) is a product decision for the maintainers, not attempted here.
- `vello_hybrid` (the other consumer of the same `vello_common::record` recording abstraction, per
  that module's own doc comment) was not checked for an equivalent recursive traversal over the
  same `Node`/`RecordedLayer` tree - its own scheduler "needs the complete layer hierarchy" per
  that doc comment, which suggests it may walk the same tree with its own (possibly also
  recursive) traversal function; not investigated this session.
