use std::hint::black_box;
use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

macro_rules! check {
    ($name:expr, $actual:expr, $expected:expr) => {
        let actual = $actual;
        let expected = $expected;
        if actual == expected {
            println!("OK   {}: {:?}", $name, actual);
        } else {
            println!("FAIL {}: got {:?}, expected {:?}", $name, actual, expected);
        }
    };
}

fn main() {
    let a = AtomicU8::new(black_box(100));
    a.fetch_max(black_box(200), Ordering::SeqCst);
    check!("AtomicU8::fetch_max(100,200)", a.load(Ordering::SeqCst), 200u8);

    let a = AtomicU8::new(black_box(200));
    a.fetch_min(black_box(100), Ordering::SeqCst);
    check!("AtomicU8::fetch_min(200,100)", a.load(Ordering::SeqCst), 100u8);

    let a = AtomicI8::new(black_box(100));
    a.fetch_min(black_box(-1), Ordering::SeqCst);
    check!("AtomicI8::fetch_min(100,-1)", a.load(Ordering::SeqCst), -1i8);

    let a = AtomicI8::new(black_box(-1));
    a.fetch_max(black_box(100), Ordering::SeqCst);
    check!("AtomicI8::fetch_max(-1,100)", a.load(Ordering::SeqCst), 100i8);

    let a = AtomicU16::new(black_box(100));
    a.fetch_max(black_box(60000), Ordering::SeqCst);
    check!("AtomicU16::fetch_max(100,60000)", a.load(Ordering::SeqCst), 60000u16);

    let a = AtomicI16::new(black_box(100));
    a.fetch_min(black_box(-1), Ordering::SeqCst);
    check!("AtomicI16::fetch_min(100,-1)", a.load(Ordering::SeqCst), -1i16);

    let a = AtomicU32::new(black_box(100));
    a.fetch_max(black_box(u32::MAX), Ordering::SeqCst);
    check!("AtomicU32::fetch_max(100,u32::MAX)", a.load(Ordering::SeqCst), u32::MAX);

    let a = AtomicI32::new(black_box(100));
    a.fetch_min(black_box(i32::MIN), Ordering::SeqCst);
    check!("AtomicI32::fetch_min(100,i32::MIN)", a.load(Ordering::SeqCst), i32::MIN);

    let a = AtomicI32::new(black_box(i32::MIN));
    a.fetch_max(black_box(100), Ordering::SeqCst);
    check!("AtomicI32::fetch_max(i32::MIN,100)", a.load(Ordering::SeqCst), 100i32);

    let a = AtomicU64::new(black_box(100));
    a.fetch_max(black_box(u64::MAX), Ordering::SeqCst);
    check!("AtomicU64::fetch_max(100,u64::MAX)", a.load(Ordering::SeqCst), u64::MAX);

    let a = AtomicI64::new(black_box(100));
    a.fetch_min(black_box(i64::MIN), Ordering::SeqCst);
    check!("AtomicI64::fetch_min(100,i64::MIN)", a.load(Ordering::SeqCst), i64::MIN);
}
