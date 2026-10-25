use png::Filter;

fn main() {
    let filter = Filter::Avg;
    png::benchable_apis::unfilter(filter, 0u8, &vec![], &mut vec![]);
}
