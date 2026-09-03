use std::hint::black_box;

fn main() {
    let v0: u16 = black_box(0u16 as u16);
    let v1: u16 = black_box(1u16 as u16);
    let v2: u16 = black_box(65535u16 as u16);
    let v3: u16 = black_box(65534u16 as u16);
    let v4: u16 = black_box(32768u16 as u16);
    let v5: u16 = black_box(32767u16 as u16);
    let v6: u16 = black_box(32769u16 as u16);
    println!("u16->f32/0: {}", v0 as f32);
    println!("u16->f64/0: {}", v0 as f64);
    println!("u16->f32/1: {}", v1 as f32);
    println!("u16->f64/1: {}", v1 as f64);
    println!("u16->f32/2: {}", v2 as f32);
    println!("u16->f64/2: {}", v2 as f64);
    println!("u16->f32/3: {}", v3 as f32);
    println!("u16->f64/3: {}", v3 as f64);
    println!("u16->f32/4: {}", v4 as f32);
    println!("u16->f64/4: {}", v4 as f64);
    println!("u16->f32/5: {}", v5 as f32);
    println!("u16->f64/5: {}", v5 as f64);
    println!("u16->f32/6: {}", v6 as f32);
    println!("u16->f64/6: {}", v6 as f64);
}
