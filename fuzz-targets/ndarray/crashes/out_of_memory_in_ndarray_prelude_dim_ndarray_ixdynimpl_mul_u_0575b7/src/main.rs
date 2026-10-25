fn main() {
    let _ = <ndarray::prelude::Dim<ndarray::IxDynImpl> as ::core::ops::Mul<usize>>::mul(<ndarray::prelude::Dim<ndarray::IxDynImpl>>::zeros(1000000000usize), 0usize);
}
