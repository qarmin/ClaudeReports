use mio::Events;

fn main() {
    let events = Events::with_capacity(1000000000usize);
    let _ = events.iter();
}
