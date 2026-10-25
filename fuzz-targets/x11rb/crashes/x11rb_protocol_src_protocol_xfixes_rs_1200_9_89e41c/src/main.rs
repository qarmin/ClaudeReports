use x11rb_protocol::protocol::xfixes::GetCursorImageReply;

fn main() {
    <x11rb_protocol::protocol::xfixes::GetCursorImageReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetCursorImageReply { sequence: 0u16, length: 0u32, x: 0i16, y: 0i16, width: 0u16, height: 0u16, xhot: 0u16, yhot: 1u16, cursor_serial: 0u32, cursor_image: vec![0u32] }, &mut vec![]);
}
