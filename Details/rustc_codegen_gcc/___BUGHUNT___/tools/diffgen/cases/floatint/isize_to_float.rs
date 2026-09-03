use std::hint::black_box;

fn main() {
    let v0: isize = black_box(0isize as isize);
    let v1: isize = black_box(1isize as isize);
    let v2: isize = black_box(-1isize as isize);
    let v3: isize = black_box(-9223372036854775808isize as isize);
    let v4: isize = black_box(-9223372036854775807isize as isize);
    let v5: isize = black_box(9223372036854775807isize as isize);
    let v6: isize = black_box(9223372036854775806isize as isize);
    println!("isize->f32/0: {}", v0 as f32);
    println!("isize->f64/0: {}", v0 as f64);
    println!("isize->f32/1: {}", v1 as f32);
    println!("isize->f64/1: {}", v1 as f64);
    println!("isize->f32/2: {}", v2 as f32);
    println!("isize->f64/2: {}", v2 as f64);
    println!("isize->f32/3: {}", v3 as f32);
    println!("isize->f64/3: {}", v3 as f64);
    println!("isize->f32/4: {}", v4 as f32);
    println!("isize->f64/4: {}", v4 as f64);
    println!("isize->f32/5: {}", v5 as f32);
    println!("isize->f64/5: {}", v5 as f64);
    println!("isize->f32/6: {}", v6 as f32);
    println!("isize->f64/6: {}", v6 as f64);
}
