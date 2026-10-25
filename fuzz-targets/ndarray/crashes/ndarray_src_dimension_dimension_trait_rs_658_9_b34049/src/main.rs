use ndarray::prelude::Ix2;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 2]> as ndarray::Dimension>::size(&Ix2(10000000000000000000usize, 6usize));
}
