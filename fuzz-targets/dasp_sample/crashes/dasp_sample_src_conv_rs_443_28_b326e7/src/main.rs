use dasp_sample::U24;

fn main() {
    let _ = dasp_sample::conv::u24::to_f64(U24::new_unchecked(i32::MIN));
}
