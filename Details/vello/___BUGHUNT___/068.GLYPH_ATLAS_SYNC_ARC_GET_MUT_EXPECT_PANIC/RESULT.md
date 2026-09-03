# GLYPH_ATLAS_SYNC_ARC_GET_MUT_EXPECT_PANIC

`sparse_strips/vello_cpu/src/text.rs`'s `sync_glyph_cache`/`clear_evicted_glyph_atlas_regions`
each do `Arc::get_mut(&mut glyph_resources.pixmaps[page_index]).expect("... uniquely owned ...")`.
Uniqueness depends on a cross-module contract enforced only by convention: `render.rs`'s
`before_render`/`after_render` (called at the start/end of every `render_with`) register/
unregister each atlas page's `Arc<Pixmap>` clone into the (separate) `ImageRegistry`, and nothing
in between is supposed to hold a third clone past the end of `render_with`. The candidate asked
whether a panic partway through `render_with` - a scenario the code's own adjacent `TODO` comment
already flags as having caveats around calling `render_with` again without resetting - could break
this invariant for a later, otherwise-unrelated frame.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `text.rs`/`render.rs` end-to-end to trace the actual lifecycle of the registry's atlas-page clones (`register_atlas_page` in `before_render`->`sync_glyph_cache`, temporary clones taken by `ImageResolver::resolve` during rasterization, `destroy_atlas_page` in `after_render`->`maintain_glyph_cache`) | Static | Confirmed the single-threaded and multi-threaded dispatchers' `rasterize()` calls are synchronous (`rayon`'s `install`/`update_par` block until done), so all `resolve()`-cloned `Arc`s are dropped by the time `rasterize()` returns - no live-thread-holds-a-clone race under normal (non-panicking) execution. But `resources.after_render()` is a single, unconditional statement placed *after* `self.dispatcher.rasterize(...)` in `render_with`, with nothing making it run if `rasterize()` (or anything else between `before_render` and it) panics. |
| Manual boundary-value/control-flow construction: build a scenario where `render_with` panics for an unrelated, ordinary reason (an `ImageSource::OpaqueId` referencing an image the caller already destroyed - a realistic caller mistake, panics at `fine/mod.rs`'s `"Image {:?} not found in registry"`) partway through rasterization of a frame that also used the glyph atlas, catch that panic with `catch_unwind` (simulating a caller that guards individual frames against panics - a legitimate, common defensive pattern), then render a second, unrelated, perfectly ordinary frame reusing the same (persistent, as documented) `Resources` | In-tree `#[test]` in `sparse_strips/vello_cpu/src/render.rs`, debug and release builds, default features and `--features png` | Reproduced immediately: frame 2 (which introduces a *new* glyph, forcing a new pending atlas upload/replay for the same page frame 1 used) panics with the `Arc::get_mut` `.expect()` - deterministic, 5/5 runs in debug, also reproduces in `--release`. Confirmed against two different inputs in the same class (CONFIRMATION.md section 7): an outline/COLR glyph (Roboto) hits the "replay" call site (`text.rs:175`), a bitmap glyph (Noto Color Emoji CBTF) hits the sibling "bitmap upload" call site (`text.rs:150`). |
| Standalone `repro/` driving `vello_cpu`'s real public API (`RenderContext`, `Resources`, `register_image`/`destroy_image`, `glyph_run`/`atlas_cache`, `render_with`) end-to-end, no internal types touched | `cargo run`, debug build, both fresh `git clone` (network) and a local worktree checkout | Same result: both scenarios' frame 2 panics on pristine code, 3/3 runs (2 direct + 1 via `just test`'s `reproduce-before`), deterministic (no seed/thread-count dependence - single-threaded, `num_threads` not touched). |

No further escalation (miri/ASAN/fuzzing) was needed or appropriate here - this is a plain,
always-checked (`debug_assertions`-independent) safe-Rust `Option::expect()` panic driven by
ordinary control flow, not memory-unsafety or malformed-input parsing; the mechanism was fully
understood from the second approach's first run, and the third approach (a real, standalone,
public-API-only repro) confirmed it end-to-end.

## Status

**Confirmed.** Reproduces 5/5 debug-build runs and reproduces in `--release` too (in-tree test),
plus 3/3 runs of the standalone `repro/` (both against a fresh network `git clone` and a local
checkout) - fully deterministic, no seed/thread/layout dependence. Confirmed in two inputs of the
same class (an outline/COLR glyph and a bitmap glyph, hitting the two different `Arc::get_mut`
call sites in `sync_glyph_cache`).

**Fully reproducible**: **Yes**. `repro/` drives `vello_cpu`'s real, public API
(`RenderContext::render_with`, `Resources::register_image`/`destroy_image`, `glyph_run`/
`atlas_cache`) exactly as a real embedder would, with no internal (`pub(crate)`) type touched and
no hand-constructed internal state standing in for anything. This is a single-process, single-
caller sequencing bug (not a two-participant/network scenario) - CONFIRMATION.md section 11's
"single-process is fine" carve-out applies directly (there is no second participant to fake): the
whole scenario is one caller making two, in-order, ordinary calls into the crate's own persistent,
documented "one `Resources` per renderer, reused across frames" usage pattern, with a real,
independently-reachable panic (a destroyed-image reference) as the trigger for frame 1.

## Class

crash

## Root cause

`RenderContext::render_with` (`sparse_strips/vello_cpu/src/render.rs`) calls
`resources.before_render(...)` (which, when the glyph atlas is in use, registers a clone of each
atlas page's `Arc<Pixmap>` into the private `ImageRegistry` via `register_atlas_page`), then
`self.dispatcher.rasterize(...)`, then unconditionally `resources.after_render()` (which
un-registers those same clones via `destroy_atlas_page`, restoring unique ownership of
`glyph_resources.pixmaps` for the next frame). `sync_glyph_cache`'s and
`clear_evicted_glyph_atlas_regions`'s `Arc::get_mut(...).expect("... uniquely owned ...")` calls
assume that restoration already happened by the time they run.

Nothing makes `after_render()` run if anything between it and `before_render()` panics - in
particular, if `self.dispatcher.rasterize(...)` panics for any reason unrelated to the glyph atlas
itself (demonstrated here via a destroyed-image reference, but any panic reachable from rasterizing
a frame that also uses the glyph atlas has the same effect), the registry keeps holding that
frame's atlas-page clone(s) forever - there is no other code path that ever revisits or corrects
this. The very next time `sync_glyph_cache` runs (the next successful `render_with` call on the
same, reused `Resources`, even for a completely unrelated, correctly-written frame), `Arc::get_mut`
on that page fails because the registry's stale clone is still there, and the `.expect()` panics -
turning a one-time, unrelated failure in an earlier frame into a permanent, confusingly-labeled
panic in every later frame that needs to sync a new glyph onto that page.

This is exactly the kind of "convention, not type-level guarantee" gap the candidate description
named: `before_render`/`after_render` and `ImageRegistry::register_atlas_page`/`destroy_atlas_page`
are meant to be paired, but nothing enforces the pairing across a panic (Rust's default unwind
behavior does not, by itself, run arbitrary non-`Drop` code on the way out - and no `Drop` impl
here currently reasserts the invariant either).

## Introduced by

46cc30b7 "Migrate vello sparse strips renderers to use glifo (#1562)" (2026-04-14). `git blame` on
both `Arc::get_mut(...).expect(...)` call sites in `text.rs` (the "replay" one at what is now line
175, and the "bitmap upload" one at what is now line 150) attributes their introduction to this
exact commit, which is also where the atlas-page/`ImageRegistry` register-then-destroy design
itself was introduced. The `before_render`/`after_render` pairing in `render.rs`'s `render_with`
was reshuffled by a later commit (011f4ff5 "vello_cpu: Unify render APIs (#1665)", 2026-06-10) but
the same unconditional, non-panic-safe call sequencing (`before_render` ... `rasterize` ...
`after_render`, nothing in between able to survive a panic) has been present continuously since
46cc30b7.

## The fix

Made `sync_glyph_cache` self-healing instead of merely assuming the invariant: at its very start,
before touching `glyph_resources.pixmaps`, it now unconditionally calls
`self.image_registry.destroy_atlas_page(page_index)` for every currently-known page. Normally this
is a harmless no-op (`after_render`'s `maintain_glyph_cache` already removed every page from the
registry at the end of the previous frame, so `destroy_atlas_page` just returns `false`). But if
the previous frame panicked before `after_render` ran, this proactively undoes the stale
registration left behind, restoring unique ownership *before* the `Arc::get_mut` calls that assume
it - so a failure in a previous, unrelated frame can no longer turn a later, otherwise-correct
frame's glyph sync into a panic.

This is a 3-line-of-logic, additive fix (`patch.diff`, +15/-0) scoped to the one function that
actually needs the invariant re-established; it does not change behavior on any previously-passing
path (verified: full project test suite identical before/after, see below) and does not attempt to
change unwind/panic semantics elsewhere (an alternative considered: wrapping the whole
`before_render`/`rasterize`/`after_render` sequence in `catch_unwind` + `resume_unwind` in
`render.rs` - rejected as a larger, `std`-only, no-std-incompatible change for the same net effect
this smaller, unconditional-cleanup fix already achieves).

Verified against both inputs in the same bug class (CONFIRMATION.md section 7): re-running
`repro/` against the patched tree, both the outline/COLR-glyph scenario and the bitmap-glyph
scenario now render frame 2 successfully instead of panicking (see `output.txt` for the full
before/after transcript, captured from an actual `cargo run` of `repro/` against a real clone of
this project at `base_commit`, both before and after `git apply`-ing `patch.diff`).

- Project test suite (`cargo test -p vello_cpu --lib`, default features): **81 passed, 0 failed**,
  identical before and after.
- `cargo test -p vello_cpu --lib --features multithreading,f32_pipeline`: **87 passed, 0 failed**,
  identical before and after.
- `cargo test -p vello_cpu --lib --all-features`: **87 passed, 0 failed**, identical before and
  after.
- `cargo test -p glifo --lib`: **22 passed, 0 failed**, identical before and after (sanity baseline
  - `glifo` itself isn't touched by this fix).
- No new test failures introduced; the existing suite never previously exercised a mid-`render_with`
  panic followed by a reused `Resources`, so this is a genuine, previously-untested gap being
  closed, not a regression risk.

## What's not done / open questions

- The fix is scoped to `sync_glyph_cache` (both its `Arc::get_mut` call sites). The third,
  sibling call site in `clear_evicted_glyph_atlas_regions` (`text.rs`, "... uniquely owned during
  region clearing") was **not** independently exercised this session - it's only reached when a
  glyph gets evicted from the cache (`drain_pending_clear_rects` yielding at least one entry),
  which needs enough distinct glyphs cached to trigger eviction, not just one panicking frame plus
  one following frame. Since `sync_glyph_cache` now runs its defensive cleanup for every known page
  before *any* of the three call sites' code runs (uploads, replays, *and* whatever
  `maintain_glyph_cache`/`clear_evicted_glyph_atlas_regions` would later see), the same stale-clone
  state that would have broken this third call site is already cleaned up earlier in the same
  frame - reasoned to be covered by the same fix, but not separately reproduced with its own
  dedicated repro this session. A future candidate wanting to construct that specific repro would
  need to fill the atlas enough to force an eviction in the same frame that follows a mid-render
  panic.
- The general shape of this bug (any code path that pairs a "register into a shared registry"/
  "un-register" step around a fallible operation, without a `Drop`/`catch_unwind`-based guarantee
  that the un-register step always runs) is not otherwise grepped for across the rest of
  `vello_cpu`/`vello_hybrid` this session - `ImageRegistry`'s own plain `register`/`destroy` (for
  ordinary user-uploaded images, not atlas pages) has no equivalent paired-cleanup step at all, so
  it isn't structurally at risk the same way, but this specific pattern (paired register/destroy
  around a fallible middle step) might be worth a grep in a future session if similar designs show
  up elsewhere in the codebase.
- Did not attempt to make the underlying `rasterize()` panic paths themselves impossible (e.g.
  making `ImageSource::OpaqueId` resolution failures a recoverable `Result` instead of a panic) -
  that's a separate, much larger API design question, and out of scope for this bounded fix.
