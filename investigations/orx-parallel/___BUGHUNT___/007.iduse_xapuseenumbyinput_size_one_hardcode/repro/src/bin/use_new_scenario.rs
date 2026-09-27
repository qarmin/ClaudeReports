// Real end-to-end attempt at the scenario findings.toml/candidate 007 describes: a `ParUse`
// pipeline entered via `use_new` whose underlying xap has `Size = Bin` (built by `.filter_map`,
// a map-only-cardinality-changing step), then trying to call `.enumerate()` on the resulting
// `ParUse` pipeline. This exercises the crate's real public API end-to-end - no hand-built stand-
// in for anything.
//
// This is a *compile-time* trigger: if the bug (`IdUse`'s `XapUseEnumByInput` impl hard-coding
// `Size = One`) actually blocked something otherwise valid, this file would fail to compile with
// the pristine crate and succeed once the bound is relaxed to plain `X: XapEnumByInput` (no
// `Size = One` constraint). See `plain_scenario.rs` for the control experiment showing the same
// failure occurs even with no `IdUse`/`use_new` involved at all.

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
