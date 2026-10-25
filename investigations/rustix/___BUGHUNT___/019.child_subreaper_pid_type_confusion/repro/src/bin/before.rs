// Demonstrates that `child_subreaper()`'s `Option<Pid>` return type is bogus: the kernel's
// PR_GET_CHILD_SUBREAPER only ever reports 0/1 (a boolean), so the "pid" rustix hands back after
// a `set_child_subreaper(Some(pid))` round-trip is never the pid that was set - it is always
// `Pid::from_raw(1)` (PID 1 / init), a real but totally unrelated process.
use rustix::process::{child_subreaper, set_child_subreaper, Pid};

fn main() {
    let before = child_subreaper().unwrap();
    assert_eq!(before, None, "expected no subreaper set initially, got {:?}", before);

    // Arbitrary distinctive "pid" chosen specifically because it is NOT the real pid of any
    // process that could plausibly exist (and is also not 0/1), to make the mismatch obvious.
    let set_pid = Pid::from_raw(424242).expect("valid nonzero pid value");
    set_child_subreaper(Some(set_pid)).unwrap();

    let after = child_subreaper().unwrap();
    println!("set_child_subreaper(Some({:?})) -> child_subreaper() returned {:?}", set_pid, after);

    // Restore the original process state regardless of outcome.
    set_child_subreaper(None).unwrap();

    if after == Some(set_pid) {
        println!("did not reproduce: returned pid matched what was set");
        std::process::exit(0);
    } else {
        println!(
            "REPRODUCED: child_subreaper() claims the subreaper is {:?}, a fabricated pid from a \
             boolean kernel flag, not the pid {:?} that was actually set",
            after, set_pid
        );
        std::process::exit(1);
    }
}
