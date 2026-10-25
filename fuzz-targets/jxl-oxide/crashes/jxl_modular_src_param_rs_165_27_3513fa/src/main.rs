use jxl_modular::ChannelShift;

fn main() {
    let channel_shift = ChannelShift::Shifts(46u32);
    let _ = channel_shift.shift_size((0u32, 0u32));
}
