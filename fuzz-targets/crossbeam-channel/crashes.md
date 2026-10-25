# crossbeam-channel - crashes

Source: https://github.com/crossbeam-rs/crossbeam @ 099d0469efcfacb32527bd9451a4fb62586ae537

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let guard = crossbeam_epoch::pin();
let _ = guard.collector();
let wait_group = WaitGroup::default();
wait_group.clone().wait();
let _ = RecvTimeoutError::from(RecvError::from(RecvError::from(RecvError)));</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crossbeam_utils::sync::wait_group::WaitGroup::wait (src/sync/wait_group.rs:129)
  repro_timeout_in_crossbeam_utils_sync_waitgroup_wait_430314::main (src/main.rs:9)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let parker = Parker::new();
parker.park();</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  crossbeam_utils::sync::parker::Inner::park (src/sync/parker.rs:379)
  crossbeam_utils::sync::parker::Parker::park (src/sync/parker.rs:110)
  repro_timeout_in_crossbeam_utils_sync_parker_park_68e584::main (src/main.rs:5)
  (5 std/libc frames omitted)</pre></td></tr>
</table>
