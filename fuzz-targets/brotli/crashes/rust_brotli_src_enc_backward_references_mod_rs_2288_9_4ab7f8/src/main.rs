fn main() {
    let mut union_hasher = <brotli::enc::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator> as ::core::default::Default>::default();
    <brotli::enc::UnionHasher<brotli::ffi::alloc_util::BrotliSubclassableAllocator> as brotli::enc::backward_references::AnyHasher>::StoreEvenVec4(&mut union_hasher, &vec![], 0usize, 0usize);
}
