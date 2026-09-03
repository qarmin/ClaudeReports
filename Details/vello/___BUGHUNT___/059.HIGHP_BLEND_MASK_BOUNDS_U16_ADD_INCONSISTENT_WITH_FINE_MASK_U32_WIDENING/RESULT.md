# highp `blend`'s mask-bounds check adds in raw u16, unlike the sibling `Fine::mask`

`sparse_strips/vello_cpu/src/fine/highp/mod.rs:211` (`HighPCompositeExt::blend`'s mask-sampling
closure) computes its bounds check as `start_x < width && start_y + 3 < height` entirely in raw
`u16` arithmetic. The near-identical sibling `Fine::mask` (`sparse_strips/vello_cpu/src/fine/mod.rs`,
~lines 726-741) deliberately widens to `u32` first (`u32::from(row_y) + u32x4::from_slice(...)`)
specifically to avoid this same addition overflowing. The lowp kernel's own `blend`
(`sparse_strips/vello_cpu/src/fine/lowp/mod.rs`, ~line 244 onward) has the identical raw-`u16`
pattern as the highp one. Flagged as a "fixed once, missed the sibling call site(s)" smell worth
checking before any change to the tile-height-alignment assumption that currently keeps it safe.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant/boundary-value analysis of the call chain that produces `start_y` | Static reading: `Tile::HEIGHT = 4`; `Tile::new_clamped` clamps tile `y` to `u16::MAX / 4 = 16383`; `Fine::row_y` is always set via `rasterize_region`'s `scene_y = region.row_idx as u16 * Tile::HEIGHT` (and identically in `region.rs:146`); `Pixmap`/`RenderContext` `height` is a `u16`, so at most 65535 | `start_y` passed into `blend`/`mask` is always an exact multiple of 4, and bounded by the canvas height (itself `<= u16::MAX`). The largest multiple of 4 that is `<= u16::MAX` is 65532, so the largest `start_y + 3` can ever be is 65535 = `u16::MAX` exactly - no overflow, by a hard numeric coincidence (65536 is divisible by 4). |
| Direct-call debug-assertions test: added a temporary `#[test]` calling `<F32Kernel as FineKernel<_>>::blend` directly with a hand-constructed `start_y = u16::MAX - 1` (65534, violating the alignment invariant established above) and a small real `Mask`, dev profile (debug-assertions on) | `cargo test -p vello_cpu --lib fine::highp::boundary_tests -- --nocapture`, repeated 4/4 runs | Panics deterministically 4/4 runs at the exact flagged line: `sparse_strips/vello_cpu/src/fine/highp/mod.rs:211:53: attempt to add with overflow`. Confirms the raw-`u16`-addition mechanism is real *if* the alignment invariant were ever violated - but this test bypasses every real call site (the input was hand-constructed, not derived from any real `Fine::row_y`), so on its own it does not show the bug is reachable through the actual rendering pipeline. Test was reverted after this check; not part of `patch.diff` (there is none). |
| Real end-to-end check via the project's public API (`RenderContext`/`Mask`/`Pixmap`, the crate's actual public entry points), at the two boundary canvas heights that matter | `height = u16::MAX` (65535, not tile-aligned) and `height = 65532` (largest tile-aligned `u16` value), `width = 4`, a full-canvas luminance `Mask` set via `set_mask`, filling the bottom-most rows, dev profile (debug-assertions on); see `repro/` | `height = 65535`: panics, but at a *different*, unrelated, pre-existing location - `sparse_strips/vello_common/src/util.rs:189` inside `RectU16::snap_to_tile_coordinates`, whose own doc comment already states "This method will panic if we have a viewport of size u16::MAX and draw at the very edge, but better than returning a wrong result." This fires before rendering ever reaches the flagged mask/blend code. `height = 65532` (the true reachable maximum `start_y` can take): completes normally, no panic - reproduced as a negative result 2/2 times (once via direct `cargo run` in the investigation worktree, once more via `just test` against an independent fresh clone at the base commit). |

## Status

**Not reproducible** (via any real entry point of the public API).

