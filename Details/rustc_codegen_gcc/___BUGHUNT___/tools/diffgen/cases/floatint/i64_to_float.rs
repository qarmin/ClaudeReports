use std::hint::black_box;

fn main() {
    let v0: i64 = black_box(0i64 as i64);
    let v1: i64 = black_box(1i64 as i64);
    let v2: i64 = black_box(-1i64 as i64);
    let v3: i64 = black_box(-9223372036854775808i64 as i64);
    let v4: i64 = black_box(-9223372036854775807i64 as i64);
    let v5: i64 = black_box(9223372036854775807i64 as i64);
    let v6: i64 = black_box(9223372036854775806i64 as i64);
    println!("i64->f32/0: {}", v0 as f32);
    println!("i64->f64/0: {}", v0 as f64);
    println!("i64->f32/1: {}", v1 as f32);
    println!("i64->f64/1: {}", v1 as f64);
    println!("i64->f32/2: {}", v2 as f32);
    println!("i64->f64/2: {}", v2 as f64);
    println!("i64->f32/3: {}", v3 as f32);
    println!("i64->f64/3: {}", v3 as f64);
    println!("i64->f32/4: {}", v4 as f32);
    println!("i64->f64/4: {}", v4 as f64);
    println!("i64->f32/5: {}", v5 as f32);
    println!("i64->f64/5: {}", v5 as f64);
    println!("i64->f32/6: {}", v6 as f32);
    println!("i64->f64/6: {}", v6 as f64);
}
