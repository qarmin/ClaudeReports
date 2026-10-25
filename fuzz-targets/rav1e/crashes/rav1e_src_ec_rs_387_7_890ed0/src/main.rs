use rav1e::bench::ec::WriterRecorder;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::count_signed_subexp_with_ref(&WriterRecorder::new(), 0i32, 0i32, 0i32, 0u8, i32::MIN);
}
