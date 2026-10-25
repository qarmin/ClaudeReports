use x11rb_protocol::protocol::xf86vidmode::ClockFlag;
use x11rb_protocol::protocol::xf86vidmode::GetDotClocksReply;

fn main() {
    <x11rb_protocol::protocol::xf86vidmode::GetDotClocksReply as x11rb_protocol::x11_utils::Serialize>::serialize_into(&GetDotClocksReply { sequence: 0u16, length: 0u32, flags: ClockFlag::from(0u16), clocks: 0u32, maxclocks: 0u32, clock: vec![0u32] }, &mut vec![]);
}
