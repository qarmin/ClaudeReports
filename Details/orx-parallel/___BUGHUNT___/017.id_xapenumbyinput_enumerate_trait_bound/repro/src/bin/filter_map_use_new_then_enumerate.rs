// Re-verification of the sibling file's shape (src/infallible_use/xap_variants/id.rs, `IdUse<X,
// U>`'s `XapUseEnumByInput` impl, bounded `impl<X: XapEnumByInput<Size = One>, U> ...`) at this
// candidate's own base_commit, independent of candidate 007's own investigation (same conclusion,
// re-run standalone here since candidate 017 was seeded as nominally "distinct" from 007 and
// needed its own verification before being folded into the same finding).
//
// Real end-to-end use of the public API: a `Size = Bin` xap (via `.filter_map`) wrapped in
// `IdUse` via `use_new`, then `.enumerate()` on the resulting `ParUse`.

use orx_parallel::*;

fn main() {
    let values: Vec<(usize, i32)> = (0..10)
        .into_par()
        .filter_map(|x| if x % 2 == 0 { Some(x) } else { None })
        .use_new(|_| ())
        .enumerate()
        .map(|_, (i, x)| (i, x))
        .collect();

    println!("{:?}", values);
}
