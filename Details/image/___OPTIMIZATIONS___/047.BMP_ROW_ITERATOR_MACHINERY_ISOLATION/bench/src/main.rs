// Follow-up to candidate 046, answering the exact "What's not done" item it left open and the
// top-of-backlog IDEAS.md entry it produced: candidate 046 found store+lookup+unpack (all
// deliberately branch-free, direct-pointer-store) still explains only 22.6-32.3% of real BMP
// decode time at 8/4/2/1-bit. The next never-yet-isolated component named there is the row/
// iterator *machinery* around the real `set_Nbit_pixel_run` functions
// (src/codecs/bmp/decoder.rs): `ChunksExactMut`, the `Option`-returning `pixel_iter.next()` calls
// (instead of a raw pointer write), and the outer per-row closure `with_rows_resumable` invokes
// once per scanline (instead of one flat loop over the whole image).
//
// This candidate extends candidate 046's harness (kept every prior function unchanged, re-measured
// this session per MEASUREMENT.md's paired-comparison discipline) with two new specialized
// variants, each isolating one more layer on top of `unpack_volatile_specialized` (candidate 046's
// direct-pointer floor):
//
//   - "unpack_volatile_specialized_chunksmut": identical extraction+lookup, but the store goes
//     through a real `std::slice::ChunksExactMut<u32>` (chunk size 1, standing in for the real
//     decoder's `row.chunks_exact_mut(num_channels)`) with an `Option`-returning `.next()` call
//     matched via `if let Some(pixel) = ... else { unreachable!() }` - the exact `Option`-handling
//     shape `set_8bit_pixel_run`/etc. use, just with the always-true branch instead of the real
//     "ran out of pixels" early return (which the harness's own row/density accounting makes
//     provably unreachable, kept as `unreachable!()` rather than deleted so the branch's presence
//     in codegen is still real, not optimized away by construction).
//   - "unpack_volatile_specialized_rowclosure": same as `_chunksmut`, but the whole per-pixel loop
//     is wrapped in a closure invoked once per "row" by a separate, `#[inline(never)]`
//     `with_rows_like` function - the same generic-`FnMut`-over-a-loop-of-slices shape as the real
//     decoder's `with_rows_resumable`, so a fresh `ChunksExactMut` is constructed per row (matching
//     `let mut pixel_iter = row.chunks_exact_mut(num_channels);` being called once per scanline)
//     rather than once for the whole image.
//
// Both new variants are still fully branch-free with respect to bounds checking (palette lookup
// and input-byte load both use `get_unchecked`) and use the same const-generic `DENSITY`
// specialization as candidate 046's `_specialized` variants, so the comparison isolates only the
// iterator-machinery/closure cost, not a dispatch-overhead or bounds-check confound.
//
// Row width per size regime is chosen to divide the buffer's pixel count evenly at every density
// (1/2/4/8) and to roughly match a real scanline width at that scale:
//   - "small": ROW_WIDTH = 440 (153,560 / 440 = 349 rows) - order of magnitude of
//     Core_8_Bit.bmp/Core_4_Bit.bmp's 435-pixel width.
//   - "large": ROW_WIDTH = 4032 (12,192,768 / 4032 = 3024 rows) - literally this workspace's usual
//     4032x3024 "realistic photo" scale, so row count and width both match a real image exactly.

use std::hint::black_box;
use std::time::Instant;

const SMALL_PIXELS: usize = 153_560; // ~= Core_8_Bit.bmp/Core_4_Bit.bmp (435x353), rounded up to a multiple of 8
const LARGE_PIXELS: usize = 4032 * 3024; // this workspace's usual "realistic photo" scale
const SMALL_ROW_WIDTH: usize = 440; // 153_560 / 440 = 349 rows, divisible by 1/2/4/8
const LARGE_ROW_WIDTH: usize = 4032; // 12_192_768 / 4032 = 3024 rows (this scenario's real width)
const DENSITIES: [usize; 4] = [1, 2, 4, 8];
const ROUNDS: usize = 30;

