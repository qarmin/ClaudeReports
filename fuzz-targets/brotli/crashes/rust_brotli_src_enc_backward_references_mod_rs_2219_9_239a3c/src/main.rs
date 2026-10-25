fn main() {
    let mut union_hasher = <brotli::enc::UnionHasher<brotli::enc::StandardAlloc> as ::core::default::Default>::default();
    let _ = <brotli::enc::UnionHasher<brotli::enc::StandardAlloc> as brotli::enc::backward_references::AnyHasher>::GetHasherCommon(&mut union_hasher);
}
