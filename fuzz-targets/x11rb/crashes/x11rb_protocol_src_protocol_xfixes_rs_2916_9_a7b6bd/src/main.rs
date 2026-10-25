use x11rb_protocol::protocol::xfixes::GetCursorImageAndNameReply;

fn main() {
    <x11rb_protocol::protocol::xfixes::GetCursorImageAndNameReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetCursorImageAndNameReply { sequence: 0u16, length: 0u32, x: 0i16, y: 0i16, width: 0u16, height: 1u16, xhot: 0u16, yhot: 0u16, cursor_serial: 0u32, cursor_atom: 0u32, cursor_image: vec![0u32], name: vec![] }, &mut vec![]);
}
