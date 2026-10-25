use brotli::concat::BroCatli;

fn main() {
    let _ = BroCatli::try_new_with_window_size(100u8).unwrap().stream(&vec![], &mut 1usize, &mut vec![82u8], &mut 0usize);
}
