use ndarray::Axis;

fn main() {
    let _ = <ndarray::prelude::Dim<[usize; 1]> as ndarray::RemoveAxis>::remove_axis(&<ndarray::prelude::Dim<[usize; 1]> as ::core::default::Default>::default(), Axis(1usize));
}
