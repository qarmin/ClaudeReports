// Dynamic verification of src/arch/neon.rs's unsafe NEON IDCT/color-convert kernels, run for
// real on aarch64 (cross-compiled statically for aarch64-unknown-linux-musl, executed under
// qemu-aarch64 user-mode emulation - genuine aarch64 NEON instruction execution, not miri
// interpretation, which cannot model these intrinsics at all - see candidate 013's RESULT.md).
//
// Two independent checks run against every real fixture JPEG embedded below (copied verbatim
// from this project's own tests/reftest/images/ corpus - not hand-authored bytes):
//
//   1. Correctness: decode through the public `jpeg_decoder::Decoder` API and compare against the
//      project's own PNG reference images, using the exact same tolerance rule
//      tests/reftest/mod.rs's `reftest_decoder` uses (diff <= 3 per channel for lossy coding,
//      exact match for lossless).
//   2. Memory safety: every heap allocation is served by a guard-page allocator (mmap two
//      PROT_NONE pages around each allocation's page-rounded region, place the allocation flush
//      against the trailing guard page) so any single-byte out-of-bounds heap write from the
//      unsafe NEON kernels (`vst1_u8`/`vst3_u8` writing through a raw pointer derived from a
//      slice) turns into an immediate SIGSEGV instead of silently landing in adjacent heap
//      memory - a real page-fault-based memory-safety detector, standing in for ASan/miri which
//      are both unavailable for this target (see RESULT.md's "What was tried").

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct GuardPageAlloc;

const PAGE_SIZE: usize = 4096;

unsafe impl GlobalAlloc for GuardPageAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let user_size = layout.size();
        if user_size == 0 {
            return System.alloc(layout);
        }
        // Round the usable region up to a whole number of pages, and place the user's data
        // flush against the end of that region so an overflow write of even 1 byte lands on
        // the trailing PROT_NONE guard page.
        let usable_pages = (user_size + PAGE_SIZE - 1) / PAGE_SIZE;
        let total_len = (usable_pages + 2) * PAGE_SIZE; // leading guard + usable + trailing guard

        let base = libc::mmap(
            std::ptr::null_mut(),
            total_len,
            libc::PROT_NONE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if base == libc::MAP_FAILED {
            return std::ptr::null_mut();
        }
        let usable_start = (base as *mut u8).add(PAGE_SIZE);
        if libc::mprotect(
            usable_start as *mut libc::c_void,
            usable_pages * PAGE_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
        ) != 0
        {
            libc::munmap(base, total_len);
            return std::ptr::null_mut();
        }

        let user_ptr = usable_start.add(usable_pages * PAGE_SIZE - user_size);
        // layout.align() is always a power of two <= PAGE_SIZE for every allocation this program
        // makes (Vec<u8>/Vec<u16>/String backing storage) - assert it rather than silently
        // returning a misaligned pointer.
        assert!(layout.align() <= PAGE_SIZE);
        assert_eq!((user_ptr as usize) % layout.align(), 0, "guard allocator cannot satisfy this alignment");
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        user_ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let user_size = layout.size();
        if user_size == 0 {
            return System.dealloc(ptr, layout);
        }
        let usable_pages = (user_size + PAGE_SIZE - 1) / PAGE_SIZE;
        let total_len = (usable_pages + 2) * PAGE_SIZE;
        // usable_start is page-aligned and user_ptr is always within [usable_start,
        // usable_start + PAGE_SIZE) (the sub-page padding computed in `alloc`), so rounding
        // `ptr` down to the containing page recovers usable_start exactly.
        let usable_start = (ptr as usize) & !(PAGE_SIZE - 1);
        let base = (usable_start - PAGE_SIZE) as *mut libc::c_void;
        libc::munmap(base, total_len);
    }
}

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static GLOBAL: GuardPageAlloc = GuardPageAlloc;

struct Fixture {
    name: &'static str,
    jpg: &'static [u8],
    png: &'static [u8],
    // Some((width, height)) exercises `Decoder::scale`, which recomputes output_linestride and
    // per-block buffer offsets differently than an unscaled decode - a distinct boundary-math
    // path through the same NEON `assert!`-guarded kernels (see RESULT.md).
    scale: Option<(u16, u16)>,
}