#[inline(never)]
unsafe fn fill_store_natural(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
    let mut val: u32 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            val = val.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    val
}

#[inline(never)]
unsafe fn fill_store_volatile(buf: *mut u32, total_pixels: usize, density: usize) -> u32 {
    let mut val: u32 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            val = val.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    val
}

#[inline(never)]
unsafe fn fill_lookup_natural(buf: *mut u32, total_pixels: usize, density: usize, palette: &[u32; 256]) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    idx as u32
}

#[inline(never)]
unsafe fn fill_lookup_volatile(buf: *mut u32, total_pixels: usize, density: usize, palette: &[u32; 256]) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < density {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += density;
    }
    idx as u32
}

/// Extract the `d`-th (0-based) sub-pixel palette index out of `byte`, for a given `density`,
/// using exactly the shift/mask patterns the real BMP decoder uses per bit depth
/// (src/codecs/bmp/decoder.rs `set_8bit_pixel_run`/`set_4bit_pixel_run`/`set_2bit_pixel_run`/
/// `set_1bit_pixel_run`).
#[inline(always)]
fn extract_index(byte: u8, density: usize, d: usize) -> u8 {
    match density {
        1 => byte,
        2 => {
            if d == 0 {
                byte >> 4
            } else {
                byte & 0xf
            }
        }
        4 => (byte >> (6 - 2 * d)) & 0x3,
        8 => (byte >> (7 - d)) & 0x1,
        _ => unreachable!(),
    }
}

