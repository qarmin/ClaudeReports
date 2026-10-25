use x11rb_protocol::protocol::xkb::DeviceLedInfo;
use x11rb_protocol::protocol::xkb::LedClass;

fn main() {
    <x11rb_protocol::protocol::xkb::DeviceLedInfo as x11rb_protocol::x11_utils::Serialize>::serialize_into(&DeviceLedInfo { led_class: LedClass::from(0u8), led_id: 0u16, names_present: 0u32, maps_present: 0u32, phys_indicators: 0u32, state: 0u32, names: vec![0u32], maps: vec![] }, &mut vec![]);
}
