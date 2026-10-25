fn main() {
    let _ = <font_types::Point<u32> as ::core::ops::Add<font_types::Point<u32>>>::add(<font_types::Point<u32>>::new(1u32, 0u32), <font_types::Point<u32>>::broadcast(u32::MAX));
}
