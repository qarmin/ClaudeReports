use x11rb_protocol::protocol::glx::ReadPixelsReply;

fn main() {
    <x11rb_protocol::protocol::glx::ReadPixelsReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&ReadPixelsReply { sequence: 0u16, data: vec![0u8] }, &mut vec![]);
}
