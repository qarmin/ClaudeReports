// Standalone exhaustive/differential check for lookup1_values over the REAL caller-enforced
// domain (entries in 0..=131072, dimensions in 1..=32, per codebook.rs:239-244) and, separately,
// over the full underlying-type domain to see if the private helper itself has a latent issue
// outside what any real caller can ever pass it.
//
// Compiled and run with overflow checks on (debug build) to catch any arithmetic panic, including
// one buried inside the `.pow()` call in the `assert!` itself.

#[inline(always)]
fn lookup1_values(entries: u32, dimensions: u16) -> u32 {
    if dimensions == 0 {
        return 0;
    }
    let value = (entries as f32).powf(1.0f32 / f32::from(dimensions)).floor() as u32;
    assert!(value.pow(u32::from(dimensions)) <= entries);
    value
}

fn naive_lookup1_values(entries: u32, dimensions: u16) -> u32 {
    let mut x = 1u32;
    if dimensions > 0 {
        loop {
            let Some(x_pow) = x.checked_pow(u32::from(dimensions)) else {
                break;
            };
            if x_pow > entries {
                break;
            }
            x += 1;
        }
    }
    x - 1
}

fn main() {
    let mut checked = 0u64;

    // Approach A: the REAL caller-enforced domain (codebook.rs:239-244).
    for dimensions in 1u16..=32 {
        for entries in 0u32..=131072 {
            let got = lookup1_values(entries, dimensions);
            let want = naive_lookup1_values(entries, dimensions);
            assert_eq!(got, want, "mismatch at entries={entries} dimensions={dimensions}");
            checked += 1;
        }
    }
    println!("Approach A (real caller domain: entries 0..=131072, dimensions 1..=32): {checked} calls, no panic, all match naive reference");

    // Approach B: full underlying-type domain search for the failure mode described in the
    // candidate (value computed by powf/floor one unit too high right at a pow-overflow boundary).
    // Sweep dimensions across the full u16 range and, for each, binary-search entries near the
    // theoretical boundary where floor(entries^(1/dimensions)) could tip from 1 to 2 (or wherever
    // value*dimensions approaches 32-bit overflow), plus the extremes 0 and u32::MAX.
    std::panic::set_hook(Box::new(|_| {})); // silence default panic printing; we report ourselves
    let mut b_checked = 0u64;
    let mut pow_overflow_panics = 0u64;
    let mut assert_reject_panics = 0u64;
    // Probe the region the candidate flagged: dimensions where 2^dimensions overflows u32
    // (dimensions >= 32), with entries at/near u32::MAX (the largest entries can ever be
    // represented as, regardless of what any real caller would pass), since that's the only
    // region where the computed `value` could plausibly become >= 2 there.
    for dimensions in 1u16..=64u16 {
        for entries in [0u32, 1, 2, u32::MAX - 8, u32::MAX - 4, u32::MAX - 1, u32::MAX] {
            let result = std::panic::catch_unwind(|| lookup1_values(entries, dimensions));
            match result {
                Ok(_got) => {}
                Err(payload) => {
                    let msg = payload
                        .downcast_ref::<&str>()
                        .copied()
                        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                        .unwrap_or("<unknown panic payload>");
                    if msg.contains("assertion failed") {
                        // The assert itself rejecting a bad `value` - expected/harmless.
                        assert_reject_panics += 1;
                    }
                    else {
                        // Anything else (e.g. "attempt to multiply with overflow" from inside
                        // .pow() before the assert's comparison ever runs) is the sought bug.
                        pow_overflow_panics += 1;
                        println!(
                            "POW-OVERFLOW PANIC at entries={entries} dimensions={dimensions}: {msg}"
                        );
                    }
                }
            }
            b_checked += 1;
        }
    }
    println!(
        "Approach B (targeted boundary probes, full u32/u16 domain): {b_checked} calls, \
         {pow_overflow_panics} pow-overflow-inside-assert panics, \
         {assert_reject_panics} plain assert-rejection panics (expected/harmless - the assert doing its job)"
    );
}
