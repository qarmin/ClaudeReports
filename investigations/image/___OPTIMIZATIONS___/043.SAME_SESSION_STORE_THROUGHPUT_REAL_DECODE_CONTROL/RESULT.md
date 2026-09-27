<!--
Measurement-only investigation whose "fix" is a re-application of already-known candidates
008/011's fix (patch.diff below is byte-identical to candidates 025/037's) purely so the real
BMP decode could be built and timed in this same session - no new source-level idea. Per
SKILL.md's "patch.diff is mandatory whenever the fix is a change to the tested project", the
patch.diff exists here because real source was actually built and measured during this
candidate, the same convention candidate 037 used for the identical reason.
-->

# Same-session real-BMP-decode control for the store-throughput comparison

Answers `IDEAS.md`'s top-of-backlog entry left by candidate 041: candidate 041's
store-share-of-real-decode-time table compared its own same-session synthetic
store-throughput ceiling against candidate 037's real-decode multi-core wall-clock numbers,
captured in a *different* session/machine-state - flagged explicitly as unconfirmed
cross-session risk ("What's not done", candidate 041). This candidate re-measures **both** the
synthetic store-throughput microbenchmark (candidate 041's own `bench/`, unmodified) **and** the
real BMP decode's wall-clock cost (candidate 037's own approach: `examples/_wallclock_bmp_loop.rs`
driver + `hyperfine`, same four BMP files, same patch) back-to-back in one session, removing the
cross-session confound candidate 041 flagged.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidates 037/041/042 - `HEAD`
had not moved since those sessions).

1. **Synthetic store-throughput microbenchmark**: copied candidate 041's `bench/` verbatim (no
   source changes - same `fill_natural`/`fill_volatile` loops, same 4 densities x 2 variants x 2
   size regimes, same 30-round round-robin design). Built once (`cargo build --release`) and ran
   4 times (2x `taskset -c 3`, 2x unpinned), matching candidate 041's own repeat count. `uptime`
   0.11-0.16 throughout (well under the 24-core budget). Raw output in
   `evidence/synth_{single,unpinned}_run{1,2}.txt`.

