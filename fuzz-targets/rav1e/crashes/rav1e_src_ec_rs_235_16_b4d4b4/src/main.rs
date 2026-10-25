use rav1e::bench::cdef::CDEF_VERY_LARGE;
use rav1e::bench::ec::WriterRecorder;

fn main() {
    let u16_v = CDEF_VERY_LARGE;
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::StorageBackend>::store(&mut writer_base, u16::MAX, u16_v, 1u16);
}
