use brotli::concat::BroCatli;

fn main() {
    let mut bro_catli = BroCatli::try_new_with_window_size(100u8).unwrap();
    let _ = bro_catli.finish(&mut vec![], &mut 1usize);
}
