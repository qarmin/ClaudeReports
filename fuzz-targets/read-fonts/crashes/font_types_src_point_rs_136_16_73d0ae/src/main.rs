use read_fonts::tables::variations::NOT_COMPUTED;

fn main() {
    let _ = <read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::TupleDelta>::new(0u16, 1000000000i32, 0i32).apply_scalar::<i32>(NOT_COMPUTED);
}
