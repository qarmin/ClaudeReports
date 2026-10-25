use rawler::formats::bmff::BoxHeader;
use rawler::formats::bmff::stsc::StscBox;

fn main() {
    let stsc_box = StscBox { header: BoxHeader::default(), version: 0u8, flags: 0u32, entries: vec![] };
    let _ = stsc_box.get_entry_for_sample(1u32);
}
