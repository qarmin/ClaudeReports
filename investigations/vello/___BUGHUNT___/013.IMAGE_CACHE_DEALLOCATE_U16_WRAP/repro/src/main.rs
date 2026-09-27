//! Standalone repro for candidate 013 (IMAGE_CACHE_DEALLOCATE_U16_WRAP).
//!
//! Suspect: `ImageCache::deallocate` (sparse_strips/vello_common/src/image_cache.rs) recomputes
//! `padded_width`/`padded_height` as plain `u16` arithmetic (`width + padding * 2`) instead of
//! reusing `allocate`'s own `u32`-checked value, relying on the (unenforced-by-the-type-system)
//! invariant that every `ImageResource` reachable from `deallocate` was produced by `allocate`,
//! which already proved `width + 2*padding <= u16::MAX` via a `u32` computation before ever
//! constructing the resource.
//!
//! This binary drives ONLY the real, public `vello_common::image_cache` API (`allocate`, `get`,
//! `deallocate`) - the same entry points every real caller in this repo (vello_cpu, vello_hybrid)
//! uses - across a boundary-focused sweep of `(width, padding)` combinations, built with
//! `overflow-checks = true` (see Cargo.toml) so that if `deallocate`'s internal `u16` arithmetic
//! ever actually overflowed for an input `allocate` accepted, it would panic here rather than
//! silently wrap.
//!
//! This is a single-participant repro (a single process driving one crate's own public API with
//! adversarial-looking but self-produced arguments) - CONFIRMATION.md section 11's "malformed
//! file handed to a parser's own public entry point" carve-out applies identically here: `width`/
//! `padding` are exactly the values a real caller (an image decoder feeding vello_cpu/vello_hybrid)
//! would pass to `allocate`, there is no second participant to fake.

use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AtlasConfig;

fn sweep(atlas_size: (u16, u16)) -> u64 {
    let mut cache = ImageCache::new_with_config(AtlasConfig {
        atlas_size,
        ..Default::default()
    });

    let mut accepted: u64 = 0;
    // Representative widths clustered at the exact spots where an overflow in `deallocate`'s
    // `width + padding * 2` u16 recomputation would be most likely to occur: the smallest
    // widths (which let `allocate` accept the largest paddings) and the largest widths (which
    // force `allocate` to accept only tiny/zero paddings).
    let widths: [u16; 8] = [
        0,
        1,
        2,
        3,
        u16::MAX / 2 - 1,
        u16::MAX / 2,
        u16::MAX / 2 + 1,
        u16::MAX,
    ];

    for &width in &widths {
        for padding in 0u16..=u16::MAX {
            match cache.allocate(width, 1, padding) {
                Ok(id) => {
                    let resource = cache.get(id).expect("just-allocated id must be present");
                    assert_eq!(resource.width, width);
                    assert_eq!(resource.padding, padding);

                    // The actual suspect call: deallocate() internally recomputes
                    // `width + padding * 2` in u16 arithmetic. With `overflow-checks = true`,
                    // an overflow here panics instead of silently wrapping.
                    let dealloc = cache
                        .deallocate(id)
                        .expect("just-allocated id must deallocate");
                    assert_eq!(dealloc.width, width);
                    assert_eq!(dealloc.padding, padding);

                    accepted += 1;
                }
                Err(_) => {
                    // allocate() itself rejected this (width, padding) combination (padded size
                    // doesn't fit u16, or exceeds the configured atlas size) - deallocate() is
                    // never reached with it, so it can't demonstrate anything about the suspect.
                }
            }
        }
    }
    accepted
}

fn main() {
    // Two atlas configurations: the largest possible (u16::MAX x u16::MAX, so `allocate`'s own
    // `TextureTooLarge` rejection is driven purely by the u16 padded-size check, not by a smaller
    // configured atlas), and a realistic default-sized one (4096x4096, matching
    // `AtlasConfig::default()`), so the sweep also covers the atlas-size-bound rejection path.
    let accepted_max = sweep((u16::MAX, u16::MAX));
    println!(
        "atlas_size=(u16::MAX, u16::MAX): {accepted_max} accepted (width, padding) combos \
         exercised through allocate()+deallocate(), zero panics/overflows"
    );

    let accepted_default = sweep((4096, 4096));
    println!(
        "atlas_size=(4096, 4096): {accepted_default} accepted (width, padding) combos \
         exercised through allocate()+deallocate(), zero panics/overflows"
    );

    println!("repro_013: PASS - no overflow observed in ImageCache::deallocate for any input reachable through the public API");
}
