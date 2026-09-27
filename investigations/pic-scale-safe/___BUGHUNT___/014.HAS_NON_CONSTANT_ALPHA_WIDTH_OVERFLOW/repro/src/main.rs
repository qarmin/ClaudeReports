// Standalone reproduction: `has_non_constant_alpha`/`has_non_constant_alpha_f32_impl`
// (src/alpha_check.rs, backing all 6 public `has_non_constant_alpha_*` functions) compute the
// per-row chunk size as `width * CHANNELS` using plain (non-checked) usize multiplication before
// passing it to `store.chunks_exact(...)`.
//
// `width` is a caller-supplied parameter independent of `store.len()` - it does not have to match
// the slice's actual size for the multiplication to run. A large `width` (close to `usize::MAX`)
// overflows `width * CHANNELS`:
//   - In a debug build (overflow-checks on, Rust's dev-profile default): panics immediately with
//     "attempt to multiply with overflow", *before* any per-pixel scanning happens.
//   - In a release build (overflow-checks off): the multiplication silently wraps mod 2^64. For a
//     `width` chosen so the wrapped result is exactly 0, `chunks_exact` then panics with its own
//     "chunk size must be non-zero" (the same panic candidate 002 already documented, but reached
//     via arithmetic wraparound here instead of `width == 0` directly). For other `width` values
//     the wrapped result can also land on some small nonzero chunk size unrelated to the real
//     `width`, which would silently scan the store with the wrong row length instead of crashing -
//     not exercised by a crash-producing case here, but the same wrap is the root cause.
//
// This never causes out-of-bounds memory access - `chunks_exact` only ever reads within the
// slice's own bounds, wrapped chunk size or not. This is a panic / silent-wrong-result bug, not
// a memory-safety bug.
//
// Run as a DEBUG build (`cargo run`, not `--release`) to see the "attempt to multiply with
// overflow" panic for cases 1/2. Case 3 (wrap-to-exactly-zero) panics in *both* debug and release,
// just with a different message in each ("attempt to multiply with overflow" in debug,
// "chunk size must be non-zero" in release), since the wrap only depends on the runtime value of
// `width * CHANNELS`, not on overflow-checks being enabled.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

fn main() {
    use pic_scale_safe::{has_non_constant_alpha_la8, has_non_constant_alpha_rgba8, has_non_constant_alpha_rgba_f32};

    let mut reproduced = false;

    // Case 1: width large enough that width * CHANNELS (CHANNELS=4 for rgba8) overflows usize
    // outright. The store itself is tiny (4 bytes) - width is independent of store.len().
    {
        let store = [0u8; 4];
        let width = usize::MAX / 2;
        let result = std::panic::catch_unwind(|| has_non_constant_alpha_rgba8(&store, width));
        match result {
            Err(_) => {
                println!("case1 (has_non_constant_alpha_rgba8, width=usize::MAX/2): PANICKED (multiply overflow)");
                reproduced = true;
            }
            Ok(v) => println!("case1 (has_non_constant_alpha_rgba8, width=usize::MAX/2): did not panic, returned {v}"),
        }
    }

    // Case 2: same shape, f32 impl (separate function, same bug).
    {
        let store = [0f32; 4];
        let width = usize::MAX / 2;
        let result = std::panic::catch_unwind(|| has_non_constant_alpha_rgba_f32(&store, width));
        match result {
            Err(_) => {
                println!("case2 (has_non_constant_alpha_rgba_f32, width=usize::MAX/2): PANICKED (multiply overflow)");
                reproduced = true;
            }
            Ok(v) => println!("case2 (has_non_constant_alpha_rgba_f32, width=usize::MAX/2): did not panic, returned {v}"),
        }
    }

    // Case 3: width chosen so width * CHANNELS wraps to exactly 0 mod 2^64 (CHANNELS=4). Panics
    // in both debug (overflow panic) and release (chunk-size-zero panic) builds.
    {
        let store = [1u8, 2, 3, 4];
        let width = usize::MAX / 4 + 1; // width * 4 wraps to 0
        let result = std::panic::catch_unwind(|| has_non_constant_alpha_rgba8(&store, width));
        match result {
            Err(_) => {
                println!("case3 (has_non_constant_alpha_rgba8, width wraps row_len to 0): PANICKED");
                reproduced = true;
            }
            Ok(v) => println!("case3 (has_non_constant_alpha_rgba8, width wraps row_len to 0): did not panic, returned {v}"),
        }
    }

    // Sanity check: an ordinary well-formed, non-constant-alpha input must still return true and
    // must be unaffected by any fix.
    {
        let store = [10u8, 200, 10u8, 50]; // 2 LA8 pixels, alpha differs (200 vs 50)
        let v = has_non_constant_alpha_la8(&store, 2);
        if !v {
            println!("sanity (has_non_constant_alpha_la8, width=2, varying alpha): UNEXPECTED false");
            reproduced = true;
        } else {
            println!("sanity (has_non_constant_alpha_la8, width=2, varying alpha): ok, detected non-constant alpha");
        }
    }

    if reproduced {
        eprintln!(
            "REPRODUCED - width * CHANNELS overflow/wraparound in has_non_constant_alpha/has_non_constant_alpha_f32_impl"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - all overflow/wrap boundary inputs returned gracefully, and the well-formed sanity input still returns the correct answer");
    }
}