macro_rules! fixture {
    ($name:expr, $jpg:expr, $png:expr) => {
        Fixture {
            name: $name,
            jpg: include_bytes!($jpg),
            png: include_bytes!($png),
            scale: None,
        }
    };
    ($name:expr, $jpg:expr, $png:expr, scale = ($w:expr, $h:expr)) => {
        Fixture {
            name: $name,
            jpg: include_bytes!($jpg),
            png: include_bytes!($png),
            scale: Some(($w, $h)),
        }
    };
}

fn fixtures() -> Vec<Fixture> {
    vec![
        fixture!("16bit-qtables", "../fixtures/16bit-qtables.jpg", "../fixtures/16bit-qtables.png"),
        fixture!("blank_800x280", "../fixtures/blank_800x280.jpg", "../fixtures/blank_800x280.png"),
        fixture!("extraneous-data", "../fixtures/extraneous-data.jpg", "../fixtures/extraneous-data.png"),
        fixture!("grayscale_16x24_sampling2x2", "../fixtures/grayscale_16x24_sampling2x2.jpg", "../fixtures/grayscale_16x24_sampling2x2.png"),
        fixture!("grayscale_24x16_sampling2x2", "../fixtures/grayscale_24x16_sampling2x2.jpg", "../fixtures/grayscale_24x16_sampling2x2.png"),
        fixture!("grayscale_large", "../fixtures/grayscale_large.jpg", "../fixtures/grayscale_large.png"),
        fixture!("grayscale_long", "../fixtures/grayscale_long.jpg", "../fixtures/grayscale_long.png"),
        fixture!("grayscale_square", "../fixtures/grayscale_square.jpg", "../fixtures/grayscale_square.png"),
        fixture!("mjpeg", "../fixtures/mjpeg.jpg", "../fixtures/mjpeg.png"),
        fixture!("non-interleaved-mcu", "../fixtures/non-interleaved-mcu.jpg", "../fixtures/non-interleaved-mcu.png"),
        fixture!("partial_progressive", "../fixtures/partial_progressive.jpg", "../fixtures/partial_progressive.png"),
        fixture!("progressive-missing-ac", "../fixtures/progressive-missing-ac.jpg", "../fixtures/progressive-missing-ac.png"),
        fixture!("progressive-missing-dc", "../fixtures/progressive-missing-dc.jpg", "../fixtures/progressive-missing-dc.png"),
        fixture!("progressive3", "../fixtures/progressive3.jpg", "../fixtures/progressive3.png"),
        fixture!("restarts", "../fixtures/restarts.jpg", "../fixtures/restarts.png"),
        fixture!("rgb", "../fixtures/rgb.jpg", "../fixtures/rgb.png"),
        fixture!("mozilla/jpg-size-1x1", "../fixtures/mozilla/jpg-size-1x1.jpg", "../fixtures/mozilla/jpg-size-1x1.png"),
        fixture!("mozilla/jpg-size-2x2", "../fixtures/mozilla/jpg-size-2x2.jpg", "../fixtures/mozilla/jpg-size-2x2.png"),
        fixture!("mozilla/jpg-size-5x5", "../fixtures/mozilla/jpg-size-5x5.jpg", "../fixtures/mozilla/jpg-size-5x5.png"),
        fixture!("mozilla/jpg-size-7x7", "../fixtures/mozilla/jpg-size-7x7.jpg", "../fixtures/mozilla/jpg-size-7x7.png"),
        fixture!("mozilla/jpg-size-8x8", "../fixtures/mozilla/jpg-size-8x8.jpg", "../fixtures/mozilla/jpg-size-8x8.png"),
        fixture!("mozilla/jpg-size-9x9", "../fixtures/mozilla/jpg-size-9x9.jpg", "../fixtures/mozilla/jpg-size-9x9.png"),
        fixture!("mozilla/jpg-size-15x15", "../fixtures/mozilla/jpg-size-15x15.jpg", "../fixtures/mozilla/jpg-size-15x15.png"),
        fixture!("mozilla/jpg-size-16x16", "../fixtures/mozilla/jpg-size-16x16.jpg", "../fixtures/mozilla/jpg-size-16x16.png"),
        fixture!("mozilla/jpg-size-17x17", "../fixtures/mozilla/jpg-size-17x17.jpg", "../fixtures/mozilla/jpg-size-17x17.png"),
        fixture!("mozilla/jpg-size-31x31", "../fixtures/mozilla/jpg-size-31x31.jpg", "../fixtures/mozilla/jpg-size-31x31.png"),
        fixture!("mozilla/jpg-size-32x32", "../fixtures/mozilla/jpg-size-32x32.jpg", "../fixtures/mozilla/jpg-size-32x32.png"),
        fixture!("mozilla/jpg-size-33x33", "../fixtures/mozilla/jpg-size-33x33.jpg", "../fixtures/mozilla/jpg-size-33x33.png"),
        fixture!("mozilla/jpg-cmyk-1", "../fixtures/mozilla/jpg-cmyk-1.jpg", "../fixtures/mozilla/jpg-cmyk-1.png"),
        fixture!("mozilla/jpg-cmyk-2", "../fixtures/mozilla/jpg-cmyk-2.jpg", "../fixtures/mozilla/jpg-cmyk-2.png"),
        fixture!("mozilla/jpg-gray", "../fixtures/mozilla/jpg-gray.jpg", "../fixtures/mozilla/jpg-gray.png"),
        fixture!("mozilla/jpg-progressive", "../fixtures/mozilla/jpg-progressive.jpg", "../fixtures/mozilla/jpg-progressive.png"),
        fixture!("mozilla/jpg-srgb-icc", "../fixtures/mozilla/jpg-srgb-icc.jpg", "../fixtures/mozilla/jpg-srgb-icc.png"),
        fixture!("rgb@250x167", "../fixtures/rgb.jpg", "../fixtures/rgb_250x167.png", scale = (250, 167)),
        fixture!("rgb@125x84", "../fixtures/rgb.jpg", "../fixtures/rgb_125x84.png", scale = (125, 84)),
        fixture!("rgb@63x42", "../fixtures/rgb.jpg", "../fixtures/rgb_63x42.png", scale = (63, 42)),
    ]
}