The flagged inconsistency (raw `u16` vs. the sibling's `u32`-widened arithmetic) is real and was
directly confirmed to panic when called with an artificially misaligned `start_y` (see the second
row above). However, no real caller can ever produce such a `start_y`: `Fine::row_y` is always
derived as `<tile row index> * Tile::HEIGHT` (a multiple of 4), and the tile row index is itself
bounded by the canvas height, which is a `u16` (max 65535). Because 65536 is exactly divisible by
4, the largest possible `start_y` (65532) plus 3 lands exactly on `u16::MAX` (65535) without
overflowing - a hard numeric coincidence of the current constants (`Tile::HEIGHT = 4`, height
stored as `u16`), not a defensive check. This was independently confirmed by a real end-to-end
render at that exact maximum height (see third row above): no panic.

**Fully reproducible**: N/A (no live bug found - see Status). The *investigation itself* is fully
reproducible on demand via `just test`, verified 2/2 (once directly, once via a fresh `just test`
clone-and-run, byte-identical "did not reproduce" output both times).

## Class

N/A (not Confirmed).

## Root cause

Not a live bug in the current codebase - a genuine but currently-inert inconsistency between two
near-identical call sites. `Fine::mask` (`fine/mod.rs`) widens `row_y` to `u32` before adding the
`[0, 1, 2, 3]` row offsets specifically to make the addition overflow-proof regardless of what
`row_y` happens to be; `HighPCompositeExt::blend`'s (and the lowp kernel's) own mask-sampling
closure does the equivalent addition in raw `u16` instead, relying implicitly on the invariant that
`start_y` is always a multiple of `Tile::HEIGHT` and the canvas height fits in `u16` - an invariant
that happens to make the raw addition exactly safe today, but isn't enforced or documented at the
call site itself. The two versions arose at different times (see "Introduced by"): the u32-widened
`Fine::mask` came from a later full frontend rewrite that apparently adopted the safer pattern
without back-porting it to the older `blend` mask-sampling code the rewrite didn't touch.

## Introduced by

The raw-`u16` `blend` mask-bounds check: 568208835d0bdb01d9880953631efd5da4d15582 "Add support for
non-isolated masks (#1237)" (2025-11-29) - this is where `start_y + 3 < height` was first added to
`highp/mod.rs`'s `blend` (the lowp kernel's identical pattern was introduced in the same commit).

For context (not the "bug", but why the inconsistency exists): the `u32`-widened `Fine::mask` in
`fine/mod.rs` was introduced later by 9c292860 "vello_cpu: A full rewrite of the frontend (#1701)",
which rewrote `Fine::mask` with the safer widening but did not touch (and therefore did not
retrofit the same fix into) `blend`'s own separate mask-sampling closures in `highp/mod.rs` and
`lowp/mod.rs`.

## The fix

No fix applied - Status is Not reproducible, and per SKILL.md's candidate loop this is a valid,
complete negative result on its own. No `patch.diff` is included.

That said, since a future change (widening `Tile::HEIGHT` past a power of two dividing 65536, or
letting canvas height exceed `u16`) could turn this from "safe by coincidence" into a real
reachable overflow, a defensive-only change (not applied here, since Status is Not reproducible and
this session found no live behavior to verify a fix against) would be to make `blend`'s mask-bounds
check widen to `u32` the same way `Fine::mask` already does, in both `highp/mod.rs` and
`lowp/mod.rs`, purely for consistency with the sibling and to remove the implicit invariant
dependency - not because it fixes anything reachable today.

- Project test suite: N/A, no fix was applied. Baseline sanity check only (`cargo test -p
  vello_cpu --lib --release`, pristine worktree at the investigated commit, no changes): **81
  passed; 0 failed; 0 ignored**.

## What's not done / open questions

- The lowp kernel's `blend` (`sparse_strips/vello_cpu/src/fine/lowp/mod.rs`) has the identical
  raw-`u16` pattern and was read but not separately drilled into with its own direct-call test -
  the same invariant analysis (tile-aligned `start_y`, `u16`-bounded height) applies identically to
  it, so it's presumed to share the same "not reproducible today" conclusion, but this wasn't
  independently verified with its own repro.
- A new, unrelated suspect was noticed while building the real end-to-end check: rendering at
  `height = u16::MAX` (65535, the theoretical maximum `Pixmap`/`RenderContext` height) panics
  unconditionally in `RectU16::snap_to_tile_coordinates` (`sparse_strips/vello_common/src/util.rs:189`,
  `Option::unwrap()` on `None` from `checked_next_multiple_of`) when filling near the bottom edge -
  this is already flagged in that function's own doc comment as a known limitation, but wasn't
  otherwise investigated as its own candidate this session. Worth a future `IDEAS.md` entry.
- No `miri`/ASAN/fuzzing was used - the suspected bug class (an arithmetic overflow gated by an
  invariant established purely through static code reading) is better suited to, and was fully
  characterized by, boundary-value construction plus a debug-assertions build; a fuzz harness
  driving the real public API would need to search canvas dimensions up to 65535 with a mask, which
  the two targeted boundary heights checked here (65535 and 65532) already cover exhaustively for
  this specific arithmetic question.