#[inline(never)]
unsafe fn fill_unpack_natural(
    buf: *mut u32,
    total_pixels: usize,
    density: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < density {
            let idx = extract_index(byte, density, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += density;
        byte_idx += 1;
    }
    acc
}

#[inline(never)]
unsafe fn fill_unpack_volatile(
    buf: *mut u32,
    total_pixels: usize,
    density: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < density {
            let idx = extract_index(byte, density, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += density;
        byte_idx += 1;
    }
    acc
}

/// Same lookup/store as `fill_lookup_volatile`, but with `DENSITY` a compile-time const generic
/// instead of a runtime parameter - the fair baseline for `fill_unpack_volatile_specialized`
/// below, isolating the unpack arithmetic's own cost from the effect of specialization itself.
#[inline(never)]
unsafe fn fill_lookup_volatile_specialized<const DENSITY: usize>(
    buf: *mut u32,
    total_pixels: usize,
    palette: &[u32; 256],
) -> u32 {
    let mut idx: u8 = 0;
    let mut i = 0usize;
    while i < total_pixels {
        let mut d = 0usize;
        while d < DENSITY {
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            idx = idx.wrapping_add(1);
            d += 1;
        }
        i += DENSITY;
    }
    idx as u32
}

/// Same extraction/lookup/store as `fill_unpack_volatile`, but with `DENSITY` a compile-time
/// const generic instead of a runtime parameter, and storing via a raw pointer write - candidate
/// 046's direct-pointer floor, the baseline every new variant in this candidate is measured
/// against.
#[inline(never)]
unsafe fn fill_unpack_volatile_specialized<const DENSITY: usize>(
    buf: *mut u32,
    total_pixels: usize,
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < DENSITY {
            let idx = extract_index(byte, DENSITY, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            unsafe {
                std::ptr::write_volatile(buf.add(i + d), val);
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += DENSITY;
        byte_idx += 1;
    }
    acc
}

/// New this candidate. Same extraction/lookup as `fill_unpack_volatile_specialized`, but the
/// store goes through a real `std::slice::ChunksExactMut<u32>` (chunk size 1, standing in for the
/// real decoder's `row.chunks_exact_mut(num_channels)`) with an `Option`-returning `.next()` call,
/// matched the same way `set_8bit_pixel_run`/etc. match it (`if let Some(pixel) = pixel_iter.next()
/// { ... } else { <early exit> }`) - here the else-branch is `unreachable!()` since the harness's
/// own accounting guarantees the iterator never runs dry, but the branch itself is still emitted
/// and real, not deleted by construction. Isolates the iterator-machinery cost alone, still
/// flat-loop (no per-row closure) - see `_rowclosure` below for that additional layer.
#[inline(never)]
fn fill_unpack_volatile_specialized_chunksmut<const DENSITY: usize>(
    buf: &mut [u32],
    input: &[u8],
    palette: &[u32; 256],
) -> u32 {
    let total_pixels = buf.len();
    let mut pixel_iter = buf.chunks_exact_mut(1);
    let mut i = 0usize;
    let mut byte_idx = 0usize;
    let mut acc: u32 = 0;
    while i < total_pixels {
        let byte = unsafe { *input.get_unchecked(byte_idx) };
        let mut d = 0usize;
        while d < DENSITY {
            let idx = extract_index(byte, DENSITY, d);
            let val = unsafe { *palette.get_unchecked(idx as usize) };
            if let Some(pixel) = pixel_iter.next() {
                unsafe {
                    std::ptr::write_volatile(pixel.as_mut_ptr(), val);
                }
            } else {
                unreachable!();
            }
            acc = acc.wrapping_add(val);
            d += 1;
        }
        i += DENSITY;
        byte_idx += 1;
    }
    acc
}

/// The same shape as the real decoder's `with_rows_resumable`: a separate, non-inlined function
/// generic over `F: FnMut(&mut [u32]) -> u32`, slicing `buf` into `row_width`-sized rows and
/// invoking `func` once per row.
#[inline(never)]
fn with_rows_like<F>(buf: &mut [u32], row_width: usize, mut func: F) -> u32
where
    F: FnMut(&mut [u32]) -> u32,
{
    let n_rows = buf.len() / row_width;
    let mut acc = 0u32;
    for r in 0..n_rows {
        let row = &mut buf[r * row_width..][..row_width];
        acc = acc.wrapping_add(func(row));
    }
    acc
}

/// New this candidate. Same extraction/lookup/`ChunksExactMut`+`Option` store as
/// `fill_unpack_volatile_specialized_chunksmut`, but invoked once per "row" through
/// `with_rows_like`'s generic closure, instead of once for the whole flat buffer - matching
/// `read_palettized_pixel_data`'s real structure, where `let mut pixel_iter =
/// row.chunks_exact_mut(num_channels);` is constructed fresh inside the per-row closure
/// `with_rows_resumable` calls, not once for the whole image.
#[inline(never)]
fn fill_unpack_volatile_specialized_rowclosure<const DENSITY: usize>(
    buf: &mut [u32],
    input: &[u8],
    palette: &[u32; 256],
    row_width: usize,
) -> u32 {
    let bytes_per_row = row_width / DENSITY;
    let mut byte_offset = 0usize;
    with_rows_like(buf, row_width, |row| {
        let row_input = &input[byte_offset..][..bytes_per_row];
        byte_offset += bytes_per_row;
        let mut pixel_iter = row.chunks_exact_mut(1);
        let mut local_acc = 0u32;
        for &byte in row_input {
            let mut d = 0usize;
            while d < DENSITY {
                let idx = extract_index(byte, DENSITY, d);
                let val = unsafe { *palette.get_unchecked(idx as usize) };
                if let Some(pixel) = pixel_iter.next() {
                    unsafe {
                        std::ptr::write_volatile(pixel.as_mut_ptr(), val);
                    }
                } else {
                    unreachable!();
                }
                local_acc = local_acc.wrapping_add(val);
                d += 1;
            }
        }
        local_acc
    })
}

#[derive(Clone, Copy)]
enum Variant {
    StoreNatural,
    StoreVolatile,
    LookupNatural,
    LookupVolatile,
    UnpackNatural,
    UnpackVolatile,
    LookupVolatileSpecialized,
    UnpackVolatileSpecialized,
    UnpackVolatileSpecializedChunksMut,
    UnpackVolatileSpecializedRowClosure,
}

struct Config {
    size_name: &'static str,
    total_pixels: usize,
    row_width: usize,
    density: usize,
    variant: Variant,
    variant_name: &'static str,
}

/// Reference (scalar, obviously-correct) sub-pixel index extraction, independent of
/// `extract_index`/the unpack loops - used only by the verify_* functions below.
fn reference_extract(byte: u8, density: usize, d: usize) -> u8 {
    match density {
        1 => byte,
        2 => {
            if d == 0 {
                (byte & 0xf0) >> 4
            } else {
                byte & 0x0f
            }
        }
        4 => {
            let shift = 6 - 2 * d;
            (byte >> shift) & 0x3
        }
        8 => {
            let bit = 0x80u8 >> d;
            u8::from((byte & bit) != 0)
        }
        _ => unreachable!(),
    }
}

/// Run `fill_unpack_natural`/`fill_unpack_volatile`/`fill_unpack_volatile_specialized` on a small
/// buffer for every density and compare every output pixel against a from-scratch reference
/// extraction - not just a checksum. Unchanged from candidate 046.
fn verify_unpack_correctness(palette: &[u32; 256]) {
    for &density in &DENSITIES {
        let n_bytes = 97; // arbitrary, not a multiple of anything, to catch off-by-one tail bugs
        let total_pixels = n_bytes * density;
        let input: Vec<u8> = (0..n_bytes).map(|i| (i as u32).wrapping_mul(2654435761) as u8).collect();
        let mut expected = vec![0u32; total_pixels];
        for (byte_idx, &byte) in input.iter().enumerate() {
            for d in 0..density {
                let idx = reference_extract(byte, density, d);
                let idx2 = extract_index(byte, density, d);
                assert_eq!(
                    idx, idx2,
                    "extract_index disagrees with reference at density={density} d={d} byte={byte:#x}"
                );
                expected[byte_idx * density + d] = palette[idx as usize];
            }
        }

        let mut buf_natural = vec![0xFFFF_FFFFu32; total_pixels];
        let mut buf_volatile = vec![0xFFFF_FFFFu32; total_pixels];
        unsafe {
            fill_unpack_natural(buf_natural.as_mut_ptr(), total_pixels, density, &input, palette);
            fill_unpack_volatile(buf_volatile.as_mut_ptr(), total_pixels, density, &input, palette);
        }
        assert_eq!(
            buf_natural, expected,
            "fill_unpack_natural produced wrong pixels at density={density}"
        );
        assert_eq!(
            buf_volatile, expected,
            "fill_unpack_volatile produced wrong pixels at density={density}"
        );

        let mut buf_specialized = vec![0xFFFF_FFFFu32; total_pixels];
        unsafe {
            match density {
                1 => fill_unpack_volatile_specialized::<1>(buf_specialized.as_mut_ptr(), total_pixels, &input, palette),
                2 => fill_unpack_volatile_specialized::<2>(buf_specialized.as_mut_ptr(), total_pixels, &input, palette),
                4 => fill_unpack_volatile_specialized::<4>(buf_specialized.as_mut_ptr(), total_pixels, &input, palette),
                8 => fill_unpack_volatile_specialized::<8>(buf_specialized.as_mut_ptr(), total_pixels, &input, palette),
                _ => unreachable!(),
            }
        };
        assert_eq!(
            buf_specialized, expected,
            "fill_unpack_volatile_specialized produced wrong pixels at density={density}"
        );
    }
    eprintln!("verify_unpack_correctness: OK (all 4 densities, per-pixel exact match against reference)");
}

/// New this candidate. Verifies `fill_unpack_volatile_specialized_chunksmut` and
/// `fill_unpack_volatile_specialized_rowclosure` produce exactly the same per-pixel output as the
/// same from-scratch reference extraction used above, for all 4 densities, including the
/// row-splitting accounting the `_rowclosure` variant adds (multiple rows, not just one).
fn verify_iterator_variants_correctness(palette: &[u32; 256]) {
    for &density in &DENSITIES {
        let bytes_per_row = 32; // arbitrary, divisible into every density's sub-pixel count
        let n_rows = 3;
        let row_width = bytes_per_row * density;
        let n_bytes = bytes_per_row * n_rows;
        let total_pixels = n_bytes * density;
        let input: Vec<u8> = (0..n_bytes).map(|i| (i as u32).wrapping_mul(2654435761).wrapping_add(0x1234) as u8).collect();
        let mut expected = vec![0u32; total_pixels];
        for (byte_idx, &byte) in input.iter().enumerate() {
            for d in 0..density {
                let idx = reference_extract(byte, density, d);
                expected[byte_idx * density + d] = palette[idx as usize];
            }
        }

        let mut buf_chunksmut = vec![0xFFFF_FFFFu32; total_pixels];
        match density {
            1 => fill_unpack_volatile_specialized_chunksmut::<1>(&mut buf_chunksmut, &input, palette),
            2 => fill_unpack_volatile_specialized_chunksmut::<2>(&mut buf_chunksmut, &input, palette),
            4 => fill_unpack_volatile_specialized_chunksmut::<4>(&mut buf_chunksmut, &input, palette),
            8 => fill_unpack_volatile_specialized_chunksmut::<8>(&mut buf_chunksmut, &input, palette),
            _ => unreachable!(),
        };
        assert_eq!(
            buf_chunksmut, expected,
            "fill_unpack_volatile_specialized_chunksmut produced wrong pixels at density={density}"
        );

        let mut buf_rowclosure = vec![0xFFFF_FFFFu32; total_pixels];
        match density {
            1 => fill_unpack_volatile_specialized_rowclosure::<1>(&mut buf_rowclosure, &input, palette, row_width),
            2 => fill_unpack_volatile_specialized_rowclosure::<2>(&mut buf_rowclosure, &input, palette, row_width),
            4 => fill_unpack_volatile_specialized_rowclosure::<4>(&mut buf_rowclosure, &input, palette, row_width),
            8 => fill_unpack_volatile_specialized_rowclosure::<8>(&mut buf_rowclosure, &input, palette, row_width),
            _ => unreachable!(),
        };
        assert_eq!(
            buf_rowclosure, expected,
            "fill_unpack_volatile_specialized_rowclosure produced wrong pixels at density={density}"
        );
    }
    eprintln!(
        "verify_iterator_variants_correctness: OK (chunksmut + rowclosure, all 4 densities, per-pixel exact match)"
    );
}

fn main() {
    // Fixed 256-entry palette, non-trivial values so the compiler can't fold the lookup into a
    // constant - still fits comfortably in L1 (1 KiB), so no cache-miss cost is intended here.
    let mut palette = [0u32; 256];
    for (i, p) in palette.iter_mut().enumerate() {
        *p = (i as u32).wrapping_mul(0x9E3779B1) ^ 0xDEADBEEF;
    }
    let palette = black_box(palette);

    verify_unpack_correctness(&palette);
    verify_iterator_variants_correctness(&palette);

    let mut configs = Vec::new();
    for &(size_name, total_pixels, row_width) in &[
        ("small", SMALL_PIXELS, SMALL_ROW_WIDTH),
        ("large", LARGE_PIXELS, LARGE_ROW_WIDTH),
    ] {
        for &density in &DENSITIES {
            for &(variant, variant_name) in &[
                (Variant::StoreNatural, "store_natural"),
                (Variant::StoreVolatile, "store_volatile"),
                (Variant::LookupNatural, "lookup_natural"),
                (Variant::LookupVolatile, "lookup_volatile"),
                (Variant::UnpackNatural, "unpack_natural"),
                (Variant::UnpackVolatile, "unpack_volatile"),
                (Variant::LookupVolatileSpecialized, "lookup_volatile_specialized"),
                (Variant::UnpackVolatileSpecialized, "unpack_volatile_specialized"),
                (
                    Variant::UnpackVolatileSpecializedChunksMut,
                    "unpack_volatile_specialized_chunksmut",
                ),
                (
                    Variant::UnpackVolatileSpecializedRowClosure,
                    "unpack_volatile_specialized_rowclosure",
                ),
            ] {
                assert_eq!(total_pixels % density, 0, "pixel count must divide evenly by density");
                assert_eq!(total_pixels % row_width, 0, "pixel count must divide evenly by row_width");
                assert_eq!(row_width % density, 0, "row_width must divide evenly by density");
                configs.push(Config {
                    size_name,
                    total_pixels,
                    row_width,
                    density,
                    variant,
                    variant_name,
                });
            }
        }
    }

    // One persistent buffer per distinct size, reused (and re-touched every round) across all
    // configs of that size so paging/first-touch cost isn't part of the timed region.
    let mut small_buf = vec![0u32; SMALL_PIXELS];
    let mut large_buf = vec![0u32; LARGE_PIXELS];

    // Input-byte buffers for the "unpack" variants, one per size, sized generously enough
    // (total_pixels bytes, i.e. density=1's worst case) to cover every density's byte count.
    let mut small_input = vec![0u8; SMALL_PIXELS];
    for (i, b) in small_input.iter_mut().enumerate() {
        *b = (i as u32).wrapping_mul(2654435761) as u8;
    }
    let small_input = black_box(small_input);
    let mut large_input = vec![0u8; LARGE_PIXELS];
    for (i, b) in large_input.iter_mut().enumerate() {
        *b = (i as u32).wrapping_mul(2654435761) as u8;
    }
    let large_input = black_box(large_input);

    // times_ns[config_index][round]
    let mut times_ns: Vec<Vec<u64>> = vec![Vec::with_capacity(ROUNDS); configs.len()];
    let mut checksum: u64 = 0;

    for _round in 0..ROUNDS {
        for (ci, cfg) in configs.iter().enumerate() {
            let buf: &mut [u32] = match cfg.size_name {
                "small" => &mut small_buf,
                "large" => &mut large_buf,
                _ => unreachable!(),
            };
            let input: &[u8] = match cfg.size_name {
                "small" => &small_input,
                "large" => &large_input,
                _ => unreachable!(),
            };
            let total_pixels = black_box(cfg.total_pixels);
            let density = black_box(cfg.density);
            let row_width = black_box(cfg.row_width);

            let t0 = Instant::now();
            let ret = match cfg.variant {
                Variant::StoreNatural => unsafe {
                    fill_store_natural(black_box(buf.as_mut_ptr()), total_pixels, density)
                },
                Variant::StoreVolatile => unsafe {
                    fill_store_volatile(black_box(buf.as_mut_ptr()), total_pixels, density)
                },
                Variant::LookupNatural => unsafe {
                    fill_lookup_natural(black_box(buf.as_mut_ptr()), total_pixels, density, &palette)
                },
                Variant::LookupVolatile => unsafe {
                    fill_lookup_volatile(black_box(buf.as_mut_ptr()), total_pixels, density, &palette)
                },
                Variant::UnpackNatural => unsafe {
                    fill_unpack_natural(black_box(buf.as_mut_ptr()), total_pixels, density, input, &palette)
                },
                Variant::UnpackVolatile => unsafe {
                    fill_unpack_volatile(black_box(buf.as_mut_ptr()), total_pixels, density, input, &palette)
                },
                Variant::UnpackVolatileSpecialized => unsafe {
                    let ptr = black_box(buf.as_mut_ptr());
                    match density {
                        1 => fill_unpack_volatile_specialized::<1>(ptr, total_pixels, input, &palette),
                        2 => fill_unpack_volatile_specialized::<2>(ptr, total_pixels, input, &palette),
                        4 => fill_unpack_volatile_specialized::<4>(ptr, total_pixels, input, &palette),
                        8 => fill_unpack_volatile_specialized::<8>(ptr, total_pixels, input, &palette),
                        _ => unreachable!(),
                    }
                },
                Variant::LookupVolatileSpecialized => unsafe {
                    let ptr = black_box(buf.as_mut_ptr());
                    match density {
                        1 => fill_lookup_volatile_specialized::<1>(ptr, total_pixels, &palette),
                        2 => fill_lookup_volatile_specialized::<2>(ptr, total_pixels, &palette),
                        4 => fill_lookup_volatile_specialized::<4>(ptr, total_pixels, &palette),
                        8 => fill_lookup_volatile_specialized::<8>(ptr, total_pixels, &palette),
                        _ => unreachable!(),
                    }
                },
                Variant::UnpackVolatileSpecializedChunksMut => {
                    let buf_ref = black_box(&mut buf[..total_pixels]);
                    match density {
                        1 => fill_unpack_volatile_specialized_chunksmut::<1>(buf_ref, input, &palette),
                        2 => fill_unpack_volatile_specialized_chunksmut::<2>(buf_ref, input, &palette),
                        4 => fill_unpack_volatile_specialized_chunksmut::<4>(buf_ref, input, &palette),
                        8 => fill_unpack_volatile_specialized_chunksmut::<8>(buf_ref, input, &palette),
                        _ => unreachable!(),
                    }
                }
                Variant::UnpackVolatileSpecializedRowClosure => {
                    let buf_ref = black_box(&mut buf[..total_pixels]);
                    match density {
                        1 => fill_unpack_volatile_specialized_rowclosure::<1>(buf_ref, input, &palette, row_width),
                        2 => fill_unpack_volatile_specialized_rowclosure::<2>(buf_ref, input, &palette, row_width),
                        4 => fill_unpack_volatile_specialized_rowclosure::<4>(buf_ref, input, &palette, row_width),
                        8 => fill_unpack_volatile_specialized_rowclosure::<8>(buf_ref, input, &palette, row_width),
                        _ => unreachable!(),
                    }
                }
            };
            let ns = t0.elapsed().as_nanos() as u64;

            checksum = checksum.wrapping_add(black_box(ret) as u64);
            times_ns[ci].push(ns);
        }
    }

    println!("checksum(ignore) = {checksum:#x}");
    println!("rounds = {ROUNDS} (warmup discarded = {})", (ROUNDS / 10).max(1));
    println!();
    println!(
        "{:<7} {:>8} {:>38} {:>10} {:>10} {:>10} {:>10} {:>12} {:>10}",
        "size", "density", "variant", "min_ns", "p50_ns", "mean_ns", "max_ns", "GB/s(p50)", "ns/px(p50)"
    );

    for (ci, cfg) in configs.iter().enumerate() {
        let warmup = (ROUNDS / 10).max(1).min(ROUNDS - 1);
        let mut stable = times_ns[ci][warmup..].to_vec();
        stable.sort_unstable();
        let n = stable.len();
        let min = stable[0];
        let p50 = stable[n / 2];
        let mean = stable.iter().sum::<u64>() / n as u64;
        let max = stable[n - 1];
        let total_bytes = cfg.total_pixels * 4;
        let gbps = (total_bytes as f64) / (p50 as f64); // bytes/ns = GB/s
        let ns_per_px = (p50 as f64) / (cfg.total_pixels as f64);

        println!(
            "{:<7} {:>8} {:>38} {:>10} {:>10} {:>10} {:>10} {:>12.3} {:>10.4}",
            cfg.size_name, cfg.density, cfg.variant_name, min, p50, mean, max, gbps, ns_per_px
        );
    }

    println!();
    print!("raw_ns = {{");
    for (ci, cfg) in configs.iter().enumerate() {
        print!(
            "\"{}x{}x{}\": {:?}, ",
            cfg.size_name, cfg.density, cfg.variant_name, times_ns[ci]
        );
    }
    println!("}}");
}
