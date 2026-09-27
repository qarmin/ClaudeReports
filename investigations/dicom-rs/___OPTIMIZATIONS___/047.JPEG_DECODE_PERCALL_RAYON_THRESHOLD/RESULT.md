# JPEG decode per-call rayon-pool threshold guard

Follow-up to candidate 030 (`RAYON_JPEG_FEW_FRAMES`), whose own "what's not done" section left
open a middle ground: instead of a build-time `default-features` flip (all-or-nothing for the
whole process), gate rayon dispatch behind a per-call frame-count/pixel-count threshold, the same
pattern candidates 003/004/006/012/023 already use for `dicom-pixeldata`'s own `par_iter()`
call sites. This candidate investigates whether `jpeg-decoder` 0.3.2's public `Decoder` API
allows selecting the worker backend per call, and (since it doesn't) implements and measures the
one externally-reachable substitute: routing few-frame decode calls through a small, dedicated
`rayon::ThreadPool` instead of the ambient global one, in
`transfer-syntax-registry/src/adapters/jpeg.rs` (the crate that actually calls
`jpeg_decoder::Decoder::decode()` - `dicom-pixeldata`'s own `rayon` feature just forwards to this
crate's `rayon` feature, per `pixeldata/Cargo.toml`'s `rayon = ["dep:rayon", ...,
"dicom-transfer-syntax-registry/rayon"]`).

**API investigation finding (no patch possible here):** `jpeg-decoder` 0.3.2 already has its own
internal per-call heuristic - `Decoder::select_worker` (`decoder.rs:245`) always requests
`PreferWorkerKind::Multithreaded`, but downgrades it to `Immediate` (no threading at all) when the
frame's `width * height <= 128*128` (16,384 pixels). This threshold is a private `const` with no
public setter, getter, or `Decoder` builder method to override it, and there is no other public
API to force `Immediate` for a larger image or to pass in a specific `Worker`/pool. For any
realistic DICOM image (this test file is 640x480 = 307,200 pixels/frame, ~19x over that
threshold), `select_worker` always picks `Multithreaded` regardless of how many frames the caller
plans to decode in this call - there is nothing to gate from the caller's side via jpeg-decoder's
own API.

The one externally-reachable lever: `jpeg-decoder`'s rayon worker
(`src/worker/rayon.rs`) does not hold a reference to a specific pool - it calls bare
`rayon::in_place_scope(...)` and `.par_chunks_mut(...)`, which run on whatever pool is "ambient"
for the calling thread (the global pool by default, or a custom pool if the call happens inside
that pool's own `ThreadPool::install(...)` closure - ordinary, documented rayon behavior, not an
API added for this). This candidate's patch exploits exactly that: wrap `decoder.decode()` in a
lazily-built, dedicated small pool via `.install()` for a call site's few-frame case, and leave
the call unwrapped (ambient global pool, current behavior) for the many-frame case.

## What was tried

Two rayon-pool sizes for the small pool were tried under the guard (`FEW_FRAMES_RAYON_THRESHOLD =
4` frames per call; `decode_frame` is always 1, `decode`'s bulk loop passes its own `nr_frames`):
`num_threads(2)` and `num_threads(1)`. Both gave the same qualitative (regression) result; the
table below reports the `num_threads(1)` variant, which is what `patch.diff` contains.

`pydicom/color3d_jpeg_baseline.dcm` copy at `harness/bench_pipeline/testdata/` (640x480x3,
120-frame JPEG Baseline), via `___OPTIMIZATIONS___/tools/paired.sh` (alternating A/B,
`RUNS=9`, one pair discarded as warmup by the harness's own `stats()`). Box load during this
session was heavy and variable (`uptime` 15-30, several sibling `___OPTIMIZATIONS___` candidates
building/benchmarking in parallel) - every timed run below still went through
`flock .locks/build.lock` so no build/benchmark/test ever overlapped a sibling's, but the box's
own background CPU pressure is reflected in the somewhat noisier per-sample spread than a quiet
box would show; the paired win/loss counts are what carries the conclusion, not the absolute
p50s.

| Variant | mode | single-core p50 (`taskset -c 0`) | multi-core p50 (unpinned) |
|---|---|---|---|
| baseline (unguarded, ambient global pool always) | `decode-frame-raw` (1 frame/call) | 1.278ms | 1.771ms |
| patched, small pool `num_threads(2)` | `decode-frame-raw` | 3.091ms (+141.9%, 0/9 pairs patched-faster) | not separately re-run (2-thread variant superseded by 1-thread below) |
| patched, small pool `num_threads(1)` | `decode-frame-raw` | 3.064ms (+138.6%, 0/9) | 3.567ms (+101.4%, 0/9) |
| baseline | `decode-raw` (120 frames/call, bulk) | 193.474ms | 172.083ms |
| patched, `num_threads(1)` (threshold not triggered, ambient pool used, same as baseline) | `decode-raw` | 196.698ms (+1.7%, 0/9 patched-faster, i.e. 9/9 baseline-faster) | 174.482ms (+1.4%, 2/9 - not consistent) |

## Result

**This is a real, measured regression, not a win, for the exact scenario the candidate targeted**
(few-frame/single-frame decode calls): routing `decode_frame`'s single `Decoder::decode()` call
through a small dedicated `rayon::ThreadPool::install()` is **~101-142% slower** than leaving it
on the ambient global pool, consistently (9/9 or better paired wins for baseline in every round),
both single-core-pinned and unpinned multi-core, and independent of the small pool's thread count
(2 threads and 1 thread both regress by roughly the same amount). The bulk (`decode-raw`,
120 frames/call) path is correctly unaffected by the guard's threshold (routes through the same
ambient pool as baseline in both builds) - it shows only a small, single-core-consistent +1.7%
(9/9) that is not from pool selection (the threshold isn't triggered here) but from the
`decode_one_frame` wrapper's own added function-call/branch overhead on every frame, paid
120 times per bulk call regardless of which pool ends up being used; the unpinned multi-core
bulk comparison is within noise (2/9, not consistent).

## Does it change behavior/output?

- **Bit-exact?** Yes. `decode-frame-raw` and `decode-raw` checksums (`fnv1a` over the decoded
  bytes, printed by the harness) are identical between baseline and patched builds:
  `decode-frame-raw` -> `0x779c7e483f6c072b`, `decode-raw` -> `0x5ca32d369351efd1`, both builds,
  every run.
- **Is this a breaking change?** No public API signature changes; adds a new direct optional
  `rayon` dependency to `dicom-transfer-syntax-registry`'s own `Cargo.toml` (forwarded from its
  existing `rayon` feature, which already pulled `rayon` in transitively via
  `jpeg-decoder?/rayon`) - no observable behavior change for any downstream consumer beyond the
  measured timing regression above.
- Project test suite (`cargo test --workspace --release`): **537 passed, 0 failed, 6 ignored,
  0 build errors (`grep -cE '^error'` = 0), 58 `test result:` lines** - identical before
  (baseline, patch reverted) and after (patch applied). No new failures either way.

## Why the result looks the way it does

`rayon::ThreadPool::install()` on a pool other than the ambient ("current") one always performs a
real cross-thread handoff: the calling thread injects the closure as a job for the target pool
and parks (blocks) waiting for a completion signal, one of that pool's own worker threads (a real,
separate OS thread, spawned when the pool was built, kept alive for the pool's lifetime) picks the
job up, runs it, and wakes the caller back up. This round-trip (park + wake, twice) is paid on
**every** `install()` call, regardless of how many threads the pool has - `num_threads(1)`
doesn't avoid it, it just means there's exactly one worker thread available to receive the
handoff. The ambient global pool that jpeg-decoder's own `rayon::in_place_scope`/`par_chunks_mut`
calls fall back to when *not* wrapped in a custom `.install()` does not pay this extra outer
handoff for the "am I already on a pool thread" case relevant here - the initiating call
(`decoder.decode()` -> `WorkerScope::with` -> `rayon::in_place_scope`) runs directly on the
calling thread with `in_place_scope`, spawning inner tasks via the *global* pool only if/when
that pool is idle and available to steal them, and rayon additionally sizes its global pool's
default thread count from `std::thread::available_parallelism()`, which itself already reflects
`taskset`/cgroup affinity restrictions (i.e. under `taskset -c 0` the global pool already reduces
itself close to a single effective worker without any code in this codebase asking it to) - so
the "self-contention from an oversized thread pool" problem candidate 030 measured for the
*build-time* rayon-vs-no-rayon comparison does not reappear here as something a *smaller custom
pool* can additionally fix; the custom pool's own unavoidable extra handoff cost dominates
instead, on every single call, for a per-frame job small enough that this handoff cost is a much
larger fraction of the total decode time than it would be for a bulk multi-frame call (matching
candidate 030's own finding that rayon dispatch/wake overhead is close to a fixed tax per
`Decoder::decode()` call, not something that scales down with less total work per call - this
candidate's small-pool variant simply *adds* a second, separate instance of that same kind of
fixed tax on top, rather than removing the first one).

## What's not done / open questions

- Not tried: routing the few-frame case through `rayon::ThreadPoolBuilder::build_global()` at
  process start instead of a per-call custom pool. This would only be a genuine "no thread-pool
  overhead at all" option if it could reduce the *global* pool itself to 0 extra threads for the
  whole process, but `build_global()` is one-shot (panics if the global pool was already
  initialized by any other code path first) and is inherently process-wide, not a per-call
  decision - it doesn't fit the "per-call frame-count/pixel-count threshold guard" shape this
  candidate was scoped to, and was not measured.
- Not tried: patching `jpeg-decoder` itself (out of scope per this workspace's own vendoring
  rules - would need its own separate `___OPTIMIZATIONS___/` per SKILL.md's "Vendoring a suspect
  dependency" section) to expose `select_worker`'s threshold as a public `Decoder` setting, or to
  accept an externally-supplied `WorkerScope`/pool reference. Given the mechanism found here (any
  extra `install()` on a *different* pool adds its own handoff tax rather than removing the
  existing one), the more promising upstream fix would be exposing a way to force `Immediate`
  (no threading, not even a 1-thread pool) regardless of image size for a given `Decoder`
  instance/call - not investigated further since it requires an upstream API change, not something
  this repository's own source can express.
- Only tested against one file (`pydicom/color3d_jpeg_baseline.dcm`, 640x480, 120 frames); the
  small-pool overhead is unlikely to depend on file size/frame count given the mechanism
  identified above, but this wasn't independently re-confirmed on a smaller or larger image.
