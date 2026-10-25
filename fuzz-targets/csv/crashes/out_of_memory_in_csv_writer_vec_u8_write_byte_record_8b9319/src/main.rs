use csv::ByteRecord;

fn main() {
    let mut writer = <csv::Writer<Vec<u8>>>::from_writer(vec![]);
    let _ = writer.write_byte_record(&ByteRecord::with_capacity(4294967296usize, 0usize));
}
