use font_types::Fixed;
use read_fonts::tables::glyf::Transform;
use read_fonts::tables::glyf::outline::Unscaled;
use read_fonts::tables::gvar::GlyphDelta;

fn main() {
    let _ = <read_fonts::tables::glyf::outline::Unscaled as read_fonts::tables::glyf::outline::Scale>::component_offset(&Unscaled, <read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::TupleDelta>::new(0u16, 0i32, i32::MAX).apply_scalar::<i32>(Fixed::ONE), &Transform::default(), false, Some(GlyphDelta { position: 1u16, x_delta: 0i32, y_delta: 1i32 }.apply_scalar::<font_types::Fixed>(<font_types::Fixed as ::core::ops::Neg>::neg(Fixed::NEG_ONE))), false);
}
