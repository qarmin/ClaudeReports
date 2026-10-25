use rmp::Marker;
use rmp::decode::LenError;
use rmp::encode::ByteBuf;

fn main() {
    let len_error = LenError::Truncated(std::num::NonZero::new(1000000000000000000usize).unwrap());
    let usize_v = len_error.len();
    let _ = Marker::from(rmp::encode::write_ext_meta::<rmp::encode::ByteBuf>(&mut ByteBuf::with_capacity(usize_v), 1u32, 0i8).unwrap());
}
