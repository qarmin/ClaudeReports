use x11rb_protocol::protocol::xinput::GetDevicePropertyItems;
use x11rb_protocol::protocol::xinput::GetDevicePropertyReply;

fn main() {
    <x11rb_protocol::protocol::xinput::GetDevicePropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetDevicePropertyReply { xi_reply_type: 0u8, sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 0u32, device_id: 0u8, items: GetDevicePropertyItems::InvalidValue(0u8) }, &mut vec![]);
}
