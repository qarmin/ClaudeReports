use brotli::enc::static_dict::BROTLI_UNALIGNED_STORE64;

fn main() {
    BROTLI_UNALIGNED_STORE64(&mut vec![], 0u64);
}
