// Candidate 017: checks whether `Id<I>`'s `XapEnumByInput` impl
// (src/infallible/xap_variants/id.rs) itself carries a restrictive trait bound that blocks
// `.enumerate()` after a cardinality-changing step. Real end-to-end use of the crate's public API
// - no hand-built stand-in for anything.
//
// `Id<I>`'s own impl is `impl<I> XapEnumByInput for Id<I>` - completely unconstrained (see
// map_then_enumerate_control.rs for proof it works fine wherever reached). This scenario fails to
// compile, but the trait-solver diagnostic below points at `OneF<Id<i32>, ...>` (the type
// `.filter()` produces) never implementing `XapEnumByInput` at all - a different, already-known
// architecture-level gap (findings.toml's own `architecture`-typed finding on `OneF`/`BinF`/
// `ManyF`/`OneX`/`BinX`/`ManyX`), not a bound on `Id`'s own impl.

use orx_parallel::*;

fn main() {
    let pairs: Vec<(usize, i32)> = (0..10)
        .into_par()
        .filter(|x| *x % 2 == 0)
        .enumerate()
        .collect();

    println!("{:?}", pairs);
}
