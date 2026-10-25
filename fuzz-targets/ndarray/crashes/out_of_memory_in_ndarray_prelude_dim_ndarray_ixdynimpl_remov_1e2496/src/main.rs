use ndarray::Axis;

fn main() {
    let _ = <ndarray::prelude::Dim<ndarray::IxDynImpl> as ndarray::RemoveAxis>::remove_axis(&<ndarray::prelude::Dim<ndarray::IxDynImpl>>::zeros(1000000000usize), Axis(0usize));
}
