use csv::StringRecord;

fn main() {
    let string_record = StringRecord::with_capacity(1000000000000000000usize, 0usize);
    let _ = string_record.iter();
}
