# Drop `rayon` from `dicom-pixeldata`'s default features

Re-checks the backlog idea from session 1: `dicom-pixeldata`'s `default = ["rayon", "native"]`
(`pixeldata/Cargo.toml`) turns on `jpeg-decoder`'s `worker::rayon` backend for every consumer that
doesn't explicitly opt out, and a callgrind profile of bulk multi-frame JPEG decode showed ~7-9%
of total `Ir` in pure rayon/crossbeam dispatch machinery. The patch tested here is the one-line
change that removes `rayon` from that default list (`default = ["native"]`), leaving every other
default behavior (native JPEG/RLE/deflate) untouched.

Harness gained one new mode for this candidate: `pixel-frame` (decodes a single frame via
`decode_pixel_data_frame(0)`, parsing the file once outside the timed loop) - added specifically
to model "decode a small number of frames per call" as opposed to the existing `pixel` mode,
which decodes every frame of the file in one `decode_pixel_data()` call. See `README.md`.

**Harness bug found and fixed while setting this candidate up**: `harness/bench_pipeline/Cargo.toml`
had a direct dependency on `dicom-transfer-syntax-registry` with default features left on
(no `default-features = false`), left over from an earlier session and never exercised by
`main.rs` (grep confirms no `dicom_transfer_syntax_registry` symbol is used there at all). Cargo's
feature unification means that dependency's own `default = ["rayon", "simd"]` was activated
project-wide regardless of what `dicom-pixeldata`'s own `rayon` feature was set to - so the first
"no-rayon" binary built for this candidate still linked `rayon`/`crossbeam` (confirmed via
`nm | grep -c rayon_core` before/after: 208 hits despite `pixeldata`'s own feature being off).
Fixed by adding `default-features = false` to that harness dependency line. This is a
harness-only fix (not part of `patch.diff` - see below), but is left in place in the canonical
harness copy since it affects every future candidate that touches rayon-gated code in `pixeldata`.

## What was tried

Two binaries, both release builds of `harness/bench_pipeline` (with the harness fix above already
applied), differing only in `pixeldata/Cargo.toml`'s `default` feature line:

| Variant | `pixeldata` `default` | rayon/crossbeam symbols linked (`nm \| grep -c`) |
|---|---|---|
| baseline | `["rayon", "native"]` (unmodified) | 366 |
| patched | `["native"]` (this candidate's `patch.diff`) | 0 |

Timing, `pydicom/color3d_jpeg_baseline.dcm` (6.14MB, JPEG Baseline, multi-frame; `pixel-frame`
decodes only frame 0, `pixel` decodes all frames in the file in one call). All runs via
`tools/paired.sh` (alternating A/B, `RUNS` as noted); `taskset -c 0` used for single-core.
Background load on this box during this session: `uptime` load average ~9.8 (see
`README.md`/`projects/dicom-rs.md` - this box has real external load noise).

| Scenario | single-core p50 (baseline -> patched) | multi-core p50 (baseline -> patched) |
|---|---|---|
| `pixel-frame` (decode 1 frame/call), iters=2000 | round A: 2.450ms -> 1.934ms (-21.1%, 8/9 pairs); round B: 2.447ms -> 1.470ms (-39.9%, 9/9 pairs) - **consistent, patched faster** | round 1 (iters=300): 2.239ms -> 2.299ms (+2.7%, 3/9); round 2 (iters=2000): 2.356ms -> 2.311ms (-1.9%, 7/9); round 3 (iters=3000): 2.224ms -> 2.092ms (-6.0%, 11/15); round 4 (iters=3000, repeat): 2.182ms -> 2.283ms (+4.6%, 1/15) - **not consistent, direction flips between rounds** |
| `pixel` (decode all ~120 frames/call), iters=15 | 202.344ms -> 203.831ms (+0.7%, 1/9 pairs "B faster", i.e. 8/9 baseline faster) - marginal, baseline very slightly faster | round 1: 187.912ms -> 211.875ms (+12.8%, 2/9); round 2 (repeat): 186.249ms -> 210.113ms (+12.8%, 0/9) - **consistent, baseline (rayon) faster** |

Same-binary noise-floor control (`pixel-frame`, unpinned, 9 runs of the baseline binary against
itself): min 2.268ms, max 2.466ms - an ~8.7% spread, i.e. comparable in magnitude to the
unpinned `pixel-frame` A/B deltas above. This is why the unpinned `pixel-frame` rounds disagree
in direction between repeats: the real effect at this granularity, if any, is at or below this
box's noise floor for that specific scenario.

Callgrind (`Ir`, deterministic instruction count, not wall-clock) on `pixel-frame` iters=50:
baseline 1,862,017,717 in one run, 2,300,012,164 in a second run of the *same unmodified binary*
(23.5% swing) - i.e. **`Ir` itself is not deterministic for the rayon-enabled build**, because
valgrind serializes the program's threads and how much of the (real) work-stealing / spin-waiting
loop runs before another thread grabs the job varies between runs. The one baseline profile kept
(1,862,017,717 Ir) attributed `crossbeam_deque::Stealer::steal` (0.68%) + `rayon_core::registry`
iterator chain (0.55%) + `crossbeam_epoch::default::with_handle` (0.46%) = **~1.7% of `Ir`** to
pure thread-pool dispatch overhead for a single-frame decode - much smaller than the ~7-9% found
in session 1's *bulk* (`pixel`, all-120-frames) profile, consistent with per-call rayon dispatch
cost being roughly fixed per `Decoder::decode()` call and therefore a *smaller* fraction of a
call that does less total work in this specific counting, not a larger one. Given the
non-determinism above, this single-run percentage is not trusted as a precise figure, only as
supporting evidence that the wall-clock-neutral result for unpinned `pixel-frame` is plausible
(the effect is genuinely small relative to the actual decode work, not just hidden by measurement
error).

## Result

Not a single-direction win or regression - it depends on both (a) how many frames are decoded per
call and (b) whether the process can actually use more than one core:

- **Single-core-constrained + few-frame decode** (e.g. a container/VM pinned to 1 vCPU decoding
  one frame at a time, as a viewer scrolling through a multi-frame study might): removing `rayon`
  from defaults is a **real, reproducible win**, -21% to -40% wall-clock across 2 rounds (8/9 and
  9/9 paired wins) - the rayon-enabled build still spins up its worker pool and those threads
  contend with the main thread for the single pinned core, even though there's no other core for
  them to actually help on.
- **Unrestricted multi-core (realistic default: 24 cores available) + few-frame decode**: **no
  measurable wall-clock difference** - 4 rounds swing between -6.0% and +4.6% with no consistent
  direction, all within or close to this box's own same-binary noise floor (~8.7%) for this
  specific scenario.
- **Unrestricted multi-core + bulk (all-frames) decode**: removing `rayon` is a **consistent
  regression**, +12.8% wall-clock across 2 rounds (9/9 and 2/9-but-same-magnitude paired) - here
  the parallelism rayon provides across the JPEG fragments genuinely pays for its own dispatch
  cost and then some.
- **Single-core-constrained + bulk decode**: a wash either way (+0.7%, small effect, leans
  slightly toward keeping rayon) - unsurprising, since one core can't parallelize regardless of
  which build is used.

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum` of the full decoded+converted image bytes (`dump-image` mode)
  is identical between baseline and patched builds on `pydicom/color3d_jpeg_baseline.dcm`
  (`172d096545f3e79f4b3b4b1182059926aaf265c335ffd3fedd8206e557d6b142` both). Per-mode
  length/sum checksums (`pixel`, `pixel-frame`, `ndarray` modes) also matched exactly.
- **Is this a breaking change?** Yes, in the sense that matters for a Cargo feature default: any
  downstream crate currently relying on `dicom-pixeldata`'s default features implicitly enabling
  `rayon` (e.g. code that calls `rayon::` APIs itself, expecting the dependency to already be
  present) would need to add the feature explicitly after this change. It does not change any
  public API signature or file-format output.
- Project test suite (`cargo test --workspace --release`): **537 passed, 0 failed, 6 ignored,
  0 build errors (`grep -cE '^error'` = 0), 58 `test result:` lines** - identical before and after
  the patch. No new failures.

## Why the result looks the way it does

`jpeg-decoder`'s rayon-backed worker dispatches each `Decoder::decode()` call's row-upsampling
work onto the (lazily-initialized, process-wide) rayon thread pool - this happens once per JPEG
image decoded, regardless of whether the caller is decoding one frame (`decode_pixel_data_frame`)
or all of them in a loop (`decode_pixel_data`'s registered-decoder path, which also decodes frame
by frame internally for this transfer syntax). The dispatch/steal/pin overhead is close to a fixed
per-call tax, not something that scales down for a smaller number of frames - so a caller that
only ever asks for one frame at a time pays that tax on every call without the offsetting benefit
(more cores actually doing useful parallel work) that a bulk multi-frame call gets. On unrestricted
multi-core hardware that tax is small enough to be lost in this box's own measurement noise; on a
single-core-constrained deployment it's not a tax at all but genuine self-contention, and shows up
clearly and reproducibly.

## What's not done / open questions

- Only tested on one JPEG Baseline multi-frame file (`pydicom/color3d_jpeg_baseline.dcm`, 120
  frames, 640x480x3). Not tested against a file with only 2-5 frames (a more realistic "few
  frames" file than a 120-frame one) - the callgrind evidence suggests the per-call overhead
  doesn't depend on total frame count, but this wasn't independently confirmed on a smaller file.
- Not tested against JPEG2000/JPEG-LS/other codecs that also gate on `dicom-transfer-syntax-registry`'s
  own `rayon` feature (openjpeg-sys, jpeg2k) - this candidate only touched the `jpeg-decoder`
  path.
- The unpinned/multi-core `pixel-frame` noise floor (~8.7%) is specific to this box's current
  background load (`uptime` ~9.8 during this session) - a quieter box might show a small but real
  effect in either direction that this session's noise floor couldn't resolve either way.
- A middle ground not tried: instead of an all-or-nothing default, gate rayon dispatch behind a
  per-call frame-count or pixel-count threshold inside `jpeg-decoder` itself (not something
  dicom-rs can patch directly, since it's a third-party crate) - or expose a runtime choice in
  `dicom-pixeldata` between `decode_pixel_data` (bulk, benefits from rayon) and
  `decode_pixel_data_frame` (few-frame, doesn't) rather than a single build-time feature flag
  covering both call patterns. Added to `IDEAS.md`.
