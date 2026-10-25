use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let _ = TransformDictionaryWord(&mut vec![], &vec![], 0i32, 4i32);
}
