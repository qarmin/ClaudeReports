use x11rb_protocol::protocol::glx::GetQueryObjectuivARBReply;
use x11rb_protocol::protocol::xinput::Fp3232;
use x11rb_protocol::protocol::xinput::KeyEventFlags;
use x11rb_protocol::protocol::xinput::RawKeyPressEvent;

fn main() {
    let get_query_objectuiv_a_r_b_reply = GetQueryObjectuivARBReply { sequence: 0u16, length: 0u32, datum: 0u32, data: vec![] };
    let u32_v = get_query_objectuiv_a_r_b_reply.n();
    <x11rb_protocol::protocol::xinput::RawKeyPressEvent as x11rb_protocol::x11_utils::Serialize>::serialize_into(&RawKeyPressEvent { response_type: 0u8, extension: 0u8, sequence: 0u16, length: 1u32, event_type: 1u16, deviceid: 0u16, time: u32_v, detail: 0u32, sourceid: 0u16, flags: KeyEventFlags::from(0u32), valuator_mask: vec![], axisvalues: vec![Fp3232 { integral: 0i32, frac: 0u32 }], axisvalues_raw: vec![] }, &mut vec![]);
}
