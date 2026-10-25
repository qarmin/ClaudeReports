fn main() {
    let _ = brotli_decompressor::brotli_decode_prealloc(&vec![], &mut vec![], &mut vec![], &mut vec![], &mut vec![]);
}
