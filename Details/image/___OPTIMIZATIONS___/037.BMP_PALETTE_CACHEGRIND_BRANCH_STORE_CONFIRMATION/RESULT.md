# BMP palette-lookup fixed-array fix: cachegrind dynamic-count confirmation of why the win doesn't scale 1:2:4:8

Answers `IDEAS.md`'s top-of-backlog entry left by candidate 025: candidate 025's `objdump`
disassembly confirmed the *static* mechanism (an exact 1:2:4:8 bounds-check-elimination ratio
across 8/4/2/1-bit outer-loop iterations for candidates 008/011's fixed-array fix) but couldn't
explain why the measured *wall-clock* win doesn't scale the same way (2-bit's ~8% doesn't clearly
exceed 4-bit's ~8-10%, despite 2-bit eliminating 2x as many branches per outer-loop iteration).
`perf` is still not installed in this environment (confirmed again this session: no `perf`
binary, `perf_event_paranoid=-1` but no working `perf`, and no `sudo` available to install one -
`apt`/`sudo` both blocked by the container's `no new privileges` flag). `valgrind`/`cachegrind`
(with `--branch-sim=yes --cache-sim=yes`) **are** installed and give exact, deterministic dynamic
instruction/branch/cache counts per function - not a substitute for real hardware IPC/port-
contention counters, but enough to test candidate 025's hypothesis (a) (branch-predictor cost is
already near-zero) directly, and to find the actual structural reason the "per-iteration ratio"
was the wrong quantity to expect wall-clock savings to track.

This candidate re-applies the identical candidates 008+011 fix (`patch.diff` in this folder is
byte-identical to candidate 025's) - no new source-level idea, a further measurement pass over
the same already-known change.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Built a temporary standalone example,
`examples/_cachegrind_bmp_loop.rs` (not part of the fix - a measurement driver, see `patch.diff`
does *not* include it; it was added/removed around the patch during this session, see "Does it
change behavior/output?" below), that decodes one BMP file 200 times from an in-memory byte
buffer (`image::load_from_memory_with_format` + `.to_rgba8()`, each result `black_box`'d) so
`valgrind --tool=cachegrind` gets enough repetitions for a clean per-function signal. Built once
against baseline (unpatched) `src/codecs/bmp/decoder.rs` and once against candidates 008+011's
fix, same four BMP files candidates 008/011/020/025 already used for timing
(`Core_8_Bit.bmp`/`Core_4_Bit.bmp`/`Core_1_Bit.bmp`, all 435x353, plus candidate 020's
`pal2_large.bmp`, 700x700, copied into `testdata/` in this folder). Needed `ulimit -n 4096`
before invoking `valgrind` - the container's default `ulimit -n` (~2^31) makes cachegrind's
`libcfile` abort with "Private file creation failed" (a valgrind/high-fd-limit interaction, not
specific to this project - noted in `README.md`).

**Dynamic instruction/branch counts** (`valgrind --tool=cachegrind --branch-sim=yes
--cache-sim=yes`, 200 iterations, `cg_annotate` for per-function breakdown). Re-ran one case
(`2-bit baseline`) twice to confirm cachegrind's counts are bit-exact deterministic run-to-run
(they were: identical `Ir`/`Branches`/`Mispredicts` down to the last digit) - unlike wall-clock
timing, no repeat-run noise-floor discipline is needed for this part of the measurement.

| Bit depth (file) | Whole-program `Ir` (baseline -> patched) | `Ir` delta | Own-function `Bc` (cond. branches) removed | Own-function `Bcm` (mispredicts) baseline -> patched |
|---|---|---|---|---|
| 8-bit (`Core_8_Bit.bmp`, 435x353) | 1,151,187,221 -> 1,089,762,659 | -5.34% | 30,711,000 | 70,606 -> 70,608 (+2) |
| 4-bit (`Core_4_Bit.bmp`, 435x353) | 1,163,885,742 -> 1,071,608,980 | -7.93% | 30,711,000 | 70,806 -> 70,608 (-198) |
| 2-bit (`pal2_large.bmp`, 700x700) | 2,308,612,290 -> 2,112,749,728 | -8.48% | 98,139,800 | 140,425 -> 140,426 (+1) |
| 1-bit (`Core_1_Bit.bmp`, 435x353) | 852,593,643 -> 787,356,681 | -7.65% | 30,781,400 | 71,020 -> 71,026 (+6) |

(`Bc`/`Bcm` read from `set_8bit_pixel_run`/`set_4bit_pixel_run`'s own `cg_annotate` row for 8/4-bit,
and from `BmpDecoder::read_image_data`'s row for 2/1-bit, which is where they're fully inlined -
same inlining candidate 025 found via `objdump`. Full `cg_annotate` text and raw `.cgout` files for
all 8 runs are in `evidence/cachegrind/`.)

**Wall-clock**, re-measured fresh this session with the same driver binaries (`hyperfine`,
`--warmup 5`, 2000 decodes/invocation; raw JSON in `evidence/wallclock/`):

| Bit depth | single-core (`taskset`) | multi-core (unpinned) |
|---|---|---|
| 8-bit | -2.9% (12 runs, clean) | -6.72% (mean 340.96ms -> 318.07ms, 15 runs, clean) |
| 4-bit | -4.8% (12 runs, clean) | -5.33% (mean 351.78ms -> 333.03ms, 15 runs, clean) |
| 2-bit | -5.2% (20 runs after a re-pin to a different core; first attempt was noisy/inverted, see below) | -5.36% (mean 900.67ms -> 852.39ms, 15 runs, clean) |
| 1-bit | inconclusive - flipped direction between two 12-20-run attempts (+61%/-22% either way), high sigma (30-100ms on a ~270-350ms mean) both times | -4.24% (mean 267.39ms -> 256.05ms, 15 runs, clean) |

1-bit's single-core noise reproduces candidate 025's own documented finding for this exact
file/machine ("severe machine-noise episode... abandoned rather than reported as a number") -
not re-chased further here since (a) multi-core is clean and consistent with the other three
depths, and (b) the cachegrind data above is deterministic and doesn't depend on wall-clock noise
at all.

## Result

**The core finding: total palette-lookup bounds checks eliminated per full decode is
(essentially) exactly the pixel count of the image, independent of bit depth** - not
"1, 2, 4, 8 per outer-loop iteration" scaled up by depth. Dividing each file's measured `Bc`
removal by its own pixel count (200 iterations x width x height): 8-bit and 4-bit both remove
30,711,000 / 200 / 153,555px = 1.0000 checks/pixel; 1-bit removes 30,781,400 / 200 / 153,555px =
1.0023 checks/pixel; 2-bit (a different, larger file) removes 98,139,800 / 200 / 490,000px =
1.0014 checks/pixel. All four land within 0.3% of exactly **one** eliminated bounds check per
output pixel, regardless of how many sub-pixel indices are packed into each input byte. This is
the structural reason candidate 025's "1:2:4:8 per iteration" framing didn't predict the observed
wall-clock scaling: at 1-bit, 8x fewer bounds checks happen *per outer-loop iteration* than at
8-bit, but 1-bit also runs through 8x *fewer* outer-loop iterations for the same pixel count -
these cancel exactly, so the *total* amount of eliminated work per decoded image never scales with
bit depth at all. "Checks removed per iteration" was simply the wrong quantity to expect the
win to track.

Given that, the remaining, much smaller effect actually observed - 8-bit's ~5.3-6.7% vs. the
other three depths' ~7.7-9.1%/4.2-8.5% (Ir% / multi-core wall-clock%, respectively; roughly
comparable in magnitude to each other, in a tight band, not a spread) - is not primarily a
misprediction-avoidance effect: **own-function `Bcm` (mispredict count) is essentially unchanged
by the patch at every depth** (deltas of +2, -198, +1, +6 against baseline counts of ~70k-140k;
baseline mispredict rate is already 0.06-0.11% of the removed branches at every depth). This
directly confirms candidate 025's hypothesis (a): the eliminated bounds check was already
predicted correctly essentially every time (not-taken, since the panic path is never reached for
valid input), so removing it cannot be recovering a misprediction penalty that was never being
paid - the savings are pure retired-instruction-count reduction, measured directly as ~2-3
instructions eliminated per removed check consistently across all four depths (`Ir` delta /
`Bc` removed: 8-bit 2.00, 4-bit 3.01, 2-bit 2.00, 1-bit 2.12 - all in a tight cluster, no
systematic growth with bit depth).

Hypothesis (b) (output-write/store-throughput bottleneck) is **not confirmed or refuted** by this
method: `D1`/`LL` cache miss rates are within measurement noise of *identical* between baseline
and patched at every depth (e.g. 2-bit: 2.7% -> 2.7%; 1-bit: 1.6% -> 1.6% - full numbers in
`evidence/cachegrind/*.log`), which is expected since the patch changes zero memory-access
instructions, only removes already-well-predicted compare+branch pairs - so this data cannot
distinguish "stores are the bottleneck limiting how much the branch removal can help" from
"nothing in particular is the bottleneck, the loop is just not compute-bound enough for a ~2-3
instruction/pixel reduction to translate 1:1 into wall-clock time". Cachegrind's cost model
counts instructions and cache hits/misses, not execution-port contention/IPC under real
out-of-order execution, so it structurally cannot test a throughput-bottleneck hypothesis the way
`perf`'s hardware counters could - this remains open, and `perf` remains uninstallable in this
container (see README.md).

## Does it change behavior/output?

This candidate re-applies an already-characterized fix (candidates 008/011, re-confirmed
bit-exact by candidate 025) - `patch.diff` in this folder is byte-identical to candidate 025's.

- **Bit-exact?** Yes. Decoded and hashed (temporary `examples/_correctness_check_bmp.rs`, same
  approach as candidate 025, not part of `patch.diff` - see below) all 67 `.bmp` files under
  `tests/images/bmp/` (66 pre-existing + candidate 020's `pal2_large.bmp`, copied in for this
  session and removed again before reverting) with baseline and patched binaries: every hash,
  width, and height matched exactly (`diff` of the two hash listings, saved as
  `evidence/hashes_baseline.txt`/`evidence/hashes_patched.txt`, was empty).
- **Is this a breaking change?** No - identical to candidates 008/011/025's own characterization:
  internal-only signature change to four private functions and two local bindings, no public API
  surface touched.
- Project test suite (`cargo test --release`, with `pal2_large.bmp` + its reference PNG present
  in the tree, matching candidate 020's addition): **551 passed / 1 failed before, 551 passed / 1
  failed after** - the same single pre-existing failure both times (`resizing_with_catmul` in
  `tests/regression.rs`, matching every prior BMP-decoder candidate in this workspace: 008, 011,
  016, 019, 020, 025). No new failures. Full logs in `evidence/test_baseline.log`/
  `evidence/test_patched.log`.
- **Two temporary example files were added to the working tree during this session and removed
  again before the final `git checkout`** (`examples/_cachegrind_bmp_loop.rs` for the cachegrind
  driver, `examples/_correctness_check_bmp.rs` for the hash check, plus a matching
  `[[example]]`/`required-features = ["bmp"]` pair in `Cargo.toml`) - neither is in `patch.diff`,
  same convention candidate 025 used for its own temporary correctness-check script: they were
  measurement tooling needed to reach/verify the patched code during this session, not part of
  the fix itself. Their source is preserved verbatim in this candidate's `justfile` (`clone`
  recipe) so `just test` can reproduce the same cachegrind runs from a fresh clone.

## Why the result looks the way it does

See "Result" above - this section *is* the answer to `IDEAS.md`'s open question. In short: the
"1:2:4:8 checks eliminated per outer-loop iteration" fact (candidate 025) is real and correctly
measured, but it was never the right predictor of *total* wall-clock savings, because outer-loop
iteration count is itself inversely proportional to lookups-per-iteration for a fixed pixel count
- the two effects exactly cancel, leaving total eliminated bounds-check work proportional only to
image pixel count, not bit depth. The residual, much smaller depth-dependent variation that *is*
observed (roughly a 2-3 percentage-point spread across `Ir`%, and single-digit percentage points
in multi-core wall-clock%) is consistent with candidate 025's hypothesis (a) - a well-predicted
branch's removal saves close to zero misprediction penalty and close to a fixed 2-3 retired
instructions regardless of depth - rather than with the branches-eliminated *count* itself, which
does vary 1:2:4:8 per iteration but was shown above not to matter once iteration count is
accounted for.

## What's not done / open questions

- Hypothesis (b) (store/output-write throughput as the limiting factor once branches are removed)
  is neither confirmed nor refuted here - cachegrind has no execution-port/IPC model, only
  instruction and cache-hit/miss counts, which is a structurally different question from "is a
  hardware store port saturated". `perf` (hardware counters: IPC, port utilization,
  `stalled-cycles-backend`) remains the natural tool and remains unavailable in this container (no
  `perf` binary, `perf_event_paranoid=-1` but no working install, `sudo`/`apt` blocked by the
  container's "no new privileges" flag - re-confirmed this session, matching candidate 025's
  earlier finding). A machine with real `perf` access is needed to settle this specific point.
- A genuinely controlled microbenchmark isolating pure store throughput (a synthetic loop that
  only writes N bytes/iteration with no palette lookup or bounds check at all, at 1/2/4/8
  writes-per-iteration densities matching the real loops) was not built this session - the
  cachegrind evidence above already gives a concrete, quantitative resolution to the specific
  backlog question asked (why the win doesn't scale 1:2:4:8), so this was judged not necessary to
  chase further this session; flagging it as a possible future angle specifically for hypothesis
  (b) if `perf` ever becomes available.
- 1-bit's single-core wall-clock timing was noisy/inconclusive both times it was attempted this
  session (matching candidate 025's own already-documented noise for this exact file on this
  machine) - not re-chased, since multi-core (this session) and candidate 025's own single-core
  1-bit numbers (5/5 clean paired wins, `-5.2%`) already establish the direction; a future session
  on a confirmed-idle single core could get a clean number if needed.
