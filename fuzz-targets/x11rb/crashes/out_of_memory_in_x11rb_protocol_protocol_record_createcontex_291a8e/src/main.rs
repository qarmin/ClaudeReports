use x11rb_protocol::protocol::record::CreateContextRequest;
use x11rb_protocol::x11_utils::RequestHeader;

fn main() {
    let vec = vec![82u8, 73u8, 70u8, 70u8, 0u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8];
    let _ = CreateContextRequest::try_parse_request(RequestHeader { major_opcode: 0u8, minor_opcode: 1u8, remaining_length: 0u32 }, &vec);
}
