use bytesize::ByteSize;

fn main() {
    let _ = <bytesize::ByteSize as ::core::ops::Sub<bytesize::ByteSize>>::sub(ByteSize::kib(0u64), ByteSize::kib(1u64));
}
