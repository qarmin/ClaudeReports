use ndarray::Slice;

fn main() {
    let slice = Slice::new(0isize, None, -1000000000000000000isize);
    let _ = slice.step_by(10isize);
}
