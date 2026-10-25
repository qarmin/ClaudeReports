use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let i32_v = brotli_decompressor::transform::kNumTransforms;
    let _ = TransformDictionaryWord(&mut vec![], &vec![], i32_v, 0i32);
}
