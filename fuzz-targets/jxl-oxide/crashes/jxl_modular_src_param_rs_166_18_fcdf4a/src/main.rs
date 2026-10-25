use jxl_modular::ChannelShift;

fn main() {
    let channel_shift = ChannelShift::from_shift(1u32);
    let _ = channel_shift.shift_size((u32::MAX, 0u32));
}
