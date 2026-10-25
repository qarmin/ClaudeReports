use brotli::enc::backward_references::H6Sub;

fn main() {
    let _ = <brotli::enc::backward_references::H6Sub as brotli::enc::backward_references::AdvHashSpecialization>::load_and_mix_word(&H6Sub { hash_mask: 0u64, hash_shift_: 0i32, bucket_size_: 0u32, block_mask_: 0u32, block_bits_: 0i32 }, &vec![]);
}
