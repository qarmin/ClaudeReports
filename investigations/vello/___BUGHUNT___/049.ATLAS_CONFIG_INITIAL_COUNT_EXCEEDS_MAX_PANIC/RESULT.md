# ATLAS_CONFIG_INITIAL_COUNT_EXCEEDS_MAX_PANIC

`vello_common::multi_atlas::MultiAtlasManager::new` loops `config.initial_atlas_count` times
calling `self.create_atlas().expect(...)`, but `create_atlas()` returns `Err` once
`self.atlases.len() >= self.config.max_atlases`. `AtlasConfig` is a fully `pub` struct with two
unrelated, unvalidated fields (`initial_atlas_count`, `max_atlases`) - nothing in the type or its
constructors enforces `initial_atlas_count <= max_atlases`. This looked suspicious because it is a
public, embedder-facing config struct (part of `vello_hybrid`'s public `MemorySettings` and
directly constructible via `vello_common`) whose two independent fields have a cross-field
invariant enforced only by a panic deep inside the callee, not by validation at the config's
construction site or its main consuming constructor.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build | `AtlasConfig { initial_atlas_count: 4, max_atlases: 2, ..Default::default() }` via a standalone crate calling the real public `MultiAtlasManager::new`, `cargo run` (debug) | Panics every run: "Failed to create initial atlas: AtlasLimitReached { max_atlases: 2, diagnostics: Unavailable }" |
| Same, repeated 4 more times | identical config, identical debug build | Panicked all 4 additional times (5/5 total) |
| Same, additional boundary values in the same class | `initial_atlas_count: usize::MAX, max_atlases: 0`; `initial_atlas_count: 9, max_atlases: 8` | Both panic identically |
| Code reading: is this reachable through `vello_hybrid`'s real GPU renderer entry points too? | Read `Renderer::new_with` (wgpu and webgl backends) | Not reachable there: both call `settings.memory_settings.normalize(&device_limits)` before `Resources::new`, and `normalize` clamps `initial_atlas_count` to `max_atlases` (`sparse_strips/vello_hybrid/src/render/common.rs:124-126`). So the higher-level `vello_hybrid::Renderer` API is already defended against this specific misconfiguration. |

Only one tool class was needed (manual boundary-value construction on a debug build) since this is
a plain, deterministic logic error in an unchecked-invariant loop, not something needing
miri/ASAN/fuzzing to surface.

## Status

**Confirmed**: reproduces 5/5 runs, debug build (`cargo run`, no `--release`), 100% deterministic
(no seed/layout dependency - it is a direct arithmetic/logic condition, not memory-layout
sensitive). Also reproduces in `--release` builds against a fresh, independent clone (see
`repro/`/`output.txt`), since the loop bound check is unconditional Rust code, not a
`debug_assert!`.

**Fully reproducible**: **Yes**. `repro/` depends on `vello_common` via its real, fully public API
(`vello_common::multi_atlas::{AtlasConfig, MultiAtlasManager}`, both `pub` items of a published
crate) and calls the exact same `MultiAtlasManager::new` that every real caller (including
`vello_hybrid::ImageCache`/`Resources`, and any third-party crate depending on `vello_common`
directly) goes through. There is no second participant to fake here: `AtlasConfig` is a plain data
struct with no invariant enforced anywhere in its type or a builder, so any embedder assembling one
by hand (exactly as `repro/src/main.rs` does, including via the idiomatic
`..Default::default()` pattern) is a completely genuine, unremarkable way to produce this exact
value - the same class of "malformed input handed to a public entry point" carve-out `CONFIRMATION.md`
section 11 describes for a parser's `open_file`, applied to a public constructor + a public config
struct instead of a public file-parsing entry point.

## Class

