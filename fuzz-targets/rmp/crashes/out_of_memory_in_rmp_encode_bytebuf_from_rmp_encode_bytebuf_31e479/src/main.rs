use rmp::encode::ByteBuf;

fn main() {
    let _ = ByteBuf::from(ByteBuf::with_capacity(4294967295usize));
}
