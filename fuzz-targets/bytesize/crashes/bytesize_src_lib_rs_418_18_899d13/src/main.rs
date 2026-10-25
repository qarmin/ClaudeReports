use bytesize::ByteSize;

fn main() {
    let _ = <bytesize::ByteSize as ::core::ops::Add<bytesize::ByteSize>>::add(ByteSize::mib(1u64), ByteSize::b(u64::MAX));
}
