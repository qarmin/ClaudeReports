use font_types::Fixed;
use read_fonts::tables::glyf::Transform;
use read_fonts::tables::glyf::outline::Scale26Dot6;
use read_fonts::tables::gvar::GlyphDelta;

fn main() {
    let _ = <read_fonts::tables::glyf::outline::Scale26Dot6 as read_fonts::tables::glyf::outline::Scale>::component_offset(&Scale26Dot6::new(None, 0u16), GlyphDelta { position: 1u16, x_delta: 65536i32, y_delta: 0i32 }.apply_scalar::<i32>(Fixed::MAX), &Transform::default(), false, Some(GlyphDelta { position: 1u16, x_delta: 1i32, y_delta: 0i32 }.apply_scalar::<font_types::Fixed>(Fixed::NEG_ONE)), false);
}
