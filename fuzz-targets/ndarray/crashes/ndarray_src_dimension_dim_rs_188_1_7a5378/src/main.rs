use ndarray::prelude::Ix5;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 5]> as ::core::ops::Sub<ndarray::prelude::Dim<[usize; 5]>>>::sub(Ix5(0usize, 0usize, 0usize, 0usize, 0usize), Ix5(0usize, 0usize, 0usize, 0usize, 1usize));
}
