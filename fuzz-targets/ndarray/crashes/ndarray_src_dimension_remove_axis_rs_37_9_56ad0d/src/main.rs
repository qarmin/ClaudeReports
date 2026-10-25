use ndarray::Axis;
use ndarray::prelude::Ix2;

fn main() {
    let axis = Axis(1000000000usize);
    let _ = <ndarray::prelude::Dim<[usize; 2]> as ndarray::RemoveAxis>::remove_axis(&Ix2(0usize, 0usize), axis);
}
