use x11rb_protocol::protocol::randr::GetOutputPropertyReply;

fn main() {
    <x11rb_protocol::protocol::randr::GetOutputPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetOutputPropertyReply { format: 0u8, sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 0u32, data: vec![35u8] }, &mut vec![]);
}
