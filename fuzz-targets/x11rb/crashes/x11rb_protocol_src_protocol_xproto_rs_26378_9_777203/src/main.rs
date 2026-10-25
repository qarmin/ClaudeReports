use x11rb_protocol::protocol::xproto::GetModifierMappingReply;

fn main() {
    <x11rb_protocol::protocol::xproto::GetModifierMappingReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetModifierMappingReply { sequence: 0u16, length: 0u32, keycodes: vec![0u8] }, &mut vec![]);
}
