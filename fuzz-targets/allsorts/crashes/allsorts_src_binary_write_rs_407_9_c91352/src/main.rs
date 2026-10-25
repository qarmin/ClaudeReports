use allsorts::binary::write::WriteCounter;
use allsorts::tables::F2Dot14;

fn main() {
    let mut write_counter = WriteCounter::new();
    let _ = <allsorts::binary::write::WriteCounter as allsorts::binary::write::WriteContext>::write_zeros(&mut write_counter, usize::MAX);
    let _ = <allsorts::tables::F2Dot14 as allsorts::binary::write::WriteBinary>::write::<allsorts::binary::write::WriteCounter>(&mut write_counter, F2Dot14::from(0i16));
}
