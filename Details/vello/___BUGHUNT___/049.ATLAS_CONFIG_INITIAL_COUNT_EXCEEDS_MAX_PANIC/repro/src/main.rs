// Standalone reproduction for candidate 049.
//
// Calls vello_common's real, fully public API exactly as an embedder would: build an
// `AtlasConfig` and hand it to `MultiAtlasManager::new`. Nothing here is a hand-authored
// stand-in for another participant - `AtlasConfig` is a plain public struct with no invariant
// enforced by a constructor, so any embedder assembling it (including via struct-update syntax
// off `..Default::default()`, as shown here) can independently produce this exact value.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use vello_common::multi_atlas::{AtlasConfig, MultiAtlasManager};

fn try_config(label: &str, config: AtlasConfig) -> bool {
    let result = std::panic::catch_unwind(|| MultiAtlasManager::new(config));
    match result {
        Ok(manager) => {
            println!("{label}: OK, atlas_count={}", manager.atlas_count());
            false
        }
        Err(payload) => {
            eprintln!("{label}: REPRODUCED - panicked: {payload:?}");
            true
        }
    }
}

fn main() {
    // The exact case from the candidate description: an embedder sets initial_atlas_count
    // higher than max_atlases, e.g. to pre-warm several atlases while also capping growth.
    let reproduced = try_config(
        "initial_atlas_count=4, max_atlases=2",
        AtlasConfig {
            initial_atlas_count: 4,
            max_atlases: 2,
            ..Default::default()
        },
    );

    // A couple of additional boundary values in the same class, to show the bug isn't limited
    // to this one specific pair of numbers.
    try_config(
        "initial_atlas_count=usize::MAX, max_atlases=0",
        AtlasConfig {
            initial_atlas_count: usize::MAX,
            max_atlases: 0,
            ..Default::default()
        },
    );
    try_config(
        "initial_atlas_count=max_atlases+1 (9 vs 8)",
        AtlasConfig {
            initial_atlas_count: 9,
            max_atlases: 8,
            ..Default::default()
        },
    );

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - suspect code path completed normally");
    }
}
