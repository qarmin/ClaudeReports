#![no_main]

use libfuzzer_sys::fuzz_target;
use rustix::event::Timespec;

// Second one-off harness target: differential-tests `Timespec::checked_add`/
// `checked_sub` (public, pure, no syscall) against an independent i128
// total-nanoseconds reference computation, restricted to the documented
// precondition (`0 <= tv_nsec < 1_000_000_000`) so a precondition violation
// itself is never reported as a false "bug".
fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    let a_sec = i64::from_le_bytes(data[0..8].try_into().unwrap());
    let b_sec = i64::from_le_bytes(data[8..16].try_into().unwrap());
    let a_nsec = (u32::from_le_bytes(data[16..20].try_into().unwrap()) % 1_000_000_000) as i64;
    let b_nsec = (u32::from_le_bytes(data[20..24].try_into().unwrap()) % 1_000_000_000) as i64;

    let a = Timespec {
        tv_sec: a_sec,
        tv_nsec: a_nsec,
    };
    let b = Timespec {
        tv_sec: b_sec,
        tv_nsec: b_nsec,
    };

    let reference_add = (a_sec as i128) * 1_000_000_000 + a_nsec as i128
        + (b_sec as i128) * 1_000_000_000
        + b_nsec as i128;
    match a.checked_add(b) {
        Some(sum) => {
            assert!(
                (0..1_000_000_000).contains(&sum.tv_nsec),
                "checked_add produced out-of-range tv_nsec: {:?} + {:?} = {:?}",
                a,
                b,
                sum
            );
            let got = (sum.tv_sec as i128) * 1_000_000_000 + sum.tv_nsec as i128;
            assert_eq!(
                got, reference_add,
                "checked_add mismatch: {:?} + {:?} = {:?} (expected total ns {})",
                a, b, sum, reference_add
            );
        }
        None => {
            // Only acceptable if the reference computation is itself out of
            // `i64` seconds range (allowing for a +/-1s carry/borrow).
            let ref_sec = reference_add.div_euclid(1_000_000_000);
            assert!(
                ref_sec > i64::MAX as i128 || ref_sec < i64::MIN as i128,
                "checked_add returned None but reference fits: {:?} + {:?}, ref_sec={}",
                a,
                b,
                ref_sec
            );
        }
    }

    let reference_sub = (a_sec as i128) * 1_000_000_000 + a_nsec as i128
        - ((b_sec as i128) * 1_000_000_000 + b_nsec as i128);
    match a.checked_sub(b) {
        Some(diff) => {
            assert!(
                (0..1_000_000_000).contains(&diff.tv_nsec),
                "checked_sub produced out-of-range tv_nsec: {:?} - {:?} = {:?}",
                a,
                b,
                diff
            );
            let got = (diff.tv_sec as i128) * 1_000_000_000 + diff.tv_nsec as i128;
            assert_eq!(
                got, reference_sub,
                "checked_sub mismatch: {:?} - {:?} = {:?} (expected total ns {})",
                a, b, diff, reference_sub
            );
        }
        None => {
            let ref_sec = reference_sub.div_euclid(1_000_000_000);
            assert!(
                ref_sec > i64::MAX as i128 || ref_sec < i64::MIN as i128,
                "checked_sub returned None but reference fits: {:?} - {:?}, ref_sec={}",
                a,
                b,
                ref_sec
            );
        }
    }
});
