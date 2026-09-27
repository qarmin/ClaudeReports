use std::hint::black_box;

fn main() {
    { let v: u128 = black_box(0u128 as u128); let r = v as i8; println!("direct/u128->i8/0: {}", r); }
    { let v: u128 = black_box(0u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/0: {}", r); }
    { let v: u128 = black_box(1u128 as u128); let r = v as i8; println!("direct/u128->i8/1: {}", r); }
    { let v: u128 = black_box(1u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/1: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211455u128 as u128); let r = v as i8; println!("direct/u128->i8/2: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211455u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/2: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211454u128 as u128); let r = v as i8; println!("direct/u128->i8/3: {}", r); }
    { let v: u128 = black_box(340282366920938463463374607431768211454u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/3: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105728u128 as u128); let r = v as i8; println!("direct/u128->i8/4: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105728u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/4: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105727u128 as u128); let r = v as i8; println!("direct/u128->i8/5: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105727u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/5: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105729u128 as u128); let r = v as i8; println!("direct/u128->i8/6: {}", r); }
    { let v: u128 = black_box(170141183460469231731687303715884105729u128 as u128); let reinterpreted: i128 = unsafe { std::mem::transmute(v) }; let back: u128 = unsafe { std::mem::transmute(reinterpreted) }; let r = back as i8; println!("transmute-roundtrip/u128->i8/6: {}", r); }
}
