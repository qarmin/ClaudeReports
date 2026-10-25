use rmp::encode::ByteBuf;

fn main() {
    let _ = ByteBuf::with_capacity(4294967296usize);
}
