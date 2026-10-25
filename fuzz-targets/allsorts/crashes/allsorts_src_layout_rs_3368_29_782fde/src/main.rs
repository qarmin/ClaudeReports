use allsorts::layout::Coverage;
use allsorts::layout::SingleSubst;

fn main() {
    let single_subst = SingleSubst::Format2 { coverage: std::sync::Arc::new(Coverage::Format2 { coverage_range_array: vec![<allsorts::layout::CoverageRangeRecord as allsorts::binary::read::ReadFrom>::read_from((0u16, 1u16, u16::MAX))] }), substitute_glyph_array: vec![] };
    let _ = single_subst.apply_glyph(1u16);
}
