// Standalone reproduction for candidate 101: `impl TryFrom<Timespec> for Duration` casts
// `tv_nsec` straight into `u32` with no range validation, even though the function is `TryFrom`
// and documents (on the `Timespec` struct itself) that `tv_nsec` "must be less than
// 1_000_000_000". A negative or overlong `tv_nsec` is silently misinterpreted into a wrong
// `Duration` instead of being rejected.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present -
// an out-of-range `tv_nsec` was silently accepted and produced a wrong `Duration`).

use rustix::event::Timespec;
use std::time::Duration;

struct Case {
    ts: Timespec,
    // What a correct implementation should do: None means "should be rejected as invalid",
    // Some(d) means "should succeed with this exact Duration".
    expected: Option<Duration>,
}

fn main() {
    let cases = [
        // tv_nsec == -1: documented invariant (0 <= tv_nsec < 1_000_000_000) is violated by a
        // negative value. A correct implementation rejects this instead of reinterpreting the
        // bit pattern of -1i64 as the u32 0xFFFFFFFF (4_294_967_295).
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: -1 },
            expected: None,
        },
        // tv_nsec == -1_000_000_000: a whole negated second, still invalid per the same
        // invariant.
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: -1_000_000_000 },
            expected: None,
        },
        // tv_nsec == 1_000_000_000: exactly one nanosecond over the documented upper bound.
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: 1_000_000_000 },
            expected: None,
        },
        // tv_nsec almost double the valid range.
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: 1_999_999_999 },
            expected: None,
        },
        // i64::MIN: an extreme out-of-range value a corrupted/non-kernel-sourced Timespec could
        // carry.
        Case {
            ts: Timespec { tv_sec: 0, tv_nsec: i64::MIN },
            expected: None,
        },
        // Sanity check: in-range values must keep converting correctly (the fix must not reject
        // valid input).
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: 0 },
            expected: Some(Duration::new(5, 0)),
        },
        Case {
            ts: Timespec { tv_sec: 5, tv_nsec: 999_999_999 },
            expected: Some(Duration::new(5, 999_999_999)),
        },
    ];

    let mut reproduced = false;

    for case in cases {
        let result: Result<Duration, _> = Duration::try_from(case.ts);
        match (&case.expected, &result) {
            (None, Ok(wrong)) => {
                reproduced = true;
                eprintln!(
                    "REPRODUCED: Timespec {{ tv_sec: {}, tv_nsec: {} }} (invalid tv_nsec) was \
                     silently accepted as {:?} instead of being rejected",
                    case.ts.tv_sec, case.ts.tv_nsec, wrong
                );
            }
            (None, Err(_)) => {
                println!(
                    "ok: Timespec {{ tv_sec: {}, tv_nsec: {} }} correctly rejected",
                    case.ts.tv_sec, case.ts.tv_nsec
                );
            }
            (Some(expected), Ok(got)) if got == expected => {
                println!(
                    "ok: Timespec {{ tv_sec: {}, tv_nsec: {} }} -> {:?} (correct)",
                    case.ts.tv_sec, case.ts.tv_nsec, got
                );
            }
            (Some(expected), got) => {
                reproduced = true;
                eprintln!(
                    "REPRODUCED (different bug): Timespec {{ tv_sec: {}, tv_nsec: {} }} -> {:?}, \
                     expected Ok({:?})",
                    case.ts.tv_sec, case.ts.tv_nsec, got, expected
                );
            }
        }
    }

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - all cases handled correctly");
    }
}
