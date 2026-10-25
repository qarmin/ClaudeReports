use ndarray::prelude::Ix1;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 1]> as ::core::ops::Add<usize>>::add(Ix1(usize::MAX), 1usize);
}
