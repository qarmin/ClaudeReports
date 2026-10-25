use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat12;
use allsorts::tag::LOCA;

fn main() {
    let u32_v = LOCA;
    let cmap_subtable = CmapSubtable::Format12(CmapSubtableFormat12 { language: 0u32, groups: vec![<allsorts::tables::cmap::SequentialMapGroup as allsorts::binary::read::ReadFrom>::read_from((0u32, u32_v, u32::MAX))] });
    let _ = cmap_subtable.map_glyph(u32_v);
}
