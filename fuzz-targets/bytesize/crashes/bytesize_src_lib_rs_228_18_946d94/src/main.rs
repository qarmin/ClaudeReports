use bytesize::ByteSize;

fn main() {
    let _ = ByteSize::pb(u64::MAX);
}
