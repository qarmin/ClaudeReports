use x11rb_protocol::protocol::xinput::Fp3232;
use x11rb_protocol::protocol::xinput::GroupInfo;
use x11rb_protocol::protocol::xinput::ModifierInfo;
use x11rb_protocol::protocol::xinput::TouchBeginEvent;
use x11rb_protocol::protocol::xinput::TouchEventFlags;

fn main() {
    <x11rb_protocol::protocol::xinput::TouchBeginEvent as x11rb_protocol::x11_utils::Serialize>::serialize_into(&TouchBeginEvent { response_type: 0u8, extension: 0u8, sequence: 0u16, length: 0u32, event_type: 0u16, deviceid: 0u16, time: 0u32, detail: 1u32, root: 0u32, event: 0u32, child: 0u32, root_x: 0i32, root_y: 0i32, event_x: 0i32, event_y: 0i32, sourceid: 0u16, flags: TouchEventFlags::TOUCH_EMULATING_POINTER, mods: ModifierInfo::default(), group: GroupInfo::default(), button_mask: vec![], valuator_mask: vec![], axisvalues: vec![Fp3232::default()] }, &mut vec![]);
}
