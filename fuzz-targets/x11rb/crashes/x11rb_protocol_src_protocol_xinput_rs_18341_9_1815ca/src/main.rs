use x11rb_protocol::protocol::xinput::PointerEventFlags;
use x11rb_protocol::protocol::xinput::RawButtonPressEvent;

fn main() {
    <x11rb_protocol::protocol::xinput::RawButtonPressEvent as x11rb_protocol::x11_utils::Serialize>::serialize_into(&RawButtonPressEvent { response_type: 0u8, extension: 0u8, sequence: 0u16, length: 0u32, event_type: 0u16, deviceid: 0u16, time: 0u32, detail: 0u32, sourceid: 0u16, flags: PointerEventFlags::POINTER_EMULATED, valuator_mask: vec![1u32], axisvalues: vec![], axisvalues_raw: vec![] }, &mut vec![]);
}
