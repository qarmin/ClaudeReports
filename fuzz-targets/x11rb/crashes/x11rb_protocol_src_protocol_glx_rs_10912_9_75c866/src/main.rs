use x11rb_protocol::protocol::glx::GetSeparableFilterReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetSeparableFilterReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetSeparableFilterReply { sequence: 0u16, row_w: 0i32, col_h: 0i32, rows_and_cols: vec![84u8] }, &mut vec![]);
}
