use x11rb_protocol::protocol::glx::GetCompressedTexImageARBReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetCompressedTexImageARBReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetCompressedTexImageARBReply { sequence: 0u16, size: 0i32, data: vec![0u8] }, &mut vec![]);
}
