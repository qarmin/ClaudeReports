use std::hint::black_box;

fn main() {
    let v0: u32 = black_box(0u32 as u32);
    let v1: u32 = black_box(1u32 as u32);
    let v2: u32 = black_box(4294967295u32 as u32);
    let v3: u32 = black_box(4294967294u32 as u32);
    let v4: u32 = black_box(2147483648u32 as u32);
    let v5: u32 = black_box(2147483647u32 as u32);
    let v6: u32 = black_box(2147483649u32 as u32);
    println!("u32->f32/0: {}", v0 as f32);
    println!("u32->f64/0: {}", v0 as f64);
    println!("u32->f32/1: {}", v1 as f32);
    println!("u32->f64/1: {}", v1 as f64);
    println!("u32->f32/2: {}", v2 as f32);
    println!("u32->f64/2: {}", v2 as f64);
    println!("u32->f32/3: {}", v3 as f32);
    println!("u32->f64/3: {}", v3 as f64);
    println!("u32->f32/4: {}", v4 as f32);
    println!("u32->f64/4: {}", v4 as f64);
    println!("u32->f32/5: {}", v5 as f32);
    println!("u32->f64/5: {}", v5 as f64);
    println!("u32->f32/6: {}", v6 as f32);
    println!("u32->f64/6: {}", v6 as f64);
}
