use x11rb_protocol::protocol::xkb::GetIndicatorMapReply;
use x11rb_protocol::protocol::xkb::IndicatorMap;

fn main() {
    <x11rb_protocol::protocol::xkb::GetIndicatorMapReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetIndicatorMapReply { device_id: 0u8, sequence: 0u16, length: 0u32, which: 0u32, real_indicators: 0u32, n_indicators: 0u8, maps: vec![IndicatorMap::default()] }, &mut vec![]);
}
