use dasp_sample::U48;

fn main() {
    let u48 = U48::new_unchecked(i64::MIN);
    let _ = dasp_sample::conv::u48::to_i48(u48);
}
