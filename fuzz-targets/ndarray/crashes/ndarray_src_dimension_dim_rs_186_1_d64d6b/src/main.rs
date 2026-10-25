use ndarray::Axis;
use ndarray::prelude::Ix5;

fn main() {
    let axis = Axis(1usize);
    let usize_v = axis.index();
    let _ = <ndarray::prelude::Dim<[usize; 5]> as ::core::ops::Add<ndarray::prelude::Dim<[usize; 5]>>>::add(Ix5(0usize, 0usize, usize_v, 0usize, usize::MAX), Ix5(0usize, 0usize, 0usize, usize_v, 1usize));
}
