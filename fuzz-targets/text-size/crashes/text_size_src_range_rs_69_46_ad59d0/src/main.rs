use text_size::TextRange;
use text_size::TextSize;

fn main() {
    let _ = TextRange::at(TextSize::new(u32::MAX), TextSize::try_from(1usize).unwrap());
}
