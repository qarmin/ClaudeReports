use seahash::State;

fn main() {
    let u64_v = seahash::reference::hash(&vec![]);
    let mut state = State::new(0u64, 0u64, 0u64, u64_v);
    state.pop(0u64);
}
