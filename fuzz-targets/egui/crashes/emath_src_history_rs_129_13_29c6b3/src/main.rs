fn main() {
    let mut history = <emath::History<u8>>::new(1usize..1usize, 0.0f32);
    history.add(1.0f64, 0u8);
    history.add(0.0f64, 0u8);
}
