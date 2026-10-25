use x11rb_protocol::protocol::glx::GetTexImageReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetTexImageReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetTexImageReply { sequence: 0u16, width: 0i32, height: 0i32, depth: 0i32, data: vec![73u8] }, &mut vec![]);
}
