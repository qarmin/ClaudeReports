fn main() {
    let a: f64 = 4294967295.0;
    assert_eq!(a as u32, u32::MAX);
    let b: f64 = -1.0;
    assert_eq!(b as i32, -1i32);
    let c: f64 = 1.8446744073709552e19;
    assert_eq!(c as u64, 18446744073709551615u64);
    let d: f64 = -9223372036854775808.0;
    assert_eq!(d as i64, i64::MIN);
    let e: f64 = 1.8446744073709552e19;
    assert_eq!(e as usize, usize::MAX, "usize case");
    let f: f64 = -9223372036854775808.0;
    assert_eq!(f as isize, isize::MIN, "isize case");
    println!("OK");
}
