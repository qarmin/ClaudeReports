use std::hint::black_box;

fn main() {
    let v0: usize = black_box(0usize as usize);
    let v1: usize = black_box(1usize as usize);
    let v2: usize = black_box(18446744073709551615usize as usize);
    let v3: usize = black_box(18446744073709551614usize as usize);
    let v4: usize = black_box(9223372036854775808usize as usize);
    let v5: usize = black_box(9223372036854775807usize as usize);
    let v6: usize = black_box(9223372036854775809usize as usize);
    println!("usize->f32/0: {}", v0 as f32);
    println!("usize->f64/0: {}", v0 as f64);
    println!("usize->f32/1: {}", v1 as f32);
    println!("usize->f64/1: {}", v1 as f64);
    println!("usize->f32/2: {}", v2 as f32);
    println!("usize->f64/2: {}", v2 as f64);
    println!("usize->f32/3: {}", v3 as f32);
    println!("usize->f64/3: {}", v3 as f64);
    println!("usize->f32/4: {}", v4 as f32);
    println!("usize->f64/4: {}", v4 as f64);
    println!("usize->f32/5: {}", v5 as f32);
    println!("usize->f64/5: {}", v5 as f64);
    println!("usize->f32/6: {}", v6 as f32);
    println!("usize->f64/6: {}", v6 as f64);
}
