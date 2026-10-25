use text_size::TextRange;
use text_size::TextSize;

fn main() {
    let text_size = TextSize::from(u32::MAX);
    let text_size2 = TextSize::new(0u32);
    let text_size3 = TextSize::new(1u32);
    let _ = <text_size::TextRange as ::core::ops::Add<text_size::TextSize>>::add(TextRange::at(text_size3, text_size2), text_size);
}
