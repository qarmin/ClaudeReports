use x11rb_protocol::protocol::glx::QueryContextReply;

fn main() {
    <x11rb_protocol::protocol::glx::QueryContextReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&QueryContextReply { sequence: 1u16, length: 0u32, attribs: vec![0u32] }, &mut vec![]);
}
