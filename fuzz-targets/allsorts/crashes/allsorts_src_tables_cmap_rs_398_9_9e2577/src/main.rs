use allsorts::binary::write::WriteBuffer;
use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat4;

fn main() {
    let mut write_buffer = WriteBuffer::new();
    let _ = <allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteBuffer>(&mut write_buffer, CmapSubtable::Format4(CmapSubtableFormat4 { language: 0u16, end_codes: vec![], start_codes: vec![], id_deltas: vec![], id_range_offsets: vec![], glyph_id_array: vec![] }));
}
