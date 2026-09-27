# jpeg2000_hint_hang_leftover

From `___BUGHUNT___/IDEAS.md`'s backlog (candidate 008's own leftover observation, re-checked
after candidate 009's fix): candidate 008's 100000x100000 malformed-JPEG2000 PoC PDF was reported
to still hang/thrash past a 20s timeout through the real, top-level `hayro::render()`, *even with*
candidate 009's downscale-hint fix applied (which now correctly computes a small target). The open
question left unanswered was whether that hang is a separate, CPU-bound (non-allocation) unbounded
work item elsewhere in the JPEG2000 decode pipeline - similar in shape to candidate 003's JBIG2
hang - that neither candidate 008's nor candidate 009's own fix would cover.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Applied both candidate 008's (`hayro-jpeg2000/src/j2c/decode.rs`, `MAX_TOTAL_SAMPLE_COUNT` allocation cap) and candidate 009's (`hayro/src/image.rs`, downscale-hint origin-subtraction fix) `patch.diff` together to the real working tree, added a temporary `eprintln!` in `hayro-jpeg2000/src/j2c/codestream.rs::read_header` printing `min_num_resolution_levels`/`skipped_resolution_levels`/the resulting downscaled `image_width()`/`image_height()` (reverted before write-up), then rebuilt `hayro`'s own `examples/render.rs` and ran it against the exact same 800-byte PoC PDF (`cm 100 0 0 100 50 50` placement) `--release` | `cargo build --release -p hayro --example render`, `timeout 25` | Completed in ~106ms (process-level, includes startup), exit 0, no hang. Debug print showed `min_num_resolution_levels=2 skipped_resolution_levels=1 target_resolution=Some((100, 100)) image_width_after=50000 image_height_after=50000` - i.e. candidate 009's fix does correctly compute a small `(100, 100)` hint, but the underlying PoC codestream's own declared `min_num_resolution_levels=2` (inherited unmodified from the small real fixture candidate 008 hex-patched) caps `skipped_resolution_levels` at 1, so the *actual* post-downscale dimensions are still `50000x50000` - exactly the "small, fixed factor - nowhere near enough" shape the backlog note predicted. |
| Same rebuild, but with candidate 008's fix reverted (only candidate 009's fix kept), to check whether *that* specific combination (correct-but-still-large hint, no allocation cap) is what actually produced the originally reported "hang/thrashes" observation | `git checkout -- hayro-jpeg2000/src/j2c/decode.rs` (candidate 008's fix removed), rebuild, same PoC PDF | Not run to completion (see "What's not done" - judged too risky to run unbounded on a shared dev machine); reasoned instead from the debug-print numbers above: `50000 * 50000 = 2.5e9` samples/component, `* 2 components * 4 bytes/f32 = 20,000,000,000` bytes (20GB) - a `checked_mul` that does *not* overflow `usize` (unlike candidate 008's own `100000x100000` no-hint case, which was also checked_mul-safe but produced an 80GB request that a real allocator visibly failed on instantly). 20GB is within this dev machine's total RAM (`free -h`: 29Gi total, 24Gi available at the time), so a real allocator attempting it is plausible to actually succeed (or thrash while doing so) rather than fail fast - a very different, much slower failure shape than an instant `memory allocation of N bytes failed` abort. This is offered as the most likely explanation for the earlier session's "hangs/thrashes past a 20s timeout" observation (which was very likely captured *before* candidate 008's own allocation cap was reapplied on top of candidate 009's fix, not with both fixes combined) - not independently re-verified by actually letting a 20GB allocation run on shared infrastructure. |
| Built a standalone, self-contained `repro/` (own `Cargo.toml`, real `hayro`/`hayro-syntax` public API, no source instrumentation) that builds the identical PoC PDF and calls the real, public, top-level `hayro::render()`, timing it internally with `std::time::Instant`; ran via `just test` against a **fresh clone** of the pristine `base_commit` tree with **both** candidate 008's and candidate 009's `patch.diff` applied together (concatenated into this candidate's own `prerequisite_fixes.diff`, since a candidate folder can't reference another candidate folder's files) | `just test` (real `git clone` + `git apply` + `cargo run --release`), then 4 more direct repeat runs of the built binary (5 total) | 5/5 runs: `render()` returns in 169-230 **microseconds** every time (not milliseconds, not seconds) - see `output.txt`. No hang, no thrashing, no timeout needed. |

## Status

**Not reproducible.** The suspected separate, CPU-bound (non-allocation) hang does not exist:
once candidate 008's and candidate 009's own already-verified fixes are applied **together** (the
realistic scenario if both get merged, which is what this candidate set out to check), the exact
same PoC PDF that was reported to hang for 20+ seconds instead completes in well under a
millisecond, 5/5 runs, via the real `hayro::render()` entry point.

## Root cause (of the original, now-refuted "hang" observation)

There is no additional unbounded-CPU code path to root-cause - the JPEG2000 decode pipeline
downstream of `DecoderContext::reset` (`build::build`, `segment::parse`,
`decode_component_tile_bit_planes`, IDWT, MCT, sign-shift) all iterate over resolution levels and
sample counts that are already bounded by the *actual*, post-downscale `image_width()`/
`image_height()` (already reduced by `skipped_resolution_levels` at header-parse time, before any
of these stages run) and by the codestream's own real (small, 111-byte) entropy-coded tile-part
data - none of it scales with the raw, attacker-declared `Xsiz`/`Ysiz` once downscaling has been
applied. The only place that ever scaled directly with the raw declared dimensions was exactly the
eager `SimdBuffer::zeros(sample_count)` allocation in `DecoderContext::reset` that candidate 008's
fix already caps.

The most likely explanation for why the earlier session (candidate 009) observed an apparent hang
even "with the downscale-hint fix applied": the downscale-hint fix alone still leaves the *actual*
allocation size for this PoC around 20GB (`50000 * 50000 * 2 components * 4 bytes`, bounded by
`min_num_resolution_levels=2` capping `skipped_resolution_levels` at 1) - within, or close to, this
kind of dev machine's total physical RAM, unlike the un-hinted `100000x100000` case's 80GB request,
which fails an allocator immediately and visibly. A ~20GB allocation attempt is far more likely to
manifest as a slow, resource-contending "hang/thrashing" (matching the exact wording used) than a
clean, fast abort - without candidate 008's own cap also being present to reject it up front. In
other words, this was very likely candidate 008's own already-diagnosed and already-fixed bug,
observed through a test that (plausibly) hadn't re-applied candidate 008's own fix alongside
candidate 009's at the time.

## The fix

No fix from this candidate - nothing new was found to fix. Both fixes that make this PoC
safe (candidate 008's allocation cap, candidate 009's hint-computation fix) already exist,
independently verified and recorded, in `../008.jpeg2000_dimension_cap_missing_oom/patch.diff` and
`../009.image_downscale_hint_missing_origin_subtraction/patch.diff`. This candidate's own
`prerequisite_fixes.diff` is only a concatenation of those two existing, already-merged-into-
their-own-candidate-folders patches, reproduced here for `just test`'s self-containment - it does
not introduce anything new.

- Project test suite: not run for this candidate (no source change of its own - the two
  prerequisite patches applied here already have their own before/after test-suite numbers
  recorded in their own `RESULT.md` files).

## What's not done / open questions

- The "only candidate 009's fix, without candidate 008's fix" combination (the specific state
  hypothesized above as what the original "hang" observation actually tested) was **not**
  independently re-run to completion on this session's shared dev machine - deliberately, to avoid
  attempting a real ~20GB allocation (close to the machine's ~24GB available RAM at the time,
  observed via `free -h`) that could destabilize other work on the same host. The explanation above
  is inferred from the arithmetic (`checked_mul` doesn't overflow at this size, unlike the 80GB
  no-hint case) and from the debug-print-confirmed `skipped_resolution_levels`/downscaled-dimension
  values, not from an actually-executed thrash. A future session with a dedicated, memory-limited
  sandbox (e.g. `systemd-run --scope -p MemoryMax=...`, unavailable in this session's environment -
  no user D-Bus session) could confirm this mechanically rather than by inference.
- This candidate only re-checked the exact PoC from candidates 008/009 (a 2-component, 111-byte
  real codestream with a hex-patched `100000x100000`/`XTsiz`/`YTsiz` header declaring
  `min_num_resolution_levels=2`). A codestream with a different, still-small
  `min_num_resolution_levels` combined with a much larger declared size might still land in a
  window where the post-downscale sample count is large enough to be slow but small enough to
  evade candidate 008's `MAX_TOTAL_SAMPLE_COUNT` cap (256M samples) - that boundary itself wasn't
  swept for a genuinely slow-but-under-the-cap case, only this one specific PoC.
- Does not revisit or change candidate 008's or candidate 009's own `RESULT.md` findings - both
  remain accurate as originally written; this candidate closes out the specific "what's not
  done" open item they each flagged, with a negative result (no further code path needed fixing).
