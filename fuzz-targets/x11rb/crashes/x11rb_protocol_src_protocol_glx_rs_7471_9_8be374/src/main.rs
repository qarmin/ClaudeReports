use x11rb_protocol::protocol::glx::GetPolygonStippleReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetPolygonStippleReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetPolygonStippleReply { sequence: 0u16, data: vec![66u8] }, &mut vec![]);
}
