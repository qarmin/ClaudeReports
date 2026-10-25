use x11rb_protocol::protocol::record::EnableContextReply;

fn main() {
    <x11rb_protocol::protocol::record::EnableContextReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&EnableContextReply { category: 0u8, sequence: 0u16, element_header: 0u8, client_swapped: false, xid_base: 0u32, server_time: 0u32, rec_sequence_num: 0u32, data: vec![0u8] }, &mut vec![]);
}
