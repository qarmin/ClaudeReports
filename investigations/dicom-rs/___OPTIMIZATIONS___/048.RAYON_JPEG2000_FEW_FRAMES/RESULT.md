# JPEG 2000 native (openjpeg-sys) internal threading: few-frame/small-image vs bulk/large-image

Follow-up to candidate 030 (`RAYON_JPEG_FEW_FRAMES`), whose own "what's not done" section
explicitly left `dicom-transfer-syntax-registry`'s other rayon/thread-gated codecs unchecked
(only the baseline `jpeg-decoder`/rayon path was tested there). This candidate checks the
`jpeg2k` crate's `openjpeg-sys` backend instead: `transfer-syntax-registry/Cargo.toml`'s
`openjpeg-sys-threads` feature (`= ["rayon", "openjpeg-sys", "jpeg2k/threads"]`, already present
in the tree, off by default) forwards to `jpeg2k`'s own `threads` feature, which forwards again
to `openjpeg-sys?/threads` - i.e. it enables the *native C OpenJPEG library's own internal
multithreading* (tile-level parallelism inside a single image decode), a completely different
mechanism from candidate 030's `jpeg-decoder`/rayon row-upsampling worker pool. No source patch
was written or applied for this candidate - see "Does it change behavior/output?" below; this is
a pure measurement of an existing, already-wired, off-by-default feature flag.

`dicom_transfer_syntax_registry::adapters::jpeg2k::Jpeg2000Adapter` only implements
`decode_frame` (single frame); the trait's default `decode` (bulk, all frames) is a plain
sequential loop calling `decode_frame` once per frame (`encoding/src/adapters.rs:348-354`) - so,
unlike `jpeg-decoder`'s persistent global rayon pool (candidate 030's mechanism, where dispatch
cost amortizes across repeated calls to the same warm pool), every single JPEG 2000 frame decode
here is an independent `Image::from_bytes()` call with its own from-scratch thread-pool
setup/teardown inside the C library. This turned out to make **image size (pixel count per
frame), not frame count per call**, the variable that actually predicts the few-frame vs bulk
tradeoff for this codec - the opposite axis from candidate 030's finding, discovered by testing a
size/timing matrix rather than assuming the same axis applied.

## What was tried

Two builds of `harness/bench_pipeline` (copied into an isolated git worktree at
`.worktrees/cand048`, no Cargo.toml edits needed - features enabled directly via
`--features dicom-transfer-syntax-registry/<feature>` on the command line):

- **baseline**: `--no-default-features --features dicom-transfer-syntax-registry/openjpeg-sys`
  (native OpenJPEG C backend, threading off)
- **threaded**: `--no-default-features --features dicom-transfer-syntax-registry/openjpeg-sys-threads`
  (same backend, C-library internal multithreading on)

Both backed by the same `jpeg2k 0.10.1` / `openjpeg-sys 1.0.12`. Four real `dicom-test-files`
fixtures (`pydicom/...`), copied into this folder's own `testdata/`, spanning image size:

| File | Rows x Cols | Frames | baseline decode-frame-raw p50 |
|---|---|---|---|
| `emri_small_jpeg_2k_lossless.dcm` | 64x64 | 10 | 0.339 ms |
| `693_J2KR.dcm` | 512x512 | 1 | 11.4 ms |
| `MR2_J2KR.dcm` | 1024x1024 | 1 | 54.9 ms |
| `RG3_J2KR.dcm` | 1760x1760 | 1 | 80.8 ms |

