# Canary cases - proof the detector actually detects something

Every program here is a KNOWN, already-confirmed-reproducing miscompilation (sourced from
`../../_reference/known_miscompile_issues.md`), planted here deliberately so `check_canaries.sh`
can prove `diff_runner.py`'s detection logic genuinely fires on a real divergence - not just that
it's been running clean against thousands of generator-produced programs because nothing is wrong
with them. Run this any time it's unclear whether "0 divergences across N campaign rounds" means
"the harness works and found nothing new" (expected - real rustc miscompilations are rare) versus
"the harness is silently broken" (which `check_canaries.sh` would catch immediately).

**Unlike every other corpus in this tool, a clean run here is a FAILURE.** `check_canaries.sh`
exits non-zero if any canary does NOT produce a divergence - that means the detector regressed.

## Cases

| File | Issue | Known-correct vs known-wrong | Verified on this host |
|---|---|---|---|
| `case_163311.rs` | [#163311](https://github.com/rust-lang/rust/issues/163311) | `-C opt-level=0/1` prints `2` (correct); `-C opt-level=2/3` prints `3` (wrong) | Yes, 2026-10-07: reproduces identically on both `rustc +nightly` (1.100.0-nightly) and the system's `stable` (1.99.0) - this bug isn't nightly-exclusive |

## Usage

```bash
./check_canaries.sh                      # against system default `rustc` (stable)
./check_canaries.sh --rustc "rustc +nightly"
```

Exits 0 only if EVERY canary produced a DIVERGENCE (detector confirmed working); exits 1 and
prints which canary silently passed (no divergence detected) otherwise - that is the signal to
stop and debug `diff_runner.py`'s comparison logic before trusting a clean campaign round again.

## Adding a new canary

Pick another `Still reproduces? = Yes` row from `../../_reference/known_miscompile_issues.md`,
save its exact MCVE as `caseNNNNN.rs` here (issue number in the filename), and add a row to the
table above noting the known-correct vs known-wrong command/output pair so a future session can
re-verify it by hand if `check_canaries.sh` ever reports it stopped diverging (which would mean
the bug was fixed upstream, not that the detector broke - re-confirm against a fresh nightly
before assuming the latter).
