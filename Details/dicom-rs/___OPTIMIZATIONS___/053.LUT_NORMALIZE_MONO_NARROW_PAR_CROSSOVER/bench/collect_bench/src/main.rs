// Isolates exactly the `collect()` step of pixeldata's `mono_image_with_narrow`/`_par`
// (a 16-bit LUT map over a Vec<u16> source, collected into a fresh Vec<u16>), deliberately
// excluding the surrounding `ImageBuffer::from_raw` construction so callgrind's Ir count
// reflects only the map+collect codegen, not buffer validation/wrapping.
//
// Usage: collect_bench <seq|par> <n_elements>
use rayon::prelude::*;

fn build_lut() -> Vec<u16> {
    (0..=u16::MAX as u32).map(|i| (i.wrapping_mul(37) & 0xffff) as u16).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = &args[1];
    let n: usize = args[2].parse().unwrap();

    let lut = build_lut();
    let source: Vec<u16> = (0..n).map(|i| (i as u32 & 0xffff) as u16).collect();

    let data: Vec<u16> = match mode.as_str() {
        "seq" => source.iter().copied().map(|i| lut[i as usize]).collect(),
        "par" => source.par_iter().copied().map(|i| lut[i as usize]).collect(),
        _ => panic!("unknown mode"),
    };

    let checksum: u64 = data.iter().map(|&x| x as u64).sum();
    println!("{} n={} checksum={:#x}", mode, n, checksum);
}
