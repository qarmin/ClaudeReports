use std::sync::atomic::{AtomicI8, AtomicU8, AtomicIsize, AtomicUsize, Ordering};

fn main() {
    let mut failures = 0;

    // Store loaded value directly (no cast) into a new place, then use it in arithmetic.
    let a = AtomicI8::new(-100);
    let loaded = a.load(Ordering::Relaxed); // stored as local i8
    let b = loaded / 3; // signed division
    let expected = (-100i8) / 3;
    if b != expected {
        println!("FAIL signed div: {} != {}", b, expected);
        failures += 1;
    } else {
        println!("OK signed div: {}", b);
    }

    let au = AtomicU8::new(250);
    let loadedu = au.load(Ordering::Relaxed);
    let c = loadedu / 3;
    let expectedu = 250u8 / 3;
    if c != expectedu {
        println!("FAIL unsigned div: {} != {}", c, expectedu);
        failures += 1;
    } else {
        println!("OK unsigned div: {}", c);
    }

    // isize / usize atomics
    let ai = AtomicIsize::new(-12345);
    let li = ai.load(Ordering::Relaxed);
    if li >= 0 {
        println!("FAIL isize compare: {} >= 0", li);
        failures += 1;
    } else {
        println!("OK isize compare: {} < 0", li);
    }
    let wi = li as i64;
    if wi != -12345i64 {
        println!("FAIL isize widen: {} != -12345", wi);
        failures += 1;
    } else {
        println!("OK isize widen: {}", wi);
    }

    let au2 = AtomicUsize::new(usize::MAX);
    let lu = au2.load(Ordering::Relaxed);
    let wu = lu as u128;
    if wu != usize::MAX as u128 {
        println!("FAIL usize widen: {} != {}", wu, usize::MAX);
        failures += 1;
    } else {
        println!("OK usize widen: {}", wu);
    }

    // Direct comparison between two loaded atomics of same signedness
    let x = AtomicU8::new(200);
    let y = AtomicU8::new(100);
    let lx = x.load(Ordering::Relaxed);
    let ly = y.load(Ordering::Relaxed);
    if lx > ly {
        println!("OK atomic-atomic unsigned compare: {} > {}", lx, ly);
    } else {
        println!("FAIL atomic-atomic unsigned compare: {} not > {}", lx, ly);
        failures += 1;
    }

    println!("TOTAL_FAILURES={}", failures);
}
