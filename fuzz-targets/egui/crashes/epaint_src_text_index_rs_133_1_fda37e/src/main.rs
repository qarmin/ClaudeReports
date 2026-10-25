use epaint::text::ByteIndex;

fn main() {
    let _ = <epaint::text::ByteIndex as ::core::ops::Sub<epaint::text::ByteIndex>>::sub(ByteIndex(0usize), ByteIndex(1usize));
}
