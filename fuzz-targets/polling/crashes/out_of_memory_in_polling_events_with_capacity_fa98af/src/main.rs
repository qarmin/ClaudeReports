use polling::Events;

fn main() {
    let _ = Events::with_capacity(std::num::NonZero::new(1000000000usize).unwrap());
}
