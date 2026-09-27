# CLIP_LEAF_CPU_SHIM_STACK_POP_UNWRAP_UNBALANCED_CLIP

`vello_shaders/src/cpu/clip_leaf.rs`'s `clip_leaf_main` (the CPU emulation of classic vello's
`clip_leaf.wgsl` compute shader, used whenever `RendererOptions::use_cpu` is `true`) does
`let tos = stack.pop().unwrap();` for every `EndClip` entry in `clip_inp`, with no check that a
matching `BeginClip` was ever pushed onto the stack first. `vello_encoding::Encoding`'s own public
builder API (`encode_begin_clip`/`encode_end_clip`) always keeps `BeginClip`/`EndClip` balanced
(`encode_end_clip` is a guarded no-op unless `n_open_clips > 0`), and `Encoding::append` preserves
that invariant by induction (it only ever concatenates two streams that individually satisfy it),
so this isn't reachable through `vello::Scene`'s normal drawing API alone. It *is* reachable through
`vello::Scene::encoding_mut()` - a real, public, and explicitly documented ("this can be used to
more easily create invalid scenes, and so should be used with care") escape hatch that hands out a
`&mut Encoding`, whose fields (`draw_tags`, `path_tags`, `n_paths`, `n_clips`, `n_open_clips`) are
all `pub` - letting a caller append a raw `DrawTag::END_CLIP` directly, bypassing
`encode_end_clip`'s guard entirely.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `Encoding::encode_begin_clip`/`encode_end_clip`/`append`, `Scene::push_layer`/`pop_layer`, `Resolver::resolve` (`vello_encoding/src/resolve.rs`), and every call site of `append`/`push_layer`/`pop_layer` in `vello/src/scene.rs` (including the `skrifa` `ColorPainter` COLRv1 glyph-rendering callbacks) to check whether the *safe*, guarded public API alone can ever produce an unbalanced `BeginClip`/`EndClip` sequence | Reading only | No: `encode_end_clip`'s `n_open_clips > 0` guard, plus `append`'s pure counter-sum + stream-concatenation (which preserves LIFO nesting by induction over two already-balanced-or-open-prefix streams), together make it structurally impossible to reach an `EndClip` with no preceding, unconsumed `BeginClip` through `Scene`'s normal drawing methods alone - even across `Scene::append`, unmatched extra `pop_layer()` calls (a no-op), or a COLRv1 font whose paint graph pushes/pops layers unevenly (still routed through the same guarded `push_layer`/`pop_layer`). This ruled out the "malformed merge across `Encoding::append`" hypothesis from the candidate description as not reachable via the safe API. |
| Found `vello::Scene::encoding_mut()` - a real, public, `#[doc]`-warned ("can be used to more easily create invalid scenes") accessor that bypasses the guard - and used it (matching the existing precedent of this project's own confirmed candidate 051, `SWAP_LAST_PATH_TAGS_LEN_UNDERFLOW`, which used the same "public raw `Encoding` field access" class of trigger) to append a raw `DrawTag::END_CLIP` (plus the matching dummy `PathTag::PATH`/`n_paths`/`n_clips` bookkeeping that `encode_end_clip` itself performs, minus its guard) after one normal `scene.fill(..)` call, then rendered through the real `vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`, driven by a real (software Vulkan/SwiftShader) `wgpu::Device` | Release build, `WGPU_BACKEND=vulkan`, `VK_ICD_FILENAMES` pointed at SwiftShader's ICD JSON, `RUST_BACKTRACE=1`, 3 runs | Panics deterministically, 3/3 runs, at exactly `vello_shaders/src/cpu/clip_leaf.rs:59:35`, `called \`Option::unwrap()\` on a \`None\` value` |
| Second, differently-shaped input in the same bug class (per `CONFIRMATION.md` section 7): one legitimate, balanced `push_layer`/`pop_layer` pair (stack genuinely empties once), followed by a *second*, separately-appended dangling `END_CLIP` with no matching begin at all | Same build/config, 1 run against pristine code, 1 run against patched code | Pristine: panics identically at `clip_leaf.rs:59:35`. Patched: `rendered OK, no panic` |
| Attempt + verify a bounded fix (treat `stack.pop() == None` as "no active clip" instead of unwrapping) | Release build, both repro shapes, 3/3 + 1/1 runs before and after, project test suite before/after | See "The fix" below |

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs (first repro shape) + 1/1 (second, differently-shaped
repro), release build, real `wgpu::Device` (SwiftShader software Vulkan ICD). See `output.txt` for
the full captured panics and test-suite output, `repro/` for the runnable reproduction.

**Fully reproducible**: **Yes**. The repro drives the real, public entry point
(`vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`) with a real `Scene`
through a real (software) `wgpu::Device` - the exact same call a real application using
`use_cpu: true` makes. Per `CONFIRMATION.md` section 11, this is a single-participant case: the
"adversarial" input is a single external artifact (an `Encoding` built by a caller of the public
`vello_encoding`/`vello::Scene::encoding_mut()` API, exactly the same kind of state a real, if
unusual, caller of that public surface can genuinely construct - not bytes/state standing in for
what some other process would send). No second participant is faked anywhere in this repro.

## Class

crash (unguarded `Option::unwrap()` panic; the real GPU/WGSL shader, `clip_leaf.wgsl`, hits the
analogous condition - `parent == -1` in its `search_link`/parent-chain resolution when there's no
matching begin - and degrades to a wrong-but-non-crashing result via wgpu/naga's implicit
storage-buffer bounds clamp on the resulting out-of-range `clip_inp[parent]` read, rather than a
crash; this is the same CPU-shim-vs-GPU-shader asymmetry pattern already documented in this
workspace's own 047/066/075 candidates for other CPU-shim files)

## Root cause

`clip_leaf_main` iterates `clip_inp[0..config.layout.n_clips]` in encode order, maintaining a
`Vec<ClipStackElement>` as a stack: a `BeginClip` entry (`path_ix >= 0`) pushes, an `EndClip` entry
(`path_ix < 0`) is supposed to pop the matching `BeginClip`. The pop is
`let tos = stack.pop().unwrap();` (line 59, unchanged since the CPU shim's introduction) - if
`clip_inp` ever contains an `EndClip` for which the stack is already empty (no unconsumed
`BeginClip` precedes it), this panics instead of handling the condition. Reaching that state
through `Encoding`'s *guarded* public API (`encode_begin_clip`/`encode_end_clip`, and `append`,
which sums `n_clips`/`n_open_clips` and concatenates `draw_tags` - preserving balance by induction)
is not possible: `encode_end_clip` is a no-op whenever `n_open_clips == 0`. It *is* reachable via
`vello::Scene::encoding_mut()`, a real public accessor whose own doc comment already flags it as
capable of "creat[ing] invalid scenes" - a caller using it to append a raw `DrawTag::END_CLIP`
(mirroring `encode_end_clip`'s own internal bookkeeping - pushing a matching dummy `PathTag::PATH`
and incrementing `n_paths`/`n_clips` - but skipping the `n_open_clips` guard) produces exactly this
state. `Resolver::resolve` performs no validation of `BeginClip`/`EndClip` nesting anywhere before
handing the packed buffers to the render pipeline, so the malformed encoding reaches
`clip_leaf_main` unchanged.

## Introduced by

d2c2a267 "Add CPU shaders" (2023-09-19). This is the commit that added the entire CPU-shader
emulation module (`vello_shaders/src/cpu/`, originally at `src/cpu_shader/clip_leaf.rs`),
including `clip_leaf.rs`'s `stack.pop().unwrap()` at what was then line 59 - unchanged since
(confirmed via `git log -p --follow` and `git blame` on the current file). `Scene::encoding_mut()`
(the accessor this repro uses to reach the bug) is a separate, later addition, but is not the
introducing commit for the bug itself - the missing check was already latent in `clip_leaf_main`
from the CPU shim's very first commit, `Scene::encoding_mut()` only later made it reachable from
outside the crate.

## The fix

Changed the unconditional `stack.pop().unwrap()` to a `stack.pop()` match: when `Some(tos)`, behave
exactly as before (write `draw_monoids`, compute `bbox` from the new stack top or `BIG_BBOX`);
when `None` (no matching `BeginClip`), skip the `draw_monoids` writes entirely (there is no `tos`
to source them from) and report `BIG_BBOX` for `clip_bboxes[global_ix]` - the same "no active clip"
value already used when the stack legitimately becomes empty. This exactly preserves output for
every well-formed (balanced) encoding, and turns the malformed-input case from a panic into a
silent, safe degradation, mirroring how the real GPU/WGSL shader already degrades (a clamped
out-of-range read) instead of crashing for the analogous condition.

```rust
} else {
    // end clip
    let bbox = if let Some(tos) = stack.pop() {
        draw_monoids[clip_el.ix as usize].path_ix = tos.path_ix;
        draw_monoids[clip_el.ix as usize].scene_offset =
            draw_monoids[tos.parent_ix as usize].scene_offset;
        draw_monoids[clip_el.ix as usize].info_offset =
            draw_monoids[tos.parent_ix as usize].info_offset;
        if let Some(nos) = stack.last() {
            nos.bbox
        } else {
            BIG_BBOX
        }
    } else {
        // No matching `BeginClip` for this `EndClip` (a malformed `Encoding` - the clip-nesting
        // invariant isn't enforced by `Encoding`'s public API, see `Scene::encoding_mut`).
        // Degrade gracefully instead of panicking.
        BIG_BBOX
    };
    clip_bboxes[global_ix as usize] = bbox;
}
```

Verified: re-ran both `repro/` binaries against the patched tree - `clip_underflow` (3/3 runs) and
`clip_underflow_variant2` (a second, differently-shaped malformed input: one legitimate
push/pop pair followed by a separate dangling end-clip) both print `rendered OK, no panic`
instead of panicking (both panicked 100% of the time against pristine code). See `output.txt` for
the full before/after captures.

- Project test suite (`cargo test -p vello_shaders -p vello_encoding -p vello --release`): 31
  passed, 0 failed before the patch (all of `vello_shaders`'/`vello_encoding`'s/`vello`'s unit +
  doc tests); 31 passed, 0 failed after (identical - the patch only changes behavior for the
  previously-panicking "unmatched `EndClip`" input, which no existing test exercises).

## What's not done / open questions

- The real GPU/WGSL `clip_leaf.wgsl` shader was read but not actually run against the same
  malformed encoding in this session (no CPU-shim bypass was needed to reproduce the CPU-side
  bug, and this workspace's established precedent - candidates 047/066/075 - is that GPU storage
  buffer OOB reads/writes clamp silently rather than crash; this candidate reasons from that same
  established precedent rather than re-verifying it against this exact input) - a maintainer
  wanting pixel-level confirmation of "GPU degrades gracefully" specifically for this shader would
  need to actually run it once with `use_cpu: false`.
- This is the same underlying gap (a caller-constructible `Encoding` with unbalanced
  `BeginClip`/`EndClip`) as candidate 048's finding style and candidate 051's
  `SWAP_LAST_PATH_TAGS_LEN_UNDERFLOW` - `Encoding`'s public field/accessor surface has no
  central validation of any of its documented invariants (clip nesting here; "at least one
  transform/style before any draw object" per its own doc comment elsewhere). A single,
  centralized "is this a well-formed encoding" check (e.g. behind a `debug_assert!`/`cfg(debug_assertions)`
  validation pass in `Resolver::resolve`) would catch this whole class at once instead of requiring
  a defensive patch in each downstream CPU-shim consumer separately - out of scope for this
  session's bounded fix.
