use parking::Parker;

fn main() {
    let parker = Parker::new();
    parker.park();
}
