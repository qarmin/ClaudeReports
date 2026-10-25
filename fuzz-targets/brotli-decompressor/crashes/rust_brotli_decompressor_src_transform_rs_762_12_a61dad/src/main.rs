use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let _ = TransformDictionaryWord(&mut vec![], &vec![], -100000000i32, 0i32);
}
