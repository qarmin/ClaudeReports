use rmp::encode::ByteBuf;

fn main() {
    let _ = ByteBuf::with_capacity(usize::MAX);
}
