use x11rb_protocol::protocol::randr::GetProviderPropertyReply;

fn main() {
    <x11rb_protocol::protocol::randr::GetProviderPropertyReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetProviderPropertyReply { format: 74u8, sequence: 0u16, length: 0u32, type_: 0u32, bytes_after: 0u32, num_items: 1000000000u32, data: vec![] }, &mut vec![]);
}
