use x11rb_protocol::protocol::randr::GetScreenInfoReply;
use x11rb_protocol::protocol::randr::Rotation;
use x11rb_protocol::protocol::randr::ScreenSize;

fn main() {
    <x11rb_protocol::protocol::randr::GetScreenInfoReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetScreenInfoReply { rotations: Rotation::from(0u16), sequence: 0u16, length: 0u32, root: 0u32, timestamp: 0u32, config_timestamp: 0u32, size_id: 0u16, rotation: Rotation::from(0u8), rate: 0u16, n_info: 1u16, sizes: vec![ScreenSize::default(), ScreenSize::default()], rates: vec![] }, &mut vec![]);
}
