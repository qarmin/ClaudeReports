use epaint::text::CharIndex;

fn main() {
    let _ = <epaint::text::CharIndex as ::core::ops::Sub<usize>>::sub(CharIndex::default(), 1usize);
}
