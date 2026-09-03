use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

fn main() {
    let mut failures = 0;

    macro_rules! check_signed {
        ($atom:ty, $val:expr, $wide:ty) => {{
            let a = <$atom>::new($val);
            let loaded = a.load(Ordering::Relaxed);
            let widened = loaded as $wide;
            let expected = ($val as $wide);
            if widened != expected {
                println!("FAIL {} widen: loaded={} widened={} expected={}", stringify!($atom), loaded, widened, expected);
                failures += 1;
            } else {
                println!("OK {} widen: {} -> {}", stringify!($atom), loaded, widened);
            }
            if loaded < 0 as _ {
                println!("OK {} negative-compare: {} < 0", stringify!($atom), loaded);
            } else {
                println!("FAIL {} negative-compare: {} not < 0 (expected negative)", stringify!($atom), loaded);
                failures += 1;
            }
        }};
    }

    macro_rules! check_unsigned {
        ($atom:ty, $val:expr, $wide:ty) => {{
            let a = <$atom>::new($val);
            let loaded = a.load(Ordering::Relaxed);
            let widened = loaded as $wide;
            let expected = ($val as $wide);
            if widened != expected {
                println!("FAIL {} widen: loaded={} widened={} expected={}", stringify!($atom), loaded, widened, expected);
                failures += 1;
            } else {
                println!("OK {} widen: {} -> {}", stringify!($atom), loaded, widened);
            }
        }};
    }

    check_signed!(AtomicI8, -1i8, i32);
    check_signed!(AtomicI8, i8::MIN, i64);
    check_signed!(AtomicI16, -1i16, i32);
    check_signed!(AtomicI16, i16::MIN, i64);
    check_signed!(AtomicI32, -1i32, i64);
    check_signed!(AtomicI32, i32::MIN, i64);
    check_signed!(AtomicI64, -1i64, i64);

    check_unsigned!(AtomicU8, 200u8, i32);
    check_unsigned!(AtomicU8, 255u8, u32);
    check_unsigned!(AtomicU16, 60000u16, u32);
    check_unsigned!(AtomicU16, 60000u16, i32);
    check_unsigned!(AtomicU32, u32::MAX, u64);
    check_unsigned!(AtomicU32, u32::MAX, i64);
    check_unsigned!(AtomicU64, u64::MAX, u64);

    println!("TOTAL_FAILURES={}", failures);
}
