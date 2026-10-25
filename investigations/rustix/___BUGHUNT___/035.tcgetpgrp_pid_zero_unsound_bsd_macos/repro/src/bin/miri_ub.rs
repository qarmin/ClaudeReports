// Supplementary escalation (tool #2): Miri can't perform real syscalls (no `posix_openpt`/
// `tcgetpgrp` support - see output.txt), so this binary isolates just the final step of
// before.rs's real pty-driven repro - constructing `Pid::from_raw_unchecked(0)` - using a literal
// `0` in place of the pty-obtained value (before.rs/output.txt already establishes, via a real
// kernel syscall, that the real macOS/BSD-unguarded code path receives exactly this value from a
// genuine `tcgetpgrp(2)` call). This isolates and confirms, independent of any platform, that
// constructing a `Pid` from that value is real, Miri-detectable Undefined Behavior - not merely a
// documented-but-maybe-harmless contract violation.
use rustix::process::Pid;

fn main() {
    let pid: i32 = 0;
    let p = unsafe { Pid::from_raw_unchecked(pid) };
    println!("constructed (should be unreachable under Miri): {:?}", p);
}
