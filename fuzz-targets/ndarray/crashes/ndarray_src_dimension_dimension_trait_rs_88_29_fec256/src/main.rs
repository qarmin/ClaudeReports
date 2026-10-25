use ndarray::prelude::Ix6;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 6]> as ndarray::Dimension>::size(&Ix6(1usize, 1usize, 1usize, usize::MAX, 256usize, 4294967296usize));
}
