// Control experiment: the exact same `.filter_map(...).enumerate()` shape as
// `use_new_scenario.rs`, but with `use_new`/`IdUse` removed entirely (plain `infallible::Par`,
// no `use_new` in the pipeline at all).
//
// This does NOT go through `IdUse`'s `XapUseEnumByInput` impl at all - `EnumeratePar::enumerate`
// (src/infallible/par_enum.rs) only requires `Self::Xap: XapEnumByInput`, and `Self::Xap` here is
// `OneF<Id<i32>, ...>` (the type `.filter_map` produces), never touching `IdUse`.
//
// If this also fails to compile with the same "trait bounds not satisfied" shape, that proves the
// real blocker for `use_new_scenario.rs` is that `OneF`/`OneX`/`BinF`/`BinX`/`ManyF`/`ManyX` (every
// step that actually introduces `Size = Bin`/`Size = Many`) never implement `XapEnumByInput` at
// all anywhere in the crate - not the `IdUse`-specific `Size = One` bound candidate 007 is about.

use orx_parallel::*;

fn main() {
    let values: Vec<(usize, i32)> = (0..10)
        .into_par()
        .filter_map(|x| if x % 2 == 0 { Some(x) } else { None })
        .enumerate()
        .collect();

    println!("{:?}", values);
}
