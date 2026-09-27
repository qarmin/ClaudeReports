use std::hint::black_box;

fn main() {
    { let v: i128 = black_box(0i128 as i128); let r = v as i32; println!("direct/i128->i32/0: {}", r); }
    { let v: i128 = black_box(0i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/0: {}", r); }
    { let v: i128 = black_box(1i128 as i128); let r = v as i32; println!("direct/i128->i32/1: {}", r); }
    { let v: i128 = black_box(1i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/1: {}", r); }
    { let v: i128 = black_box(-1i128 as i128); let r = v as i32; println!("direct/i128->i32/2: {}", r); }
    { let v: i128 = black_box(-1i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/2: {}", r); }
    { let v: i128 = black_box(-170141183460469231731687303715884105728i128 as i128); let r = v as i32; println!("direct/i128->i32/3: {}", r); }
    { let v: i128 = black_box(-170141183460469231731687303715884105728i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/3: {}", r); }
    { let v: i128 = black_box(-170141183460469231731687303715884105727i128 as i128); let r = v as i32; println!("direct/i128->i32/4: {}", r); }
    { let v: i128 = black_box(-170141183460469231731687303715884105727i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/4: {}", r); }
    { let v: i128 = black_box(170141183460469231731687303715884105727i128 as i128); let r = v as i32; println!("direct/i128->i32/5: {}", r); }
    { let v: i128 = black_box(170141183460469231731687303715884105727i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/5: {}", r); }
    { let v: i128 = black_box(170141183460469231731687303715884105726i128 as i128); let r = v as i32; println!("direct/i128->i32/6: {}", r); }
    { let v: i128 = black_box(170141183460469231731687303715884105726i128 as i128); let reinterpreted: u128 = unsafe { std::mem::transmute(v) }; let back: i128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i32; println!("transmute-roundtrip/i128->i32/6: {}", r); }
}
