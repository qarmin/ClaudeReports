use read_fonts::tables::glyf::outline::BufferLengths;

fn main() {
    let buffer_lengths = BufferLengths { points: 0u32, contours: 0u32, unscaled: 0u32, deltas: u32::MAX, iup: 0u32, composite_deltas: 1u32 };
    let _ = buffer_lengths.packed_len::<read_fonts::tables::glyf::outline::Unscaled>();
}
