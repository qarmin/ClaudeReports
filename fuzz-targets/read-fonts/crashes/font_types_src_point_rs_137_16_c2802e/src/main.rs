fn main() {
    let _ = <read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::TupleDelta>::new(0u16, 0i32, 1000000000i32).apply_scalar::<i32>(<font_types::Fixed as font_types::Scalar>::from_raw([0u8, 10u8, 0u8, 0u8]));
}
