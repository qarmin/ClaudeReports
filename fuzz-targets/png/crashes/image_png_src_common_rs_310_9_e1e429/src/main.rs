use png::BlendOp;
use png::DisposeOp;
use png::FrameControl;

fn main() {
    let mut frame_control = FrameControl { sequence_number: u32::MAX, width: 0u32, height: 0u32, x_offset: 0u32, y_offset: 0u32, delay_num: 0u16, delay_den: 0u16, dispose_op: DisposeOp::Previous, blend_op: BlendOp::Over };
    frame_control.inc_seq_num(1u32);
}
