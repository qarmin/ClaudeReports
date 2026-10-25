use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let _ = TransformDictionaryWord(&mut vec![], &vec![71u8], 1i32, 0i32);
}
