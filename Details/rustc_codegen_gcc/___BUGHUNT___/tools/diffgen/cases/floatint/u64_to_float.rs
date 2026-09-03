use std::hint::black_box;

fn main() {
    let v0: u64 = black_box(0u64 as u64);
    let v1: u64 = black_box(1u64 as u64);
    let v2: u64 = black_box(18446744073709551615u64 as u64);
    let v3: u64 = black_box(18446744073709551614u64 as u64);
    let v4: u64 = black_box(9223372036854775808u64 as u64);
    let v5: u64 = black_box(9223372036854775807u64 as u64);
    let v6: u64 = black_box(9223372036854775809u64 as u64);
    println!("u64->f32/0: {}", v0 as f32);
    println!("u64->f64/0: {}", v0 as f64);
    println!("u64->f32/1: {}", v1 as f32);
    println!("u64->f64/1: {}", v1 as f64);
    println!("u64->f32/2: {}", v2 as f32);
    println!("u64->f64/2: {}", v2 as f64);
    println!("u64->f32/3: {}", v3 as f32);
    println!("u64->f64/3: {}", v3 as f64);
    println!("u64->f32/4: {}", v4 as f32);
    println!("u64->f64/4: {}", v4 as f64);
    println!("u64->f32/5: {}", v5 as f32);
    println!("u64->f64/5: {}", v5 as f64);
    println!("u64->f32/6: {}", v6 as f32);
    println!("u64->f64/6: {}", v6 as f64);
}
