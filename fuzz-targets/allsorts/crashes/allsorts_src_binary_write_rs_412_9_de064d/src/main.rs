use allsorts::binary::write::WriteCounter;
use allsorts::tables::cmap::owned::CmapSubtable;
use allsorts::tables::cmap::owned::CmapSubtableFormat12;

fn main() {
    let mut write_counter = WriteCounter::new();
    let _ = <allsorts::tables::cmap::owned::CmapSubtable as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteCounter>(&mut write_counter, CmapSubtable::Format12(CmapSubtableFormat12 { language: 0u32, groups: vec![] }));
    let _ = <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_zeros(&mut write_counter, usize::MAX);
}
