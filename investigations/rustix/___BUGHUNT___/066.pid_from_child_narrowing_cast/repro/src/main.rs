// Real end-to-end attempt to trigger candidate 066: Pid::from_child(src/pid.rs:68-73) casts
// std::process::Child::id(): u32 to i32 with no range check before calling
// Pid::from_raw_unchecked, whose own debug_assert!(raw > 0) is stricter than the documented
// Safety contract ("raw is non-zero"). The only way this cast can actually misbehave is if a
// real OS-assigned child PID is ever >= 2^31 (so the cast produces zero or a negative value).
//
// This spawns a large number of REAL child processes via std::process::Command (the real,
// unfaked "second participant": the OS kernel's own PID allocator), and calls rustix's real
// public Pid::from_child on each one, exactly as a genuine caller would.
//
// Exit code convention: 0 = did not reproduce, non-zero/panic = reproduced.

use rustix::process::Pid;
use std::process::Command;

fn main() {
    let n: u32 = std::env::var("REPRO_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);

    let mut max_id: u32 = 0;
    let mut min_id: u32 = u32::MAX;

    for i in 0..n {
        let mut child = Command::new("true")
            .spawn()
            .or_else(|_| Command::new("/bin/true").spawn())
            .expect("failed to spawn a real child process");
        let raw_id = child.id();
        max_id = max_id.max(raw_id);
        min_id = min_id.min(raw_id);

        // The real suspect call: this is the exact code path candidate 066 is about.
        let pid = Pid::from_child(&child);

        assert_eq!(
            pid.as_raw_pid() as u32,
            raw_id,
            "iteration {i}: Pid::from_child round-trip mismatch"
        );
        assert!(
            pid.as_raw_pid() > 0,
            "iteration {i}: REPRODUCED - Pid::from_child produced a non-positive pid {} from a real child.id() of {}",
            pid.as_raw_pid(),
            raw_id
        );

        let _ = child.wait();
    }

    println!("spawned {n} real child processes via std::process::Command");
    println!("observed real OS-assigned child.id() range: min={min_id} max={max_id}");
    println!(
        "distance from the highest observed real id to the i32 overflow boundary (i32::MAX): {}",
        (i32::MAX as i64) - (max_id as i64)
    );
    if let Ok(pid_max) = std::fs::read_to_string("/proc/sys/kernel/pid_max") {
        println!("/proc/sys/kernel/pid_max on this machine = {}", pid_max.trim());
    }

    println!("did not reproduce - every real OS-assigned pid stayed far below the i32 overflow boundary");
}