2. **Real BMP decode wall-clock**, measured in the *same* session, right after step 1: added a
   temporary example driver, `examples/_wallclock_bmp_loop.rs` (identical in content to candidate
   037's `_cachegrind_bmp_loop.rs`, renamed - decodes one BMP file N times via
   `image::load_from_memory_with_format(...).to_rgba8()`, each result `black_box`'d), plus a
   temporary `_correctness_check_bmp.rs` hash-check driver (identical to candidate 037's), with
   matching `[[example]]` entries added to `Cargo.toml`. Neither driver nor the `Cargo.toml`
   entries are part of `patch.diff` - measurement tooling, not the fix, same convention as
   candidate 037.
   - Copied candidate 020/037's `pal2_large.bmp` (700x700, 2-bit) + its reference PNG into
     `tests/images/bmp/images/` and `tests/reference/bmp/images/` for the 2-bit case (matching
     candidates 020/025/037's own setup).
   - Built a **baseline** binary (unpatched `src/codecs/bmp/decoder.rs`), then applied
     candidates 008/011's fixed-array patch (`git apply`, byte-identical to candidate 025/037's
     `patch.diff`) and built a **patched** binary.
   - Bit-exact correctness check (`_correctness_check_bmp` over all 67 `.bmp` files under
     `tests/images/bmp/`, baseline vs. patched): **0 differences** (`evidence/hashes_baseline.txt`
     / `evidence/hashes_patched.txt`, `diff` empty).
   - `hyperfine --warmup 3 -m 10`, single-core (`taskset -c 3`) and multi-core (unpinned), 2000
     decodes/invocation, same four files/bit-depths as candidates 020/025/037
     (`Core_8_Bit.bmp`, `Core_4_Bit.bmp`, `pal2_large.bmp`, `Core_1_Bit.bmp`). `uptime` 1.01-1.34
     throughout (well under the 24-core budget). Raw JSON + text in
     `evidence/wc_{single,multi}_*.json` / `evidence/wallclock_{single,multi}.txt`.
   - Project test suite (`cargo test --release`), baseline and patched, both in this same
     session: **551 passed / 1 failed both times**, the same single pre-existing failure every
     prior BMP-decoder candidate in this workspace has recorded (`resizing_with_catmul` in
     `tests/regression.rs`, unrelated to BMP decoding). Full logs:
     `evidence/cargo_test_baseline.log` / `evidence/cargo_test_patched.log`.
3. **Reverted the working tree** (`git checkout -- src/codecs/bmp/decoder.rs Cargo.toml`, removed
   the two temporary example files and the copied `pal2_large.bmp`/reference PNG) before writing
   this folder. `git status` in the main project shows only `___OPTIMIZATIONS___/`/`___OTHER___/`
   untracked, nothing else.

**Multi-core patched wall-clock, this session** (mean of 10 runs each, `hyperfine`):

| Bit depth | File | Patched mean (multi-core) | ns/pixel |
|---|---|---|---|
| 8-bit | `Core_8_Bit.bmp` (435x353, 153,555px) | 312.6 ms / 2000 decodes | 1.0177 |
| 4-bit | `Core_4_Bit.bmp` (435x353, 153,555px) | 327.5 ms / 2000 decodes | 1.0664 |
| 2-bit | `pal2_large.bmp` (700x700, 490,000px) | 853.8 ms / 2000 decodes | 0.8712 |
| 1-bit | `Core_1_Bit.bmp` (435x353, 153,555px) | 254.2 ms / 2000 decodes | 0.8277 |

**Same-session synthetic store-throughput ceiling** (best `min_ns` across the 4 runs, `small` size
regime - 153,560px, matching the 8/4/1-bit files' actual pixel count, same tier candidate 041's
own comparison table used):

| Density | natural ns/px | volatile ns/px |
|---|---|---|
| 1 | 0.5890 | 0.5519 |
| 2 | 0.3679 | 0.3678 |
| 4 | 0.2759 | 0.2299 |
| 8 | 0.0921 | 0.1839 |

**Same-session store-share-of-real-decode-time table** (this candidate's own numbers, both halves
measured in this one session):

| Bit depth | Density | Real decode ns/px (this session) | Pure-store floor ns/px (natural/volatile) | Pure-store share of real time |
|---|---|---|---|---|
| 8-bit | 1 | 1.0177 | 0.589 / 0.552 | 57.9% / 54.2% |
| 4-bit | 2 | 1.0664 | 0.368 / 0.368 | 34.5% / 34.5% |
| 2-bit | 4 | 0.8712 | 0.276 / 0.230 | 31.7% / 26.4% |
| 1-bit | 8 | 0.8277 | 0.092 / 0.184 | 11.1% / 22.2% |

**Candidate 041's original cross-session table, for direct comparison** (real decode ns/px from
candidate 037's separate session; store floor from candidate 041's own session):

| Bit depth | Density | Real decode ns/px (candidate 037) | Pure-store floor ns/px (natural/volatile) | Pure-store share |
|---|---|---|---|---|
| 8-bit | 1 | 1.0357 | 0.593 / 0.554 | 57% / 53% |
| 4-bit | 2 | 1.0844 | 0.370 / 0.370 | 34% / 34% |
| 2-bit | 4 | 0.8698 | 0.278 / 0.230 | 32% / 26% |
| 1-bit | 8 | 0.8337 | 0.092 / 0.184 | 11% / 22% |

## Result

**The same-session store-share percentages match candidate 041's original cross-session numbers
to within 0.2-1.3 percentage points at every bit depth/variant** (8-bit: 57.9/54.2 vs. 57/53;
4-bit: 34.5/34.5 vs. 34/34; 2-bit: 31.7/26.4 vs. 32/26; 1-bit: 11.1/22.2 vs. 11/22). The real
decode's own ns/pixel numbers moved by only 0.6-1.7% between the two sessions (8-bit +1.8%,
4-bit +1.7%, 2-bit -0.2%, 1-bit -0.7% - candidate 037's numbers minus this session's, as a
fraction of this session's number), and the synthetic microbenchmark's numbers moved by less
than 1% at every density/variant (compare this candidate's `small` table above to candidate 041's
own, reproduced verbatim in `evidence/synth_*` and cited above). Removing the cross-session
confound candidate 041 flagged did not change the conclusion: it reproduces the same result to
within normal run-to-run noise, closing that specific open item without altering candidate 041's
finding (pure store cost is a large, bit-depth-dependent share of real 8-bit/4-bit decode time,
a much smaller share of 1-bit's, and the same-session control shows this isn't a session-drift
artifact).

## Does it change behavior/output?

**Measurement-only investigation** - the patch applied during this candidate (candidates
008/011's fixed-array fix, byte-identical to candidate 025/037's `patch.diff`) is not a new
source-level idea; it was re-applied only to build a real decode binary to time in this session,
exactly as candidate 037 did for the same reason. `patch.diff` in this folder is present (per
SKILL.md's "mandatory whenever the fix is a change to the tested project" - real source was
built and measured, so the patch that produced it must be recorded), and is byte-identical to
candidates 025/037's own `patch.diff`.

- **Bit-exact?** Yes. All 67 `.bmp` files under `tests/images/bmp/` (66 pre-existing +
  `pal2_large.bmp`) hash identically between baseline and patched builds this session
  (`evidence/hashes_baseline.txt` / `evidence/hashes_patched.txt`, `diff` empty).
- **Is this a breaking change?** No - same characterization as candidates 008/011/025/037:
  internal-only signature change to four private functions and two local bindings, no public API
  surface touched.
- Project test suite (`cargo test --release`, `pal2_large.bmp` + reference PNG present, matching
  candidate 020/037's setup): **551 passed / 1 failed before (baseline), 551 passed / 1 failed
  after (patched)** - the same single pre-existing `resizing_with_catmul` failure both times, no
  new failures. Full logs: `evidence/cargo_test_baseline.log` / `evidence/cargo_test_patched.log`.
- Working tree fully reverted (`src/codecs/bmp/decoder.rs`, `Cargo.toml`, the two temporary
  example files, and the copied `pal2_large.bmp`/reference PNG) before this folder was written;
  `git status` shows only `___OPTIMIZATIONS___/`/`___OTHER___/` untracked.

## Why the result looks the way it does

Both halves of the comparison (real BMP decode via `hyperfine`, synthetic store-throughput
microbenchmark) are wall-clock timings on the same idle, otherwise-unloaded machine
(`uptime` 0.1-1.3 throughout this whole candidate), so the small residual differences from
candidate 041's cross-session numbers (well under 2%) are consistent with ordinary run-to-run
timing noise rather than any systematic session-to-session machine-state drift - unlike, e.g.,
candidate 030's unexplained load-average spike or candidate 017's noted absolute-number drift
across sessions, which motivated this candidate in the first place. This session's machine state
was evidently close enough to the one candidate 037/041 ran on that a same-session re-measurement
essentially reproduces the earlier cross-session comparison rather than revealing it was
confounded.

## What's not done / open questions

- This closes candidate 041's specific "What's not done" item (cross-session confound in the
  store-share comparison) but does not add anything new toward candidate 041/042's still-open
  question of what the residual (47-75% of real time, unexplained by store+lookup alone) actually
  is - see candidate 042's own "What's not done" for the next concrete step (isolating bit-field
  unpacking/extraction cost).
- Only one additional session/machine-state was checked against candidate 037/041's original one;
  this narrows (but doesn't eliminate) the possibility that both sessions happened to share a
  similar machine state that a third, more different session might not reproduce - the workspace's
  own documented noise episodes (candidate 030) suggest this is possible in principle, just not
  observed here.
- `perf` remains uninstallable in this container (no `perf` binary, `sudo`/`apt` blocked by "no
  new privileges" - confirmed again this session), so this is still a wall-clock-only comparison,
  not a hardware-counter-confirmed one.
