use std::hint::black_box;
use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

fn main() {
    // AtomicU8::fetch_max with a value >= 128 (top bit set): unsigned semantics say
    // 200 > 100, but if the CAS-loop comparison is signed, 200 (as i8 = -56) < 100.
    let a = AtomicU8::new(black_box(100));
    let prev = a.fetch_max(black_box(200), Ordering::SeqCst);
    assert_eq!(prev, 100, "AtomicU8::fetch_max prev value wrong");
    assert_eq!(a.load(Ordering::SeqCst), 200, "AtomicU8::fetch_max(100,200) should be 200 (unsigned)");

    let a = AtomicU8::new(black_box(200));
    let prev = a.fetch_min(black_box(100), Ordering::SeqCst);
    assert_eq!(prev, 200, "AtomicU8::fetch_min prev value wrong");
    assert_eq!(a.load(Ordering::SeqCst), 100, "AtomicU8::fetch_min(200,100) should be 100 (unsigned)");

    // AtomicI8::fetch_min with negative values: signed semantics say -1 < 100,
    // but if the CAS-loop comparison is unsigned, -1 (as u8 = 255) > 100.
    let a = AtomicI8::new(black_box(100));
    let prev = a.fetch_min(black_box(-1), Ordering::SeqCst);
    assert_eq!(prev, 100, "AtomicI8::fetch_min prev value wrong");
    assert_eq!(a.load(Ordering::SeqCst), -1, "AtomicI8::fetch_min(100,-1) should be -1 (signed)");

    let a = AtomicI8::new(black_box(-1));
    let prev = a.fetch_max(black_box(100), Ordering::SeqCst);
    assert_eq!(prev, -1, "AtomicI8::fetch_max prev value wrong");
    assert_eq!(a.load(Ordering::SeqCst), 100, "AtomicI8::fetch_max(-1,100) should be 100 (signed)");

    // 16-bit
    let a = AtomicU16::new(black_box(100));
    a.fetch_max(black_box(60000), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), 60000, "AtomicU16::fetch_max(100,60000) should be 60000 (unsigned)");

    let a = AtomicI16::new(black_box(100));
    a.fetch_min(black_box(-1), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), -1, "AtomicI16::fetch_min(100,-1) should be -1 (signed)");

    // 32-bit boundary: i32::MIN, u32::MAX
    let a = AtomicU32::new(black_box(100));
    a.fetch_max(black_box(u32::MAX), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), u32::MAX, "AtomicU32::fetch_max(100,u32::MAX) should be u32::MAX (unsigned)");

    let a = AtomicI32::new(black_box(100));
    a.fetch_min(black_box(i32::MIN), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), i32::MIN, "AtomicI32::fetch_min(100,i32::MIN) should be i32::MIN (signed)");

    let a = AtomicI32::new(black_box(i32::MIN));
    a.fetch_max(black_box(100), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), 100, "AtomicI32::fetch_max(i32::MIN,100) should be 100 (signed)");

    // 64-bit boundary
    let a = AtomicU64::new(black_box(100));
    a.fetch_max(black_box(u64::MAX), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), u64::MAX, "AtomicU64::fetch_max(100,u64::MAX) should be u64::MAX (unsigned)");

    let a = AtomicI64::new(black_box(100));
    a.fetch_min(black_box(i64::MIN), Ordering::SeqCst);
    assert_eq!(a.load(Ordering::SeqCst), i64::MIN, "AtomicI64::fetch_min(100,i64::MIN) should be i64::MIN (signed)");

    println!("OK");
}
