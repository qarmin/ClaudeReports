use x11rb_protocol::protocol::glx::GetConvolutionFilterReply;

fn main() {
    <x11rb_protocol::protocol::glx::GetConvolutionFilterReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetConvolutionFilterReply { sequence: 0u16, width: 0i32, height: 0i32, data: vec![104u8] }, &mut vec![]);
}
