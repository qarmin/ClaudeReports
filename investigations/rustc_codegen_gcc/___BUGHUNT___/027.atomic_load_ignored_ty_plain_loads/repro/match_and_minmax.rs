use std::sync::atomic::{AtomicI8, AtomicU8, Ordering};

fn main() {
    let mut failures = 0;

    let a = AtomicI8::new(-5);
    let v = a.load(Ordering::Relaxed);
    let r = match v {
        i8::MIN..=-1 => "negative",
        0 => "zero",
        1..=i8::MAX => "positive",
    };
    if r != "negative" {
        println!("FAIL match: got {}", r);
        failures += 1;
    } else {
        println!("OK match: {}", r);
    }

    let au = AtomicU8::new(200);
    let vu = au.load(Ordering::Relaxed);
    let ru = match vu {
        0..=127 => "low",
        128..=255 => "high",
    };
    if ru != "high" {
        println!("FAIL match unsigned: got {}", ru);
        failures += 1;
    } else {
        println!("OK match unsigned: {}", ru);
    }

    // min/max builtins on loaded value
    let m = v.min(-10);
    if m != -10 {
        println!("FAIL min: {}", m);
        failures += 1;
    } else {
        println!("OK min: {}", m);
    }
    let mu = vu.max(50);
    if mu != 200 {
        println!("FAIL max unsigned: {}", mu);
        failures += 1;
    } else {
        println!("OK max unsigned: {}", mu);
    }

    println!("TOTAL_FAILURES={}", failures);
}
