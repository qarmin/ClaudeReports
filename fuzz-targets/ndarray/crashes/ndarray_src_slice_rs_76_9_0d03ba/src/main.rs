use ndarray::Axis;
use ndarray::Slice;

fn main() {
    let axis = Axis(0usize);
    let usize_v = axis.index();
    let slice = Slice::from(usize_v..0usize);
    let _ = slice.step_by(0isize);
}
