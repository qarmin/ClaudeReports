use ndarray::Axis;
use ndarray::prelude::IxDyn;

fn main() {
    let axis = Axis(0usize);
    let _ = <ndarray::prelude::Dim<ndarray::IxDynImpl> as ndarray::RemoveAxis>::remove_axis(&IxDyn(&[]), axis);
}
