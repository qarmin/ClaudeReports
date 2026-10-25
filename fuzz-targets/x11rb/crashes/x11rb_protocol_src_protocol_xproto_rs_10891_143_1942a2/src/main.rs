use x11rb_protocol::protocol::xproto::GetPropertyReply;

fn main() {
    <x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetPropertyReply { format: 100u8, sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, value_len: 1000000000u32, value: vec![] }, &mut vec![]);
}
