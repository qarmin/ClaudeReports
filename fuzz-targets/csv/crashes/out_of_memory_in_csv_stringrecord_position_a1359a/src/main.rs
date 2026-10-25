use csv::StringRecord;

fn main() {
    let string_record = StringRecord::with_capacity(0usize, 1000000000usize);
    let _ = string_record.position();
}
