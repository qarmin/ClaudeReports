use nalgebra::base::dimension::Dyn;

fn main() {
    let _ = <nalgebra::base::dimension::Dyn as ::core::ops::Add<usize>>::add(Dyn(1usize), usize::MAX);
}
