use allsorts::layout::Coverage;

fn main() {
    let coverage = Coverage::Format2 { coverage_range_array: vec![<allsorts::layout::CoverageRangeRecord as allsorts::binary::read::ReadFrom>::read_from((1u16, 0u16, 0u16))] };
    let _ = coverage.glyph_count();
}
