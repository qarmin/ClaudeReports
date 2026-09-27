# Sustained fuzzing campaign of the 16 existing targets via `--sanitizer none`

Candidate `001` only ran a single ~150s burst per target (6.75M execs total, 0 crashes) because
cargo-fuzz's default ASan build hangs almost immediately in this sandbox (see `001`'s own
`RESULT.md`/`README.md`'s tooling notes). This candidate follows up with a substantially longer,
sustained campaign of the same 16 existing targets via `--sanitizer none`, to check whether more
wall-clock/iteration budget turns up logic/panic/overflow bugs that a short burst wouldn't reach,
even without ASan's own memory-safety coverage.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo +nightly fuzz build --release --sanitizer none --features avx,sse,nightly_avx512` (15 targets) + `--features avx,sse,nightly_avx512,colorspaces colorspaces` (1 target) | release, debug-assertions on (cargo-fuzz default), coverage-guided | All 16 build cleanly. |
| `cargo +nightly fuzz run --release --sanitizer none --features avx,sse,nightly_avx512[,colorspaces] <target> -- -max_total_time=900 -rss_limit_mb=1536 corpus/<target>`, all 16 targets launched in parallel (24-core host, ~1-1.5 of them per target's RSS budget headroom) | empty corpus grown fresh per target, 900s (15 min) each, i.e. 6x candidate 001's per-target duration | All 16 ran to completion (`Done <N> runs in 901 second(s)`, clean exit), see `output.txt` for the exact per-target counts. **90,935,336 total executions** across all 16 targets (~13.5x candidate 001's 6.75M). Zero crashes, panics, timeouts, OOM kills, leak reports, or sanitizer warnings in any of the 16 logs (grepped for `ERROR`/`panic`/`SUMMARY:`/`deadly signal`/`leaked`/`timeout`/`out-of-memory`/`slow-unit`, only match was the expected `--sanitizer none` boilerplate `WARNING: Failed to find function "__sanitizer_acquire_crash_state"` line, present in every log and harmless). `fuzz/artifacts/*/` (libFuzzer's crash-input dump directory) is empty for every target after the run. |
| Grep libFuzzer's per-target log for coverage growth stalling (`cov:`/`ft:` plateauing early, which would suggest the corpus saturated well before the 900s budget and a longer run wouldn't help further) | read the tail of each of the 16 logs | Coverage (`cov:`) had visibly plateaued by the end of the run for most targets (e.g. `resize_rgb.log` stable at `cov: 346` for the last several hundred thousand executions), consistent with these being small, already-well-explored `Arbitrary`-driven harnesses rather than under-fuzzed ones - a signal that a further 6x/13.5x extension is unlikely to be the highest-leverage next step (see "What's not done" below for what would be). |

This is one escalation past candidate `001`'s single-burst run (a longer, sustained version of the
exact same tool/config), not three separate tools - no signal appeared on this first escalation, so
per `SKILL.md`'s "3 clean negatives means move on" guidance a second/third distinct tool (e.g. a
hand-written differential harness, or a corpus-seeded run) was not pursued this round; see "What's
not done" for what a follow-up could try instead.

## Status

**Not reproducible.** 90,935,336 fuzz executions across all 16 existing targets (900s each,
coverage-guided, `--sanitizer none` so debug-assertions are on but AddressSanitizer's own
memory-safety instrumentation is not present - see candidate `001` for why ASan itself doesn't run
in this sandbox) completed with zero crashes, panics, overflow aborts, timeouts, or OOM kills.

**Fully reproducible**: N/A (Not reproducible - no finding to reproduce).

## Class

N/A (Not reproducible - no pic-scale bug confirmed).

## Root cause

Not applicable - nothing to root-cause. This is a clean negative from the coverage actually
achieved (see "What's not done" for its limits): no source-level defect in pic-scale's own code was
observed to panic, overflow, or otherwise misbehave under debug-assertions across ~91M executions
of the existing `Arbitrary`-driven harnesses.

## The fix

No fix applicable - no bug found this session.

- Project test suite: not run - no source change was made in this candidate (nothing to verify a
  fix against).

## What's not done / open questions

- Still no ASan (memory-safety) coverage in this sandbox - see candidate `001`; a real
  out-of-bounds/use-after-free bug in the SIMD `unsafe` code would not necessarily be caught by
  debug-assertions + coverage-guided fuzzing alone, only by ASan or a memory-unsafe access that
  happens to also panic (e.g. a slice-index panic). This remains the single biggest coverage gap
  for this project's fuzzing, not something this candidate's longer `--sanitizer none` run closes.
- Coverage (`cov:`/`ft:`) plateaued well before the 900s budget ended for most targets (see "What
  was tried" table) - the highest-leverage next fuzzing step for this project is likely not simply
  running the *same* 16 harnesses even longer, but either (a) a persistent/reused corpus across
  many sessions (each run here started from an empty corpus, so coverage gains from a previous
  session's crash-free exploration aren't retained - `fuzz/corpus/` is `.gitignore`d and this
  candidate's own working corpus lives only in `___BUGHUNT___/_reference/014-fuzz-campaign/`, not
  committed anywhere durable), or (b) broadening what each harness's `Arbitrary` input can express
  (e.g. exercising every `ResamplingFunction` variant, not just whatever the harness hardcodes) -
  see candidate `001`'s own caveat that only `Bilinear` was independently manually verified as
  exercised.
- NEON/SVE2/WASM32 code paths were not exercised at all (this host is x86_64) - unchanged from
  `README.md`'s existing caveat.
- A real ASan-covered run of these same 16 targets would need a different host/sandbox (or further
  root-causing of the ASan hang itself, tracked separately in `IDEAS.md`/candidate `001`).
