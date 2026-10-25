// NOT a reproduction of the real bug (no real OS ever produces a child.id() this large - see
// output.txt / RESULT.md for the cross-platform ceilings that make that unreachable). This exists
// purely to document the root-cause *mechanism* in isolation: it hand-constructs the u32 value a
// genuine OS pid allocator would need to produce for the cast in `Pid::from_child` to misbehave,
// and feeds it straight to the same unsafe primitive `from_child` calls
// (`Pid::from_raw_unchecked`), to show the debug-build panic / release-build silent-negative-Pid
// split described by the candidate. Per CONFIRMATION.md section 11, this alone does not and
// cannot justify a `Confirmed` verdict for candidate 066.

use rustix::process::Pid;

fn main() {
    let hypothetical_os_pid: u32 = 3_000_000_000; // > i32::MAX; no real OS ever assigns this
    let cast = hypothetical_os_pid as i32;
    println!(
        "hypothetical child.id() = {hypothetical_os_pid} (u32) as i32 = {cast} (negative: {})",
        cast < 0
    );

    // SAFETY: deliberately violating the "non-zero" contract's spirit (passing a negative value)
    // to observe what `from_raw_unchecked` actually does in debug vs release, mirroring exactly
    // what `Pid::from_child` would do if `child.id()` ever returned `hypothetical_os_pid`.
    let pid = unsafe { Pid::from_raw_unchecked(cast) };
    println!(
        "from_raw_unchecked({cast}) returned normally (this line is only reached in a release \
         build, where debug_assert! is compiled out): as_raw_pid() = {}",
        pid.as_raw_pid()
    );
}