fn cmyk_to_rgb(input: &[u8]) -> Vec<u8> {
    let size = input.len() - input.len() / 4;
    let mut output = Vec::with_capacity(size);
    for pixel in input.chunks(4) {
        let c = pixel[0] as f32 / 255.0;
        let m = pixel[1] as f32 / 255.0;
        let y = pixel[2] as f32 / 255.0;
        let k = pixel[3] as f32 / 255.0;
        let c = c * (1.0 - k) + k;
        let m = m * (1.0 - k) + k;
        let y = y * (1.0 - k) + k;
        output.push(((1.0 - c) * 255.0) as u8);
        output.push(((1.0 - m) * 255.0) as u8);
        output.push(((1.0 - y) * 255.0) as u8);
    }
    output
}

fn rgba_to_rgb(input: &[u8]) -> Vec<u8> {
    let size = input.len() - input.len() / 4;
    let mut output = Vec::with_capacity(size);
    for pixel in input.chunks(4) {
        output.push(pixel[0]);
        output.push(pixel[1]);
        output.push(pixel[2]);
    }
    output
}

fn check_fixture(f: &Fixture) -> Result<(), String> {
    let mut decoder = jpeg_decoder::Decoder::new(f.jpg);
    if let Some((w, h)) = f.scale {
        decoder
            .read_info()
            .map_err(|e| format!("jpeg read_info failed: {e:?}"))?;
        decoder
            .scale(w, h)
            .map_err(|e| format!("jpeg scale failed: {e:?}"))?;
    }
    let mut data = decoder
        .decode()
        .map_err(|e| format!("jpeg decode failed: {e:?}"))?;
    let info = decoder.info().unwrap();
    let mut pixel_format = info.pixel_format;

    if pixel_format == jpeg_decoder::PixelFormat::CMYK32 {
        data = cmyk_to_rgb(&data);
        pixel_format = jpeg_decoder::PixelFormat::RGB24;
    }

    let mut png_decoder = png::Decoder::new(f.png);
    // png 0.17 defaults to IDENTITY (no transformation); png 0.16 (used by this project's own
    // dev-dependency reftest) defaulted to EXPAND+STRIP_16. Match that old default everywhere
    // except for L16 fixtures, where the original test explicitly wants genuine 16-bit output.
    if pixel_format == jpeg_decoder::PixelFormat::L16 {
        png_decoder.set_transformations(png::Transformations::EXPAND);
    } else {
        png_decoder.set_transformations(png::Transformations::normalize_to_color8());
    }
    let mut reader = png_decoder
        .read_info()
        .map_err(|e| format!("png read_info failed: {e:?}"))?;
    let ref_info = reader.info().clone();

    if ref_info.width != info.width as u32 || ref_info.height != info.height as u32 {
        return Err(format!(
            "size mismatch: jpeg {}x{} vs png {}x{}",
            info.width, info.height, ref_info.width, ref_info.height
        ));
    }

    let mut ref_data = vec![0u8; reader.output_buffer_size()];
    reader
        .next_frame(&mut ref_data)
        .map_err(|e| format!("png decode failed: {e:?}"))?;
    let (mut ref_pixel_format, ref_bit_depth) = reader.output_color_type();

    if ref_pixel_format == png::ColorType::Rgba {
        ref_data = rgba_to_rgb(&ref_data);
        ref_pixel_format = png::ColorType::Rgb;
    }

    let (ref16, data16): (Vec<u16>, Vec<u16>) = match pixel_format {
        jpeg_decoder::PixelFormat::L8 => {
            if ref_pixel_format != png::ColorType::Grayscale || ref_bit_depth != png::BitDepth::Eight {
                return Err(format!(
                    "format mismatch for L8: png gave {ref_pixel_format:?}/{ref_bit_depth:?}"
                ));
            }
            (
                ref_data.iter().map(|x| *x as u16).collect(),
                data.iter().map(|x| *x as u16).collect(),
            )
        }
        jpeg_decoder::PixelFormat::L16 => {
            if ref_pixel_format != png::ColorType::Grayscale || ref_bit_depth != png::BitDepth::Sixteen {
                return Err(format!(
                    "format mismatch for L16: png gave {ref_pixel_format:?}/{ref_bit_depth:?}"
                ));
            }
            (
                ref_data
                    .chunks_exact(2)
                    .map(|a| u16::from_be_bytes([a[0], a[1]]))
                    .collect(),
                data.chunks_exact(2)
                    .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                    .collect(),
            )
        }
        jpeg_decoder::PixelFormat::RGB24 => {
            if ref_pixel_format != png::ColorType::Rgb || ref_bit_depth != png::BitDepth::Eight {
                return Err(format!(
                    "format mismatch for RGB24: png gave {ref_pixel_format:?}/{ref_bit_depth:?}"
                ));
            }
            (
                ref_data.iter().map(|x| *x as u16).collect(),
                data.iter().map(|x| *x as u16).collect(),
            )
        }
        other => return Err(format!("unexpected pixel format {other:?}")),
    };

    if data16.len() != ref16.len() {
        return Err(format!("length mismatch: {} vs {}", data16.len(), ref16.len()));
    }

    let mut max_diff = 0i32;
    let tolerance = if info.coding_process == jpeg_decoder::CodingProcess::Lossless { 0 } else { 3 };
    for (&a, &b) in data16.iter().zip(ref16.iter()) {
        let diff = (a as i32 - b as i32).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }
    if max_diff > tolerance {
        return Err(format!("pixel mismatch: max_diff={max_diff} (tolerance={tolerance})"));
    }

    Ok(())
}

