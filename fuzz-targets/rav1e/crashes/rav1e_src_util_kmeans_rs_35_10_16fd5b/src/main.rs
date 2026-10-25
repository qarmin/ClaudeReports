fn main() {
    let _ = rav1e::bench::util::kmeans::<i64, 4>(&[1i64, 9223372036854775806i64]);
}
