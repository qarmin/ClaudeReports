fn main() {
    let _ = <font_types::Point<u32> as ::core::ops::Mul<font_types::Point<u32>>>::mul(<font_types::Point<u32>>::new(1000000000u32, 0u32), <font_types::Point<u32>>::broadcast(10000u32));
}
