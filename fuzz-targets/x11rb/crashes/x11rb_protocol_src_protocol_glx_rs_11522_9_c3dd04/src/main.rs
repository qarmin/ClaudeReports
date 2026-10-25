use x11rb_protocol::protocol::glx::GetMinmaxReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetMinmaxReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetMinmaxReply { sequence: 1u16, data: vec![0u8] }, &mut vec![]);
}
