use text_size::TextRange;
use text_size::TextSize;

fn main() {
    let text_size = TextSize::from(0u32);
    let text_range = TextRange::empty(text_size);
    let text_range2 = text_range.cover_offset(text_size);
    let text_range3 = text_range2.cover(text_range);
    let _ = <text_size::TextRange as ::core::ops::Sub<text_size::TextSize>>::sub(text_range3, TextSize::try_from(1usize).unwrap());
}
