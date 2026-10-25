use ndarray::prelude::Ix5;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 5]> as ::core::ops::Mul<ndarray::prelude::Dim<[usize; 5]>>>::mul(Ix5(0usize, 0usize, 100usize, 0usize, 0usize), Ix5(0usize, 0usize, 10000000000000000000usize, 2usize, 4294967296usize));
}