Timings via `tools/paired.sh` (alternating A/B, `RUNS=9`), both builds' `decode-frame-raw` mode
(isolates `decode_pixel_data_frame(0)`, one `Jpeg2000Adapter::decode_frame` call/iteration).
Single-core via a `taskset -c 0` wrapper script around each binary; multi-core unpinned (24
logical cores, `uptime` load ~3.5-9 during this session - other `___OPTIMIZATIONS___` candidates
building/benchmarking in parallel, all builds/benches below run through
`flock .locks/build.lock` so none overlapped a sibling's). `decode-raw` (bulk, all frames/call)
was also checked on the one multi-frame fixture (`emri_small...`, 10 frames) - see below.

| Fixture | mode | single-core p50 (baseline -> threaded) | multi-core p50 (baseline -> threaded) |
|---|---|---|---|
| `emri_small...` (64x64, 10 frames) | `decode-frame-raw` (1 frame/call) | 0.321ms -> 2.298ms (+616.4%, 0/9 threaded-faster) | 0.328ms -> 1.266ms (+286.0%, 0/9) |
| `emri_small...` (64x64, 10 frames) | `decode-raw` (10 frames/call, bulk) | 3.334ms -> 22.952ms (+588.3%, 0/9) | 3.337ms -> 13.128ms (+293.4%, 0/9) |
| `693_J2KR.dcm` (512x512, 1 frame) | `decode-frame-raw` | 11.403ms -> 13.541ms (+18.8%, 0/9) | 11.485ms -> 3.604ms (-68.6%, 9/9) |
| `MR2_J2KR.dcm` (1024x1024, 1 frame) | `decode-frame-raw` | 54.930ms -> 56.700ms (+3.2%, 0/9) | 55.328ms -> 10.271ms (-81.4%, 9/9) |
| `RG3_J2KR.dcm` (1760x1760, 1 frame) | `decode-frame-raw` | 80.793ms -> 85.006ms (+5.2%, 1/9) | 80.819ms -> 18.516ms (-77.1%, 9/9) |

All paired comparisons above are fully consistent (9/9 or 0/9, "verdict: consistent" from
`paired.sh`) except `RG3_J2KR.dcm` single-core (1/9, still effectively one-directional).

## Result

Two clearly different regimes, split by per-frame pixel count rather than frames-per-call:

- **Tiny images (64x64, the only multi-frame fixture available)**: enabling
  `openjpeg-sys-threads` is a **large, consistent regression** on both single-core (+286% to
  +616%) and multi-core (+286% to +293%) - and, notably, **the regression is essentially the same
  relative magnitude for both `decode-frame-raw` (1 call) and `decode-raw` (10 calls in a bulk
  loop)**. This is the key structural difference from candidate 030: there, bulk calls amortized
  the fixed per-call dispatch tax and multi-core bulk decode actually became a net win; here, bulk
  decode is just N independent `decode_frame` calls, each paying the same tax with no persistent
  warm pool to amortize into, so "decode more frames per call" does not change the outcome at all
  for this codec.
- **Real-sized single frames (512x512 and up)**: single-core-pinned shows a **small, consistent
  regression** (+3.2% to +18.8%, i.e. no benefit and a small real cost when only one core is
  available - unsurprising, thread-pool setup/join cost with no core to actually parallelize onto)
  while unrestricted multi-core shows a **large, consistent win** (-68.6% to -81.4%, 9/9 in all
  three cases) - large enough, real per-image parallel decode work that the C library's internal
  thread-pool overhead pays for itself many times over.
- The crossover between these two regimes lies somewhere between 64x64 (4,096 px, always a
  regression regardless of core count) and 512x512 (262,144 px, a win once >1 core is available) -
  not narrowed further within this candidate's scope.

## Does it change behavior/output?

**N/A - measurement/investigation only, no patch applied to real source.** The feature being
measured (`openjpeg-sys-threads`) already exists in `transfer-syntax-registry/Cargo.toml`, is
already off by default, and no code was changed for this candidate - the comparison is baseline
feature set vs. an existing opt-in feature set, both unmodified from `HEAD`. No `patch.diff`
against the tested project (correctly - see `SKILL.md`'s "patch.diff is mandatory..." section,
second bullet), and no harness `Cargo.toml` edit either, since both variants were reached purely
via `--features dicom-transfer-syntax-registry/<name>` on the command line.

- **Bit-exact?** Yes. Decoded-pixel checksums (harness's own `fnv1a` over the decoded bytes)
  matched exactly between baseline and threaded builds for every fixture: `emri_small...` ->
  `0x950a0a291aac6686` (frame) / `0x25dc6fc51d003384` (bulk), `693_J2KR.dcm` ->
  `0x2b8c8991b916db01`, `MR2_J2KR.dcm` -> `0x5b78f39e5b95d7e0`, `RG3_J2KR.dcm` ->
  `0x99340967f779d304`.
- **Is this a breaking change?** N/A, no change proposed. If a maintainer *did* want to flip this
  feature's default on, it would be a real breaking-ish change for any single-core-constrained or
  small-image deployment (regression in both cases above), so it should stay an explicit opt-in,
  not a new default - but this candidate does not recommend or apply that.
- Project test suite: `cargo test -p dicom-transfer-syntax-registry --release --no-default-features
  --features openjpeg-sys` -> **3 passed, 0 failed** (2 + 1 lib/doctest results; all other
  `test result:` lines in the crate are 0/0/0, no other test files gated on this feature).
  `--features openjpeg-sys-threads` -> **identical: 3 passed, 0 failed**. Also ran
  `cargo test -p dicom-pixeldata --release --no-default-features --features image,native,openjpeg-sys`
  -> **75 passed (62+1+1+11), 0 failed, 2 ignored**, and the same with
  `,dicom-transfer-syntax-registry/openjpeg-sys-threads` added -> **identical: 75 passed, 0
  failed, 2 ignored**. No new failures either way, both crates.

## Why the result looks the way it does

`openjpeg-sys`'s `threads` feature enables OpenJPEG's own internal multithreading, which (unlike
`jpeg-decoder`'s rayon integration) is per-`opj_decode()`-call: the C library appears to spin up
and tear down its own thread pool/work-sharing machinery fresh for every single image decode,
with no persistent, warm, cross-call pool to amortize that setup cost into (candidate 030's
`jpeg-decoder`+rayon path, by contrast, uses one lazily-initialized *global* rayon pool shared by
every decode call in the process for the whole program's lifetime - the first call pays init cost,
every later call doesn't). That difference in pooling model is exactly why candidate 030's
"bulk calls amortize the tax, few-frame calls don't" shape does not reproduce here: for JPEG 2000,
`decode` (bulk) is dicom-rs's own plain sequential loop over `decode_frame`, so a 10-frame bulk
call pays the setup/teardown tax **ten separate times**, once per frame, identically to calling
`decode_frame` ten times by hand - there is no dicom-rs-level or jpeg2k-level batching that could
amortize it. What *does* determine whether the tax pays for itself is how much actual decode work
happens inside that one call: a 64x64 image (4,096 px) has too little tile/wavelet work for the
C library's internal parallel decode to outrun its own thread-pool overhead on any core count
tested; a 512x512 image (262,144 px, 64x more pixels) has enough that a completely different
outcome appears once more than one core is actually available to use.

## What's not done / open questions

- The crossover point (image size at which threading stops regressing on multi-core) was not
  narrowed below "somewhere between 64x64 and 512x512" - no fixture between those two sizes was
  readily available in `dicom-test-files`'s `pydicom/` set; a synthetic JPEG 2000-encoded fixture
  at e.g. 128x128/256x256 would need the `jpeg2k` crate's own *encoder* (not currently used
  anywhere in this workspace's harness) to build, which was out of scope for this candidate.
  Added to `IDEAS.md`.
- Only `dicom-test-files`' `pydicom/` JPEG 2000 fixtures were tried (all 8-or-16-bit monochrome
  radiographs/MRI); a large multi-component (RGB) JPEG 2000 image was not tested and might show a
  different per-pixel decode cost (more work per pixel could shift the crossover point lower).
- `emri_small_jpeg_2k_lossless.dcm` was the only *multi-frame* JPEG 2000 fixture found in
  `pydicom/` (10 frames, but only 64x64 each, i.e. deep in the "always regresses" tiny-image
  regime) - the "does a bulk call of *large* frames behave differently from N individual calls of
  the same large frame" question (the more direct analogue of candidate 030's own comparison) was
  not directly testable with available fixtures, since no large multi-frame JPEG 2000 DICOM file
  was found. Based on the mechanism identified above (bulk is just N independent per-frame calls
  for this codec, no shared pool), the expectation is that it would not differ from N separate
  `decode-frame-raw` calls of the same image - but this is inference from the mechanism, not
  independently measured.
- `jpeg2k`'s `openjp2` backend (pure-Rust OpenJPEG port, the other backend option in
  `transfer-syntax-registry/Cargo.toml`) has no `threads` feature at all (confirmed via the
  crate's published feature metadata: `threads = ["openjpeg-sys?/threads"]`, a no-op unless
  `openjpeg-sys` is also selected) - not a candidate for this kind of investigation at all, only
  `openjpeg-sys` is.
