use x11rb_protocol::protocol::xinput::XIGetPropertyItems;
use x11rb_protocol::protocol::xinput::XIGetPropertyReply;

fn main() {
    <x11rb_protocol::protocol::xinput::XIGetPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&XIGetPropertyReply { sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 0u32, items: XIGetPropertyItems::InvalidValue(0u8) }, &mut vec![]);
}
