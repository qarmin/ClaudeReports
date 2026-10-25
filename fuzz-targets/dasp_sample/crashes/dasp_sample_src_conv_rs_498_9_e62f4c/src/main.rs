use dasp_sample::U48;

fn main() {
    let _ = dasp_sample::conv::u48::to_i64(U48::new_unchecked(i64::MIN));
}
