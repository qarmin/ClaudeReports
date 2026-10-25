use brotli_decompressor::transform::TransformDictionaryWord;

fn main() {
    let i32_v = brotli_decompressor::transform::kNumTransforms;
    let _ = TransformDictionaryWord(&mut vec![], &vec![], 0i32, i32_v);
}
