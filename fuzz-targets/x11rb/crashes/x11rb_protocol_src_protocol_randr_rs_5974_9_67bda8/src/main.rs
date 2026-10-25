use x11rb_protocol::protocol::randr::GetProviderPropertyReply;

fn main() {
    <x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetProviderPropertyReply { format: 0u8, sequence: 1u16, length: 0u32, type_: 0u32, bytes_after: 1u32, num_items: 0u32, data: vec![0u8] }, &mut vec![]);
}
