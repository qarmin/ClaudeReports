// Standalone bench for i-slint-core's `translations::translate()` gettext path
// (candidate 084.TR_GETTEXT_CACHE). Not part of the perf_test UI harness: this exercises a
// plain Rust function directly, no Slint component/window/scene needed.
//
// Simulates the scenario the candidate describes: a `tr!()` binding re-evaluated because an
// *unrelated* interpolation argument changed, while the locale itself stays fixed. Each
// iteration passes a different `arg` (like a changing property) but the locale is set once at
// startup and never touched again.

use i_slint_core::translations::{gettext_bindtextdomain, translate};

struct OneArg(i64);
impl i_slint_core::translations::FormatArgs for OneArg {
    type Output<'a> = i64;
    fn from_index(&self, index: usize) -> Option<i64> {
        (index == 0).then_some(self.0)
    }
}

fn black_box<T>(x: T) -> T {
    std::hint::black_box(x)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iters: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(200_000);
    let mode = args.get(2).map(|s| s.as_str()).unwrap_or("changing-args");
    // Non-empty context forces `translate_gettext`'s `mangle_context`/`demangle_context` path
    // (extra String allocations on every call, on the Rust side, independent of whether the
    // underlying C locale actually has a catalog to search) - see README/RESULT.md for why the
    // real dgettext "catalog found" path itself couldn't be exercised in this environment.
    let context = if mode == "with-context" { "MyContext" } else { "" };

    // Point gettext at our hand-built test catalog: testdata/fr/LC_MESSAGES/bench.mo,
    // containing one entry "Hello, {}!" -> "Bonjour, {}!".
    let testdata_dir = std::env::var("TR_BENCH_TESTDATA")
        .expect("set TR_BENCH_TESTDATA to the folder containing fr/LC_MESSAGES/bench.mo");
    // SAFETY: single-threaded bench, set once before any gettext call.
    unsafe {
        std::env::set_var("LANGUAGE", "fr");
        std::env::set_var("LC_ALL", "C.utf8");
    }
    gettext_bindtextdomain("bench", std::path::PathBuf::from(&testdata_dir)).unwrap();

    // Confirm the catalog is actually being found before timing anything - a silent fallback
    // to "untranslated" would still run the FFI path, but would not be the realistic case.
    let probe = translate("Hello, {}!", context, "bench", &OneArg(0), 1, "");
    let translation_found = probe.as_str() == "Bonjour, 0!";
    eprintln!(
        "translation_found={translation_found} (probe={:?}, expected \"Bonjour, 0!\")",
        probe.as_str()
    );

    let warmup = iters / 10;
    let mut durations = Vec::with_capacity(iters as usize);
    let mut hash: u64 = 0;

    for i in 0..iters {
        let arg = OneArg(i as i64);
        let start = std::time::Instant::now();
        let out = if mode == "same-args" {
            black_box(translate("Hello, {}!", context, "bench", &OneArg(0), 1, ""))
        } else {
            black_box(translate("Hello, {}!", context, "bench", &arg, 1, ""))
        };
        let elapsed = start.elapsed();
        // Cheap running hash so the compiler can't eliminate the call, and so we can verify
        // correctness (bit-exact output sequence) across baseline/patched builds.
        for b in out.as_bytes() {
            hash = hash.wrapping_mul(1099511628211).wrapping_add(*b as u64);
        }
        if i >= warmup {
            durations.push(elapsed);
        }
    }

    durations.sort();
    let n = durations.len();
    let p = |q: f64| durations[((n - 1) as f64 * q) as usize].as_nanos();
    println!("mode={mode} iters={iters} translation_found={translation_found}");
    println!(
        "min={} p50={} mean={} p95={} max={}",
        durations[0].as_nanos(),
        p(0.50),
        durations.iter().map(|d| d.as_nanos()).sum::<u128>() / n as u128,
        p(0.95),
        durations[n - 1].as_nanos()
    );
    println!("hash={hash:016x}");
}
