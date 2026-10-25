use crossbeam_utils::sync::Parker;

fn main() {
    let parker = Parker::new();
    parker.park();
}
