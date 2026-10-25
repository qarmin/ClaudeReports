use rmp::encode::ByteBuf;

fn main() {
    let byte_buf = ByteBuf::with_capacity(2147483648usize);
    let _ = byte_buf.as_vec();
}
