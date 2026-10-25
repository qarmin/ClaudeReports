use x11rb_protocol::protocol::glx::GetDrawableAttributesReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetDrawableAttributesReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetDrawableAttributesReply { sequence: 0u16, length: 0u32, attribs: vec![0u32] }, &mut vec![]);
}
