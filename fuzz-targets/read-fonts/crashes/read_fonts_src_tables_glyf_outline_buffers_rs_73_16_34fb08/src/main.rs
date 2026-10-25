use read_fonts::tables::glyf::outline::BufferLengths;

fn main() {
    let buffer_lengths = BufferLengths { points: 1u32, contours: 1u32, unscaled: 0u32, deltas: 1u32, iup: u32::MAX, composite_deltas: 0u32 };
    let _ = buffer_lengths.packed_len::<read_fonts::tables::glyf::outline::ScaleF32>();
}