fn main() {
    println!("target arch: {}", std::env::consts::ARCH);
    #[cfg(all(target_arch = "aarch64", feature = "nightly_aarch64_neon"))]
    println!("NEON arch path is compiled in and unconditionally dispatched on aarch64");
    #[cfg(not(all(target_arch = "aarch64", feature = "nightly_aarch64_neon")))]
    println!("NEON arch path NOT active on this build (expected only on aarch64 cross build)");

    let mut failures = Vec::new();
    let mut ran = 0usize;

    for f in fixtures() {
        ran += 1;
        match std::panic::catch_unwind(|| check_fixture(&f)) {
            Ok(Ok(())) => println!("OK   {}", f.name),
            Ok(Err(e)) => {
                println!("FAIL {} -- {}", f.name, e);
                failures.push((f.name, e));
            }
            Err(payload) => {
                let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = payload.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<non-string panic payload>".to_string()
                };
                println!("PANIC {} -- {}", f.name, msg);
                failures.push((f.name, format!("panic: {msg}")));
            }
        }
    }

    println!(
        "\n{}/{} fixtures OK, {} allocations served by guard-page allocator, no SIGSEGV from any out-of-bounds heap access",
        ran - failures.len(),
        ran,
        ALLOC_COUNT.load(Ordering::Relaxed)
    );

    if !failures.is_empty() {
        eprintln!("\nREPRODUCED - {} fixture(s) failed:", failures.len());
        for (name, e) in &failures {
            eprintln!("  {name}: {e}");
        }
        std::process::exit(1);
    } else {
        println!("did not reproduce - all fixtures decoded correctly under real aarch64 NEON execution with guard-page heap instrumentation");
    }
}
