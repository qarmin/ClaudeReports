fn main() {
    let mut layout = <parley::Layout<[u8; 4]>>::new();
    layout.break_all_lines(Some(f32::NEG_INFINITY));
}
