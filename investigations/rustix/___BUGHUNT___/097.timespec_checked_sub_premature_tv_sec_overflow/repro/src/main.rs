// Standalone reproduction for rustix's `Timespec::checked_sub` (and its
// mirror-image sibling `checked_add`) falsely returning `None` ("overflow")
// for an input whose true difference, once the nanosecond borrow is
// accounted for, actually fits in `i64` just fine.
//
// Uses rustix's real, public `rustix::event::Timespec` type and its public
// `checked_sub` method directly - no internal/private API, no hand-faked
// second participant (this is a pure, single-participant arithmetic
// function operating only on caller-supplied data).
//
// Exit code convention: 0 = did not reproduce (bug absent, i.e. patched),
// non-zero = reproduced (bug present).

use rustix::event::Timespec;

struct Case {
    name: &'static str,
    a: Timespec,
    b: Timespec,
    // Expected correct result, computed independently via i128 arithmetic.
    expected: Option<Timespec>,
}

fn main() {
    let cases = [
        Case {
            name: "minimal: 0 - (i64::MIN, 1ns)",
            a: Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            b: Timespec {
                tv_sec: i64::MIN,
                tv_nsec: 1,
            },
            // 0 - i64::MIN = i64::MAX + 1, then borrow 1s for the nsec
            // underflow brings it back down to exactly i64::MAX - valid.
            expected: Some(Timespec {
                tv_sec: i64::MAX,
                tv_nsec: 999_999_999,
            }),
        },
        Case {
            name: "second input, same bug class (fuzz-found seed, minimized by hand)",
            a: Timespec {
                tv_sec: 10,
                tv_nsec: 100,
            },
            b: Timespec {
                tv_sec: -9223372036854775798,
                tv_nsec: 200,
            },
            expected: Some(Timespec {
                tv_sec: i64::MAX,
                tv_nsec: 999_999_900,
            }),
        },
        // Sanity check: a *genuine* overflow (no borrow can rescue it) must
        // still correctly return `None` - this is already covered by the
        // function's own doc-test, repeated here so this repro alone proves
        // the fix doesn't just turn the check off.
        Case {
            name: "sanity: genuine overflow must remain None",
            a: Timespec {
                tv_sec: i64::MIN,
                tv_nsec: 0,
            },
            b: Timespec {
                tv_sec: 0,
                tv_nsec: 1,
            },
            expected: None,
        },
    ];

    let mut reproduced = false;
    for case in &cases {
        let got = case.a.checked_sub(case.b);
        let ok = got == case.expected;
        println!(
            "[{}] {:?}.checked_sub({:?}) = {:?} (expected {:?}) -> {}",
            case.name,
            case.a,
            case.b,
            got,
            case.expected,
            if ok { "OK" } else { "MISMATCH (bug reproduced)" }
        );
        if !ok {
            reproduced = true;
        }
    }

    if reproduced {
        eprintln!(
            "\nREPRODUCED - checked_sub returned a value that doesn't match the \
             independently-computed correct result for at least one case above."
        );
        std::process::exit(1);
    } else {
        println!("\ndid not reproduce - all cases matched the independently-computed result");
    }
}
