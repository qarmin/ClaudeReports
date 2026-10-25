// Isolates the final, un-guarded construction step both `tcgetsid` implementations perform
// (`Pid::from_raw_unchecked(pid)`, with no `pid == 0` check anywhere on the path, unlike the
// sibling `tcgetpgrp` - candidate 035) with the exact value real Linux kernel source proves is a
// reachable return from the real `TIOCGSID` ioctl: `tiocgsid()`
// (drivers/tty/tty_jobctrl.c) computes `sid = pid_vnr(real_tty->ctrl.session)`, and
// `pid_vnr`/`pid_nr_ns` (kernel/pid.c) returns 0 whenever the caller's pid namespace is nested
// strictly deeper than the one the tty's controlling session was created in - see RESULT.md for
// the full kernel source citations. This sandbox cannot construct that real condition end to end
// (`unshare(CLONE_NEWUSER)`/`unshare(CLONE_NEWPID)` both fail with EPERM here - see
// real_tcgetsid's own Step 2 output), so this binary demonstrates, under Miri, that constructing
// a `Pid` from the value kernel source proves is reachable is genuine Undefined Behavior -
// independent of whether the full cross-namespace scenario can be exercised in this environment.
use rustix::process::Pid;

fn main() {
    let sid: i32 = 0;
    let p = unsafe { Pid::from_raw_unchecked(sid) };
    println!("constructed (should be unreachable under Miri): {:?}", p);
}
