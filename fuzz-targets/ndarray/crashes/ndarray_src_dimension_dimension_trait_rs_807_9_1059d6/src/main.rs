use ndarray::prelude::Ix3;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 3]> as ndarray::Dimension>::size(&Ix3(10000000000000000000usize, 255usize, 256usize));
}
