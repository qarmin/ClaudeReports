use crossbeam_channel::RecvError;
use crossbeam_channel::RecvTimeoutError;
use crossbeam_utils::sync::WaitGroup;

fn main() {
    let guard = crossbeam_epoch::pin();
    let _ = guard.collector();
    let wait_group = WaitGroup::default();
    wait_group.clone().wait();
    let _ = RecvTimeoutError::from(RecvError::from(RecvError::from(RecvError)));
}
