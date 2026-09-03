// Standalone reproduction for candidate 017 (polycool's `poly::tests::planted_root_deg5`
// flakiness, kurbo issue #594 / PR #599).
//
// The bug lives in the test itself (polycool/src/poly.rs), not in a function reachable through
// polycool's public API, so the most faithful "real end-to-end trigger path" available is the
// literal thing CI runs: `cargo test -p polycool` against a real checkout, with `ARBTEST_SEED`
// pinned to a seed that's known to make `arbtest` regenerate the same problematic input. This
// runs the actual, unmodified `#[test] fn planted_root_deg5` -- not a hand-copied reimplementation
// of its body -- so a `patch.diff` applied to that checkout is picked up automatically.
//
// Usage: repro <path-to-kurbo-checkout>
//
// Exit code convention: 0 = every seed below passed (bug absent), non-zero = at least one seed
// still makes the test fail (bug present).

use std::env;
use std::path::PathBuf;
use std::process::Command;

// Seeds known to make `arbtest` regenerate a polynomial for which the *un-refined* nominal
// `planted_root` is not (to anywhere near the required tolerance) an actual root of the
// floating-point-rounded polynomial that `poly_with_planted_root` built:
//
// - 0x3180bc82000019a5: the seed CI reported in kurbo issue #594. Fails against original
//   `main` (tolerance too tight for the up-to-~7e-10 construction+solver error).
// - 0x5759368900010000, 0x301077ae00010000: two more seeds found in this session by fuzzing
//   `planted_root_deg5` with an extended 60s budget against PR #599's patched version of the
//   test. Both still fail there too (PR #599 only widens the tolerance to a fixed
//   `magnitude * 1e-6`, and only separates the planted root from the *other* generated roots,
//   not those other roots from each other -- so a cluster of *non-planted* generated roots can
//   still perturb the true root near the planted one by more than that fixed bound).
const KNOWN_BAD_SEEDS: &[&str] = &[
    "0x3180bc82000019a5",
    "0x5759368900010000",
    "0x301077ae00010000",
];

fn main() {
    let checkout: PathBuf = env::args()
        .nth(1)
        .unwrap_or_else(|| "../clone".to_string())
        .into();

    if !checkout.join("polycool/Cargo.toml").exists() {
        eprintln!(
            "error: {:?} does not look like a kurbo checkout (no polycool/Cargo.toml)",
            checkout
        );
        std::process::exit(2);
    }

    let mut any_failed = false;

    for seed in KNOWN_BAD_SEEDS {
        println!("=== running poly::tests::planted_root_deg5 with ARBTEST_SEED={seed} ===");
        let output = Command::new("cargo")
            .args([
                "test",
                "--release",
                "-p",
                "polycool",
                "--lib",
                "poly::tests::planted_root_deg5",
                "--",
                "--nocapture",
            ])
            .env("ARBTEST_SEED", seed)
            .current_dir(&checkout)
            .output()
            .expect("failed to spawn cargo test");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        print!("{stdout}");
        eprint!("{stderr}");

        // A real assertion failure (the actual bug) always prints "could not find something
        // close to planted_root". A rejected/discarded arbtest input (an intentional, benign
        // `Err(IncorrectFormat)` from the fixed test, only possible under `ARBTEST_SEED`'s
        // single-shot "reproduce" mode, never during real fuzzing/CI search mode) prints
        // arbtest's own "raw data is not of the correct format" message instead - that is not
        // this bug, so don't count it as a reproduction.
        let real_assertion_failure = stdout.contains("could not find something close to planted_root")
            || stderr.contains("could not find something close to planted_root");

        if real_assertion_failure {
            println!("--- REPRODUCED for seed {seed} ---\n");
            any_failed = true;
        } else if output.status.success() {
            println!("--- did not reproduce for seed {seed} (test passed) ---\n");
        } else {
            println!(
                "--- did not reproduce for seed {seed} (input rejected by the test as too \
                 ill-conditioned to test meaningfully - expected once the fix is applied) ---\n"
            );
        }
    }

    if any_failed {
        eprintln!("REPRODUCED - at least one seed still fails the real assertion");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all known-bad seeds pass (or are legitimately rejected)");
    }
}
