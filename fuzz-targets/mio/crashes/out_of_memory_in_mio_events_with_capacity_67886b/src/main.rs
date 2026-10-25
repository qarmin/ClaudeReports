use mio::Events;

fn main() {
    let _ = Events::with_capacity(1000000000usize);
}
