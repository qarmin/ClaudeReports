use ndarray::Axis;
use ndarray::prelude::Ix3;
use ndarray::prelude::Ix5;

fn main() {
    let dim = Ix5(0usize, 0usize, 0usize, 1usize, 1usize);
    let usize_v = <ndarray::prelude::Dim<[usize; 5]> as ndarray::Dimension>::ndim(&dim);
    let _ = <ndarray::prelude::Dim<[usize; 3]> as ndarray::RemoveAxis>::remove_axis(&Ix3(0usize, 0usize, 0usize), Axis(usize_v));
}
