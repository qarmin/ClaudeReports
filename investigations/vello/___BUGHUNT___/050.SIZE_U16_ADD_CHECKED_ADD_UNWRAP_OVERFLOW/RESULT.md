# SIZE_U16_ADD_CHECKED_ADD_UNWRAP_OVERFLOW

Suspect: `sparse_strips/vello_common/src/geometry.rs:87-105` - `impl Add for SizeU16` and
`impl Add<u16> for SizeU16` build the result with
`self.width().checked_add(rhs.width()).unwrap()` / `.checked_add(rhs.height()).unwrap()`, backed
only by the comment "Shouldn't overflow for our use cases." `SizeU16` is used throughout
`vello_hybrid` for GPU texture/layer/atlas sizing (`scene.rs`'s `MemorySettings`
`min`/`max_texture_size`, `schedule/allocate.rs` region/texture sizes, `filter.rs` pass
dimensions), some of it influenced by user-supplied scene content (layer bbox sizes, filter kernel
sizes) and user-configured min/max texture sizes - so the concern was whether any of that
untrusted-size-influenced code can actually reach this `unwrap()` with operands close enough to
`u16::MAX` to overflow.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive manual call-site tracing: grepped every file that imports `SizeU16` (`scene.rs`, `schedule/allocate.rs`, `schedule/mod.rs`, `schedule/execute.rs`, `schedule/cursor.rs`, `schedule/test_support.rs`, `schedule/schedule_tests.rs`, `filter.rs`, `blend.rs`, `render/common.rs`, `render/wgpu/mod.rs`, `render/webgl/mod.rs`, `record.rs`, `vello_sparse_tests/tests/renderer.rs`) for every `+` and `checked_add` occurrence, then manually checked which operand types each one actually involves | Static reading, no build | Found exactly two places that add to a `SizeU16`-derived size and could plausibly overflow near `u16::MAX`: `schedule/allocate.rs::RegionProps::allocation_size` (uses the *safe* `SizeU16::checked_add` method, `.expect("layer allocation size exceeds the u16 atlas domain")`, not the `Add`/`Add<u16>` trait impls) and `render/common.rs::LayersConfig::required_intermediate_texture_size` (casts to `u32` before adding `filter_padding`, then validates with `try_from`/an explicit `TooLarge` error - never touches `SizeU16`'s `Add` impls either). No other call site anywhere in the crate invokes `size + size` or `size + u16` via the operator. |
| Build-level oracle: feature-gated both `impl Add for SizeU16` and `impl Add<u16> for SizeU16` out entirely (`#[cfg(feature = "__bughunt_marker_disable_sizeu16_add")]`, a feature never enabled anywhere), then rebuilt the whole workspace (`cargo build --workspace --all-features --tests`, and separately `cargo build -p vello_hybrid --all-features --tests --examples`) | Debug build, in a git worktree of the main project (not the main tree itself) | Both impls removed compile cleanly - **zero errors anywhere in the workspace**, including every test module, every example, `vello_bench`, `vello_sparse_tests`. `vello_common`'s own build emits `warning: unused import: core::ops::Add` once the impls are gone, confirming nothing else in that file needs the trait either. This is decisive: if any code anywhere in the workspace (production, test, example, or bench) called either impl, removing it would be a compile error, not a warning. |
| Search for generic code that could indirectly monomorphize `Add` for `SizeU16` (a function bound like `T: Add` or `T: Add<u16>` that `SizeU16` could be substituted into) | `grep -rn ": Add\b\|+ Add\b\|Add<"` across `sparse_strips/` | No generic bound on `Add`/`Add<u16>` exists anywhere in the workspace outside the `SizeU16` impls themselves. |

## Status

**Not reproducible.** All three approaches agree: the `Add`/`Add<u16>` impls for `SizeU16` are
unreachable dead code in the current codebase. No internal `vello_hybrid` code path - including
every path that processes user-supplied scene content (layer bboxes, filter kernel sizes) or
user-configured `MemorySettings`/`LayersConfig` (min/max texture size) - ever calls `size + size`
or `size + u16` on a `SizeU16`. The two real places that need to add a `SizeU16`-derived size to a
padding value (`schedule/allocate.rs::RegionProps::allocation_size`,
`render/common.rs::LayersConfig::required_intermediate_texture_size`) were both written to use
either the safe `checked_add` *method* (which returns `Option`, not `.unwrap()`-only) or manual
`u32` arithmetic validated with `try_from`, deliberately avoiding the `Add` trait impls. The
build-level removal test (rebuilding the entire workspace, including all tests/examples/benches,
with both impls entirely gone) is the strongest evidence: it compiled with zero errors.

A standalone `repro/` in this folder demonstrates both halves of this conclusion using the real
public API: Part 1 shows the isolated `Add` impl genuinely does panic at the `u16` boundary when
called directly (3/3 repeat runs, deterministic) - the underlying arithmetic hazard is real, not a
false alarm. Part 2 drives the actual public entry points a scene author would use
(`Scene::new`/`push_layer`/`pop_layer` with a near-`u16::MAX` canvas and layer bbox, and
`MemorySettings`/`LayersConfig` with `min_texture_size = max_texture_size = SizeU16::new(u16::MAX)`)
and shows none of them panic - confirming the hazard is unreached by any real caller.

Because `SizeU16` and both `Add` impls are `pub`, an external application depending directly on
`vello_common` could still call the operator itself and hit this panic - but that is the same kind
of self-inflicted footgun as calling `u16::MAX.checked_add(1).unwrap()` directly; it requires the
caller to bypass `vello_hybrid`'s own (safe) sizing logic entirely, not something reachable by
feeding adversarial scene content through `vello_hybrid`'s intended API.

**Fully reproducible**: N/A - no bug was reproduced to characterize as fully/partially
reproducible. The "not reachable from any real caller" result itself is deterministic: the
build-level removal test is unconditional (removing the impls either breaks the build or it
doesn't - there's no flakiness to a compile error), and `repro/`'s Part 2 run is deterministic
across repeat runs (same fixed sizes, no randomness involved).

## Class

N/A (not Confirmed).

## Root cause

Not applicable - no defect was observed on any reachable path. The `unwrap()`-only `Add`/`Add<u16>`
impls for `SizeU16` are real, in the sense that they would panic if fed operands close enough to
`u16::MAX`, but every actual call site in `vello_hybrid` that needs to add a padding/margin to a
`SizeU16`-derived size was already written around them (`SizeU16::checked_add`, or manual `u32`
arithmetic with an explicit `TooLarge` error), so the panic path is currently dead code rather than
a live bug reachable through untrusted scene content or user-configured settings.

## Introduced by

N/A (not Confirmed).

## The fix

No fix attempted or needed - see Status/Root cause: there is no reachable defect to fix. Leaving
the `Add`/`Add<u16>` impls as they are is a latent risk only for *future* internal code that might
someday use `+` instead of `checked_add` on `SizeU16` (see "What's not done" below), not a present
bug.

- Project test suite: N/A - no patch was applied, so there is nothing to compare before/after.

## What's not done / open questions

- The `Add`/`Add<u16>` impls remain unreachable dead code with a panicking implementation. A
  maintainer might still want to change them to saturate or return `Option`/`Result` purely as a
  defensive measure against a *future* internal caller reintroducing a live call site, since the
  current safety margin depends entirely on every existing call site having independently chosen
  to route around the operator - there is no compiler-enforced guarantee that a future PR won't add
  a new `size_a + size_b` call site that does hit the panic.
- Not checked: whether any *out-of-tree* downstream consumer of the public `vello_common`/
  `vello_hybrid` crates (outside this repository) calls the `Add` operator on `SizeU16` directly
  with values derived from untrusted input - by definition out of scope for a bug hunt scoped to
  this repository's own code.
- **New suspect noticed while building this candidate's `repro/`**: driving `Scene::new` with a
  literal `u16::MAX` canvas and a layer bbox touching the far edge hits a *different*,
  already-acknowledged panic in `vello_common::util::RectExt::snap_to_tile_coordinates`
  (`sparse_strips/vello_common/src/util.rs:172-190`, own comment: "This method will panic if we
  have a viewport of size `u16::MAX` and draw at the very edge, but better than returning a wrong
  result"). This is unrelated to this candidate's `SizeU16::Add` suspect and was already flagged as
  an open lead by candidate `043.LAYER_TEXTURE_REGION_PAYLOAD_AT_UNWRAP_INVARIANT`'s own "What's
  not done" section, but as of this session it still has no dedicated candidate folder of its own.
  `repro/src/main.rs` uses `65532` (a tile-aligned near-maximum) instead of the literal `u16::MAX`
  specifically to avoid this unrelated panic and keep this candidate's own repro focused.