crash (panic on a value the same crate's own public config type has no mechanism to reject)

## Root cause

`MultiAtlasManager::new` (`sparse_strips/vello_common/src/multi_atlas.rs:117-133`) does:

```rust
for _ in 0..config.initial_atlas_count {
    manager
        .create_atlas()
        .expect("Failed to create initial atlas");
}
```

`create_atlas()` (lines 141-147) intentionally returns `Err(AtlasError::AtlasLimitReached { .. })`
once `self.atlases.len() >= self.config.max_atlases` - this is the atlas-count cap working exactly
as designed for calls made *after* construction (see `test_atlas_limit`, which exercises this
`Err` path directly and expects it). The constructor, however, unconditionally calls `.expect(...)`
on that same `Result`, converting the cap's normal operation into an unconditional panic whenever
`initial_atlas_count > max_atlases`. `AtlasConfig` (lines 663-676) is a fully `pub` struct with
both fields independently settable and no `Default`/builder-level check tying them together, so any
caller assembling one (directly via `vello_common`, or via `vello_hybrid::MemorySettings::image_atlas_config`
if that is ever handed to a `vello_common` API that skips `vello_hybrid`'s own `normalize()` step)
can trigger this.

Note this is *not* reachable through `vello_hybrid`'s actual GPU renderer construction
(`Renderer::new`/`new_with` for both the wgpu and webgl backends): `render/common.rs`'s
`MemorySettings::normalize` clamps `initial_atlas_count` to `max_atlases` before `Resources::new`
is ever called, so the misconfiguration is silently corrected on that specific path. The bug is
real and unconditionally reachable, though, through `vello_common`'s own public API surface
(`MultiAtlasManager::new`, and equivalently `ImageCache::new_with_config`, which forwards directly
to it) - `vello_common` is a standalone published crate ("Core data structures and utilities shared
across the Vello rendering") with no dependency on `vello_hybrid`'s `normalize()` safety net, and
both `multi_atlas` and `image_cache` are `pub mod`s with `pub fn` constructors documented as
accepting "custom atlas configuration".

## Introduced by

b7aac65f "[`vello_hybrid`]: Multiple image atlases support (#1252)" - this commit introduced
`MultiAtlasManager::new` (then in `sparse_strips/vello_hybrid/src/multi_atlas.rs`, `pub(crate)`)
with the exact same unchecked `for _ in 0..config.initial_atlas_count { ... .expect(...) }` loop
body verified via `git show b7aac65f -- sparse_strips/vello_hybrid/src/multi_atlas.rs`. At that
point the type was crate-private, so it was not yet externally reachable. It became part of
`vello_common`'s public API (making it reachable by any external crate depending on
`vello_common` directly, not just `vello_hybrid`) in f102e2c5 "refact: Move `ImageCache` and
`MultiAtlasManager` to vello_common (#1457)", which changed `fn new` from `pub(crate)` to `pub`.

## The fix

Clamp the loop bound to `max_atlases` in `MultiAtlasManager::new`, mirroring exactly what
`vello_hybrid`'s own `MemorySettings::normalize` already does for the same two fields one layer up:

```rust
for _ in 0..config.initial_atlas_count.min(config.max_atlases) {
```

This is sufficient and not narrow to the one triggering input: re-tested against
`initial_atlas_count: usize::MAX, max_atlases: 0` and `initial_atlas_count: 9, max_atlases: 8`
(one-past-the-boundary) in addition to the original `4` vs `2` - all three now construct
successfully with `atlas_count()` equal to `max_atlases`, and the normal case
(`initial_atlas_count <= max_atlases`) is unaffected since `.min()` is a no-op there.

Before/after re-running `repro/` against the patched tree (see `output.txt` for the full captured
output): before the patch, all three configs panic; after `git apply patch.diff`, all three
construct successfully and the process exits 0.

- Project test suite (`cargo test -p vello_common`, main worktree at commit
  7516831426fb70f36d62c521c5477f6a7265b36a): 159 passed; 0 failed; 1 ignored - before the patch.
  159 passed; 0 failed; 1 ignored - after the patch (identical counts; the existing suite does not
  exercise `initial_atlas_count > max_atlases`, which is exactly why this went unnoticed).

## What's not done / open questions

- `ImageCache::new_dummy()` and every other call site in this codebase that constructs an
  `AtlasConfig` use compatible, in-range values (`initial_atlas_count <= max_atlases`), so no other
  internal call site was found to be currently affected - this is purely a defense against a
  misconfiguration an external embedder could introduce, not a bug currently triggered by any code
  in this repository.
- Whether `vello_hybrid`'s own `MemorySettings::normalize` should be considered a workaround for
  this rather than independent, intentional clamping is a maintainer judgment call; either way, the
  underlying `vello_common` public constructor should not be able to panic on a plain, valid-looking
  config value regardless of what any one downstream consumer happens to do before calling it.
- Did not check whether other `vello_common` public constructors have a similar
  "two independently-settable `pub` fields, cross-field invariant enforced only by a later panic"
  shape elsewhere in the crate; this was investigated as a single, already-identified candidate
  rather than a broader sweep for the same pattern.
