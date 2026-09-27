// Control experiment: a pipeline whose `Xap` chain does bottom out at `Id<I>` through a step that
// *does* implement `XapEnumByInput` (`.map()` produces `OneM<Id<I>, G>`, and `OneM`'s own impl is
// `impl<X: XapEnumByInput<Size = One>, G: Map<I = X::O>> XapEnumByInput for OneM<X, G>`, which
// `Id<I>` satisfies unconditionally since `Id`'s `Size` is always `One` and its `XapEnumByInput`
// impl has no bound at all).
//
// This is expected to compile and run correctly - proof that `Id<I>`'s own `XapEnumByInput` impl
// is not restrictive anywhere it is actually reachable through the crate's public API.

use orx_parallel::*;

fn main() {
    let pairs: Vec<(usize, usize)> = (0..10)
        .into_par()
        .map(|x| x * 2)
        .enumerate()
        .collect();

    println!("{:?}", pairs);
    assert_eq!(pairs.len(), 10);
    assert!(pairs.contains(&(0, 0)));
    assert!(pairs.contains(&(9, 18)));
    println!("OK: map().enumerate() through Id<I> works correctly");
}
