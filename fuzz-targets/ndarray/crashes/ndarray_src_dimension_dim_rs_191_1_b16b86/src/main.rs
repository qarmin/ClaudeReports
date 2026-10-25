use ndarray::prelude::Ix4;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 4]> as ::core::ops::Mul<usize>>::mul(Ix4(0usize, 0usize, 0usize, 10000000000000000000usize), 6usize);
}
