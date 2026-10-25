use x11rb_protocol::protocol::xproto::GetPropertyReply;

fn main() {
    <x11rb_protocol::protocol::xproto::GetPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetPropertyReply { format: 0u8, sequence: 0u16, length: 1u32, type_: 0u32, bytes_after: 0u32, value_len: 0u32, value: vec![66u8] }, &mut vec![]);
}
